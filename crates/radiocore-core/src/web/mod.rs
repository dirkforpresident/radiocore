//! Shared web server base — axum router, WebSocket handler, static files.
//!
//! Each application adds its own routes on top of this base.

use axum::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    response::IntoResponse,
};
use serde_json::json;
use tracing::info;

/// WebSocket handler that sends periodic status updates
pub async fn ws_status_handler(ws: WebSocketUpgrade) -> impl IntoResponse {
    ws.on_upgrade(handle_status_ws)
}

async fn handle_status_ws(mut socket: WebSocket) {
    info!("WebSocket client connected");

    loop {
        let status = json!({
            "type": "status",
            "ptt": false,
            "squelch": false,
            "rx_level": -60.0,
            "tx_level": -80.0,
        });

        if socket.send(Message::Text(status.to_string())).await.is_err() {
            break;
        }

        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    }

    info!("WebSocket client disconnected");
}
