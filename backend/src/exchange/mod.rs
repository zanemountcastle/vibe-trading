use uuid::Uuid;
use serde::{Serialize, Deserialize};
use async_trait::async_trait;

use crate::order::Order;
use crate::order::OrderStatus as OrderStatusEnum;

pub mod crypto;
// Comment out missing modules
// pub mod stock;
// pub mod forex;
// pub mod bond;

// Interface for all exchange implementations
#[async_trait]
#[allow(dead_code)]
pub trait Exchange: Send + Sync {
    fn name(&self) -> &str;
    fn exchange_type(&self) -> ExchangeType;
    fn is_connected(&self) -> bool;
    
    async fn connect(&mut self) -> Result<(), String>;
    async fn disconnect(&mut self) -> Result<(), String>;
    
    async fn get_supported_assets(&self) -> Result<Vec<String>, String>;
    async fn get_market_data(&self, symbol: &str) -> Result<MarketSnapshot, String>;
    
    async fn submit_order(&self, order: Order) -> Result<(), String>;
    async fn cancel_order(&self, order_id: Uuid) -> Result<(), String>;
    async fn get_order_status(&self, order_id: Uuid) -> Result<OrderStatusResponse, String>;
    
    async fn get_account_balance(&self) -> Result<AccountBalance, String>;
    async fn get_positions(&self) -> Result<Vec<Position>, String>;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ExchangeType {
    Stock,
    Crypto,
    Forex,
    Bond,
    Commodity,
    Option,
    Future,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketSnapshot {
    pub symbol: String,
    pub price: f64,
    pub bid: f64,
    pub ask: f64,
    pub bid_size: f64,
    pub ask_size: f64,
    pub volume: f64,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderStatusResponse {
    pub order_id: Uuid,
    pub exchange_order_id: Option<String>,
    pub status: OrderStatus,
    pub filled_quantity: f64,
    pub remaining_quantity: f64,
    pub average_price: Option<f64>,
    pub last_update: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum OrderStatus {
    Pending,
    Open,
    PartiallyFilled,
    Filled,
    Cancelled,
    Rejected,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountBalance {
    pub total: f64,
    pub available: f64,
    pub currency: String,
    pub additional_balances: Vec<(String, f64)>,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Position {
    pub symbol: String,
    pub quantity: f64,
    pub avg_price: f64,
    pub current_price: f64,
    pub unrealized_pnl: f64,
    pub realized_pnl: f64,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

#[allow(dead_code)]
pub struct ExchangeFactory;

#[allow(dead_code)]
impl ExchangeFactory {
    // Return CryptoExchange directly instead of Box<dyn Exchange>
    pub fn create_crypto_exchange(config: ExchangeConfig) -> Result<crypto::CryptoExchange, String> {
        Ok(crypto::CryptoExchange::new(config))
    }
    
    // Add other methods for different exchange types as needed
    // pub fn create_stock_exchange(...) 
    // pub fn create_forex_exchange(...) 
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExchangeConfig {
    pub name: String,
    pub exchange_type: ExchangeType,
    pub api_url: String,
    pub api_key: Option<String>,
    pub api_secret: Option<String>,
    pub additional_params: std::collections::HashMap<String, String>,
} 