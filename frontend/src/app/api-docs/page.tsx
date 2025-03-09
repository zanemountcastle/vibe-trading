'use client';

import React, { useState } from 'react';
import Link from 'next/link';

// API endpoint definition type
interface Endpoint {
  path: string;
  method: 'GET' | 'POST' | 'PUT' | 'DELETE';
  description: string;
  parameters?: {
    name: string;
    type: string;
    required: boolean;
    description: string;
  }[];
  responseExample: string;
  requestExample?: string;
}

// API categories
interface ApiCategory {
  name: string;
  description: string;
  endpoints: Endpoint[];
}

export default function ApiDocs() {
  const [activeCategory, setActiveCategory] = useState('health');
  const [activeEndpoint, setActiveEndpoint] = useState<string | null>(null);

  const apiCategories: ApiCategory[] = [
    {
      name: 'health',
      description: 'System health information',
      endpoints: [
        {
          path: '/api/health',
          method: 'GET',
          description: 'Get system health status',
          responseExample: JSON.stringify({
            status: 'ok',
            timestamp: '2023-06-15T12:34:56Z',
          }, null, 2),
        },
      ],
    },
    {
      name: 'market',
      description: 'Market data endpoints',
      endpoints: [
        {
          path: '/api/market/symbols',
          method: 'GET',
          description: 'Get all available trading symbols',
          responseExample: JSON.stringify({
            data: ['BTC/USD', 'ETH/USD', 'AAPL', 'MSFT', 'TSLA'],
          }, null, 2),
        },
        {
          path: '/api/market/data/{symbol}',
          method: 'GET',
          description: 'Get market data for a specific symbol',
          parameters: [
            {
              name: 'symbol',
              type: 'string',
              required: true,
              description: 'Trading symbol (e.g., BTC/USD)',
            },
          ],
          responseExample: JSON.stringify({
            data: {
              symbol: 'BTC/USD',
              price: 35245.67,
              bid: 35240.23,
              ask: 35250.12,
              volume: 15234.51,
              timestamp: '2023-06-15T12:34:56Z',
              exchange: 'Binance',
            },
          }, null, 2),
        },
      ],
    },
    {
      name: 'strategy',
      description: 'Trading strategy management',
      endpoints: [
        {
          path: '/api/strategy',
          method: 'GET',
          description: 'Get all available strategies',
          responseExample: JSON.stringify({
            data: [
              'Statistical Arbitrage',
              'Event Arbitrage',
              'Information Arbitrage',
              'Latency Arbitrage',
              'Day Trading',
            ],
          }, null, 2),
        },
        {
          path: '/api/strategy/active',
          method: 'GET',
          description: 'Get the currently active strategy',
          responseExample: JSON.stringify({
            data: 'Statistical Arbitrage',
          }, null, 2),
        },
        {
          path: '/api/strategy/active',
          method: 'PUT',
          description: 'Set the active strategy',
          parameters: [
            {
              name: 'name',
              type: 'string',
              required: true,
              description: 'Strategy name',
            },
          ],
          requestExample: JSON.stringify({
            name: 'Statistical Arbitrage',
          }, null, 2),
          responseExample: JSON.stringify({
            data: {
              success: true,
              message: 'Active strategy set to: Statistical Arbitrage',
            },
          }, null, 2),
        },
        {
          path: '/api/strategy/{name}/params',
          method: 'GET',
          description: 'Get parameters for a specific strategy',
          parameters: [
            {
              name: 'name',
              type: 'string',
              required: true,
              description: 'Strategy name',
            },
          ],
          responseExample: JSON.stringify({
            data: {
              correlation_threshold: 0.8,
              z_score_threshold: 2.0,
              lookback_period: 100,
              max_position_size: 100000.0,
            },
          }, null, 2),
        },
        {
          path: '/api/strategy/{name}/params',
          method: 'PUT',
          description: 'Update parameters for a specific strategy',
          parameters: [
            {
              name: 'name',
              type: 'string',
              required: true,
              description: 'Strategy name',
            },
          ],
          requestExample: JSON.stringify({
            correlation_threshold: 0.85,
            z_score_threshold: 2.5,
            lookback_period: 120,
            max_position_size: 120000.0,
          }, null, 2),
          responseExample: JSON.stringify({
            data: {
              success: true,
              message: 'Updated parameters for strategy: Statistical Arbitrage',
            },
          }, null, 2),
        },
        {
          path: '/api/strategy/evaluate',
          method: 'POST',
          description: 'Evaluate all strategies against current market data',
          responseExample: JSON.stringify({
            data: {
              timestamp: '2023-06-15T12:34:56Z',
              results: [
                {
                  strategy: 'Statistical Arbitrage',
                  confidence: 0.87,
                  expected_profit: 12500.32,
                  signals: 3,
                  is_best: true,
                },
                {
                  strategy: 'Event Arbitrage',
                  confidence: 0.75,
                  expected_profit: 8200.45,
                  signals: 2,
                  is_best: false,
                },
              ],
              best_strategy: 'Statistical Arbitrage',
            },
          }, null, 2),
        },
      ],
    },
    {
      name: 'order',
      description: 'Order management',
      endpoints: [
        {
          path: '/api/order',
          method: 'GET',
          description: 'Get all active orders',
          responseExample: JSON.stringify({
            data: [
              {
                id: '123e4567-e89b-12d3-a456-426614174000',
                symbol: 'BTC/USD',
                direction: 'buy',
                order_type: 'limit',
                quantity: 0.5,
                filled_quantity: 0.0,
                price: 34500.0,
                stop_price: null,
                status: 'submitted',
                created_at: '2023-06-15T12:30:45Z',
                updated_at: '2023-06-15T12:30:45Z',
              },
            ],
          }, null, 2),
        },
        {
          path: '/api/order',
          method: 'POST',
          description: 'Place a new order',
          requestExample: JSON.stringify({
            symbol: 'BTC/USD',
            direction: 'buy',
            order_type: 'limit',
            quantity: 0.5,
            price: 34500.0,
            time_in_force: 'gtc',
          }, null, 2),
          responseExample: JSON.stringify({
            data: {
              order_id: '123e4567-e89b-12d3-a456-426614174000',
              status: 'created',
            },
          }, null, 2),
        },
        {
          path: '/api/order/{id}',
          method: 'GET',
          description: 'Get details of a specific order',
          parameters: [
            {
              name: 'id',
              type: 'UUID',
              required: true,
              description: 'Order ID',
            },
          ],
          responseExample: JSON.stringify({
            data: {
              id: '123e4567-e89b-12d3-a456-426614174000',
              client_order_id: 'API-123',
              symbol: 'BTC/USD',
              direction: 'buy',
              order_type: 'limit',
              quantity: 0.5,
              filled_quantity: 0.0,
              price: 34500.0,
              stop_price: null,
              time_in_force: 'goodtilcanceled',
              status: 'submitted',
              exchange: 'Binance',
              created_at: '2023-06-15T12:30:45Z',
              updated_at: '2023-06-15T12:30:45Z',
              filled_at: null,
              average_fill_price: null,
              strategy_id: 'Statistical Arbitrage',
              notes: null,
            },
          }, null, 2),
        },
        {
          path: '/api/order/{id}/cancel',
          method: 'POST',
          description: 'Cancel an existing order',
          parameters: [
            {
              name: 'id',
              type: 'UUID',
              required: true,
              description: 'Order ID',
            },
          ],
          requestExample: JSON.stringify({
            reason: 'Strategy signal changed',
          }, null, 2),
          responseExample: JSON.stringify({
            data: {
              order_id: '123e4567-e89b-12d3-a456-426614174000',
              status: 'cancelled',
              reason: 'Strategy signal changed',
            },
          }, null, 2),
        },
      ],
    },
    {
      name: 'account',
      description: 'Account information',
      endpoints: [
        {
          path: '/api/account/balance',
          method: 'GET',
          description: 'Get account balance information',
          responseExample: JSON.stringify({
            data: {
              total: 1000000.0,
              available: 750000.0,
              currency: 'USD',
              additional_balances: [
                { currency: 'BTC', amount: 2.5 },
                { currency: 'ETH', amount: 30.0 },
                { currency: 'SOL', amount: 150.0 },
              ],
              timestamp: '2023-06-15T12:34:56Z',
            },
          }, null, 2),
        },
        {
          path: '/api/account/positions',
          method: 'GET',
          description: 'Get current positions',
          responseExample: JSON.stringify({
            data: [
              {
                symbol: 'BTC/USD',
                quantity: 2.5,
                avg_price: 34500.0,
                current_price: 35200.0,
                unrealized_pnl: 1750.0,
                realized_pnl: 2500.0,
                timestamp: '2023-06-15T12:34:56Z',
              },
              {
                symbol: 'ETH/USD',
                quantity: 30.0,
                avg_price: 2100.0,
                current_price: 2250.0,
                unrealized_pnl: 4500.0,
                realized_pnl: 1200.0,
                timestamp: '2023-06-15T12:34:56Z',
              },
            ],
          }, null, 2),
        },
      ],
    },
    {
      name: 'backtest',
      description: 'Strategy backtesting',
      endpoints: [
        {
          path: '/api/backtest',
          method: 'POST',
          description: 'Run a backtest for a strategy',
          requestExample: JSON.stringify({
            strategy: 'Statistical Arbitrage',
            start_date: '2023-01-01',
            end_date: '2023-06-01',
            symbols: ['BTC/USD', 'ETH/USD'],
            initial_capital: 1000000.0,
            parameters: {
              correlation_threshold: 0.8,
              z_score_threshold: 2.0,
              lookback_period: 100,
              max_position_size: 100000.0,
            },
          }, null, 2),
          responseExample: JSON.stringify({
            data: {
              id: '123e4567-e89b-12d3-a456-426614174000',
              strategy: 'Statistical Arbitrage',
              start_date: '2023-01-01',
              end_date: '2023-06-01',
              symbols: ['BTC/USD', 'ETH/USD'],
              initial_capital: 1000000.0,
              final_capital: 1150000.0,
              return_pct: 15.0,
              annualized_return_pct: 28.5,
              sharpe_ratio: 1.8,
              max_drawdown_pct: 8.5,
              trades: 120,
              win_rate_pct: 62.5,
              status: 'completed',
              timestamp: '2023-06-15T12:34:56Z',
            },
          }, null, 2),
        },
        {
          path: '/api/backtest/{id}',
          method: 'GET',
          description: 'Get results of a completed backtest',
          parameters: [
            {
              name: 'id',
              type: 'UUID',
              required: true,
              description: 'Backtest ID',
            },
          ],
          responseExample: JSON.stringify({
            data: {
              id: '123e4567-e89b-12d3-a456-426614174000',
              strategy: 'Statistical Arbitrage',
              start_date: '2023-01-01',
              end_date: '2023-06-01',
              symbols: ['BTC/USD', 'ETH/USD'],
              initial_capital: 1000000.0,
              final_capital: 1150000.0,
              return_pct: 15.0,
              annualized_return_pct: 28.5,
              sharpe_ratio: 1.8,
              max_drawdown_pct: 8.5,
              trades: 120,
              win_rate_pct: 62.5,
              status: 'completed',
              timestamp: '2023-06-15T12:34:56Z',
              monthly_returns: [
                { month: '2023-01', return_pct: 2.1 },
                { month: '2023-02', return_pct: 1.5 },
                { month: '2023-03', return_pct: -0.8 },
                { month: '2023-04', return_pct: 3.2 },
                { month: '2023-05', return_pct: 1.7 },
                { month: '2023-06', return_pct: -1.2 },
              ],
            },
          }, null, 2),
        },
      ],
    },
    {
      name: 'websocket',
      description: 'Real-time WebSocket API',
      endpoints: [
        {
          path: '/ws',
          method: 'GET',
          description: 'WebSocket connection for real-time updates',
          responseExample: `// Connect to the WebSocket
const socket = new WebSocket('ws://localhost:3000/ws');

// Send subscription message
socket.send(JSON.stringify({
  type: 'Subscribe',
  payload: {
    feed: 'market_data',
    symbols: ['BTC/USD', 'ETH/USD']
  }
}));

// Handle incoming messages
socket.onmessage = (event) => {
  const data = JSON.parse(event.data);
  console.log(data);
};`,
        },
      ],
    },
  ];

  return (
    <div className="min-h-screen bg-gray-50 dark:bg-gray-900">
      {/* Header */}
      <header className="bg-gray-900 text-white shadow-md">
        <div className="container mx-auto px-4 py-4 flex justify-between items-center">
          <div className="flex items-center">
            <Link href="/" className="text-2xl font-bold hover:text-primary-400 transition-colors">
              ARB Platform
            </Link>
            <span className="ml-2 text-xs bg-primary-600 text-white px-2 py-1 rounded">BETA</span>
          </div>
          <nav className="hidden md:flex space-x-6">
            <Link href="/dashboard" className="hover:text-primary-400 transition-colors">
              Dashboard
            </Link>
            <Link href="/trading" className="hover:text-primary-400 transition-colors">
              Trading
            </Link>
            <Link href="/strategies" className="hover:text-primary-400 transition-colors">
              Strategies
            </Link>
            <Link href="/backtesting" className="hover:text-primary-400 transition-colors">
              Backtesting
            </Link>
            <Link href="/api-docs" className="text-primary-400 border-b-2 border-primary-400 pb-1">
              API Docs
            </Link>
          </nav>
        </div>
      </header>

      <div className="container mx-auto px-4 py-8">
        <h1 className="text-3xl font-bold mb-8 dark:text-white">API Documentation</h1>
        
        <div className="flex flex-col md:flex-row gap-8">
          {/* Sidebar */}
          <div className="md:w-1/4">
            <div className="bg-white dark:bg-gray-800 rounded-lg shadow-md p-4 sticky top-4">
              <h2 className="text-xl font-semibold mb-4 dark:text-white">API Endpoints</h2>
              <ul className="space-y-2">
                {apiCategories.map((category) => (
                  <li key={category.name}>
                    <button
                      onClick={() => setActiveCategory(category.name)}
                      className={`w-full text-left p-2 rounded-lg transition-colors ${
                        activeCategory === category.name
                          ? 'bg-primary-100 dark:bg-primary-900 text-primary-600 dark:text-primary-300'
                          : 'hover:bg-gray-100 dark:hover:bg-gray-700 text-gray-700 dark:text-gray-300'
                      }`}
                    >
                      {category.name.charAt(0).toUpperCase() + category.name.slice(1)}
                    </button>
                    {activeCategory === category.name && (
                      <ul className="ml-4 mt-2 space-y-1">
                        {category.endpoints.map((endpoint) => (
                          <li key={`${endpoint.method}-${endpoint.path}`}>
                            <button
                              onClick={() => setActiveEndpoint(`${endpoint.method}-${endpoint.path}`)}
                              className={`w-full text-left p-1 rounded text-sm transition-colors ${
                                activeEndpoint === `${endpoint.method}-${endpoint.path}`
                                  ? 'bg-gray-200 dark:bg-gray-700 font-medium'
                                  : 'hover:bg-gray-100 dark:hover:bg-gray-700 text-gray-600 dark:text-gray-400'
                              }`}
                            >
                              <span className={`inline-block w-16 text-xs font-mono px-2 py-1 rounded mr-2 
                                ${endpoint.method === 'GET' ? 'bg-green-100 text-green-800 dark:bg-green-900 dark:text-green-300' : ''}
                                ${endpoint.method === 'POST' ? 'bg-blue-100 text-blue-800 dark:bg-blue-900 dark:text-blue-300' : ''}
                                ${endpoint.method === 'PUT' ? 'bg-yellow-100 text-yellow-800 dark:bg-yellow-900 dark:text-yellow-300' : ''}
                                ${endpoint.method === 'DELETE' ? 'bg-red-100 text-red-800 dark:bg-red-900 dark:text-red-300' : ''}
                              `}>
                                {endpoint.method}
                              </span>
                              {endpoint.path.split('/').pop()}
                            </button>
                          </li>
                        ))}
                      </ul>
                    )}
                  </li>
                ))}
              </ul>
            </div>
          </div>

          {/* Content */}
          <div className="md:w-3/4">
            {apiCategories
              .filter((category) => category.name === activeCategory)
              .map((category) => (
                <div key={category.name}>
                  <div className="bg-white dark:bg-gray-800 rounded-lg shadow-md p-6 mb-6">
                    <h2 className="text-2xl font-bold mb-2 dark:text-white">
                      {category.name.charAt(0).toUpperCase() + category.name.slice(1)} API
                    </h2>
                    <p className="text-gray-600 dark:text-gray-300 mb-4">{category.description}</p>
                  </div>

                  {category.endpoints.map((endpoint) => (
                    <div 
                      key={`${endpoint.method}-${endpoint.path}`} 
                      id={`${endpoint.method}-${endpoint.path}`}
                      className={`bg-white dark:bg-gray-800 rounded-lg shadow-md p-6 mb-6 ${
                        activeEndpoint === `${endpoint.method}-${endpoint.path}` ? 'ring-2 ring-primary-500' : ''
                      }`}
                    >
                      <div className="flex flex-wrap items-center gap-2 mb-4">
                        <span className={`inline-block text-sm font-mono px-2 py-1 rounded
                          ${endpoint.method === 'GET' ? 'bg-green-100 text-green-800 dark:bg-green-900 dark:text-green-300' : ''}
                          ${endpoint.method === 'POST' ? 'bg-blue-100 text-blue-800 dark:bg-blue-900 dark:text-blue-300' : ''}
                          ${endpoint.method === 'PUT' ? 'bg-yellow-100 text-yellow-800 dark:bg-yellow-900 dark:text-yellow-300' : ''}
                          ${endpoint.method === 'DELETE' ? 'bg-red-100 text-red-800 dark:bg-red-900 dark:text-red-300' : ''}
                        `}>
                          {endpoint.method}
                        </span>
                        <h3 className="text-lg font-mono dark:text-white">{endpoint.path}</h3>
                      </div>
                      
                      <p className="text-gray-600 dark:text-gray-300 mb-4">{endpoint.description}</p>
                      
                      {endpoint.parameters && endpoint.parameters.length > 0 && (
                        <div className="mb-4">
                          <h4 className="font-semibold text-gray-700 dark:text-gray-200 mb-2">Parameters</h4>
                          <div className="bg-gray-50 dark:bg-gray-900 rounded-lg overflow-hidden">
                            <table className="min-w-full">
                              <thead className="bg-gray-100 dark:bg-gray-800">
                                <tr>
                                  <th className="px-4 py-2 text-left text-xs font-medium text-gray-500 dark:text-gray-300 uppercase tracking-wider">Name</th>
                                  <th className="px-4 py-2 text-left text-xs font-medium text-gray-500 dark:text-gray-300 uppercase tracking-wider">Type</th>
                                  <th className="px-4 py-2 text-left text-xs font-medium text-gray-500 dark:text-gray-300 uppercase tracking-wider">Required</th>
                                  <th className="px-4 py-2 text-left text-xs font-medium text-gray-500 dark:text-gray-300 uppercase tracking-wider">Description</th>
                                </tr>
                              </thead>
                              <tbody className="divide-y divide-gray-200 dark:divide-gray-700">
                                {endpoint.parameters.map((param) => (
                                  <tr key={param.name}>
                                    <td className="px-4 py-2 text-sm font-mono text-gray-800 dark:text-gray-200">{param.name}</td>
                                    <td className="px-4 py-2 text-sm text-gray-600 dark:text-gray-300">{param.type}</td>
                                    <td className="px-4 py-2 text-sm text-gray-600 dark:text-gray-300">
                                      {param.required ? (
                                        <span className="text-danger-600 dark:text-danger-400">Yes</span>
                                      ) : (
                                        <span className="text-gray-400">No</span>
                                      )}
                                    </td>
                                    <td className="px-4 py-2 text-sm text-gray-600 dark:text-gray-300">{param.description}</td>
                                  </tr>
                                ))}
                              </tbody>
                            </table>
                          </div>
                        </div>
                      )}
                      
                      {endpoint.requestExample && (
                        <div className="mb-4">
                          <h4 className="font-semibold text-gray-700 dark:text-gray-200 mb-2">Request Example</h4>
                          <div className="bg-gray-800 rounded-lg p-4 overflow-auto">
                            <pre className="text-gray-100 font-mono text-sm leading-relaxed whitespace-pre-wrap">{endpoint.requestExample}</pre>
                          </div>
                        </div>
                      )}
                      
                      <div>
                        <h4 className="font-semibold text-gray-700 dark:text-gray-200 mb-2">Response Example</h4>
                        <div className="bg-gray-800 rounded-lg p-4 overflow-auto">
                          <pre className="text-gray-100 font-mono text-sm leading-relaxed whitespace-pre-wrap">{endpoint.responseExample}</pre>
                        </div>
                      </div>
                    </div>
                  ))}
                </div>
              ))}
          </div>
        </div>
      </div>
    </div>
  );
} 