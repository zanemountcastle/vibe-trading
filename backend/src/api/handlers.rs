use actix_web::{web, HttpResponse, Responder};
use chrono::Utc;
use serde::Deserialize;
use uuid::Uuid;

use crate::api::{AppState, error_response, success_response};
use crate::strategy::{StrategyParams, TradeDirection, TimeInForce};
use crate::order::{Order, OrderType};

// Health check handler
pub async fn health_check() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({
        "status": "ok",
        "timestamp": Utc::now().to_rfc3339(),
    }))
}

// Market data handlers
pub async fn get_market_data(
    state: web::Data<AppState>,
    path: web::Path<String>,
) -> impl Responder {
    let symbol = path.into_inner();
    
    // Get market data manager
    let market_data_manager = state.market_data_manager.read().await;
    
    // Get current market data
    let current_data = market_data_manager.get_current_data();
    let data = current_data.read().await;
    
    // Check if we have data for the requested symbol
    if let Some(asset_data) = data.asset_data.get(&symbol) {
        success_response(asset_data)
    } else {
        error_response(&format!("No data available for symbol: {}", symbol))
    }
}

pub async fn get_symbols(
    state: web::Data<AppState>,
) -> impl Responder {
    // Get market data manager
    let market_data_manager = state.market_data_manager.read().await;
    
    // Get current market data
    let current_data = market_data_manager.get_current_data();
    let data = current_data.read().await;
    
    // Return all available symbols
    let symbols: Vec<String> = data.asset_data.keys().cloned().collect();
    success_response(symbols)
}

// Strategy handlers
pub async fn get_strategies(
    state: web::Data<AppState>,
) -> impl Responder {
    // Get strategy manager
    let _strategy_manager = state.strategy_manager.read().await;
    
    // TODO: Implement this function in StrategyManager
    // For now, return mock data
    let strategies = vec![
        "Statistical Arbitrage".to_string(),
        "Event Arbitrage".to_string(),
        "Information Arbitrage".to_string(),
        "Latency Arbitrage".to_string(),
        "Day Trading".to_string(),
    ];
    
    success_response(strategies)
}

pub async fn get_active_strategy(
    state: web::Data<AppState>,
) -> impl Responder {
    // Get strategy manager
    let _strategy_manager = state.strategy_manager.read().await;
    
    // TODO: Implement this function in StrategyManager
    // For now, return mock data
    let active_strategy = "Statistical Arbitrage".to_string();
    
    success_response(active_strategy)
}

#[derive(Deserialize)]
pub struct SetActiveStrategyRequest {
    name: String,
}

pub async fn set_active_strategy(
    state: web::Data<AppState>,
    req: web::Json<SetActiveStrategyRequest>,
) -> impl Responder {
    // Get strategy manager
    let mut strategy_manager = state.strategy_manager.write().await;
    
    // Try to set the active strategy
    match strategy_manager.set_active_strategy(&req.name) {
        Ok(()) => {
            success_response(serde_json::json!({
                "success": true,
                "message": format!("Active strategy set to: {}", req.name),
            }))
        },
        Err(e) => {
            error_response(&e)
        }
    }
}

pub async fn get_strategy_params(
    _state: web::Data<AppState>,
    path: web::Path<String>,
) -> impl Responder {
    let name = path.into_inner();
    
    // TODO: Implement this function in StrategyManager
    // For now, return mock data
    let params = match name.as_str() {
        "Statistical Arbitrage" => {
            serde_json::json!({
                "correlation_threshold": 0.8,
                "z_score_threshold": 2.0,
                "lookback_period": 100,
                "max_position_size": 100000.0,
            })
        },
        "Event Arbitrage" => {
            serde_json::json!({
                "event_sources": ["Bloomberg", "Reuters", "Twitter"],
                "reaction_time_ms": 50,
                "max_position_size": 100000.0,
            })
        },
        "Information Arbitrage" => {
            serde_json::json!({
                "news_sources": ["Bloomberg", "Reuters", "Twitter", "Reddit"],
                "sentiment_threshold": 0.7,
                "max_position_size": 50000.0,
            })
        },
        "Latency Arbitrage" => {
            serde_json::json!({
                "exchanges": ["Binance", "Coinbase", "Kraken"],
                "min_price_difference_pct": 0.05,
                "max_position_size": 200000.0,
            })
        },
        "Day Trading" => {
            serde_json::json!({
                "time_frame_minutes": 15,
                "rsi_period": 14,
                "rsi_overbought": 70,
                "rsi_oversold": 30,
                "max_position_size": 50000.0,
            })
        },
        _ => {
            return error_response(&format!("Strategy not found: {}", name));
        }
    };
    
    success_response(params)
}

