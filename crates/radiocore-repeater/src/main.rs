//! RadioCore Repeater — Open-Source Repeater Controller
//!
//! Duplex repeater: RX radio -> audio pass-through -> TX radio
//! Features: DTMF commands, Lua logic, TTS announcements, Web-UI
//!
//! Can run standalone on ESP32-S3 (basic) or on Pi (full features).

mod logic;
mod tts;

use anyhow::Result;
use radiocore_core::{audio, serial, config};
use tracing::{info, error};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RepeaterConfig {
    pub callsign: String,
    pub location: Option<String>,
    pub audio: config::AudioConfig,
    pub repeater: RepeaterSettings,
    pub web: config::WebConfig,
    pub network: config::NetworkConfig,
    pub services: ServicesConfig,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RepeaterSettings {
    pub timeout_seconds: u32,
    pub tail_seconds: u32,
    pub ctcss_tone: Option<f32>,
    pub cw_id_interval: u32,
    pub cw_id_text: String,
    pub roger_beep: bool,
    pub logic_script: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ServicesConfig {
    pub tts_enabled: bool,
    pub tts_engine: String,
    pub weather_enabled: bool,
    pub weather_station: Option<String>,
    pub aprs_enabled: bool,
}

impl Default for RepeaterConfig {
    fn default() -> Self {
        RepeaterConfig {
            callsign: "NOCALL".into(),
            location: None,
            audio: config::AudioConfig::default(),
            repeater: RepeaterSettings {
                timeout_seconds: 180,
                tail_seconds: 3,
                ctcss_tone: None,
                cw_id_interval: 600,
                cw_id_text: "NOCALL".into(),
                roger_beep: true,
                logic_script: "config/scripts/repeater.lua".into(),
            },
            web: config::WebConfig::default(),
            network: config::NetworkConfig::default(),
            services: ServicesConfig {
                tts_enabled: false,
                tts_engine: "piper".into(),
                weather_enabled: false,
                weather_station: None,
                aprs_enabled: false,
            },
        }
    }
}

fn load_config() -> Result<RepeaterConfig> {
    let config_path = std::env::var("RADIOCORE_CONFIG")
        .unwrap_or_else(|_| "config/local.toml".into());

    if std::path::Path::new(&config_path).exists() {
        let content = std::fs::read_to_string(&config_path)?;
        Ok(toml::from_str(&content)?)
    } else {
        tracing::warn!("Config not found at {}, using defaults", config_path);
        Ok(RepeaterConfig::default())
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "radiocore=info".into()),
        )
        .init();

    info!("RadioCore Repeater v{} starting...", env!("CARGO_PKG_VERSION"));

    let config = load_config()?;
    info!("Callsign: {}", config.callsign);

    // Connect to RadioCore board (if available)
    let mut board = serial::BoardConnection::new(
        config.audio.serial_port.as_deref()
    )?;

    // Initialize audio pipeline
    let pipeline = audio::Pipeline::new(&config.audio)?;
    info!("Audio pipeline ready");

    // Initialize Lua logic engine
    let logic = logic::Engine::new(&config)?;
    info!("Lua logic engine ready");

    // Start web server
    let web_config = config.clone();
    tokio::spawn(async move {
        if let Err(e) = serve_web(web_config).await {
            error!("Web server error: {}", e);
        }
    });

    info!("RadioCore Repeater running — http://0.0.0.0:{}", config.web.port);
    info!("Press Ctrl+C to stop.");

    tokio::signal::ctrl_c().await?;
    info!("Shutting down...");

    Ok(())
}

async fn serve_web(config: RepeaterConfig) -> Result<()> {
    use axum::{routing::get, response::{Html, Json}, Router};
    use std::sync::Arc;

    let state = Arc::new(config.clone());

    let app = Router::new()
        .route("/", get(|| async { Html(include_str!("../../static/index.html")) }))
        .route("/api/status", get({
            let s = state.clone();
            move || async move {
                Json(serde_json::json!({
                    "version": env!("CARGO_PKG_VERSION"),
                    "callsign": s.callsign,
                    "mode": "repeater",
                    "status": "running",
                    "ptt": false,
                    "squelch": false,
                }))
            }
        }))
        .route("/ws", get(radiocore_core::web::ws_status_handler));

    let addr = format!("{}:{}", config.web.bind, config.web.port);
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}
