use arb_platform::order::{
    Order, OrderType, OrderStatus, OrderManager, OrderEvent
};
use arb_platform::strategy::{TradeDirection, TimeInForce};

use chrono::Utc;
use std::time::Duration;
use tokio::test;
use uuid::Uuid;

fn create_test_order(symbol: &str, direction: TradeDirection, order_type: OrderType) -> Order {
    Order {
        id: Uuid::new_v4(),
        client_order_id: format!("test-{}", Uuid::new_v4().simple()),
        symbol: symbol.to_string(),
        direction,
        order_type: order_type.clone(),
        quantity: 1.0,
        filled_quantity: 0.0,
        price: match order_type {
            OrderType::Market => None,
            _ => Some(35000.0),
        },
        stop_price: match order_type {
            OrderType::StopLoss | OrderType::StopLimit => Some(34500.0),
            _ => None,
        },
        time_in_force: TimeInForce::GoodTilCancelled,
        status: OrderStatus::Created,
        exchange: "Test Exchange".to_string(),
        created_at: Utc::now(),
        updated_at: Utc::now(),
        filled_at: None,
        average_fill_price: None,
        strategy_id: Some("test_strategy".to_string()),
        notes: None,
    }
}

#[test]
async fn test_order_status_transitions() {
    // Test valid transitions
    assert!(OrderStatus::Created.can_transition_to(&OrderStatus::PendingSubmission));
    assert!(OrderStatus::PendingSubmission.can_transition_to(&OrderStatus::Submitted));
    assert!(OrderStatus::Submitted.can_transition_to(&OrderStatus::PartiallyFilled));
    assert!(OrderStatus::PartiallyFilled.can_transition_to(&OrderStatus::Filled));
    assert!(OrderStatus::Submitted.can_transition_to(&OrderStatus::Cancelled));
    
    // Test invalid transitions
    assert!(!OrderStatus::Filled.can_transition_to(&OrderStatus::Submitted));
    assert!(!OrderStatus::Cancelled.can_transition_to(&OrderStatus::PartiallyFilled));
    assert!(!OrderStatus::Rejected.can_transition_to(&OrderStatus::Submitted));
    assert!(!OrderStatus::Failed.can_transition_to(&OrderStatus::PendingSubmission));
}

#[test]
async fn test_order_type_validation() {
    // Market order (price should be None)
    let market_order = create_test_order("BTC/USD", TradeDirection::Buy, OrderType::Market);
    assert!(market_order.price.is_none());
    
    // Limit order (price should be Some)
    let limit_order = create_test_order("BTC/USD", TradeDirection::Sell, OrderType::Limit);
    assert!(limit_order.price.is_some());
    
    // Stop Loss order (stop price should be Some)
    let stop_loss_order = create_test_order("BTC/USD", TradeDirection::Sell, OrderType::StopLoss);
    assert!(stop_loss_order.stop_price.is_some());
    
    // Stop Limit order (both price and stop price should be Some)
    let stop_limit_order = create_test_order("BTC/USD", TradeDirection::Buy, OrderType::StopLimit);
    assert!(stop_limit_order.price.is_some());
    assert!(stop_limit_order.stop_price.is_some());
}

#[test]
async fn test_order_manager_creation() {
    let manager = OrderManager::new();
    
    // Verify the manager was created successfully with empty state
    let active_orders = manager.get_active_orders().await;
    assert!(active_orders.is_empty());
}

#[test]
async fn test_order_placement() {
    let manager = OrderManager::new();
    let order = create_test_order("BTC/USD", TradeDirection::Buy, OrderType::Limit);
    
    // Place the order
    let result = manager.place_order(order.clone()).await;
    assert!(result.is_ok());
    
    let order_id = result.unwrap();
    assert_eq!(order_id, order.id);
    
    // Verify the order exists
    let retrieved_order = manager.get_order(order_id).await;
    assert!(retrieved_order.is_some());
    
    // Verify it's in the active orders
    let active_orders = manager.get_active_orders().await;
    assert_eq!(active_orders.len(), 1);
    assert_eq!(active_orders[0].id, order_id);
}

#[test]
async fn test_order_cancellation() {
    let manager = OrderManager::new();
    let order = create_test_order("BTC/USD", TradeDirection::Buy, OrderType::Limit);
    
    // Place the order
    let result = manager.place_order(order.clone()).await;
    assert!(result.is_ok());
    let order_id = result.unwrap();
    
    // Cancel the order
    let cancel_result = manager.cancel_order(order_id, "Testing cancellation".to_string()).await;
    assert!(cancel_result.is_ok());
    
    // Verify the order status is updated
    let retrieved_order = manager.get_order(order_id).await;
    assert!(retrieved_order.is_some());
    assert_eq!(retrieved_order.unwrap().status, OrderStatus::Cancelled);
    
    // Verify it's no longer in active orders
    let active_orders = manager.get_active_orders().await;
    assert!(active_orders.is_empty());
}

