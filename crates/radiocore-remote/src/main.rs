//! RadioCore Remote — Station Server for remote radio operation
//!
//! Runs at the radio site (Pi or PC next to transceiver).
//! Provides WebSocket API for remote control + Opus audio streaming.
//! Connects to radio via Hamlib (rigctld) for CAT control.

use anyhow::Result;
use radiocore_core::{audio, serial, config};
use tracing::{info, error};

mod hamlib;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RemoteConfig {
    pub callsign: String,
    pub audio: config::AudioConfig,
    pub web: config::WebConfig,
    pub network: config::NetworkConfig,
    pub hamlib: HamlibConfig,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct HamlibConfig {
    pub rigctld_host: String,
    pub rigctld_port: u16,
    pub rig_model: Option<String>,
}

impl Default for RemoteConfig {
    fn default() -> Self {
        RemoteConfig {
            callsign: "NOCALL".into(),
            audio: config::AudioConfig::default(),
            web: config::WebConfig { port: 8073, ..config::WebConfig::default() },
            network: config::NetworkConfig::default(),
            hamlib: HamlibConfig {
                rigctld_host: "127.0.0.1".into(),
                rigctld_port: 4532,
                rig_model: None,
            },
        }
    }
}

fn load_config() -> Result<RemoteConfig> {
    let config_path = std::env::var("RADIOCORE_CONFIG")
        .unwrap_or_else(|_| "config/remote.toml".into());

    if std::path::Path::new(&config_path).exists() {
        let content = std::fs::read_to_string(&config_path)?;
        Ok(toml::from_str(&content)?)
    } else {
        tracing::warn!("Config not found at {}, using defaults", config_path);
        Ok(RemoteConfig::default())
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

    info!("RadioCore Remote v{} starting...", env!("CARGO_PKG_VERSION"));

    let config = load_config()?;
    info!("Station: {}", config.callsign);

    // Connect to RadioCore board
    let board = serial::BoardConnection::new(
        config.audio.serial_port.as_deref()
    )?;

    // Initialize audio pipeline
    let pipeline = audio::Pipeline::new(&config.audio)?;
    info!("Audio pipeline ready");

    // Connect to rigctld for CAT control
    let rig = hamlib::RigConnection::new(&config.hamlib).await;
    match &rig {
        Ok(_) => info!("Hamlib connected ({}:{})", config.hamlib.rigctld_host, config.hamlib.rigctld_port),
        Err(e) => tracing::warn!("Hamlib not available: {} (running without CAT)", e),
    }

    // Start web server
    let web_config = config.clone();
    tokio::spawn(async move {
        if let Err(e) = serve_web(web_config).await {
            error!("Web server error: {}", e);
        }
    });

    info!("RadioCore Remote running — http://0.0.0.0:{}", config.web.port);
    info!("Press Ctrl+C to stop.");

    tokio::signal::ctrl_c().await?;
    info!("Shutting down...");
    Ok(())
}

async fn serve_web(config: RemoteConfig) -> Result<()> {
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
                    "mode": "remote",
                    "status": "running",
                    "freq_a": 145500000,
                    "mode_a": "FM",
                    "ptt": false,
                    "smeter_dbm": -73,
                }))
            }
        }))
        .route("/ws", get(radiocore_core::web::ws_status_handler));

    let addr = format!("{}:{}", config.web.bind, config.web.port);
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}
