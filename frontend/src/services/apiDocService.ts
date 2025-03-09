/**
 * API Documentation Service
 * 
 * This service provides examples of how to use the platform's API endpoints.
 * It includes example code that can be displayed in the documentation.
 */

export const apiDocService = {
  /**
   * Example: Getting market data for a specific symbol
   */
  getMarketDataExample: () => {
    return `
// Using fetch
fetch('/api/market/data/BTC-USD')
  .then(response => response.json())
  .then(data => console.log(data))
  .catch(error => console.error('Error:', error));

// Using axios
import axios from 'axios';

axios.get('/api/market/data/BTC-USD')
  .then(response => console.log(response.data))
  .catch(error => console.error('Error:', error));
`;
  },

  /**
   * Example: Placing an order
   */
  placeOrderExample: () => {
    return `
// Using fetch
const orderData = {
  symbol: 'BTC-USD',
  direction: 'buy',
  order_type: 'limit',
  quantity: 0.5,
  price: 35000.0,
  time_in_force: 'gtc'
};

fetch('/api/order', {
  method: 'POST',
  headers: {
    'Content-Type': 'application/json',
  },
  body: JSON.stringify(orderData),
})
  .then(response => response.json())
  .then(data => console.log(data))
  .catch(error => console.error('Error:', error));

// Using axios
import axios from 'axios';

axios.post('/api/order', orderData)
  .then(response => console.log(response.data))
  .catch(error => console.error('Error:', error));
`;
  },

  /**
   * Example: WebSocket connection for real-time updates
   */
  webSocketExample: () => {
    return `
// Establishing a WebSocket connection
const socket = new WebSocket('ws://localhost:3000/ws');

// Connection opened
socket.addEventListener('open', (event) => {
  console.log('Connected to WebSocket server');
  
  // Subscribe to market data
  socket.send(JSON.stringify({
    type: 'Subscribe',
    payload: {
      feed: 'market_data',
      symbols: ['BTC-USD', 'ETH-USD']
    }
  }));
});

// Listen for messages
socket.addEventListener('message', (event) => {
  const data = JSON.parse(event.data);
  console.log('Message from server:', data);
  
  // Handle different message types
  switch (data.type) {
    case 'MarketData':
      updatePriceChart(data.payload);
      break;
    case 'OrderUpdate':
      updateOrderStatus(data.payload);
      break;
    case 'StrategyUpdate':
      updateStrategyMetrics(data.payload);
      break;
  }
});

// Connection closed
socket.addEventListener('close', (event) => {
  console.log('Disconnected from WebSocket server');
});

// Handle errors
socket.addEventListener('error', (event) => {
  console.error('WebSocket error:', event);
});

// When you're done, close the connection
// socket.close();
`;
  },

  /**
   * Example: Running a strategy backtest
   */
  backtestExample: () => {
    return `
// Using fetch
const backtestConfig = {
  strategy: 'Statistical Arbitrage',
  start_date: '2023-01-01',
  end_date: '2023-06-01',
  symbols: ['BTC-USD', 'ETH-USD'],
  initial_capital: 1000000.0,
  parameters: {
    correlation_threshold: 0.8,
    z_score_threshold: 2.0,
    lookback_period: 100,
    max_position_size: 100000.0
  }
};

fetch('/api/backtest', {
  method: 'POST',
  headers: {
    'Content-Type': 'application/json',
  },
  body: JSON.stringify(backtestConfig),
})
  .then(response => response.json())
  .then(data => {
    console.log('Backtest result:', data);
    
    // Get detailed results using the ID
    if (data.data && data.data.id) {
      return fetch(\`/api/backtest/\${data.data.id}\`);
    }
  })
  .then(response => response.json())
  .then(detailedResults => console.log('Detailed results:', detailedResults))
  .catch(error => console.error('Error:', error));
`;
  },
}; 