#[test]
async fn test_multiple_order_placement_and_retrieval() {
    let manager = OrderManager::new();
    
    // Create and place multiple orders
    let order1 = create_test_order("BTC/USD", TradeDirection::Buy, OrderType::Limit);
    let order2 = create_test_order("ETH/USD", TradeDirection::Sell, OrderType::Market);
    let order3 = create_test_order("SOL/USD", TradeDirection::Buy, OrderType::StopLimit);
    
    let result1 = manager.place_order(order1.clone()).await;
    let result2 = manager.place_order(order2.clone()).await;
    let result3 = manager.place_order(order3.clone()).await;
    
    assert!(result1.is_ok());
    assert!(result2.is_ok());
    assert!(result3.is_ok());
    
    let id1 = result1.unwrap();
    let id2 = result2.unwrap();
    let id3 = result3.unwrap();
    
    // Verify active orders count
    let active_orders = manager.get_active_orders().await;
    assert_eq!(active_orders.len(), 3);
    
    // Cancel one order
    let cancel_result = manager.cancel_order(id2, "Testing cancellation".to_string()).await;
    assert!(cancel_result.is_ok());
    
    // Verify active orders updated
    let active_orders_after = manager.get_active_orders().await;
    assert_eq!(active_orders_after.len(), 2);
    
    // Verify individual orders by ID
    assert!(manager.get_order(id1).await.is_some());
    assert!(manager.get_order(id2).await.is_some()); // Still exists but not active
    assert!(manager.get_order(id3).await.is_some());
}

#[test]
async fn test_invalid_order_validation() {
    let manager = OrderManager::new();
    
    // Create an invalid market order with a price
    let mut invalid_market_order = create_test_order("BTC/USD", TradeDirection::Buy, OrderType::Market);
    invalid_market_order.price = Some(35000.0); // Market orders shouldn't have a price
    
    // Attempt to place the order
    let result = manager.place_order(invalid_market_order).await;
    assert!(result.is_err());
    
    // Create an invalid limit order without a price
    let mut invalid_limit_order = create_test_order("BTC/USD", TradeDirection::Sell, OrderType::Limit);
    invalid_limit_order.price = None; // Limit orders need a price
    
    // Attempt to place the order
    let result = manager.place_order(invalid_limit_order).await;
    assert!(result.is_err());
    
    // Create an invalid stop-loss order without a stop price
    let mut invalid_stop_order = create_test_order("BTC/USD", TradeDirection::Sell, OrderType::StopLoss);
    invalid_stop_order.stop_price = None; // Stop orders need a stop price
    
    // Attempt to place the order
    let result = manager.place_order(invalid_stop_order).await;
    assert!(result.is_err());
}

#[test]
async fn test_order_event_emission() {
    let manager = OrderManager::new();
    let order = create_test_order("BTC/USD", TradeDirection::Buy, OrderType::Limit);
    
    // Place the order
    let result = manager.place_order(order.clone()).await;
    assert!(result.is_ok());
    let order_id = result.unwrap();
    
    // Wait a bit for the order to be processed
    tokio::time::sleep(Duration::from_millis(50)).await;
    
    // Get the event sender for testing
    let event_sender = manager.get_event_sender();
    
    // Emit an update event
    let update_event = OrderEvent::Update {
        order_id,
        status: Some(OrderStatus::PartiallyFilled),
        filled_qty: Some(0.5),
        avg_fill_price: Some(35100.0),
    };
    
    // Send the event
    let send_result = event_sender.send(update_event).await;
    assert!(send_result.is_ok());
    
    // Give some time for event processing
    tokio::time::sleep(Duration::from_millis(100)).await;
    
    // Verify the order was updated
    let updated_order = manager.get_order(order_id).await;
    assert!(updated_order.is_some());
    
    // Verify the order status and attributes were updated correctly
    let updated_order = updated_order.unwrap();
    assert_eq!(updated_order.status, OrderStatus::PartiallyFilled);
    assert_eq!(updated_order.filled_quantity, 0.5);
    assert_eq!(updated_order.average_fill_price, Some(35100.0));
}

#[test]
async fn test_cancel_nonexistent_order() {
    let manager = OrderManager::new();
    let nonexistent_id = Uuid::new_v4();
    
    // Try to cancel an order that doesn't exist
    let result = manager.cancel_order(nonexistent_id, "Testing cancellation".to_string()).await;
    assert!(result.is_err());
}

#[test]
async fn test_get_nonexistent_order() {
    let manager = OrderManager::new();
    let nonexistent_id = Uuid::new_v4();
    
    // Try to get an order that doesn't exist
    let result = manager.get_order(nonexistent_id).await;
    assert!(result.is_none());
}

#[test]
async fn test_order_direction() {
    // Buy order
    let buy_order = create_test_order("BTC/USD", TradeDirection::Buy, OrderType::Limit);
    assert_eq!(buy_order.direction, TradeDirection::Buy);
    
    // Sell order
    let sell_order = create_test_order("ETH/USD", TradeDirection::Sell, OrderType::Market);
    assert_eq!(sell_order.direction, TradeDirection::Sell);
}

#[test]
async fn test_time_in_force() {
    // GoodTilCancelled is the default in our test function
    let gtc_order = create_test_order("BTC/USD", TradeDirection::Buy, OrderType::Limit);
    assert_eq!(gtc_order.time_in_force, TimeInForce::GoodTilCancelled);
    
    // Create a day order
    let mut day_order = create_test_order("ETH/USD", TradeDirection::Sell, OrderType::Limit);
    day_order.time_in_force = TimeInForce::Day;
    assert_eq!(day_order.time_in_force, TimeInForce::Day);
    
    // Create a fill-or-kill order
    let mut fok_order = create_test_order("SOL/USD", TradeDirection::Buy, OrderType::Market);
    fok_order.time_in_force = TimeInForce::FillOrKill;
    assert_eq!(fok_order.time_in_force, TimeInForce::FillOrKill);
    
    // Create an immediate-or-cancel order
    let mut ioc_order = create_test_order("ADA/USD", TradeDirection::Sell, OrderType::Limit);
    ioc_order.time_in_force = TimeInForce::ImmediateOrCancel;
    assert_eq!(ioc_order.time_in_force, TimeInForce::ImmediateOrCancel);
} 