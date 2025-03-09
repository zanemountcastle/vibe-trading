# ARB Platform - Frontend

Modern, responsive user interface for the ARB high-frequency trading platform. Built with React, Next.js, TypeScript, Redux, and Tailwind CSS.

## Running the Frontend

To start the frontend development server:

1. Install dependencies:
   ```bash
   npm install
   ```

2. Start the development server:
   ```bash
   npm run dev
   ```

The application will be available at http://localhost:3000

## Prerequisites

- Node.js (version 14+)
- npm (version 6+) or yarn
- Backend server running (see backend README)

## Features

- Dashboard with real-time market data
- Trading interface for placing orders
- Strategy management and configuration
- Backtesting and performance analysis
- Account and position management
- Comprehensive API documentation

## Key Pages

- `/` - Landing page
- `/dashboard` - Dashboard with market overview
- `/trading` - Order placement and management
- `/strategies` - Strategy configuration
- `/backtesting` - Strategy backtesting
- `/account` - Account management
- `/api-docs` - API documentation

## Development

The frontend codebase is organized as follows:

- `src/` - Source code
  - `app/` - Next.js app router pages
  - `components/` - React components
  - `store/` - Redux store
  - `services/` - API services
  - `hooks/` - Custom React hooks
  - `types/` - TypeScript type definitions
  - `utils/` - Utility functions

## API Integration

The frontend connects to the backend API through:

1. **REST API** - For regular requests, configured in `next.config.js` to proxy to http://localhost:8000
2. **WebSocket** - For real-time updates, connecting to ws://localhost:8000/ws

## API Documentation

The API documentation is available at `/api-docs` and provides comprehensive information about:

- Available endpoints
- Request/response formats
- Authentication requirements
- Examples in multiple languages

## Styling

The application uses Tailwind CSS for styling with custom theme configuration in `tailwind.config.js`. 

## Building for Production

To create a production build:

```bash
npm run build
npm start
```

For optimal performance, both the frontend and backend should be built in production mode and served from appropriate web servers. 