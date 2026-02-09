//! Text-to-Speech engine — Piper TTS for natural German voice.
//!
//! On Pi: live TTS generation via piper command
//! On ESP32: pre-recorded WAV files from flash

use anyhow::Result;
use tracing::{info, warn};

pub struct TtsEngine {
    engine: String,
    model: String,
}

impl TtsEngine {
    pub fn new(engine: &str) -> Self {
        TtsEngine {
            engine: engine.to_string(),
            model: "de_DE-thorsten-high".to_string(),
        }
    }

    /// Generate speech audio from text (returns PCM samples at 22050 Hz)
    pub fn speak(&self, text: &str) -> Result<Vec<f32>> {
        info!("[TTS] Generating: {}", text);

        match self.engine.as_str() {
            "piper" => self.speak_piper(text),
            "wav" => self.speak_wav(text),
            _ => {
                warn!("Unknown TTS engine: {}", self.engine);
                Ok(vec![])
            }
        }
    }

    fn speak_piper(&self, text: &str) -> Result<Vec<f32>> {
        use std::process::Command;

        let output = Command::new("piper")
            .args(["--model", &self.model, "--output_raw"])
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::null())
            .spawn();

        match output {
            Ok(mut child) => {
                use std::io::Write;
                if let Some(ref mut stdin) = child.stdin {
                    stdin.write_all(text.as_bytes())?;
                }
                let output = child.wait_with_output()?;
                // Piper outputs 16-bit PCM at 22050 Hz
                let samples: Vec<f32> = output.stdout
                    .chunks_exact(2)
                    .map(|c| i16::from_le_bytes([c[0], c[1]]) as f32 / 32768.0)
                    .collect();
                info!("[TTS] Generated {} samples", samples.len());
                Ok(samples)
            }
            Err(_) => {
                warn!("[TTS] piper not found, TTS disabled");
                Ok(vec![])
            }
        }
    }

    fn speak_wav(&self, text: &str) -> Result<Vec<f32>> {
        // Look for pre-recorded WAV file matching the text hash
        let hash = text.len(); // Simple hash for demo
        let path = format!("config/audio/{:x}.wav", hash);
        if std::path::Path::new(&path).exists() {
            info!("[TTS] Playing pre-recorded: {}", path);
            // TODO: Load and decode WAV
        } else {
            warn!("[TTS] No WAV found for: {}", text);
        }
        Ok(vec![])
    }
}
