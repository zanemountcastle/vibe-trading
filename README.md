# ARB: High-Frequency Trading Platform

A sophisticated high-frequency trading platform designed for trading stocks, bonds, cryptocurrencies, and other assets with minimal risk. The platform leverages various arbitrage strategies including event arbitrage, statistical arbitrage, information arbitrage, and latency arbitrage.

## Quick Start

Follow these steps to run the ARB platform:

1. **Start the backend server:**
   ```bash
   cd backend
   chmod +x run.sh  # Make the script executable
   ./run.sh
   ```

2. **In a new terminal, start the frontend:**
   ```bash
   cd frontend
   npm install
   npm run dev
   ```

3. **Open in your browser:**
   - Main application: [http://localhost:3000](http://localhost:3000)
   - API documentation: [http://localhost:3000/api-docs](http://localhost:3000/api-docs)

## Features

- Multi-asset trading (stocks, bonds, crypto, etc.)
- Multiple arbitrage strategies with dynamic strategy selection
- Real-time performance monitoring
- Strategy design and backtesting
- Fund management
- Autonomous 24/7 operation

## Architecture

- **Backend**: Rust for high-performance, memory-safe execution
- **Frontend**: React, TypeScript, Redux, and Tailwind CSS

## Backend Mode Options

The backend can run in two modes:

### Full Mode (Requires Rust)
If you have Rust installed, the backend will build and run the complete trading platform with all functionality.

### Simulation Mode (Requires Python 3)
If Rust is not installed, the backend will automatically run in simulation mode, which:
- Displays simulated log output
- Starts a mock API server with sample data
- Provides API endpoints that return static data

## Project Structure

```
arb/
├── backend/           # Rust-based trading engine
│   ├── src/           # Source code
│   └── Cargo.toml     # Rust dependencies
└── frontend/          # React-based user interface
    ├── src/           # Source code
    └── package.json   # Node.js dependencies
```

## Prerequisites

For full functionality:
- Rust and Cargo
- Node.js and npm/yarn
- Access to trading APIs for desired asset classes

For simulation mode:
- Python 3
- Node.js and npm/yarn

## Troubleshooting

### Backend Issues:
- If port 8000 is already in use, edit `run.sh` to change the port
- If you see "command not found" for the run script, make it executable with `chmod +x run.sh`
- Missing Rust: Install from [rust-lang.org](https://www.rust-lang.org/tools/install)

### Frontend Issues:
- If you see module resolution errors, run `npm install` again
- Make sure the backend is running on port 8000
- For TypeScript errors, run `npm run lint`

## Development Status

This project is currently in early development phase, with an iterative approach from MVP to a finished product. 