//! Audio pipeline: RX input -> ring buffer -> TX output with lock-free buffers.
//!
//! Used by all RadioCore applications:
//! - Repeater: RX radio -> buffer -> TX radio (local pass-through)
//! - Digimode: USB audio from PC <-> radio audio
//! - Remote: local audio <-> Opus network stream

use anyhow::Result;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{Device, StreamConfig};
use ringbuf::HeapRb;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use tracing::{info, warn, error};

use crate::config::AudioConfig;

pub struct Pipeline {
    rx_device: Option<Device>,
    tx_device: Option<Device>,
    sample_rate: u32,
    buffer_size: u32,
    ptt_active: Arc<AtomicBool>,
    squelch_open: Arc<AtomicBool>,
}

impl Pipeline {
    pub fn new(config: &AudioConfig) -> Result<Self> {
        let host = cpal::default_host();

        info!("Available audio devices:");
        for device in host.devices()? {
            if let Ok(name) = device.name() {
                info!("  - {}", name);
            }
        }

        let device = if config.device == "default" {
            host.default_input_device()
        } else {
            host.devices()?.find(|d| {
                d.name().map(|n| n.contains(&config.device)).unwrap_or(false)
            })
        };

        if let Some(ref dev) = device {
            info!("Using audio device: {}", dev.name().unwrap_or_default());
        } else {
            warn!("No audio device found, running in dummy mode");
        }

        Ok(Pipeline {
            rx_device: device.clone(),
            tx_device: device,
            sample_rate: config.sample_rate,
            buffer_size: config.buffer_size,
            ptt_active: Arc::new(AtomicBool::new(false)),
            squelch_open: Arc::new(AtomicBool::new(false)),
        })
    }

    pub fn ptt(&self) -> &Arc<AtomicBool> {
        &self.ptt_active
    }

    pub fn squelch(&self) -> &Arc<AtomicBool> {
        &self.squelch_open
    }

    pub fn set_ptt(&self, active: bool) {
        self.ptt_active.store(active, Ordering::Relaxed);
    }

    pub fn is_squelch_open(&self) -> bool {
        self.squelch_open.load(Ordering::Relaxed)
    }

    /// Start the audio pipeline with RX -> ring buffer -> TX
    pub fn start(&self) -> Result<()> {
        let rb = HeapRb::<f32>::new(self.buffer_size as usize * 4);
        let (mut producer, mut consumer) = rb.split();

        // RX input stream
        if let Some(ref device) = self.rx_device {
            let config = StreamConfig {
                channels: 1,
                sample_rate: cpal::SampleRate(self.sample_rate),
                buffer_size: cpal::BufferSize::Fixed(self.buffer_size),
            };

            let rx_stream = device.build_input_stream(
                &config,
                move |data: &[f32], _| {
                    for &sample in data {
                        let _ = producer.push(sample);
                    }
                },
                |err| error!("RX stream error: {}", err),
                None,
            )?;

            rx_stream.play()?;
            info!("RX audio stream started");
            std::mem::forget(rx_stream);
        }

        // TX output stream (only outputs when PTT is active)
        if let Some(ref device) = self.tx_device {
            let ptt = self.ptt_active.clone();
            let config = StreamConfig {
                channels: 1,
                sample_rate: cpal::SampleRate(self.sample_rate),
                buffer_size: cpal::BufferSize::Fixed(self.buffer_size),
            };

            let tx_stream = device.build_output_stream(
                &config,
                move |data: &mut [f32], _| {
                    if ptt.load(Ordering::Relaxed) {
                        for sample in data.iter_mut() {
                            *sample = consumer.pop().unwrap_or(0.0);
                        }
                    } else {
                        for sample in data.iter_mut() {
                            *sample = 0.0;
                        }
                        while consumer.pop().is_some() {}
                    }
                },
                |err| error!("TX stream error: {}", err),
                None,
            )?;

            tx_stream.play()?;
            info!("TX audio stream started");
            std::mem::forget(tx_stream);
        }

        Ok(())
    }
}
