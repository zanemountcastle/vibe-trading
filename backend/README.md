# ARB Platform - Backend

High-frequency trading platform backend built with Rust. This component handles all the trading logic, market data processing, strategy execution, and order management.

## Running the Backend

We provide two ways to run the backend:

### Option 1: Run with Rust (Recommended)

For full functionality, you should install Rust and build/run the native application:

1. [Install Rust and Cargo](https://www.rust-lang.org/tools/install)
2. Run the application:
   ```bash
   ./run.sh
   ```

This will build and run the Rust application with full functionality.

### Option 2: Run in Simulation Mode

If you don't have Rust installed, you can run the backend in simulation mode:

1. Ensure you have Python 3 installed
2. Run the application:
   ```bash
   ./run.sh
   ```

The script will detect that Rust is not installed and will automatically fall back to simulation mode, which:
- Displays simulated log output
- Starts a mock API server with sample data
- Provides basic API endpoints that return static data

In simulation mode, the API is accessible at `http://localhost:8000/api/` with several endpoints like:
- `/api/health` - System health status
- `/api/market/symbols` - Available trading symbols
- `/api/market/data/BTC-USD` - Market data for BTC/USD
- `/api/strategy` - Available trading strategies
- `/api/account/balance` - Account balance information

## API Documentation

For comprehensive API documentation, visit the frontend's API documentation page once both frontend and backend are running:

```
http://localhost:3000/api-docs
```

## Connecting to the Frontend

The frontend is configured to connect to the backend at `http://localhost:8000`. Make sure to:

1. Start the backend first (`./run.sh` from the backend directory)
2. Then start the frontend (`npm run dev` from the frontend directory)

Both components need to be running simultaneously for the application to work properly.

## Development

The backend codebase is organized as follows:

- `src/` - Source code
  - `main.rs` - Application entry point
  - `strategy/` - Trading strategy implementations
  - `market_data/` - Market data management
  - `order/` - Order management and execution
  - `exchange/` - Exchange integrations
  - `api/` - Web API for frontend communication
  - `risk/` - Risk management
  - `utils/` - Utility functions and helpers

To modify the code, edit the source files and run `cargo build` to rebuild.

## Dependencies

See `Cargo.toml` for a complete list of dependencies. 