use arb_platform::exchange::{Exchange, ExchangeConfig, ExchangeType};
use arb_platform::exchange::crypto::CryptoExchange;
use arb_platform::order::{Order, OrderStatus, OrderType};
use arb_platform::strategy::{TradeDirection, TimeInForce};
use std::collections::HashMap;
use chrono::Utc;
use uuid::Uuid;

#[test]
fn test_exchange_config() {
    let config = ExchangeConfig {
        name: "Test Exchange".to_string(),
        exchange_type: ExchangeType::Crypto,
        api_url: "https://api.example.com".to_string(),
        api_key: Some("test_key".to_string()),
        api_secret: Some("test_secret".to_string()),
        additional_params: HashMap::new(),
    };
    
    assert_eq!(config.name, "Test Exchange");
    assert_eq!(config.exchange_type, ExchangeType::Crypto);
}

#[test]
fn test_order_creation() {
    let order = Order {
        id: Uuid::new_v4(),
        client_order_id: "test_order".to_string(),
        symbol: "BTC/USD".to_string(),
        direction: TradeDirection::Buy,
        order_type: OrderType::Limit,
        quantity: 1.0,
        filled_quantity: 0.0,
        price: Some(35000.0),
        stop_price: None,
        time_in_force: TimeInForce::GoodTilCancelled,
        status: OrderStatus::Created,
        exchange: "Test Exchange".to_string(),
        created_at: Utc::now(),
        updated_at: Utc::now(),
        filled_at: None,
        average_fill_price: None,
        strategy_id: Some("test_strategy".to_string()),
        notes: None,
    };
    
    assert_eq!(order.symbol, "BTC/USD");
    assert_eq!(order.direction, TradeDirection::Buy);
    assert_eq!(order.order_type, OrderType::Limit);
    assert_eq!(order.status, OrderStatus::Created);
}

#[test]
fn test_order_status_transitions() {
    assert!(OrderStatus::Created.can_transition_to(&OrderStatus::PendingSubmission));
    assert!(OrderStatus::PendingSubmission.can_transition_to(&OrderStatus::Submitted));
    assert!(!OrderStatus::Filled.can_transition_to(&OrderStatus::Created));
}

#[tokio::test]
async fn test_crypto_exchange() {
    let config = ExchangeConfig {
        name: "Test Exchange".to_string(),
        exchange_type: ExchangeType::Crypto,
        api_url: "https://api.example.com".to_string(),
        api_key: Some("test_key".to_string()),
        api_secret: Some("test_secret".to_string()),
        additional_params: HashMap::new(),
    };
    
    let exchange = CryptoExchange::new(config);
    
    assert_eq!(exchange.name(), "Test Exchange");
    assert_eq!(exchange.exchange_type(), ExchangeType::Crypto);
    assert!(!exchange.is_connected());
} 