use actix_web::{web, Error, HttpRequest, HttpResponse};
use serde::{Deserialize, Serialize};
use tracing::{debug, info};

use crate::api::AppState;

/// WebSocket message types for client-server communication
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type", content = "payload")]
pub enum WsMessage {
    /// Server heartbeat
    Heartbeat,
    /// Initial connection message
    Connect {
        client_id: String,
    },
    /// Subscribe to a data feed
    Subscribe {
        feed: String,
        symbols: Option<Vec<String>>,
    },
    /// Unsubscribe from a data feed
    Unsubscribe {
        feed: String,
        symbols: Option<Vec<String>>,
    },
    /// Market data updates
    MarketData {
        symbol: String,
        price: f64,
        bid: f64,
        ask: f64,
        volume: f64,
        timestamp: String,
    },
    /// Order updates
    OrderUpdate {
        order_id: String,
        status: String,
        filled_quantity: f64,
        average_price: Option<f64>,
        timestamp: String,
    },
    /// Strategy evaluations
    StrategyUpdate {
        strategy: String,
        confidence: f64,
        expected_profit: f64,
        timestamp: String,
    },
    /// General notification
    Notification {
        level: String, // "info", "warning", "error"
        message: String,
        timestamp: String,
    },
    /// Error message
    Error {
        code: String,
        message: String,
    },
}

/// Placeholder WebSocket route that returns a message for simulation mode
#[allow(dead_code)]
pub async fn websocket_route(req: HttpRequest, _stream: web::Payload) -> Result<HttpResponse, Error> {
    info!("WebSocket connection attempt from {:?}", req.peer_addr());
    
    // In simulation mode, just return a message indicating WebSocket is not supported
    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(r#"{"status":"error","message":"WebSocket not implemented in simulation mode"}"#))
}

/// WebSocket index handler - also a placeholder for simulation mode
pub async fn ws_index(
    _req: HttpRequest, 
    _stream: web::Payload,
    _data: web::Data<AppState>
) -> Result<HttpResponse, Error> {
    debug!("WebSocket connection attempt at /ws");
    
    // In simulation mode, just return a message
    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(r#"{"status":"error","message":"WebSocket not implemented in simulation mode"}"#))
} 