pub async fn update_strategy_params(
    state: web::Data<AppState>,
    path: web::Path<String>,
    params: web::Json<serde_json::Value>,
) -> impl Responder {
    let name = path.into_inner();
    
    // Convert the JSON value to StrategyParams
    let strategy_params = StrategyParams {
        params: params.as_object()
            .unwrap_or(&serde_json::Map::new())
            .iter()
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect(),
    };
    
    // Get strategy manager
    let mut strategy_manager = state.strategy_manager.write().await;
    
    // Update the strategy parameters
    match strategy_manager.update_strategy_params(&name, strategy_params) {
        Ok(()) => {
            success_response(serde_json::json!({
                "success": true,
                "message": format!("Updated parameters for strategy: {}", name),
            }))
        },
        Err(e) => {
            error_response(&e)
        }
    }
}

pub async fn evaluate_strategies(
    state: web::Data<AppState>,
) -> impl Responder {
    // Get strategy manager and market data
    let strategy_manager = state.strategy_manager.read().await;
    let market_data_manager = state.market_data_manager.read().await;
    
    // Get current market data
    let current_data = market_data_manager.get_current_data();
    let data = current_data.read().await;
    
    // Evaluate all strategies
    let results = strategy_manager.evaluate_strategies(&data);
    
    // Get the best strategy
    let best_strategy = strategy_manager.get_best_strategy(&results);
    
    // Format the results for response
    let formatted_results: serde_json::Value = serde_json::json!({
        "timestamp": data.timestamp.to_rfc3339(),
        "results": results.iter().map(|(name, result)| {
            serde_json::json!({
                "strategy": name,
                "confidence": result.confidence,
                "expected_profit": result.expected_profit,
                "signals": result.signals.len(),
                "is_best": best_strategy.as_ref().map_or(false, |best| best == name),
            })
        }).collect::<Vec<_>>(),
        "best_strategy": best_strategy,
    });
    
    success_response(formatted_results)
}

// Order handlers
#[derive(Deserialize)]
pub struct PlaceOrderRequest {
    symbol: String,
    direction: String, // "buy" or "sell"
    order_type: String, // "market", "limit", etc.
    quantity: f64,
    price: Option<f64>,
    stop_price: Option<f64>,
    time_in_force: Option<String>, // "gtc", "ioc", etc.
    strategy_id: Option<String>,
}

pub async fn place_order(
    state: web::Data<AppState>,
    req: web::Json<PlaceOrderRequest>,
) -> impl Responder {
    // Convert request to Order
    let direction = match req.direction.to_lowercase().as_str() {
        "buy" => TradeDirection::Buy,
        "sell" => TradeDirection::Sell,
        _ => return error_response("Invalid direction: must be 'buy' or 'sell'"),
    };
    
    let order_type = match req.order_type.to_lowercase().as_str() {
        "market" => OrderType::Market,
        "limit" => OrderType::Limit,
        "stop" | "stoploss" => OrderType::StopLoss,
        "stoplimit" => OrderType::StopLimit,
        "trailingstop" => OrderType::TrailingStop,
        _ => return error_response("Invalid order type"),
    };
    
    let time_in_force = match req.time_in_force.as_deref() {
        Some("gtc") => TimeInForce::GoodTilCanceled,
        Some("ioc") => TimeInForce::ImmediateOrCancel,
        Some("fok") => TimeInForce::FillOrKill,
        Some("day") => TimeInForce::DayOnly,
        None => TimeInForce::GoodTilCanceled, // Default
        _ => return error_response("Invalid time in force"),
    };
    
    // Validate basic order parameters
    if req.quantity <= 0.0 {
        return error_response("Quantity must be positive");
    }
    
    if order_type == OrderType::Limit && req.price.is_none() {
        return error_response("Limit orders require a price");
    }
    
    if (order_type == OrderType::StopLoss || order_type == OrderType::StopLimit) && req.stop_price.is_none() {
        return error_response("Stop orders require a stop price");
    }
    
    // Create order object
    let order = Order {
        id: Uuid::new_v4(),
        client_order_id: format!("API-{}", Uuid::new_v4().as_simple()),
        symbol: req.symbol.clone(),
        direction,
        order_type,
        quantity: req.quantity,
        filled_quantity: 0.0,
        price: req.price,
        stop_price: req.stop_price,
        time_in_force,
        status: crate::order::OrderStatus::Created,
        exchange: "".to_string(), // Will be determined by order router
        created_at: Utc::now(),
        updated_at: Utc::now(),
        filled_at: None,
        average_fill_price: None,
        strategy_id: req.strategy_id.clone(),
        notes: None,
    };
    
    // Get order manager
    let order_manager = state.order_manager.read().await;
    
    // Place the order
    match order_manager.place_order(order).await {
        Ok(order_id) => {
            success_response(serde_json::json!({
                "order_id": order_id.to_string(),
                "status": "created",
            }))
        },
        Err(e) => {
            error_response(&e)
        }
    }
}

