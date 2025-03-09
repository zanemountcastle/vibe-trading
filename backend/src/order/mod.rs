use std::collections::HashMap;
use std::sync::Arc;
use chrono::{DateTime, Utc};
use tokio::sync::{RwLock, mpsc};
use tracing::{info, warn, error, debug};
use uuid::Uuid;

use crate::strategy::{TradeDirection, TimeInForce};

mod router;
// Comment out missing modules
// mod execution;
// mod risk_check;

pub use router::OrderRouter;

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OrderStatus {
    Created,
    PendingSubmission,
    Submitted,
    PartiallyFilled,
    Filled,
    Cancelled,
    Rejected,
    Failed,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum OrderType {
    Market,
    Limit,
    StopLoss,
    StopLimit,
    TrailingStop,
}

#[derive(Debug, Clone)]
pub struct Order {
    pub id: Uuid,
    pub client_order_id: String,
    pub symbol: String,
    pub direction: TradeDirection,
    pub order_type: OrderType,
    pub quantity: f64,
    pub filled_quantity: f64,
    pub price: Option<f64>,
    pub stop_price: Option<f64>,
    pub time_in_force: TimeInForce,
    pub status: OrderStatus,
    pub exchange: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub filled_at: Option<DateTime<Utc>>,
    pub average_fill_price: Option<f64>,
    pub strategy_id: Option<String>,
    pub notes: Option<String>,
}

#[allow(dead_code)]
pub enum OrderEvent {
    New(Order),
    Update {
        order_id: Uuid,
        status: OrderStatus,
        filled_quantity: Option<f64>,
        average_fill_price: Option<f64>,
        timestamp: DateTime<Utc>,
    },
    Cancel {
        order_id: Uuid,
        reason: String,
        timestamp: DateTime<Utc>,
    },
    Reject {
        order_id: Uuid,
        reason: String,
        timestamp: DateTime<Utc>,
    },
    Error {
        order_id: Option<Uuid>,
        message: String,
        timestamp: DateTime<Utc>,
    },
}

// Order Manager handles the lifecycle of orders
#[allow(dead_code)]
pub struct OrderManager {
    orders: Arc<RwLock<HashMap<Uuid, Order>>>,
    active_orders: Arc<RwLock<HashMap<Uuid, Order>>>,
    order_router: OrderRouter,
    event_sender: mpsc::Sender<OrderEvent>,
    event_receiver: Option<mpsc::Receiver<OrderEvent>>,
    shutdown_signal: Option<tokio::sync::oneshot::Sender<()>>,
}

impl OrderManager {
    pub fn new() -> Self {
        OrderManager {
            orders: Arc::new(RwLock::new(HashMap::new())),
            active_orders: Arc::new(RwLock::new(HashMap::new())),
            order_router: OrderRouter::new(),
            event_sender: mpsc::channel(100).0,
            event_receiver: Some(mpsc::channel(100).1),
            shutdown_signal: None,
        }
    }
    
    pub async fn place_order(&self, mut order: Order) -> Result<Uuid, String> {
        // Generate a unique ID if not provided
        if order.id == Uuid::nil() {
            order.id = Uuid::new_v4();
        }
        
        // Set created timestamp
        order.created_at = Utc::now();
        order.updated_at = order.created_at;
        
        // Update status
        order.status = OrderStatus::Created;
        
        // Validate the order
        self.validate_order(&order)?;
        
        // Store the order
        {
            let mut orders = self.orders.write().await;
            let mut active_orders = self.active_orders.write().await;
            
            orders.insert(order.id, order.clone());
            active_orders.insert(order.id, order.clone());
        }
        
        // Emit new order event
        self.emit_event(OrderEvent::New(order.clone())).await;
        
        // Submit the order to the router for execution
        let order_id = order.id;
        tokio::spawn({
            let order_router = self.order_router.clone();
            let event_sender = self.event_sender.clone();
            let orders = self.orders.clone();
            let active_orders = self.active_orders.clone();
            
            async move {
                // Update order status to pending submission
                Self::update_order_status(orders.clone(), order_id, OrderStatus::PendingSubmission).await;
                
                // Submit to router
                match order_router.submit_order(order.clone()).await {
                    Ok(()) => {
                        // Update status to submitted
                        Self::update_order_status(orders.clone(), order_id, OrderStatus::Submitted).await;
                        
                        // Emit update event
                        let event = OrderEvent::Update {
                            order_id,
                            status: OrderStatus::Submitted,
                            filled_quantity: None,
                            average_fill_price: None,
                            timestamp: Utc::now(),
                        };
                        
                        if let Err(e) = event_sender.send(event).await {
                            error!("Failed to emit order update event: {}", e);
                        }
                    },
                    Err(e) => {
                        error!("Failed to submit order {}: {}", order_id, e);
                        
                        // Update status to failed
                        Self::update_order_status(orders.clone(), order_id, OrderStatus::Failed).await;
                        
                        // Remove from active orders
                        {
                            let mut active = active_orders.write().await;
                            active.remove(&order_id);
                        }
                        
                        // Emit error event
                        let event = OrderEvent::Error {
                            order_id: Some(order_id),
                            message: e.to_string(),
                            timestamp: Utc::now(),
                        };
                        
                        if let Err(e) = event_sender.send(event).await {
                            error!("Failed to emit order error event: {}", e);
                        }
                    }
                }
            }
        });
        
        Ok(order_id)
    }
    
    async fn update_order_status(orders: Arc<RwLock<HashMap<Uuid, Order>>>, order_id: Uuid, status: OrderStatus) {
        let mut orders_lock = orders.write().await;
        if let Some(order) = orders_lock.get_mut(&order_id) {
            order.status = status;
            order.updated_at = Utc::now();
        }
    }
    
    pub async fn cancel_order(&self, order_id: Uuid, reason: String) -> Result<(), String> {
        // Check if order exists and is active
        let order = {
            let active_orders = self.active_orders.read().await;
            active_orders.get(&order_id).cloned()
        };
        
        match order {
            Some(order) => {
                // Only certain statuses can be cancelled
                match order.status {
                    OrderStatus::Submitted | OrderStatus::PartiallyFilled => {
                        // Submit cancel request to the router
                        self.order_router.cancel_order(order_id).await?;
                        
                        // Emit cancel event
                        self.emit_event(OrderEvent::Cancel {
                            order_id,
                            reason,
                            timestamp: Utc::now(),
                        }).await;
                        
                        Ok(())
                    },
                    _ => Err(format!("Order {} cannot be cancelled in status {:?}", order_id, order.status)),
                }
            },
            None => Err(format!("Order {} not found or not active", order_id)),
        }
    }
    
    pub async fn get_order(&self, order_id: Uuid) -> Option<Order> {
        let orders = self.orders.read().await;
        orders.get(&order_id).cloned()
    }
    
    pub async fn get_active_orders(&self) -> Vec<Order> {
        let active_orders = self.active_orders.read().await;
        active_orders.values().cloned().collect()
    }
    
    async fn emit_event(&self, event: OrderEvent) {
        if let Err(e) = self.event_sender.send(event).await {
            error!("Failed to emit order event: {}", e);
        }
    }
    
    fn validate_order(&self, order: &Order) -> Result<(), String> {
        // Basic validation checks
        if order.symbol.is_empty() {
            return Err("Order symbol cannot be empty".to_string());
        }
        
        if order.quantity <= 0.0 {
            return Err("Order quantity must be positive".to_string());
        }
        
        // Validate price for limit orders
        if order.order_type == OrderType::Limit && order.price.is_none() {
            return Err("Limit orders must specify a price".to_string());
        }
        
        // Validate stop price for stop orders
        if (order.order_type == OrderType::StopLoss || order.order_type == OrderType::StopLimit) 
            && order.stop_price.is_none() {
            return Err("Stop orders must specify a stop price".to_string());
        }
        
        // Additional validations could be added here
        
        Ok(())
    }
    
    pub async fn start_processing(&mut self) -> Result<(), String> {
        let (shutdown_tx, mut shutdown_rx) = tokio::sync::oneshot::channel();
        self.shutdown_signal = Some(shutdown_tx);
        
        let mut event_receiver = self.event_receiver.take()
            .ok_or_else(|| "Event receiver already taken".to_string())?;
            
        let orders_clone = self.orders.clone();
        let active_orders_clone = self.active_orders.clone();
        
        // Spawn a task to process order events
        tokio::spawn(async move {
            info!("Starting order event processing");
            
            loop {
                tokio::select! {
                    // Process new order events
                    Some(event) = event_receiver.recv() => {
                        Self::process_order_event(event, orders_clone.clone(), active_orders_clone.clone()).await;
                    }
                    
                    // Use mut reference without &mut
                    _ = &mut shutdown_rx => {
                        info!("Shutting down order event processing");
                        break;
                    }
                }
            }
            
            info!("Order event processing stopped");
        });
        
        Ok(())
    }
    
    async fn process_order_event(
        event: OrderEvent, 
        orders: Arc<RwLock<HashMap<Uuid, Order>>>,
        active_orders: Arc<RwLock<HashMap<Uuid, Order>>>
    ) {
        match &event {
            OrderEvent::Update { order_id, status, filled_quantity, average_fill_price, timestamp } => {
                let mut orders_lock = orders.write().await;
                
                if let Some(order) = orders_lock.get_mut(order_id) {
                    // Update order status
                    order.status = status.clone();
                    order.updated_at = *timestamp;
                    
                    // Update filled quantity if provided
                    if let Some(qty) = filled_quantity {
                        order.filled_quantity = *qty;
                    }
                    
                    // Update average fill price if provided
                    if let Some(price) = average_fill_price {
                        order.average_fill_price = Some(*price);
                    }
                    
                    // If order is completely filled, update filled timestamp
                    if *status == OrderStatus::Filled {
                        order.filled_at = Some(*timestamp);
                        
                        // Remove from active orders
                        let mut active_lock = active_orders.write().await;
                        active_lock.remove(order_id);
                    }
                    
                    debug!("Updated order {}: status={:?}, filled={:?}, avg_price={:?}", 
                        order_id, status, filled_quantity, average_fill_price);
                } else {
                    warn!("Received update for unknown order: {}", order_id);
                }
            },
            OrderEvent::Cancel { order_id, reason, timestamp } => {
                let mut orders_lock = orders.write().await;
                
                if let Some(order) = orders_lock.get_mut(order_id) {
                    // Update order status
                    order.status = OrderStatus::Cancelled;
                    order.updated_at = *timestamp;
                    order.notes = Some(reason.clone());
                    
                    // Remove from active orders
                    let mut active_lock = active_orders.write().await;
                    active_lock.remove(order_id);
                    
                    info!("Cancelled order {}: {}", order_id, reason);
                } else {
                    warn!("Received cancel for unknown order: {}", order_id);
                }
            },
            OrderEvent::Reject { order_id, reason, timestamp } => {
                let mut orders_lock = orders.write().await;
                
                if let Some(order) = orders_lock.get_mut(order_id) {
                    // Update order status
                    order.status = OrderStatus::Rejected;
                    order.updated_at = *timestamp;
                    order.notes = Some(reason.clone());
                    
                    // Remove from active orders
                    let mut active_lock = active_orders.write().await;
                    active_lock.remove(order_id);
                    
                    warn!("Rejected order {}: {}", order_id, reason);
                } else {
                    warn!("Received reject for unknown order: {}", order_id);
                }
            },
            // Other event types handled here
            _ => {}
        }
    }
    
    pub fn get_event_sender(&self) -> mpsc::Sender<OrderEvent> {
        self.event_sender.clone()
    }
    
    pub async fn shutdown(&mut self) -> Result<(), String> {
        info!("Shutting down order manager");
        
        // Send shutdown signal to event processor
        if let Some(shutdown_signal) = self.shutdown_signal.take() {
            if shutdown_signal.send(()).is_err() {
                warn!("Failed to send shutdown signal to order event processor");
            }
        }
        
        Ok(())
    }
} 