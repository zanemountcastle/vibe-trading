use arb_platform::exchange::{
    ExchangeType, MarketSnapshot, OrderStatusResponse, OrderStatus,
    AccountBalance, Position, ExchangeConfig, ExchangeFactory, Exchange
};
use chrono::Utc;
use std::collections::HashMap;
use uuid::Uuid;

#[test]
fn test_exchange_type_serde() {
    // Test serialization/deserialization of ExchangeType
    let types = vec![
        ExchangeType::Stock,
        ExchangeType::Crypto, 
        ExchangeType::Forex,
        ExchangeType::Bond,
        ExchangeType::Commodity,
        ExchangeType::Option,
        ExchangeType::Future
    ];
    
    for exchange_type in types {
        let serialized = serde_json::to_string(&exchange_type).expect("Failed to serialize");
        let deserialized: ExchangeType = serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(exchange_type, deserialized);
    }
}

#[test]
fn test_market_snapshot_creation() {
    let now = Utc::now();
    let snapshot = MarketSnapshot {
        symbol: "BTC/USD".to_string(),
        price: 35000.0,
        bid: 34990.0,
        ask: 35010.0,
        bid_size: 1.5,
        ask_size: 2.0,
        volume: 100.0,
        timestamp: now,
    };
    
    assert_eq!(snapshot.symbol, "BTC/USD");
    assert_eq!(snapshot.price, 35000.0);
    assert_eq!(snapshot.bid, 34990.0);
    assert_eq!(snapshot.ask, 35010.0);
    assert_eq!(snapshot.bid_size, 1.5);
    assert_eq!(snapshot.ask_size, 2.0);
    assert_eq!(snapshot.volume, 100.0);
    assert_eq!(snapshot.timestamp, now);
}

#[test]
fn test_order_status_serde() {
    // Test serialization/deserialization of OrderStatus
    let statuses = vec![
        OrderStatus::Pending,
        OrderStatus::Open,
        OrderStatus::PartiallyFilled,
        OrderStatus::Filled,
        OrderStatus::Cancelled,
        OrderStatus::Rejected,
        OrderStatus::Unknown,
    ];
    
    for status in statuses {
        let serialized = serde_json::to_string(&status).expect("Failed to serialize");
        let deserialized: OrderStatus = serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(status, deserialized);
    }
}

#[test]
fn test_order_status_response_creation() {
    let order_id = Uuid::new_v4();
    let now = Utc::now();
    let response = OrderStatusResponse {
        order_id,
        exchange_order_id: Some("EX123456".to_string()),
        status: OrderStatus::PartiallyFilled,
        filled_quantity: 0.5,
        remaining_quantity: 0.5,
        average_price: Some(35000.0),
        last_update: now,
    };
    
    assert_eq!(response.order_id, order_id);
    assert_eq!(response.exchange_order_id, Some("EX123456".to_string()));
    assert_eq!(response.status, OrderStatus::PartiallyFilled);
    assert_eq!(response.filled_quantity, 0.5);
    assert_eq!(response.remaining_quantity, 0.5);
    assert_eq!(response.average_price, Some(35000.0));
    assert_eq!(response.last_update, now);
}

#[test]
fn test_account_balance_creation() {
    let now = Utc::now();
    let additional_balances = vec![
        ("ETH".to_string(), 10.0),
        ("SOL".to_string(), 100.0),
    ];
    
    let balance = AccountBalance {
        total: 10000.0,
        available: 9000.0,
        currency: "USD".to_string(),
        additional_balances: additional_balances.clone(),
        timestamp: now,
    };
    
    assert_eq!(balance.total, 10000.0);
    assert_eq!(balance.available, 9000.0);
    assert_eq!(balance.currency, "USD");
    assert_eq!(balance.additional_balances, additional_balances);
    assert_eq!(balance.timestamp, now);
}

#[test]
fn test_position_creation() {
    let now = Utc::now();
    let position = Position {
        symbol: "BTC/USD".to_string(),
        quantity: 1.5,
        avg_price: 34000.0,
        current_price: 35000.0,
        unrealized_pnl: 1500.0,
        realized_pnl: 500.0,
        timestamp: now,
    };
    
    assert_eq!(position.symbol, "BTC/USD");
    assert_eq!(position.quantity, 1.5);
    assert_eq!(position.avg_price, 34000.0);
    assert_eq!(position.current_price, 35000.0);
    assert_eq!(position.unrealized_pnl, 1500.0);
    assert_eq!(position.realized_pnl, 500.0);
    assert_eq!(position.timestamp, now);
}

#[test]
fn test_exchange_config_creation() {
    let mut additional_params = HashMap::new();
    additional_params.insert("passphrase".to_string(), "test123".to_string());
    
    let config = ExchangeConfig {
        name: "Coinbase Pro".to_string(),
        exchange_type: ExchangeType::Crypto,
        api_url: "https://api.pro.coinbase.com".to_string(),
        api_key: Some("key123".to_string()),
        api_secret: Some("secret456".to_string()),
        additional_params,
    };
    
    assert_eq!(config.name, "Coinbase Pro");
    assert_eq!(config.exchange_type, ExchangeType::Crypto);
    assert_eq!(config.api_url, "https://api.pro.coinbase.com");
    assert_eq!(config.api_key, Some("key123".to_string()));
    assert_eq!(config.api_secret, Some("secret456".to_string()));
    assert_eq!(config.additional_params.get("passphrase"), Some(&"test123".to_string()));
}

#[test]
fn test_exchange_factory() {
    let config = ExchangeConfig {
        name: "Test Crypto Exchange".to_string(),
        exchange_type: ExchangeType::Crypto,
        api_url: "https://api.example.com".to_string(),
        api_key: None,
        api_secret: None,
        additional_params: HashMap::new(),
    };
    
    let result = ExchangeFactory::create_crypto_exchange(config.clone());
    assert!(result.is_ok());
    
    let exchange = result.unwrap();
    assert_eq!(exchange.name(), config.name);
    assert_eq!(exchange.exchange_type(), config.exchange_type);
    assert!(!exchange.is_connected());
} 