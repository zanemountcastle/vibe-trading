use std::sync::Arc;
use actix_web::{web, App, HttpServer, HttpResponse};
use actix_web::middleware::Logger;
use serde::Serialize;
use tokio::sync::RwLock;
use tracing::info;

use crate::strategy::StrategyManager;
use crate::market_data::MarketDataManager;
use crate::order::OrderManager;

mod handlers;
mod websocket;
// Comment out missing modules
// mod routes;
// mod auth;

#[derive(Clone)]
pub struct AppState {
    pub strategy_manager: Arc<RwLock<StrategyManager>>,
    pub market_data_manager: Arc<RwLock<MarketDataManager>>,
    pub order_manager: Arc<RwLock<OrderManager>>,
}

pub async fn start_api_server(
    strategy_manager: Arc<RwLock<StrategyManager>>,
    market_data_manager: Arc<RwLock<MarketDataManager>>,
    order_manager: Arc<RwLock<OrderManager>>,
    host: &str,
    port: u16,
) -> std::io::Result<()> {
    let app_state = AppState {
        strategy_manager,
        market_data_manager,
        order_manager,
    };
    
    info!("Starting API server on {}:{}", host, port);
    
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(app_state.clone()))
            .wrap(Logger::default())
            .service(
                web::scope("/api")
                    // Health check
                    .route("/health", web::get().to(handlers::health_check))
                    
                    // Market data routes
                    .service(
                        web::scope("/market")
                            .route("/data/{symbol}", web::get().to(handlers::get_market_data))
                            .route("/symbols", web::get().to(handlers::get_symbols))
                    )
                    
                    // Strategy routes
                    .service(
                        web::scope("/strategy")
                            .route("", web::get().to(handlers::get_strategies))
                            .route("/active", web::get().to(handlers::get_active_strategy))
                            .route("/active", web::put().to(handlers::set_active_strategy))
                            .route("/{name}/params", web::get().to(handlers::get_strategy_params))
                            .route("/{name}/params", web::put().to(handlers::update_strategy_params))
                            .route("/evaluate", web::post().to(handlers::evaluate_strategies))
                    )
                    
                    // Order routes
                    .service(
                        web::scope("/order")
                            .route("", web::post().to(handlers::place_order))
                            .route("", web::get().to(handlers::get_orders))
                            .route("/{id}", web::get().to(handlers::get_order))
                            .route("/{id}/cancel", web::post().to(handlers::cancel_order))
                    )
                    
                    // Account routes
                    .service(
                        web::scope("/account")
                            .route("/balance", web::get().to(handlers::get_account_balance))
                            .route("/positions", web::get().to(handlers::get_positions))
                    )
                    
                    // Backtest routes
                    .service(
                        web::scope("/backtest")
                            .route("", web::post().to(handlers::run_backtest))
                            .route("/{id}", web::get().to(handlers::get_backtest_result))
                    )
            )
            // WebSocket for real-time updates
            .route("/ws", web::get().to(websocket::ws_index))
    })
    .bind((host, port))?
    .run()
    .await
}

// Default error response format
#[derive(Serialize)]
pub struct ErrorResponse {
    pub error: String,
}

// Standard success response
#[derive(Serialize)]
pub struct SuccessResponse<T> {
    pub data: T,
}

// Helper function to create a standard error response
pub fn error_response(message: &str) -> HttpResponse {
    HttpResponse::BadRequest().json(ErrorResponse {
        error: message.to_string(),
    })
}

// Helper function to create a standard success response
pub fn success_response<T: Serialize>(data: T) -> HttpResponse {
    HttpResponse::Ok().json(SuccessResponse { data })
} 