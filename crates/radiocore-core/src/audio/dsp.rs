//! DSP processing: noise gate, tone detection (Goertzel), DTMF decoder.
//!
//! These are shared building blocks used by:
//! - Repeater: DTMF commands, CTCSS detection, noise gate
//! - Digimode: VOX detection
//! - Remote: VOX PTT

use std::f32::consts::PI;

/// Simple noise gate with attack/release
pub struct NoiseGate {
    threshold: f32,
    release_samples: usize,
    counter: usize,
    open: bool,
}

impl NoiseGate {
    pub fn new(threshold_db: f32, sample_rate: u32) -> Self {
        let threshold = 10f32.powf(threshold_db / 20.0);
        NoiseGate {
            threshold,
            release_samples: (sample_rate as f32 * 0.050) as usize,
            counter: 0,
            open: false,
        }
    }

    pub fn process(&mut self, sample: f32) -> f32 {
        if sample.abs() > self.threshold {
            self.counter = self.release_samples;
            self.open = true;
        } else if self.counter > 0 {
            self.counter -= 1;
        } else {
            self.open = false;
        }
        if self.open { sample } else { 0.0 }
    }

    pub fn is_open(&self) -> bool {
        self.open
    }
}

/// Goertzel algorithm for single-frequency detection (DTMF, CTCSS)
pub struct GoertzelDetector {
    coeff: f32,
    s1: f32,
    s2: f32,
    count: usize,
    block_size: usize,
    threshold: f32,
}

impl GoertzelDetector {
    pub fn new(target_freq: f32, sample_rate: u32, block_size: usize) -> Self {
        let k = (0.5 + (block_size as f32 * target_freq / sample_rate as f32)) as usize;
        let coeff = 2.0 * (2.0 * PI * k as f32 / block_size as f32).cos();
        GoertzelDetector {
            coeff,
            s1: 0.0,
            s2: 0.0,
            count: 0,
            block_size,
            threshold: 0.1,
        }
    }

    pub fn process(&mut self, sample: f32) -> Option<f32> {
        let s0 = sample + self.coeff * self.s1 - self.s2;
        self.s2 = self.s1;
        self.s1 = s0;
        self.count += 1;

        if self.count >= self.block_size {
            let power = self.s1 * self.s1 + self.s2 * self.s2 - self.coeff * self.s1 * self.s2;
            self.s1 = 0.0;
            self.s2 = 0.0;
            self.count = 0;
            Some(power / self.block_size as f32)
        } else {
            None
        }
    }

    pub fn is_detected(&self, power: f32) -> bool {
        power > self.threshold
    }
}

/// DTMF decoder using dual Goertzel detectors
pub struct DtmfDecoder {
    low: [GoertzelDetector; 4],
    high: [GoertzelDetector; 4],
}

impl DtmfDecoder {
    pub fn new(sample_rate: u32) -> Self {
        let block = (sample_rate as f32 * 0.040) as usize;
        let low_freqs = [697.0, 770.0, 852.0, 941.0];
        let high_freqs = [1209.0, 1336.0, 1477.0, 1633.0];

        DtmfDecoder {
            low: low_freqs.map(|f| GoertzelDetector::new(f, sample_rate, block)),
            high: high_freqs.map(|f| GoertzelDetector::new(f, sample_rate, block)),
        }
    }

    pub fn process(&mut self, sample: f32) -> Option<char> {
        let mut low_powers = [0.0f32; 4];
        let mut high_powers = [0.0f32; 4];
        let mut complete = false;

        for (i, det) in self.low.iter_mut().enumerate() {
            if let Some(p) = det.process(sample) {
                low_powers[i] = p;
                complete = true;
            }
        }
        for (i, det) in self.high.iter_mut().enumerate() {
            if let Some(p) = det.process(sample) {
                high_powers[i] = p;
            }
        }

        if !complete {
            return None;
        }

        let low_max = low_powers.iter().enumerate()
            .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
            .map(|(i, &p)| (i, p));
        let high_max = high_powers.iter().enumerate()
            .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
            .map(|(i, &p)| (i, p));

        if let (Some((li, lp)), Some((hi, hp))) = (low_max, high_max) {
            if lp > 0.1 && hp > 0.1 {
                let map = [
                    ['1', '2', '3', 'A'],
                    ['4', '5', '6', 'B'],
                    ['7', '8', '9', 'C'],
                    ['*', '0', '#', 'D'],
                ];
                return Some(map[li][hi]);
            }
        }

        None
    }
}
