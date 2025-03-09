#!/usr/bin/env python3
"""
A simple HTTP server for mocking the ARB Platform API.
This handles URLs without extensions better than SimpleHTTPServer.
"""

import http.server
import socketserver
import os
import json
import datetime
import sys
from pathlib import Path

# Default port
PORT = 8000

# Get port from command line argument if provided
if len(sys.argv) > 1:
    try:
        PORT = int(sys.argv[1])
    except ValueError:
        print(f"Invalid port: {sys.argv[1]}, using default: {PORT}")

class ArbAPIHandler(http.server.SimpleHTTPRequestHandler):
    """Custom handler that serves JSON for API-like URLs."""
    
    def do_GET(self):
        # Clean up path
        path = self.path.strip('/')
        
        # Root path
        if not path:
            self.serve_json_file('index.json')
            return
            
        # Handle paths without file extensions
        if '.' not in os.path.basename(path):
            # Check if this is a directory with an index.json
            if os.path.isdir(path) and os.path.exists(os.path.join(path, 'index.json')):
                self.serve_json_file(os.path.join(path, 'index.json'))
                return
                
            # Check if there's an index.json file in a directory with this name
            json_path = f"{path}/index.json"
            if os.path.exists(json_path):
                self.serve_json_file(json_path)
                return
                
            # Special case for API endpoints
            if path.startswith('api/'):
                # Try to find a matching JSON file
                api_path = path.replace('api/', 'api/', 1)
                json_path = f"{api_path}/index.json"
                if os.path.exists(json_path):
                    self.serve_json_file(json_path)
                    return
                    
                # Handle dynamic paths like /api/market/data/{symbol}
                if 'market/data' in path:
                    parts = path.split('/')
                    if len(parts) > 3:
                        symbol = parts[3]
                        # Create a dynamic response
                        self.serve_dynamic_market_data(symbol)
                        return
            
            # If we can't find a matching file, return 404
            self.send_error(404, f"File not found: {self.path}")
            return
            
        # For paths with file extensions, use the default handler
        return super().do_GET()
    
    def serve_json_file(self, path):
        """Serve a JSON file with proper headers."""
        try:
            with open(path, 'rb') as f:
                content = f.read()
                
            self.send_response(200)
            self.send_header('Content-Type', 'application/json')
            self.send_header('Content-Length', str(len(content)))
            self.send_header('Access-Control-Allow-Origin', '*')  # CORS header
            self.end_headers()
            self.wfile.write(content)
        except IOError:
            self.send_error(404, f"File not found: {path}")
    
    def serve_dynamic_market_data(self, symbol):
        """Generate a dynamic market data response for a symbol."""
        # Make a dynamic response with current timestamp and some randomness
        import random
        
        base_price = 0
        if symbol.startswith('BTC'):
            base_price = 35000 + random.uniform(-500, 500)
        elif symbol.startswith('ETH'):
            base_price = 2200 + random.uniform(-50, 50)
        elif symbol.startswith('SOL'):
            base_price = 80 + random.uniform(-5, 5)
        elif symbol.startswith('AAPL'):
            base_price = 175 + random.uniform(-2, 2)
        else:
            base_price = 100 + random.uniform(-10, 10)
            
        spread = base_price * 0.001  # 0.1% spread
        
        data = {
            "data": {
                "symbol": symbol.replace('-', '/'),
                "price": round(base_price, 2),
                "bid": round(base_price - spread/2, 2),
                "ask": round(base_price + spread/2, 2),
                "volume": round(1000 + random.uniform(0, 5000), 2),
                "timestamp": datetime.datetime.utcnow().isoformat() + 'Z',
                "exchange": random.choice(["Binance", "Coinbase", "Kraken", "FTX"])
            }
        }
        
        content = json.dumps(data, indent=2).encode('utf-8')
        
        self.send_response(200)
        self.send_header('Content-Type', 'application/json')
        self.send_header('Content-Length', str(len(content)))
        self.send_header('Access-Control-Allow-Origin', '*')  # CORS header
        self.end_headers()
        self.wfile.write(content)

def run():
    """Run the server."""
    
    # Create a directory for static files if it doesn't exist
    os.makedirs('api/health', exist_ok=True)
    os.makedirs('api/market/symbols', exist_ok=True)
    os.makedirs('api/market/data', exist_ok=True)
    os.makedirs('api/strategy', exist_ok=True)
    os.makedirs('api/account/balance', exist_ok=True)
    
    # Create default files if they don't exist
    if not os.path.exists('index.json'):
        with open('index.json', 'w') as f:
            json.dump({
                "name": "ARB Platform API",
                "version": "0.1.0",
                "status": "simulation",
                "message": "This is a simulation. For full functionality, install Rust.",
                "documentation": "/api-docs"
            }, f, indent=2)
    
    # Start the server
    with socketserver.TCPServer(("", PORT), ArbAPIHandler) as httpd:
        print(f"Serving on port {PORT}")
        try:
            httpd.serve_forever()
        except KeyboardInterrupt:
            print("Server stopped.")

if __name__ == "__main__":
    run() 