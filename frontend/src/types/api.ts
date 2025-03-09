/**
 * API Type Definitions
 * 
 * This file contains TypeScript interfaces for all API request and response data structures.
 */

// ======= Market Data Types =======

export interface MarketData {
  symbol: string;
  price: number;
  bid: number;
  ask: number;
  volume: number;
  timestamp: string;
  exchange: string;
}

// ======= Strategy Types =======

export interface StrategyParams {
  [key: string]: any;
}

export interface StrategyEvaluationResult {
  strategy: string;
  confidence: number;
  expected_profit: number;
  signals: number;
  is_best: boolean;
}

export interface StrategyEvaluation {
  timestamp: string;
  results: StrategyEvaluationResult[];
  best_strategy: string | null;
}

// ======= Order Types =======

export type OrderDirection = 'buy' | 'sell';
export type OrderType = 'market' | 'limit' | 'stop' | 'stoplimit' | 'trailingstop';
export type OrderStatus = 'created' | 'pendingsubmission' | 'submitted' | 'partiallyfilled' | 'filled' | 'cancelled' | 'rejected' | 'failed';
export type TimeInForce = 'gtc' | 'ioc' | 'fok' | 'day';

export interface OrderRequest {
  symbol: string;
  direction: OrderDirection;
  order_type: OrderType;
  quantity: number;
  price?: number;
  stop_price?: number;
  time_in_force?: TimeInForce;
  strategy_id?: string;
}

export interface Order {
  id: string;
  client_order_id: string;
  symbol: string;
  direction: OrderDirection;
  order_type: OrderType;
  quantity: number;
  filled_quantity: number;
  price: number | null;
  stop_price: number | null;
  time_in_force: TimeInForce;
  status: OrderStatus;
  exchange: string;
  created_at: string;
  updated_at: string;
  filled_at: string | null;
  average_fill_price: number | null;
  strategy_id: string | null;
  notes: string | null;
}

export interface OrderCreationResponse {
  order_id: string;
  status: string;
}

export interface CancelOrderRequest {
  reason?: string;
}

export interface CancelOrderResponse {
  order_id: string;
  status: string;
  reason: string;
}

// ======= Account Types =======

export interface Balance {
  currency: string;
  amount: number;
}

export interface AccountBalance {
  total: number;
  available: number;
  currency: string;
  additional_balances: Balance[];
  timestamp: string;
}

export interface Position {
  symbol: string;
  quantity: number;
  avg_price: number;
  current_price: number;
  unrealized_pnl: number;
  realized_pnl: number;
  timestamp: string;
}

// ======= Backtest Types =======

export interface BacktestRequest {
  strategy: string;
  start_date: string;
  end_date: string;
  symbols: string[];
  initial_capital: number;
  parameters: StrategyParams;
}

export interface MonthlyReturn {
  month: string;
  return_pct: number;
}

export interface BacktestResult {
  id: string;
  strategy: string;
  start_date: string;
  end_date: string;
  symbols: string[];
  initial_capital: number;
  final_capital: number;
  return_pct: number;
  annualized_return_pct: number;
  sharpe_ratio: number;
  max_drawdown_pct: number;
  trades: number;
  win_rate_pct: number;
  status: 'running' | 'completed' | 'failed';
  timestamp: string;
  monthly_returns?: MonthlyReturn[];
}

// ======= WebSocket Types =======

export type WebSocketFeed = 'market_data' | 'order_updates' | 'strategy_updates';

export interface WebSocketSubscribeRequest {
  type: 'Subscribe';
  payload: {
    feed: WebSocketFeed;
    symbols?: string[];
  };
}

export interface WebSocketUnsubscribeRequest {
  type: 'Unsubscribe';
  payload: {
    feed: WebSocketFeed;
    symbols?: string[];
  };
}

export interface WebSocketMarketDataMessage {
  type: 'MarketData';
  payload: MarketData;
}

export interface WebSocketOrderUpdateMessage {
  type: 'OrderUpdate';
  payload: {
    order_id: string;
    status: string;
    filled_quantity: number;
    average_price: number | null;
    timestamp: string;
  };
}

export interface WebSocketStrategyUpdateMessage {
  type: 'StrategyUpdate';
  payload: {
    strategy: string;
    confidence: number;
    expected_profit: number;
    timestamp: string;
  };
}

export interface WebSocketNotificationMessage {
  type: 'Notification';
  payload: {
    level: 'info' | 'warning' | 'error';
    message: string;
    timestamp: string;
  };
}

export interface WebSocketErrorMessage {
  type: 'Error';
  payload: {
    code: string;
    message: string;
  };
}

export type WebSocketMessage =
  | WebSocketMarketDataMessage
  | WebSocketOrderUpdateMessage
  | WebSocketStrategyUpdateMessage
  | WebSocketNotificationMessage
  | WebSocketErrorMessage; 