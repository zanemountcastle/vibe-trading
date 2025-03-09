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

## Testing

The backend includes a comprehensive testing suite to ensure code quality and reliability.

### Running Tests

To run all tests:
```bash
cargo test
```

To run a specific test file:
```bash
cargo test --test <test_file_name>
```

For example:
```bash
cargo test --test simple_test
```

To run tests with a single thread (helps avoid concurrency issues):
```bash
cargo test -- --test-threads=1
```

### Test Structure

The test suite is organized into:

1. **Unit Tests** - Testing individual components in isolation
   - Exchange module tests (`tests/unit/exchange/`)
   - Order module tests (`tests/unit/order/`)
   - Market data module tests (`tests/unit/market_data/`)
   - Strategy module tests (`tests/unit/strategy/`)

2. **Integration Tests** - Testing how components work together
   - Exchange and order workflow tests (`tests/integration/exchange_order_workflow.rs`)

3. **Simple Test** - Basic functionality tests for core components (`tests/simple_test.rs`)

### Writing New Tests

When adding new tests:

1. Use `#[tokio::test]` attribute for asynchronous tests
2. Import required traits explicitly (e.g., `use arb_platform::exchange::Exchange;`)
3. Follow existing test patterns for consistent style
4. Ensure tests clean up after themselves (especially when working with shared resources)

Example of a proper async test:
```rust
#[tokio::test]
async fn test_example() {
    // Test setup
    let config = ExchangeConfig { /* ... */ };
    let mut exchange = CryptoExchange::new(config);
    
    // Connect to the exchange
    let result = exchange.connect().await;
    assert!(result.is_ok());
    
    // Perform test assertions
    assert!(exchange.is_connected());
    
    // Clean up
    let _ = exchange.disconnect().await;
}
```

### Code Coverage

The tests aim to achieve a minimum of 95% code coverage across all modules, with particular focus on:

- Exchange connectivity and order processing
- Order management and lifecycle
- Strategy execution and signal generation
- Market data handling and processing 