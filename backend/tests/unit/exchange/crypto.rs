use arb_platform::exchange::{
    ExchangeType, ExchangeConfig, Exchange, OrderStatus, MarketSnapshot
};
use arb_platform::exchange::crypto::CryptoExchange;
use arb_platform::order::{Order, OrderType, OrderStatus as OrderOrderStatus};
use arb_platform::strategy::{TradeDirection, TimeInForce};

use chrono::Utc;
use std::collections::HashMap;
use uuid::Uuid;
use std::sync::Arc;
use tokio::test;

fn create_test_config() -> ExchangeConfig {
    ExchangeConfig {
        name: "Test Crypto Exchange".to_string(),
        exchange_type: ExchangeType::Crypto,
        api_url: "https://api.example.com".to_string(),
        api_key: Some("test_key".to_string()),
        api_secret: Some("test_secret".to_string()),
        additional_params: HashMap::new(),
    }
}

fn create_test_order() -> Order {
    Order {
        id: Uuid::new_v4(),
        client_order_id: "test_client_id".to_string(),
        symbol: "BTC/USD".to_string(),
        direction: TradeDirection::Buy,
        order_type: OrderType::Limit,
        quantity: 1.0,
        filled_quantity: 0.0,
        price: Some(35000.0),
        stop_price: None,
        time_in_force: TimeInForce::GoodTilCancelled,
        status: OrderOrderStatus::Created,
        exchange: "Test Crypto Exchange".to_string(),
        created_at: Utc::now(),
        updated_at: Utc::now(),
        filled_at: None,
        average_fill_price: None,
        strategy_id: Some("test_strategy".to_string()),
        notes: None,
    }
}

#[test]
fn test_new_crypto_exchange() {
    let config = create_test_config();
    let exchange = CryptoExchange::new(config.clone());
    
    assert_eq!(exchange.name(), config.name);
    assert_eq!(exchange.exchange_type(), config.exchange_type);
    assert!(!exchange.is_connected());
}

#[test]
fn test_name_and_type() {
    let config = create_test_config();
    let exchange = CryptoExchange::new(config.clone());
    
    assert_eq!(exchange.name(), "Test Crypto Exchange");
    assert_eq!(exchange.exchange_type(), ExchangeType::Crypto);
}

#[test]
fn test_is_connected_initially_false() {
    let config = create_test_config();
    let exchange = CryptoExchange::new(config);
    
    assert!(!exchange.is_connected());
}

#[test]
async fn test_connect_without_credentials_fails() {
    let config = ExchangeConfig {
        name: "Test Crypto Exchange".to_string(),
        exchange_type: ExchangeType::Crypto,
        api_url: "https://api.example.com".to_string(),
        api_key: None,
        api_secret: None,
        additional_params: HashMap::new(),
    };
    
    let mut exchange = CryptoExchange::new(config);
    let result = exchange.connect().await;
    
    assert!(result.is_err());
    assert!(!exchange.is_connected());
}

#[test]
async fn test_connect_with_credentials_succeeds() {
    let config = create_test_config();
    let mut exchange = CryptoExchange::new(config);
    
    let result = exchange.connect().await;
    assert!(result.is_ok());
    assert!(exchange.is_connected());
}

#[test]
async fn test_disconnect() {
    let config = create_test_config();
    let mut exchange = CryptoExchange::new(config);
    
    // First connect
    let _ = exchange.connect().await;
    assert!(exchange.is_connected());
    
    // Then disconnect
    let result = exchange.disconnect().await;
    assert!(result.is_ok());
    assert!(!exchange.is_connected());
}

#[test]
async fn test_get_supported_assets_when_not_connected() {
    let config = create_test_config();
    let exchange = CryptoExchange::new(config);
    
    let result = exchange.get_supported_assets().await;
    assert!(result.is_err());
}

#[test]
async fn test_get_supported_assets_when_connected() {
    let config = create_test_config();
    let mut exchange = CryptoExchange::new(config);
    
    // First connect
    let _ = exchange.connect().await;
    
    let result = exchange.get_supported_assets().await;
    assert!(result.is_ok());
    
    let assets = result.unwrap();
    assert!(!assets.is_empty());
    assert!(assets.contains(&"BTC/USD".to_string()));
    assert!(assets.contains(&"ETH/USD".to_string()));
}

#[test]
async fn test_get_market_data_when_not_connected() {
    let config = create_test_config();
    let exchange = CryptoExchange::new(config);
    
    let result = exchange.get_market_data("BTC/USD").await;
    assert!(result.is_err());
}

