//! RadioCore Digimode — USB Audio Interface for digital modes
//!
//! Manages the RadioCore board as a USB sound card + CAT interface.
//! Monitors audio levels, provides CAT pass-through, web status page.
//! Works with WSJT-X, fldigi, JS8Call, and any soundcard-based software.

use anyhow::Result;
use radiocore_core::{audio, serial, config};
use tracing::{info, error};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DigimodeConfig {
    pub callsign: String,
    pub audio: config::AudioConfig,
    pub web: config::WebConfig,
    pub cat: CatConfig,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CatConfig {
    pub enabled: bool,
    pub rig_port: Option<String>,
    pub baud_rate: u32,
    pub ptt_method: String, // "cat", "vox", "rts", "dtr"
}

impl Default for DigimodeConfig {
    fn default() -> Self {
        DigimodeConfig {
            callsign: "NOCALL".into(),
            audio: config::AudioConfig::default(),
            web: config::WebConfig { port: 8074, ..config::WebConfig::default() },
            cat: CatConfig {
                enabled: true,
                rig_port: None,
                baud_rate: 9600,
                ptt_method: "cat".into(),
            },
        }
    }
}

fn load_config() -> Result<DigimodeConfig> {
    let config_path = std::env::var("RADIOCORE_CONFIG")
        .unwrap_or_else(|_| "config/digimode.toml".into());

    if std::path::Path::new(&config_path).exists() {
        let content = std::fs::read_to_string(&config_path)?;
        Ok(toml::from_str(&content)?)
    } else {
        tracing::warn!("Config not found at {}, using defaults", config_path);
        Ok(DigimodeConfig::default())
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

    info!("RadioCore Digimode v{} starting...", env!("CARGO_PKG_VERSION"));

    let config = load_config()?;
    info!("Station: {}", config.callsign);

    // Connect to RadioCore board
    let board = serial::BoardConnection::new(
        config.audio.serial_port.as_deref()
    )?;

    // Initialize audio (for monitoring levels only — PC talks directly to USB audio)
    let pipeline = audio::Pipeline::new(&config.audio)?;
    info!("Audio monitor ready");

    // Start web server (status + waterfall)
    let web_config = config.clone();
    tokio::spawn(async move {
        if let Err(e) = serve_web(web_config).await {
            error!("Web server error: {}", e);
        }
    });

    info!("RadioCore Digimode running — http://0.0.0.0:{}", config.web.port);
    info!("Connect WSJT-X/fldigi to 'RadioCore' audio device.");
    info!("Press Ctrl+C to stop.");

    tokio::signal::ctrl_c().await?;
    info!("Shutting down...");
    Ok(())
}

async fn serve_web(config: DigimodeConfig) -> Result<()> {
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
                    "mode": "digimode",
                    "status": "running",
                    "ptt": false,
                    "ptt_method": s.cat.ptt_method,
                    "audio_device": s.audio.device,
                }))
            }
        }))
        .route("/ws", get(radiocore_core::web::ws_status_handler));

    let addr = format!("{}:{}", config.web.bind, config.web.port);
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}
