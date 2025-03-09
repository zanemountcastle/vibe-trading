import { createSlice, createAsyncThunk, PayloadAction } from '@reduxjs/toolkit';
import { marketService } from '../../services/marketService';

// Types
export interface MarketData {
  symbol: string;
  price: number;
  bid: number;
  ask: number;
  volume: number;
  timestamp: string;
  exchange: string;
}

interface MarketState {
  data: Record<string, MarketData>;
  symbols: string[];
  loading: boolean;
  error: string | null;
  selectedSymbol: string | null;
}

// Initial state
const initialState: MarketState = {
  data: {},
  symbols: [],
  loading: false,
  error: null,
  selectedSymbol: null,
};

// Async thunks
export const fetchSymbols = createAsyncThunk(
  'market/fetchSymbols',
  async (_, { rejectWithValue }) => {
    try {
      const response = await marketService.getSymbols();
      return response.data;
    } catch (error: any) {
      return rejectWithValue(error.response?.data?.error || 'Failed to fetch symbols');
    }
  }
);

export const fetchMarketData = createAsyncThunk(
  'market/fetchMarketData',
  async (symbol: string, { rejectWithValue }) => {
    try {
      const response = await marketService.getMarketData(symbol);
      return response.data;
    } catch (error: any) {
      return rejectWithValue(error.response?.data?.error || 'Failed to fetch market data');
    }
  }
);

// Slice
const marketSlice = createSlice({
  name: 'market',
  initialState,
  reducers: {
    setSelectedSymbol: (state, action: PayloadAction<string>) => {
      state.selectedSymbol = action.payload;
    },
    updateMarketData: (state, action: PayloadAction<MarketData>) => {
      // This is used for WebSocket updates
      const { symbol } = action.payload;
      state.data[symbol] = action.payload;
    },
  },
  extraReducers: (builder) => {
    builder
      // Fetch symbols
      .addCase(fetchSymbols.pending, (state) => {
        state.loading = true;
        state.error = null;
      })
      .addCase(fetchSymbols.fulfilled, (state, action: PayloadAction<string[]>) => {
        state.loading = false;
        state.symbols = action.payload;
        if (!state.selectedSymbol && action.payload.length > 0) {
          state.selectedSymbol = action.payload[0];
        }
      })
      .addCase(fetchSymbols.rejected, (state, action) => {
        state.loading = false;
        state.error = action.payload as string;
      })
      
      // Fetch market data
      .addCase(fetchMarketData.pending, (state) => {
        state.loading = true;
        state.error = null;
      })
      .addCase(fetchMarketData.fulfilled, (state, action: PayloadAction<MarketData>) => {
        state.loading = false;
        const { symbol } = action.payload;
        state.data[symbol] = action.payload;
      })
      .addCase(fetchMarketData.rejected, (state, action) => {
        state.loading = false;
        state.error = action.payload as string;
      });
  },
});

export const { setSelectedSymbol, updateMarketData } = marketSlice.actions;
export default marketSlice.reducer; 