pub async fn get_orders(
    state: web::Data<AppState>,
) -> impl Responder {
    // Get order manager
    let order_manager = state.order_manager.read().await;
    
    // Get active orders
    let orders = order_manager.get_active_orders().await;
    
    // Format orders for response
    let formatted_orders: Vec<serde_json::Value> = orders.iter().map(|order| {
        serde_json::json!({
            "id": order.id.to_string(),
            "symbol": order.symbol,
            "direction": match order.direction {
                TradeDirection::Buy => "buy",
                TradeDirection::Sell => "sell",
            },
            "order_type": format!("{:?}", order.order_type).to_lowercase(),
            "quantity": order.quantity,
            "filled_quantity": order.filled_quantity,
            "price": order.price,
            "stop_price": order.stop_price,
            "status": format!("{:?}", order.status).to_lowercase(),
            "created_at": order.created_at.to_rfc3339(),
            "updated_at": order.updated_at.to_rfc3339(),
        })
    }).collect();
    
    success_response(formatted_orders)
}

pub async fn get_order(
    state: web::Data<AppState>,
    path: web::Path<String>,
) -> impl Responder {
    // Parse order ID
    let order_id = match Uuid::parse_str(&path.into_inner()) {
        Ok(id) => id,
        Err(_) => return error_response("Invalid order ID format"),
    };
    
    // Get order manager
    let order_manager = state.order_manager.read().await;
    
    // Get the order
    match order_manager.get_order(order_id).await {
        Some(order) => {
            // Format order for response
            let formatted_order = serde_json::json!({
                "id": order.id.to_string(),
                "client_order_id": order.client_order_id,
                "symbol": order.symbol,
                "direction": match order.direction {
                    TradeDirection::Buy => "buy",
                    TradeDirection::Sell => "sell",
                },
                "order_type": format!("{:?}", order.order_type).to_lowercase(),
                "quantity": order.quantity,
                "filled_quantity": order.filled_quantity,
                "price": order.price,
                "stop_price": order.stop_price,
                "time_in_force": format!("{:?}", order.time_in_force).to_lowercase(),
                "status": format!("{:?}", order.status).to_lowercase(),
                "exchange": order.exchange,
                "created_at": order.created_at.to_rfc3339(),
                "updated_at": order.updated_at.to_rfc3339(),
                "filled_at": order.filled_at.map(|dt| dt.to_rfc3339()),
                "average_fill_price": order.average_fill_price,
                "strategy_id": order.strategy_id,
                "notes": order.notes,
            });
            
            success_response(formatted_order)
        },
        None => {
            error_response(&format!("Order not found: {}", order_id))
        }
    }
}

#[derive(Deserialize)]
pub struct CancelOrderRequest {
    reason: Option<String>,
}

pub async fn cancel_order(
    state: web::Data<AppState>,
    path: web::Path<String>,
    req: web::Json<CancelOrderRequest>,
) -> impl Responder {
    // Parse order ID
    let order_id = match Uuid::parse_str(&path.into_inner()) {
        Ok(id) => id,
        Err(_) => return error_response("Invalid order ID format"),
    };
    
    // Get order manager
    let order_manager = state.order_manager.read().await;
    
    // Get cancellation reason
    let reason = req.reason.clone().unwrap_or_else(|| "User requested".to_string());
    
    // Cancel the order
    match order_manager.cancel_order(order_id, reason.clone()).await {
        Ok(()) => {
            success_response(serde_json::json!({
                "order_id": order_id.to_string(),
                "status": "cancelled",
                "reason": reason,
            }))
        },
        Err(e) => {
            error_response(&e)
        }
    }
}

// Account handlers
pub async fn get_account_balance(
    _state: web::Data<AppState>,
) -> impl Responder {
    // TODO: Implement this once we have account management
    // For now, return mock data
    
    let balance = serde_json::json!({
        "total": 1000000.0,
        "available": 750000.0,
        "currency": "USD",
        "additional_balances": [
            {"currency": "BTC", "amount": 2.5},
            {"currency": "ETH", "amount": 30.0},
            {"currency": "SOL", "amount": 150.0},
        ],
        "timestamp": Utc::now().to_rfc3339(),
    });
    
    success_response(balance)
}

pub async fn get_positions(
    _state: web::Data<AppState>,
) -> impl Responder {
    // TODO: Implement this once we have position tracking
    // For now, return mock data
    
    let positions = serde_json::json!([
        {
            "symbol": "BTC/USD",
            "quantity": 2.5,
            "avg_price": 34500.0,
            "current_price": 35200.0,
            "unrealized_pnl": 1750.0,
            "realized_pnl": 2500.0,
            "timestamp": Utc::now().to_rfc3339(),
        },
        {
            "symbol": "ETH/USD",
            "quantity": 30.0,
            "avg_price": 2100.0,
            "current_price": 2250.0,
            "unrealized_pnl": 4500.0,
            "realized_pnl": 1200.0,
            "timestamp": Utc::now().to_rfc3339(),
        },
        {
            "symbol": "AAPL",
            "quantity": 500.0,
            "avg_price": 175.0,
            "current_price": 178.5,
            "unrealized_pnl": 1750.0,
            "realized_pnl": 3000.0,
            "timestamp": Utc::now().to_rfc3339(),
        },
    ]);
    
    success_response(positions)
}

