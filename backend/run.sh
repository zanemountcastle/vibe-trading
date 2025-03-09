#!/bin/bash

# Define text colors
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}ARB Platform Startup${NC}"
echo "----------------------------------------"

# Check if Rust/Cargo is installed
if command -v cargo &> /dev/null; then
    echo -e "${GREEN}✓ Rust is installed${NC}"
    
    # Build the project
    echo -e "\n${BLUE}Building ARB Platform...${NC}"
    if cargo build; then
        echo -e "${GREEN}✓ Build successful${NC}"
        
        # Create benches directory if it doesn't exist
        if [ ! -d "benches" ]; then
            mkdir -p benches
            echo "Created benches directory"
        fi
        
        # Run the actual application
        echo -e "\n${BLUE}Starting ARB Platform...${NC}"
        echo "----------------------------------------"
        cargo run
    else
        echo -e "${RED}✗ Build failed${NC}"
        echo "Please check the error messages above and fix any issues."
        exit 1
    fi
else
    echo -e "${YELLOW}⚠ Rust/Cargo is not installed or not in PATH${NC}"
    echo "Running in simulation mode..."
    echo -e "${YELLOW}To install Rust for full functionality, visit: https://www.rust-lang.org/tools/install${NC}"
    echo ""
    echo -e "${BLUE}Simulating ARB Platform startup...${NC}"
    echo "----------------------------------------"
    
    # Simulate the application output
    echo -e "${GREEN}INFO${NC}  arb_platform > Starting ARB trading platform"
    echo -e "${GREEN}INFO${NC}  arb_platform > Loading configuration..."
    echo -e "${GREEN}INFO${NC}  arb_platform > Initializing market data sources..."
    echo -e "${GREEN}INFO${NC}  arb_platform > Loading trading strategies..."
    echo -e "${GREEN}INFO${NC}  arb_platform > Initializing risk management..."
    echo -e "${GREEN}INFO${NC}  arb_platform > Setting up order management..."
    echo -e "${GREEN}INFO${NC}  arb_platform > Starting API server..."
    echo -e "${GREEN}INFO${NC}  arb_platform > Starting API server on 0.0.0.0:8000"
    echo -e "${GREEN}INFO${NC}  arb_platform > Starting main trading loop..."
    echo ""
    echo -e "${YELLOW}MOCK SERVER:${NC} Running in simulation mode. No actual trading will occur."
    echo -e "                API endpoints will return mock data."
    echo -e "                Install Rust and rebuild for full functionality."
    echo ""
    
    # Check if Python is available for the mock server
    if command -v python3 &> /dev/null; then
        echo -e "${YELLOW}Setting up a mock API server on port 8000...${NC}"
        
        # Create a temporary directory for mock API
        rm -rf temp
        mkdir -p temp
        cd temp

        # Create API response files
        mkdir -p api/health
        cat > api/health/index.json << EOF
{
  "status": "ok",
  "timestamp": "$(date -u +"%Y-%m-%dT%H:%M:%SZ")",
  "version": "0.1.0",
  "services": {
    "database": "healthy",
    "market_data": "healthy",
    "order_system": "healthy",
    "strategy_engine": "healthy"
  },
  "uptime": "00:00:01",
  "mode": "simulation"
}
EOF

        # Market data endpoints
        mkdir -p api/market/symbols
        cat > api/market/symbols/index.json << EOF
{
  "data": [
    "BTC/USD",
    "ETH/USD",
    "SOL/USD",
    "AAPL",
    "MSFT",
    "TSLA",
    "AMZN",
    "GOOGL",
    "EUR/USD",
    "JPY/USD"
  ]
}
EOF

        # Example BTC market data
        mkdir -p "api/market/data/BTC-USD"
        cat > "api/market/data/BTC-USD/index.json" << EOF
{
  "data": {
    "symbol": "BTC/USD",
    "price": 35245.67,
    "bid": 35240.23,
    "ask": 35250.12,
    "volume": 15234.51,
    "timestamp": "$(date -u +"%Y-%m-%dT%H:%M:%SZ")",
    "exchange": "Binance"
  }
}
EOF

        # Strategy endpoints
        mkdir -p api/strategy
        cat > api/strategy/index.json << EOF
{
  "data": [
    "Statistical Arbitrage",
    "Event Arbitrage",
    "Information Arbitrage",
    "Latency Arbitrage",
    "Day Trading"
  ]
}
EOF

        mkdir -p "api/strategy/Statistical-Arbitrage/params"
        cat > "api/strategy/Statistical-Arbitrage/params/index.json" << EOF
{
  "data": {
    "correlation_threshold": 0.8,
    "z_score_threshold": 2.0,
    "lookback_period": 100,
    "max_position_size": 100000.0
  }
}
EOF

        # Account balance endpoint
        mkdir -p api/account/balance
        cat > api/account/balance/index.json << EOF
{
  "data": {
    "total": 1000000.0,
    "available": 750000.0,
    "currency": "USD",
    "additional_balances": [
      {"currency": "BTC", "amount": 2.5},
      {"currency": "ETH", "amount": 30.0},
      {"currency": "SOL", "amount": 150.0}
    ],
    "timestamp": "$(date -u +"%Y-%m-%dT%H:%M:%SZ")"
  }
}
EOF

        # Root endpoint listing
        cat > api/index.json << EOF
{
  "endpoints": [
    "/api/health",
    "/api/market/symbols",
    "/api/market/data/{symbol}",
    "/api/strategy",
    "/api/strategy/{name}/params",
    "/api/account/balance"
  ],
  "message": "This is a simulation mode. Install Rust for full functionality."
}
EOF

        # Root message
        cat > index.json << EOF
{
  "name": "ARB Platform API",
  "version": "0.1.0",
  "status": "simulation",
  "message": "This is a simulation. For full functionality, install Rust.",
  "documentation": "/api-docs"
}
EOF
        
        # Copy the custom Python server if it exists
        if [ -f "../src/mock_server.py" ]; then
            cp "../src/mock_server.py" .
            echo -e "${GREEN}✓ Using custom mock server${NC}"
            python3 mock_server.py 8000 &
        else
            # Fall back to simple HTTP server
            echo -e "${YELLOW}ℹ Using simple HTTP server${NC}"
            python3 -m http.server 8000 &
        fi
        
        PY_SERVER_PID=$!
        cd ..
        
        # Kill the Python server when this script exits
        trap "kill $PY_SERVER_PID 2>/dev/null" EXIT
        
        echo -e "${GREEN}✓ Mock API server started on port 8000${NC}"
        echo -e "  Try these endpoints in your browser or with curl:"
        echo -e "  - ${BLUE}http://localhost:8000/api/health${NC}"
        echo -e "  - ${BLUE}http://localhost:8000/api/market/symbols${NC}"
        echo -e "  - ${BLUE}http://localhost:8000/api/market/data/BTC-USD${NC}"
        echo -e "  - ${BLUE}http://localhost:8000/api/strategy${NC}"
        echo -e "  - ${BLUE}http://localhost:8000/api/account/balance${NC}"
    else
        echo -e "${RED}✗ Python is not available. Cannot start mock API server.${NC}"
        echo -e "  To install Python, visit: https://www.python.org/downloads/"
    fi

    echo -e "\n${BLUE}Server is running. Press Ctrl+C to stop.${NC}"

    # Keep the script running to simulate a running server
    while true; do
        sleep 1
    done
fi 