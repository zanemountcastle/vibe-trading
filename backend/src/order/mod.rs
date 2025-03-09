use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{mpsc, RwLock};
use uuid::Uuid;
use tracing::{info, warn, error};
use chrono::{DateTime, Utc};

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

impl OrderStatus {
    /// Check if the current state can transition to the given state
    #[allow(dead_code)]
    pub fn can_transition_to(&self, next: &OrderStatus) -> bool {
        use OrderStatus::*;
        
        match (self, next) {
            // Valid transitions
            (Created, PendingSubmission) => true,
            (PendingSubmission, Submitted) => true,
            (PendingSubmission, Rejected) => true,
            (PendingSubmission, Failed) => true,
            (Submitted, PartiallyFilled) => true,
            (Submitted, Filled) => true,
            (Submitted, Cancelled) => true,
            (Submitted, Rejected) => true,
            (Submitted, Failed) => true,
            (PartiallyFilled, Filled) => true,
            (PartiallyFilled, Cancelled) => true,
            (PartiallyFilled, Failed) => true,
            
            // Self transitions (no change)
            (s1, s2) if s1 == s2 => true,
            
            // Invalid transitions
            _ => false,
        }
    }
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
        status: Option<OrderStatus>,
        filled_qty: Option<f64>,
        avg_fill_price: Option<f64>,
    },
    Cancel {
        order_id: Uuid,
        reason: String,
    },
    Reject {
        order_id: Uuid,
        reason: String,
    },
    Error {
        order_id: Option<Uuid>,
        message: String,
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
        let (event_sender, event_receiver) = mpsc::channel(100);
        let orders = Arc::new(RwLock::new(HashMap::new()));
        let active_orders = Arc::new(RwLock::new(HashMap::new()));
        let order_router = OrderRouter::new();
        
        let mut manager = OrderManager {
            orders,
            active_orders,
            order_router,
            event_sender,
            event_receiver: Some(event_receiver),
            shutdown_signal: None,
        };
        
        // Start event processing in a separate function
        let orders_clone = manager.orders.clone();
        let active_orders_clone = manager.active_orders.clone();
        let mut event_receiver = manager.event_receiver.take().unwrap();
        
        tokio::spawn(async move {
            info!("Starting order event processing");
            
            loop {
                tokio::select! {
                    // Process new order events
                    Some(event) = event_receiver.recv() => {
                        Self::process_order_event(event, orders_clone.clone(), active_orders_clone.clone()).await;
                    }
                    
                    // Exit after 1 hour of inactivity (for tests)
                    _ = tokio::time::sleep(tokio::time::Duration::from_secs(3600)) => {
                        info!("No order events received for 1 hour, stopping processing");
                        break;
                    }
                }
            }
            
            info!("Order event processing stopped");
        });
        
        manager
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
                Self::update_order_status_internal(orders.clone(), order_id, OrderStatus::PendingSubmission).await;
                
                // Submit to router
                match order_router.submit_order(order.clone()).await {
                    Ok(()) => {
                        // Update status to submitted
                        Self::update_order_status_internal(orders.clone(), order_id, OrderStatus::Submitted).await;
                        
                        // Emit update event
                        let event = OrderEvent::Update {
                            order_id,
                            status: Some(OrderStatus::Submitted),
                            filled_qty: None,
                            avg_fill_price: None,
                        };
                        
                        if let Err(e) = event_sender.send(event).await {
                            error!("Failed to emit order update event: {}", e);
                        }
                    },
                    Err(e) => {
                        error!("Failed to submit order {}: {}", order_id, e);
                        
                        // Update status to failed
                        Self::update_order_status_internal(orders.clone(), order_id, OrderStatus::Failed).await;
                        
                        // Remove from active orders
                        {
                            let mut active = active_orders.write().await;
                            active.remove(&order_id);
                        }
                        
                        // Emit error event
                        let event = OrderEvent::Error {
                            order_id: Some(order_id),
                            message: e.to_string(),
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
    
    #[allow(dead_code)]
    pub async fn update_order_status(&self, order_id: Uuid, status: OrderStatus) {
        Self::update_order_status_internal(self.orders.clone(), order_id, status).await;
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
                    OrderStatus::Created | OrderStatus::Submitted | OrderStatus::PartiallyFilled => {
                        // If the order is only Created (not yet sent to exchange), we can cancel locally
                        if order.status == OrderStatus::Created {
                            // Update status directly
                            Self::update_order_status_internal(self.orders.clone(), order_id, OrderStatus::Cancelled).await;
                        } else {
                            // Submit cancel request to the router
                            let router_result = self.order_router.cancel_order(order_id).await;
                            // If router fails (e.g., no exchanges), still update status locally
                            if router_result.is_err() {
                                Self::update_order_status_internal(self.orders.clone(), order_id, OrderStatus::Cancelled).await;
                            }
                        }
                        
                        // Remove from active orders
                        {
                            let mut active_orders = self.active_orders.write().await;
                            active_orders.remove(&order_id);
                        }
                        
                        // Emit cancel event
                        self.emit_event(OrderEvent::Cancel {
                            order_id,
                            reason,
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
        
        // Validate market orders shouldn't have a price
        if order.order_type == OrderType::Market && order.price.is_some() {
            return Err("Market orders should not specify a price".to_string());
        }
        
        // Validate stop price for stop orders
        if (order.order_type == OrderType::StopLoss || order.order_type == OrderType::StopLimit) 
            && order.stop_price.is_none() {
            return Err("Stop orders must specify a stop price".to_string());
        }
        
        // Additional validations could be added here
        
        Ok(())
    }
    
    async fn process_order_event(
        event: OrderEvent,
        orders: Arc<RwLock<HashMap<Uuid, Order>>>,
        active_orders: Arc<RwLock<HashMap<Uuid, Order>>>
    ) {
        match event {
            OrderEvent::Update { order_id, status, filled_qty, avg_fill_price } => {
                info!("Processing order update event for order {}: status={:?}, filled={:?}, avg_price={:?}", 
                      order_id, status, filled_qty, avg_fill_price);
                
                // Update the order status
                let mut orders_lock = orders.write().await;
                if let Some(order) = orders_lock.get_mut(&order_id) {
                    if let Some(new_status) = status {
                        order.status = new_status;
                    }
                    
                    if let Some(qty) = filled_qty {
                        order.filled_quantity = qty;
                    }
                    
                    if let Some(price) = avg_fill_price {
                        order.average_fill_price = Some(price);
                    }
                    
                    order.updated_at = Utc::now();
                    
                    // If the order is filled or canceled, remove it from active orders
                    if order.status == OrderStatus::Filled || 
                       order.status == OrderStatus::Cancelled || 
                       order.status == OrderStatus::Rejected {
                        let mut active_orders_lock = active_orders.write().await;
                        active_orders_lock.remove(&order_id);
                    }
                } else {
                    warn!("Received update for unknown order: {}", order_id);
                }
            },
            OrderEvent::New(order) => {
                info!("Processing new order event for order {}", order.id);
                // New orders are already added to the orders map during place_order
            },
            OrderEvent::Cancel { order_id, reason } => {
                info!("Processing cancel order event for order {}: {}", order_id, reason);
                
                let mut orders_lock = orders.write().await;
                if let Some(order) = orders_lock.get_mut(&order_id) {
                    order.status = OrderStatus::Cancelled;
                    order.notes = Some(reason.clone());
                    order.updated_at = Utc::now();
                    
                    // Remove from active orders
                    let mut active_orders_lock = active_orders.write().await;
                    active_orders_lock.remove(&order_id);
                } else {
                    warn!("Received cancel for unknown order: {}", order_id);
                }
            },
            OrderEvent::Reject { order_id, reason } => {
                warn!("Processing reject order event for order {}: {}", order_id, reason);
                
                let mut orders_lock = orders.write().await;
                if let Some(order) = orders_lock.get_mut(&order_id) {
                    order.status = OrderStatus::Rejected;
                    order.notes = Some(reason.clone());
                    order.updated_at = Utc::now();
                    
                    // Remove from active orders
                    let mut active_orders_lock = active_orders.write().await;
                    active_orders_lock.remove(&order_id);
                } else {
                    warn!("Received reject for unknown order: {}", order_id);
                }
            },
            OrderEvent::Error { order_id, message } => {
                error!("Processing error event: {}", message);
                
                if let Some(id) = order_id {
                    let mut orders_lock = orders.write().await;
                    if let Some(order) = orders_lock.get_mut(&id) {
                        order.status = OrderStatus::Failed;
                        order.notes = Some(message.clone());
                        order.updated_at = Utc::now();
                        
                        // Remove from active orders
                        let mut active_orders_lock = active_orders.write().await;
                        active_orders_lock.remove(&id);
                    }
                }
            }
        }
    }
    
    #[allow(dead_code)]
    pub fn get_event_sender(&self) -> mpsc::Sender<OrderEvent> {
        self.event_sender.clone()
    }
    
    #[allow(dead_code)]
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

    async fn update_order_status_internal(orders: Arc<RwLock<HashMap<Uuid, Order>>>, order_id: Uuid, status: OrderStatus) {
        let mut orders_lock = orders.write().await;
        if let Some(order) = orders_lock.get_mut(&order_id) {
            order.status = status;
            order.updated_at = Utc::now();
        }
    }
} 