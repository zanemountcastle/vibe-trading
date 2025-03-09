# ARB Platform API Documentation

This documentation provides details on how to interact with the ARB Trading Platform's API.

## Overview

The ARB Platform API is a RESTful API that allows developers to integrate with our high-frequency trading platform. You can access market data, manage trading strategies, place and monitor orders, and run backtests.

## Base URL

All API endpoints are relative to the base URL of the ARB Platform instance:

```
https://your-arb-instance.com/api
```

For local development, the base URL would be:

```
http://localhost:3000/api
```

## Authentication

Most API endpoints require authentication. You can authenticate using JSON Web Tokens (JWT).

1. Obtain a token by sending a POST request to `/api/auth/login` with your credentials.
2. Include the token in the Authorization header of all subsequent requests:

```
Authorization: Bearer <your_token>
```

## Rate Limiting

API requests are rate-limited to protect our services from abuse. The current limits are:

- 100 requests per minute for market data endpoints
- 20 requests per minute for order management endpoints
- 5 requests per minute for backtest endpoints

If you exceed these limits, you'll receive a 429 Too Many Requests response.

## Response Format

All responses are in JSON format. Successful responses have this structure:

```json
{
  "data": {
    // Response data here
  }
}
```

Error responses look like this:

```json
{
  "error": "Error message here"
}
```

## WebSocket Support

For real-time data, we provide a WebSocket API. Connect to `/ws` and subscribe to the data feeds you're interested in.

## API Endpoints

The API is organized into several categories:

1. **Health** - System health status
2. **Market Data** - Symbol information and real-time market data
3. **Strategy** - Trading strategy management
4. **Order** - Order placement and management
5. **Account** - Account information and positions
6. **Backtest** - Strategy backtesting

Each API endpoint is documented with:
- HTTP method and path
- Description
- Required and optional parameters
- Example requests and responses

## API Clients

We provide example code snippets for common programming languages in the documentation. For more comprehensive integration, consider using one of our client libraries:

- [JavaScript/TypeScript](https://github.com/arb-platform/arb-api-js)
- [Python](https://github.com/arb-platform/arb-api-python)
- [Rust](https://github.com/arb-platform/arb-api-rust)

## Need Help?

If you encounter any issues or have questions about the API, please contact our support team at api-support@arb-platform.com. 