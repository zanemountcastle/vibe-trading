use arb_platform::strategy::{
    Strategy, StrategyManager, StrategyState,
    TradeDirection, TimeInForce, MarketData, StrategyResult, StrategyParams, AssetType
};

use tokio::test;

// Create a wrapper struct for Strategy implementation
struct MockStrategyWrapper();

impl Strategy for MockStrategyWrapper {
    fn name(&self) -> &str {
        "Test Strategy" // Hard-coded for simplicity
    }
    
    fn description(&self) -> &str {
        "A mock strategy for testing"
    }
    
    fn asset_types(&self) -> Vec<AssetType> {
        vec![AssetType::Crypto]
    }
    
    fn evaluate(&self, _market_data: &MarketData) -> StrategyResult {
        StrategyResult {
            signals: vec![],
            confidence: 0.8,
            expected_profit: 0.5,
            timestamp: chrono::Utc::now()
        }
    }
    
    fn update_params(&mut self, _params: StrategyParams) -> Result<(), String> {
        Ok(())
    }
}

#[test]
async fn test_trade_direction() {
    let buy = TradeDirection::Buy;
    let sell = TradeDirection::Sell;
    
    assert_ne!(buy, sell);
    
    // Test reversal
    assert_eq!(buy.reverse(), sell);
    assert_eq!(sell.reverse(), buy);
}

#[test]
async fn test_time_in_force() {
    let gtc = TimeInForce::GoodTilCancelled;
    let day = TimeInForce::Day;
    let fok = TimeInForce::FillOrKill;
    let ioc = TimeInForce::ImmediateOrCancel;
    
    assert_ne!(gtc, day);
    assert_ne!(gtc, fok);
    assert_ne!(gtc, ioc);
    assert_ne!(day, fok);
    assert_ne!(day, ioc);
    assert_ne!(fok, ioc);
}

#[test]
async fn test_strategy_state_transitions() {
    // Test valid transitions
    assert!(StrategyState::Initialized.can_transition_to(&StrategyState::Ready));
    assert!(StrategyState::Ready.can_transition_to(&StrategyState::Running));
    assert!(StrategyState::Running.can_transition_to(&StrategyState::Paused));
    assert!(StrategyState::Paused.can_transition_to(&StrategyState::Running));
    assert!(StrategyState::Running.can_transition_to(&StrategyState::Stopping));
    assert!(StrategyState::Stopping.can_transition_to(&StrategyState::Stopped));
    
    // Test invalid transitions
    assert!(!StrategyState::Initialized.can_transition_to(&StrategyState::Running));
    assert!(!StrategyState::Ready.can_transition_to(&StrategyState::Initialized));
    assert!(!StrategyState::Stopped.can_transition_to(&StrategyState::Running));
    assert!(!StrategyState::Running.can_transition_to(&StrategyState::Initialized));
}

#[test]
async fn test_strategy_config_creation() {
    let _manager = StrategyManager::new();
    // Just verify that we can create a manager
    assert!(true);
}

#[test]
async fn test_register_strategy() {
    let mut manager = StrategyManager::new();
    let boxed_strategy: Box<dyn Strategy> = Box::new(MockStrategyWrapper());
    
    manager.register_strategy(boxed_strategy);
    // Just verify that we can register a strategy
    assert!(true);
}

#[test]
async fn test_set_active_strategy() {
    let mut manager = StrategyManager::new();
    let boxed_strategy: Box<dyn Strategy> = Box::new(MockStrategyWrapper());
    
    manager.register_strategy(boxed_strategy);
    
    // Set as active
    let result = manager.set_active_strategy("Test Strategy");
    assert!(result.is_ok());
} 