#[test]
async fn test_get_market_data_when_connected() {
    let config = create_test_config();
    let mut exchange = CryptoExchange::new(config);
    
    // First connect
    let _ = exchange.connect().await;
    
    let result = exchange.get_market_data("BTC/USD").await;
    assert!(result.is_ok());
    
    let snapshot = result.unwrap();
    assert_eq!(snapshot.symbol, "BTC/USD");
    assert!(snapshot.price > 0.0);
    assert!(snapshot.bid > 0.0);
    assert!(snapshot.ask > 0.0);
    assert!(snapshot.volume > 0.0);
}

#[test]
async fn test_submit_order_when_not_connected() {
    let config = create_test_config();
    let exchange = CryptoExchange::new(config);
    let order = create_test_order();
    
    let result = exchange.submit_order(order).await;
    assert!(result.is_err());
}

#[test]
async fn test_submit_order_when_connected() {
    let config = create_test_config();
    let mut exchange = CryptoExchange::new(config);
    let order = create_test_order();
    
    // First connect
    let _ = exchange.connect().await;
    
    let result = exchange.submit_order(order).await;
    assert!(result.is_ok());
}

#[test]
async fn test_cancel_order_when_not_connected() {
    let config = create_test_config();
    let exchange = CryptoExchange::new(config);
    let order_id = Uuid::new_v4();
    
    let result = exchange.cancel_order(order_id).await;
    assert!(result.is_err());
}

#[test]
async fn test_cancel_nonexistent_order() {
    let config = create_test_config();
    let mut exchange = CryptoExchange::new(config);
    let order_id = Uuid::new_v4();
    
    // First connect
    let _ = exchange.connect().await;
    
    let result = exchange.cancel_order(order_id).await;
    assert!(result.is_err());
}

#[test]
async fn test_submit_and_cancel_order() {
    let config = create_test_config();
    let mut exchange = CryptoExchange::new(config);
    let order = create_test_order();
    
    // First connect
    let _ = exchange.connect().await;
    
    // Submit the order
    let submit_result = exchange.submit_order(order.clone()).await;
    assert!(submit_result.is_ok());
    
    // Cancel the order
    let cancel_result = exchange.cancel_order(order.id).await;
    assert!(cancel_result.is_ok());
}

#[test]
async fn test_get_order_status_when_not_connected() {
    let config = create_test_config();
    let exchange = CryptoExchange::new(config);
    let order_id = Uuid::new_v4();
    
    let result = exchange.get_order_status(order_id).await;
    assert!(result.is_err());
}

#[test]
async fn test_get_nonexistent_order_status() {
    let config = create_test_config();
    let mut exchange = CryptoExchange::new(config);
    let order_id = Uuid::new_v4();
    
    // First connect
    let _ = exchange.connect().await;
    
    let result = exchange.get_order_status(order_id).await;
    assert!(result.is_err());
}

#[test]
async fn test_submit_and_get_order_status() {
    let config = create_test_config();
    let mut exchange = CryptoExchange::new(config);
    let order = create_test_order();
    
    // First connect
    let _ = exchange.connect().await;
    
    // Submit the order
    let submit_result = exchange.submit_order(order.clone()).await;
    assert!(submit_result.is_ok());
    
    // Get the order status
    let status_result = exchange.get_order_status(order.id).await;
    assert!(status_result.is_ok());
    
    let status_response = status_result.unwrap();
    assert_eq!(status_response.order_id, order.id);
    assert!(status_response.exchange_order_id.is_some());
}

#[test]
async fn test_get_account_balance_when_not_connected() {
    let config = create_test_config();
    let exchange = CryptoExchange::new(config);
    
    let result = exchange.get_account_balance().await;
    assert!(result.is_err());
}

#[test]
async fn test_get_account_balance_when_connected() {
    let config = create_test_config();
    let mut exchange = CryptoExchange::new(config);
    
    // First connect
    let _ = exchange.connect().await;
    
    let result = exchange.get_account_balance().await;
    assert!(result.is_ok());
    
    let balance = result.unwrap();
    assert_eq!(balance.currency, "USD");
    assert!(balance.total > 0.0);
    assert!(balance.available > 0.0);
    assert!(!balance.additional_balances.is_empty());
}

#[test]
async fn test_get_positions_when_not_connected() {
    let config = create_test_config();
    let exchange = CryptoExchange::new(config);
    
    let result = exchange.get_positions().await;
    assert!(result.is_err());
}

#[test]
async fn test_get_positions_when_connected() {
    let config = create_test_config();
    let mut exchange = CryptoExchange::new(config);
    
    // First connect
    let _ = exchange.connect().await;
    
    let result = exchange.get_positions().await;
    assert!(result.is_ok());
    
    let positions = result.unwrap();
    assert!(!positions.is_empty());
} 