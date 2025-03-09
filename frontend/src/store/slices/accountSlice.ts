import { createSlice, createAsyncThunk, PayloadAction } from '@reduxjs/toolkit';
import { AccountBalance, Position } from '../../types/api';

// Define a service placeholder
const accountService = {
  getBalance: async () => ({
    data: {
      total: 1000000.0,
      available: 750000.0,
      currency: 'USD',
      additional_balances: [
        { currency: 'BTC', amount: 2.5 },
        { currency: 'ETH', amount: 30.0 },
        { currency: 'SOL', amount: 150.0 },
      ],
      timestamp: new Date().toISOString(),
    } as AccountBalance
  }),
  getPositions: async () => ({
    data: [
      {
        symbol: 'BTC/USD',
        quantity: 2.5,
        avg_price: 34500.0,
        current_price: 35200.0,
        unrealized_pnl: 1750.0,
        realized_pnl: 2500.0,
        timestamp: new Date().toISOString(),
      },
      {
        symbol: 'ETH/USD',
        quantity: 30.0,
        avg_price: 2100.0,
        current_price: 2250.0,
        unrealized_pnl: 4500.0,
        realized_pnl: 1200.0,
        timestamp: new Date().toISOString(),
      },
    ] as Position[]
  })
};

// Define the initial state
interface AccountState {
  balance: AccountBalance | null;
  positions: Position[];
  loading: boolean;
  error: string | null;
}

const initialState: AccountState = {
  balance: null,
  positions: [],
  loading: false,
  error: null,
};

// Async thunks
export const fetchBalance = createAsyncThunk(
  'account/fetchBalance',
  async (_, { rejectWithValue }) => {
    try {
      const response = await accountService.getBalance();
      return response.data;
    } catch (error: any) {
      return rejectWithValue(error.response?.data?.error || 'Failed to fetch account balance');
    }
  }
);

export const fetchPositions = createAsyncThunk(
  'account/fetchPositions',
  async (_, { rejectWithValue }) => {
    try {
      const response = await accountService.getPositions();
      return response.data;
    } catch (error: any) {
      return rejectWithValue(error.response?.data?.error || 'Failed to fetch positions');
    }
  }
);

// Create the slice
const accountSlice = createSlice({
  name: 'account',
  initialState,
  reducers: {
    clearErrors: (state) => {
      state.error = null;
    },
  },
  extraReducers: (builder) => {
    builder
      // Fetch balance
      .addCase(fetchBalance.pending, (state) => {
        state.loading = true;
        state.error = null;
      })
      .addCase(fetchBalance.fulfilled, (state, action: PayloadAction<AccountBalance>) => {
        state.balance = action.payload;
        state.loading = false;
      })
      .addCase(fetchBalance.rejected, (state, action) => {
        state.loading = false;
        state.error = action.payload as string;
      })
      
      // Fetch positions
      .addCase(fetchPositions.pending, (state) => {
        state.loading = true;
        state.error = null;
      })
      .addCase(fetchPositions.fulfilled, (state, action: PayloadAction<Position[]>) => {
        state.positions = action.payload;
        state.loading = false;
      })
      .addCase(fetchPositions.rejected, (state, action) => {
        state.loading = false;
        state.error = action.payload as string;
      });
  },
});

export const { clearErrors } = accountSlice.actions;
export default accountSlice.reducer; 