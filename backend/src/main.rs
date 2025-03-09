use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

mod api;
mod exchange;
mod market_data;
mod order;
mod strategy;
// Comment out missing modules
// mod config; 
// mod trade;
// mod risk;
// mod models;
// mod utils;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber)
        .expect("Failed to set subscriber");
    
    info!("Starting ARB trading platform");
    
    // Create the application state
    let strategy_manager = Arc::new(RwLock::new(strategy::StrategyManager::new()));
    let market_data_manager = Arc::new(RwLock::new(market_data::MarketDataManager::new()));
    let order_manager = Arc::new(RwLock::new(order::OrderManager::new()));
    
    // In simulation mode, start the API server directly
    info!("Starting API server in simulation mode");
    api::start_api_server(
        strategy_manager,
        market_data_manager,
        order_manager,
        "0.0.0.0",
        8000,
    ).await?;
    
    Ok(())
} 