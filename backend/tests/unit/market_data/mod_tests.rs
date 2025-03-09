use arb_platform::market_data::{
    MarketDataManager, DataSourceType, MarketEvent, DataSource
};
use arb_platform::exchange::MarketSnapshot;

use chrono::Utc;
use tokio::test;

// Create a mock data source for testing
struct MockDataSource {
    name: String,
    source_type: DataSourceType,
    symbols: Vec<String>,
    is_connected: bool,
}

impl DataSource for MockDataSource {
    fn name(&self) -> &str {
        &self.name
    }
    
    fn source_type(&self) -> &DataSourceType {
        &self.source_type
    }
    
    fn connect(&mut self) -> Result<(), String> {
        self.is_connected = true;
        Ok(())
    }
    
    fn disconnect(&mut self) -> Result<(), String> {
        self.is_connected = false;
        Ok(())
    }
    
    fn is_connected(&self) -> bool {
        self.is_connected
    }
    
    fn subscribe(&mut self, symbols: &[String]) -> Result<(), String> {
        for symbol in symbols {
            if !self.symbols.contains(symbol) {
                self.symbols.push(symbol.clone());
            }
        }
        Ok(())
    }
    
    fn unsubscribe(&mut self, symbols: &[String]) -> Result<(), String> {
        self.symbols.retain(|s| !symbols.contains(s));
        Ok(())
    }
}

fn create_test_data_source() -> Box<dyn DataSource> {
    Box::new(MockDataSource {
        name: "Test Source".to_string(),
        source_type: DataSourceType::CryptoExchange("Test Exchange".to_string()),
        symbols: vec!["BTC/USD".to_string()],
        is_connected: false,
    })
}

#[test]
async fn test_market_data_source_creation() {
    let source = create_test_data_source();
    
    assert_eq!(source.name(), "Test Source");
    match source.source_type() {
        DataSourceType::CryptoExchange(name) => assert_eq!(name, "Test Exchange"),
        _ => panic!("Expected CryptoExchange source type"),
    }
    assert!(!source.is_connected());
}

#[test]
async fn test_market_data_update_creation() {
    let now = Utc::now();
    let _snapshot = MarketSnapshot {
        symbol: "BTC/USD".to_string(),
        price: 35000.0,
        volume: 10.5,
        bid: 34990.0,
        ask: 35010.0,
        bid_size: 1.5,
        ask_size: 2.0,
        timestamp: now,
    };
    
    let event = MarketEvent::PriceUpdate {
        symbol: "BTC/USD".to_string(),
        price: 35000.0,
        volume: Some(10.5),
        bid: Some(34990.0),
        ask: Some(35010.0),
        exchange: "Test Exchange".to_string(),
        timestamp: now,
    };
    
    match &event {
        MarketEvent::PriceUpdate { symbol, price, volume, bid, ask, exchange, timestamp } => {
            assert_eq!(symbol, "BTC/USD");
            assert_eq!(*price, 35000.0);
            assert_eq!(*volume, Some(10.5));
            assert_eq!(*bid, Some(34990.0));
            assert_eq!(*ask, Some(35010.0));
            assert_eq!(exchange, "Test Exchange");
            assert_eq!(*timestamp, now);
        },
        _ => panic!("Expected PriceUpdate event"),
    }
}

#[test]
async fn test_market_data_manager_creation() {
    let manager = MarketDataManager::new();
    
    // Verify initial state by checking that the current data is empty
    let current_data = manager.get_current_data();
    let data = current_data.read().await;
    assert!(data.asset_data.is_empty());
}

#[test]
async fn test_add_data_source() {
    let mut manager = MarketDataManager::new();
    let source = create_test_data_source();
    
    let result = manager.add_data_source(source);
    assert!(result.is_ok());
}

#[test]
async fn test_add_duplicate_data_source() {
    let mut manager = MarketDataManager::new();
    let source1 = create_test_data_source();
    let source2 = create_test_data_source(); // Same name
    
    let result1 = manager.add_data_source(source1);
    assert!(result1.is_ok());
    
    let result2 = manager.add_data_source(source2);
    assert!(result2.is_err());
}

#[test]
async fn test_remove_data_source() {
    let mut manager = MarketDataManager::new();
    let source = create_test_data_source();
    let name = source.name().to_string();
    
    let result = manager.add_data_source(source);
    assert!(result.is_ok());
    
    let remove_result = manager.remove_data_source(&name);
    assert!(remove_result.is_ok());
} 