// Backtest handlers
#[derive(Deserialize)]
pub struct BacktestRequest {
    strategy: String,
    start_date: String,
    end_date: String,
    symbols: Vec<String>,
    initial_capital: f64,
    #[allow(dead_code)]
    parameters: serde_json::Value,
}

pub async fn run_backtest(
    req: web::Json<BacktestRequest>,
) -> impl Responder {
    // TODO: Implement actual backtesting
    // For now, return mock data
    
    // Generate a random backtest ID
    let backtest_id = Uuid::new_v4();
    
    // Simulate backtesting delay
    tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
    
    let result = serde_json::json!({
        "id": backtest_id.to_string(),
        "strategy": req.strategy,
        "start_date": req.start_date,
        "end_date": req.end_date,
        "symbols": req.symbols,
        "initial_capital": req.initial_capital,
        "final_capital": req.initial_capital * 1.15, // 15% return
        "return_pct": 15.0,
        "annualized_return_pct": 28.5,
        "sharpe_ratio": 1.8,
        "max_drawdown_pct": 8.5,
        "trades": 120,
        "win_rate_pct": 62.5,
        "status": "completed",
        "timestamp": Utc::now().to_rfc3339(),
    });
    
    success_response(result)
}

pub async fn get_backtest_result(
    path: web::Path<String>,
) -> impl Responder {
    // Parse backtest ID
    let backtest_id = match Uuid::parse_str(&path.into_inner()) {
        Ok(id) => id,
        Err(_) => return error_response("Invalid backtest ID format"),
    };
    
    // TODO: Implement actual backtest result retrieval
    // For now, return mock data
    
    let result = serde_json::json!({
        "id": backtest_id.to_string(),
        "strategy": "Statistical Arbitrage",
        "start_date": "2023-01-01",
        "end_date": "2023-12-31",
        "symbols": ["BTC/USD", "ETH/USD"],
        "initial_capital": 1000000.0,
        "final_capital": 1150000.0, // 15% return
        "return_pct": 15.0,
        "annualized_return_pct": 28.5,
        "sharpe_ratio": 1.8,
        "max_drawdown_pct": 8.5,
        "trades": 120,
        "win_rate_pct": 62.5,
        "status": "completed",
        "timestamp": Utc::now().to_rfc3339(),
        "monthly_returns": [
            {"month": "2023-01", "return_pct": 2.1},
            {"month": "2023-02", "return_pct": 1.5},
            {"month": "2023-03", "return_pct": -0.8},
            {"month": "2023-04", "return_pct": 3.2},
            {"month": "2023-05", "return_pct": 1.7},
            {"month": "2023-06", "return_pct": -1.2},
            {"month": "2023-07", "return_pct": 2.5},
            {"month": "2023-08", "return_pct": 1.9},
            {"month": "2023-09", "return_pct": 0.8},
            {"month": "2023-10", "return_pct": -0.5},
            {"month": "2023-11", "return_pct": 1.6},
            {"month": "2023-12", "return_pct": 2.2},
        ],
    });
    
    success_response(result)
}

// Update the function signatures with unused state parameters
#[allow(dead_code)]
async fn get_health(
    _state: web::Data<AppState>,
) -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({
        "status": "ok",
        "version": "0.1.0",
        "services": {
            "database": "healthy",
            "market_data": "healthy",
            "order_system": "healthy",
            "strategy_engine": "healthy"
        },
        "uptime": "00:00:01",
        "mode": "simulation"
    }))
}

#[allow(dead_code)]
async fn get_api_documentation(
    _state: web::Data<AppState>,
) -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({
        "documentation": "API documentation available at /api-docs",
        "version": "0.1.0",
        "endpoints": [
            "/api/health",
            "/api/market/symbols",
            "/api/market/data/{symbol}",
            "/api/strategy",
            "/api/strategy/{name}/params",
            "/api/account/balance"
        ]
    }))
}

#[allow(dead_code)]
async fn websocket_documentation(
    _state: web::Data<AppState>,
) -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({
        "documentation": "WebSocket API provides real-time updates",
        "endpoint": "/ws",
        "message_types": [
            "Connect", "Subscribe", "Unsubscribe", "MarketData", 
            "OrderUpdate", "StrategyUpdate", "Notification"
        ],
        "status": "simulation mode - not currently available"
    }))
} 