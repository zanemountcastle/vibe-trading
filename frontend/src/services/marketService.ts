import axios from 'axios';
import { MarketData } from '../store/slices/marketSlice';

const API_URL = '/api';

export const marketService = {
  // Get all available symbols
  getSymbols: async () => {
    return axios.get(`${API_URL}/market/symbols`);
  },
  
  // Get market data for a specific symbol
  getMarketData: async (symbol: string) => {
    return axios.get<MarketData>(`${API_URL}/market/data/${symbol}`);
  },
  
  // Subscribe to real-time market data updates
  subscribeToMarketData: (
    symbols: string[],
    onData: (data: MarketData) => void,
    onError: (error: any) => void
  ) => {
    // This would be implemented with WebSockets
    // For now, we'll simulate with a polling mechanism
    const intervalId = setInterval(async () => {
      try {
        for (const symbol of symbols) {
          const response = await axios.get<MarketData>(`${API_URL}/market/data/${symbol}`);
          onData(response.data);
        }
      } catch (error) {
        onError(error);
      }
    }, 5000); // Poll every 5 seconds
    
    // Return a function to unsubscribe
    return () => {
      clearInterval(intervalId);
    };
  },
}; 