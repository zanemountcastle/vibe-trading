use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::info;
use uuid::Uuid;

use super::Order;
use crate::exchange::Exchange;
use crate::exchange::crypto::CryptoExchange;

#[derive(Clone)]
pub struct OrderRouter {
    // Since we only have CryptoExchange implemented for now, use concrete types
    exchanges: Arc<RwLock<HashMap<String, CryptoExchange>>>,
    primary_exchange_map: Arc<RwLock<HashMap<String, String>>>, // Maps asset to primary exchange
}

#[allow(dead_code, unused_variables)]
impl OrderRouter {
    pub fn new() -> Self {
        OrderRouter {
            exchanges: Arc::new(RwLock::new(HashMap::new())),
            primary_exchange_map: Arc::new(RwLock::new(HashMap::new())),
        }
    }
    
    // Register exchange with concrete type
    pub async fn register_exchange(&self, exchange: CryptoExchange) -> Result<(), String> {
        let name = exchange.name().to_string();
        info!("Registering exchange: {}", name);
        
        let mut exchanges = self.exchanges.write().await;
        if exchanges.contains_key(&name) {
            return Err(format!("Exchange {} already registered", name));
        }
        
        exchanges.insert(name, exchange);
        Ok(())
    }
    
    pub async fn set_primary_exchange(&self, asset: &str, exchange: &str) -> Result<(), String> {
        let mut primary_map = self.primary_exchange_map.write().await;
        primary_map.insert(asset.to_string(), exchange.to_string());
        
        info!("Set primary exchange for {}: {}", asset, exchange);
        Ok(())
    }
    
    pub async fn submit_order(&self, order: Order) -> Result<(), String> {
        // Determine the exchange to use
        let exchange_name = if !order.exchange.is_empty() {
            // Use specified exchange
            order.exchange.clone()
        } else {
            // Use primary exchange for this asset
            let primary_map = self.primary_exchange_map.read().await;
            match primary_map.get(&order.symbol) {
                Some(name) => name.clone(),
                None => return Err(format!("No primary exchange defined for {}", order.symbol)),
            }
        };
        
        // Get the exchange
        let exchanges = self.exchanges.read().await;
        let exchange = exchanges.get(&exchange_name)
            .ok_or_else(|| format!("Exchange {} not found", exchange_name))?;
        
        // Submit the order
        exchange.submit_order(order).await
    }
    
    pub async fn cancel_order(&self, order_id: Uuid) -> Result<(), String> {
        // We need to try all exchanges since we don't know which one has the order
        let exchanges = self.exchanges.read().await;
        if exchanges.is_empty() {
            return Err("No exchanges registered".to_string());
        }
        
        // Try each exchange
        for (name, exchange) in exchanges.iter() {
            match exchange.cancel_order(order_id).await {
                Ok(_) => {
                    info!("Order {} cancelled on {}", order_id, name);
                    return Ok(());
                }
                Err(_) => {
                    // This exchange doesn't have the order, try the next one
                    continue;
                }
            }
        }
        
        Err(format!("Order {} not found on any exchange", order_id))
    }
    
    pub async fn get_exchange_for_asset(&self, symbol: &str) -> Option<String> {
        let primary_map = self.primary_exchange_map.read().await;
        primary_map.get(symbol).cloned()
    }
    
    pub async fn get_supported_exchanges(&self) -> Vec<String> {
        let exchanges = self.exchanges.read().await;
        exchanges.keys().cloned().collect()
    }
    
    pub async fn get_supported_assets(&self) -> Vec<String> {
        let mut assets = Vec::new();
        let exchanges = self.exchanges.read().await;
        
        for exchange in exchanges.values() {
            if let Ok(exchange_assets) = exchange.get_supported_assets().await {
                for asset in exchange_assets {
                    if !assets.contains(&asset) {
                        assets.push(asset);
                    }
                }
            }
        }
        
        assets
    }
} 