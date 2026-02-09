//! DTMF command buffer — collects digits, '#' terminates command.

use std::time::{Duration, Instant};

pub struct DtmfBuffer {
    digits: String,
    last_digit_time: Option<Instant>,
    timeout: Duration,
}

impl DtmfBuffer {
    pub fn new() -> Self {
        DtmfBuffer {
            digits: String::new(),
            last_digit_time: None,
            timeout: Duration::from_secs(3),
        }
    }

    pub fn push(&mut self, digit: char) -> Option<String> {
        self.last_digit_time = Some(Instant::now());

        if digit == '#' && !self.digits.is_empty() {
            let cmd = format!("{}#", self.digits);
            self.digits.clear();
            return Some(cmd);
        }

        self.digits.push(digit);
        None
    }

    pub fn tick(&mut self) {
        if let Some(last) = self.last_digit_time {
            if last.elapsed() > self.timeout && !self.digits.is_empty() {
                self.digits.clear();
                self.last_digit_time = None;
            }
        }
    }
}
