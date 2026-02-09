//! Serial protocol for RadioCore board communication.
//!
//! Commands sent to ESP32-S3 over USB CDC:
//!   PTT ON / PTT OFF
//!   STATUS
//!   LEVEL
//!   SET RX_GAIN <0-24>
//!   SET TX_GAIN <-96 to 0>
//!   SET SAMPLE_RATE 48000
//!   INFO
//!   MODE REPEATER / DIGIMODE / REMOTE

use anyhow::Result;
use tracing::{info, warn};

pub struct BoardConnection {
    port: Option<Box<dyn serialport::SerialPort>>,
}

impl BoardConnection {
    pub fn new(port_name: Option<&str>) -> Result<Self> {
        let port = if let Some(name) = port_name {
            match serialport::new(name, 115200)
                .timeout(std::time::Duration::from_millis(100))
                .open()
            {
                Ok(p) => {
                    info!("Board connected on {}", name);
                    Some(p)
                }
                Err(e) => {
                    warn!("Could not open {}: {} (running without board)", name, e);
                    None
                }
            }
        } else {
            info!("No serial port configured, running without board");
            None
        };

        Ok(BoardConnection { port })
    }

    pub fn send_command(&mut self, cmd: &str) -> Result<Option<String>> {
        if let Some(ref mut port) = self.port {
            use std::io::{Write, BufRead, BufReader};
            write!(port, "{}\n", cmd)?;

            let mut reader = BufReader::new(port.try_clone()?);
            let mut response = String::new();
            reader.read_line(&mut response)?;
            Ok(Some(response.trim().to_string()))
        } else {
            Ok(None)
        }
    }

    pub fn ptt_on(&mut self) -> Result<()> {
        self.send_command("PTT ON")?;
        Ok(())
    }

    pub fn ptt_off(&mut self) -> Result<()> {
        self.send_command("PTT OFF")?;
        Ok(())
    }

    pub fn set_rx_gain(&mut self, db: i32) -> Result<()> {
        self.send_command(&format!("SET RX_GAIN {}", db))?;
        Ok(())
    }

    pub fn set_tx_gain(&mut self, db: i32) -> Result<()> {
        self.send_command(&format!("SET TX_GAIN {}", db))?;
        Ok(())
    }

    pub fn get_status(&mut self) -> Result<Option<String>> {
        self.send_command("STATUS")
    }

    pub fn is_connected(&self) -> bool {
        self.port.is_some()
    }
}
