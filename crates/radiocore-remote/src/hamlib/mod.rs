//! Hamlib integration — connects to rigctld for CAT control.
//!
//! Supports: frequency, mode, PTT, S-meter, power, SWR.
//! Protocol: rigctld TCP text protocol (default port 4532).

use anyhow::Result;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpStream;
use tracing::{info, warn};

use crate::HamlibConfig;

pub struct RigConnection {
    stream: TcpStream,
}

impl RigConnection {
    pub async fn new(config: &HamlibConfig) -> Result<Self> {
        let addr = format!("{}:{}", config.rigctld_host, config.rigctld_port);
        let stream = TcpStream::connect(&addr).await?;
        info!("Connected to rigctld at {}", addr);
        Ok(RigConnection { stream })
    }

    async fn command(&mut self, cmd: &str) -> Result<String> {
        self.stream.write_all(format!("{}\n", cmd).as_bytes()).await?;
        let mut reader = BufReader::new(&mut self.stream);
        let mut response = String::new();
        reader.read_line(&mut response).await?;
        Ok(response.trim().to_string())
    }

    pub async fn get_freq(&mut self) -> Result<u64> {
        let resp = self.command("f").await?;
        Ok(resp.parse().unwrap_or(0))
    }

    pub async fn set_freq(&mut self, hz: u64) -> Result<()> {
        self.command(&format!("F {}", hz)).await?;
        Ok(())
    }

    pub async fn get_mode(&mut self) -> Result<String> {
        let resp = self.command("m").await?;
        Ok(resp.split_whitespace().next().unwrap_or("FM").to_string())
    }

    pub async fn set_mode(&mut self, mode: &str, passband: u32) -> Result<()> {
        self.command(&format!("M {} {}", mode, passband)).await?;
        Ok(())
    }

    pub async fn set_ptt(&mut self, active: bool) -> Result<()> {
        self.command(&format!("T {}", if active { 1 } else { 0 })).await?;
        Ok(())
    }

    pub async fn get_smeter(&mut self) -> Result<i32> {
        let resp = self.command("l STRENGTH").await?;
        Ok(resp.parse().unwrap_or(-60))
    }
}
