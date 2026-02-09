//! Base configuration types shared across all RadioCore applications.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioConfig {
    pub device: String,
    pub sample_rate: u32,
    pub buffer_size: u32,
    pub rx_gain: f32,
    pub tx_gain: f32,
    pub serial_port: Option<String>,
}

impl Default for AudioConfig {
    fn default() -> Self {
        AudioConfig {
            device: "default".into(),
            sample_rate: 48000,
            buffer_size: 128,
            rx_gain: 0.0,
            tx_gain: 0.0,
            serial_port: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebConfig {
    pub port: u16,
    pub bind: String,
    pub auth_enabled: bool,
    pub admin_password: Option<String>,
}

impl Default for WebConfig {
    fn default() -> Self {
        WebConfig {
            port: 8080,
            bind: "0.0.0.0".into(),
            auth_enabled: false,
            admin_password: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkConfig {
    pub enabled: bool,
    pub opus_bitrate: u32,
}

impl Default for NetworkConfig {
    fn default() -> Self {
        NetworkConfig {
            enabled: false,
            opus_bitrate: 64000,
        }
    }
}
