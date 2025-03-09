use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use tracing::{info, error};

// Comment out missing modules
// mod event_arbitrage;
// mod statistical_arbitrage;
// mod information_arbitrage;
// mod latency_arbitrage;
// mod day_trading;

// Common traits and structures for all strategies
#[allow(dead_code)]
pub trait Strategy: Send + Sync {
    fn name(&self) -> &str;
    fn description(&self) -> &str;
    fn asset_types(&self) -> Vec<AssetType>;
    fn evaluate(&self, market_data: &MarketData) -> StrategyResult;
    fn update_params(&mut self, params: StrategyParams) -> Result<(), String>;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AssetType {
    Stock,
    Bond,
    Crypto,
    Forex,
    Commodity,
    Option,
    Future,
    ETF,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketData {
    // This will be expanded to include various market data types
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub asset_data: HashMap<String, AssetData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetData {
    pub symbol: String,
    pub asset_type: AssetType,
    pub price: f64,
    pub volume: f64,
    pub bid: f64,
    pub ask: f64,
    pub exchange: String,
    // Additional fields will be added based on asset type
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StrategyResult {
    pub signals: Vec<TradeSignal>,
    pub confidence: f64, // 0.0 to 1.0
    pub expected_profit: f64,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradeSignal {
    pub asset: String,
    pub direction: TradeDirection,
    pub quantity: f64,
    pub limit_price: Option<f64>,
    pub stop_price: Option<f64>,
    pub time_in_force: TimeInForce,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TradeDirection {
    Buy,
    Sell,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TimeInForce {
    GoodTilCanceled,
    ImmediateOrCancel,
    FillOrKill,
    DayOnly,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StrategyParams {
    pub params: HashMap<String, serde_json::Value>,
}

// The StrategyManager handles creation, updating, and selection of strategies
#[allow(dead_code)]
pub struct StrategyManager {
    strategies: HashMap<String, Box<dyn Strategy>>,
    active_strategy: Option<String>,
}

#[allow(dead_code, unused_variables)]
impl StrategyManager {
    pub fn new() -> Self {
        StrategyManager {
            strategies: HashMap::new(),
            active_strategy: None,
        }
    }

    pub fn register_strategy(&mut self, strategy: Box<dyn Strategy>) {
        let name = strategy.name().to_string();
        info!("Registering strategy: {}", name);
        self.strategies.insert(name, strategy);
    }

    pub fn set_active_strategy(&mut self, name: &str) -> Result<(), String> {
        if self.strategies.contains_key(name) {
            info!("Setting active strategy to: {}", name);
            self.active_strategy = Some(name.to_string());
            Ok(())
        } else {
            let error_msg = format!("Strategy not found: {}", name);
            error!("{}", error_msg);
            Err(error_msg)
        }
    }

    pub fn evaluate_strategies(&self, market_data: &MarketData) -> HashMap<String, StrategyResult> {
        let mut results = HashMap::new();
        
        for (name, strategy) in &self.strategies {
            info!("Evaluating strategy: {}", name);
            
            let result = strategy.evaluate(market_data);
            
            info!("Strategy {} evaluation complete, confidence: {}", name, result.confidence);
            
            results.insert(name.clone(), result);
        }
        
        results
    }

    pub fn get_best_strategy(&self, results: &HashMap<String, StrategyResult>) -> Option<String> {
        results.iter()
            .max_by(|a, b| {
                a.1.expected_profit.partial_cmp(&b.1.expected_profit).unwrap_or(std::cmp::Ordering::Equal)
            })
            .map(|(name, _)| name.clone())
    }

    pub fn get_active_strategy_signals(&self, market_data: &MarketData) -> Option<StrategyResult> {
        match &self.active_strategy {
            Some(name) => {
                if let Some(strategy) = self.strategies.get(name) {
                    Some(strategy.evaluate(market_data))
                } else {
                    None
                }
            }
            None => None,
        }
    }

    pub fn update_strategy_params(&mut self, name: &str, params: StrategyParams) -> Result<(), String> {
        if let Some(strategy) = self.strategies.get_mut(name) {
            strategy.update_params(params)
        } else {
            Err(format!("Strategy not found: {}", name))
        }
    }
} 