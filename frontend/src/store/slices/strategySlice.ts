import { createSlice, createAsyncThunk, PayloadAction } from '@reduxjs/toolkit';
import { StrategyParams, StrategyEvaluation } from '../../types/api';

// Define a service placeholder
const strategyService = {
  getStrategies: async () => ({ 
    data: ['Statistical Arbitrage', 'Event Arbitrage', 'Information Arbitrage', 'Latency Arbitrage', 'Day Trading']
  }),
  getActiveStrategy: async () => ({ data: 'Statistical Arbitrage' }),
  setActiveStrategy: async (name: string) => ({ 
    data: { success: true, message: `Active strategy set to: ${name}` }
  }),
  getStrategyParams: async (name: string) => ({ 
    data: {
      correlation_threshold: 0.8,
      z_score_threshold: 2.0,
      lookback_period: 100,
      max_position_size: 100000.0
    }
  }),
  updateStrategyParams: async (name: string, params: StrategyParams) => ({ 
    data: { success: true, message: `Updated parameters for strategy: ${name}` }
  }),
  evaluateStrategies: async () => ({
    data: {
      timestamp: new Date().toISOString(),
      results: [
        {
          strategy: 'Statistical Arbitrage',
          confidence: 0.85,
          expected_profit: 12500.0,
          signals: 3,
          is_best: true
        },
        {
          strategy: 'Event Arbitrage',
          confidence: 0.72,
          expected_profit: 8200.0,
          signals: 2,
          is_best: false
        }
      ],
      best_strategy: 'Statistical Arbitrage'
    }
  })
};

// Define the initial state
interface StrategyState {
  strategies: string[];
  activeStrategy: string | null;
  strategyParams: Record<string, any>;
  evaluationResults: StrategyEvaluation | null;
  loading: boolean;
  error: string | null;
}

const initialState: StrategyState = {
  strategies: [],
  activeStrategy: null,
  strategyParams: {},
  evaluationResults: null,
  loading: false,
  error: null
};

// Async thunks
export const fetchStrategies = createAsyncThunk(
  'strategy/fetchStrategies',
  async (_, { rejectWithValue }) => {
    try {
      const response = await strategyService.getStrategies();
      return response.data;
    } catch (error: any) {
      return rejectWithValue(error.response?.data?.error || 'Failed to fetch strategies');
    }
  }
);

export const fetchActiveStrategy = createAsyncThunk(
  'strategy/fetchActiveStrategy',
  async (_, { rejectWithValue }) => {
    try {
      const response = await strategyService.getActiveStrategy();
      return response.data;
    } catch (error: any) {
      return rejectWithValue(error.response?.data?.error || 'Failed to fetch active strategy');
    }
  }
);

export const setActiveStrategy = createAsyncThunk(
  'strategy/setActiveStrategy',
  async (name: string, { rejectWithValue }) => {
    try {
      const response = await strategyService.setActiveStrategy(name);
      return { name, ...response.data };
    } catch (error: any) {
      return rejectWithValue(error.response?.data?.error || 'Failed to set active strategy');
    }
  }
);

export const fetchStrategyParams = createAsyncThunk(
  'strategy/fetchStrategyParams',
  async (name: string, { rejectWithValue }) => {
    try {
      const response = await strategyService.getStrategyParams(name);
      return { name, params: response.data };
    } catch (error: any) {
      return rejectWithValue(error.response?.data?.error || 'Failed to fetch strategy parameters');
    }
  }
);

export const updateStrategyParams = createAsyncThunk(
  'strategy/updateStrategyParams',
  async ({ name, params }: { name: string; params: any }, { rejectWithValue }) => {
    try {
      const response = await strategyService.updateStrategyParams(name, params);
      return { name, params, ...response.data };
    } catch (error: any) {
      return rejectWithValue(error.response?.data?.error || 'Failed to update strategy parameters');
    }
  }
);

export const evaluateStrategies = createAsyncThunk(
  'strategy/evaluateStrategies',
  async (_, { rejectWithValue }) => {
    try {
      const response = await strategyService.evaluateStrategies();
      return response.data;
    } catch (error: any) {
      return rejectWithValue(error.response?.data?.error || 'Failed to evaluate strategies');
    }
  }
);

// Create the slice
const strategySlice = createSlice({
  name: 'strategy',
  initialState,
  reducers: {
    clearErrors: (state) => {
      state.error = null;
    },
  },
  extraReducers: (builder) => {
    builder
      // Fetch strategies
      .addCase(fetchStrategies.pending, (state) => {
        state.loading = true;
        state.error = null;
      })
      .addCase(fetchStrategies.fulfilled, (state, action: PayloadAction<string[]>) => {
        state.strategies = action.payload;
        state.loading = false;
      })
      .addCase(fetchStrategies.rejected, (state, action) => {
        state.loading = false;
        state.error = action.payload as string;
      })
      
      // Fetch active strategy
      .addCase(fetchActiveStrategy.pending, (state) => {
        state.loading = true;
        state.error = null;
      })
      .addCase(fetchActiveStrategy.fulfilled, (state, action: PayloadAction<string>) => {
        state.activeStrategy = action.payload;
        state.loading = false;
      })
      .addCase(fetchActiveStrategy.rejected, (state, action) => {
        state.loading = false;
        state.error = action.payload as string;
      })
      
      // Set active strategy
      .addCase(setActiveStrategy.pending, (state) => {
        state.loading = true;
        state.error = null;
      })
      .addCase(setActiveStrategy.fulfilled, (state, action: PayloadAction<{name: string; success: boolean; message: string}>) => {
        state.activeStrategy = action.payload.name;
        state.loading = false;
      })
      .addCase(setActiveStrategy.rejected, (state, action) => {
        state.loading = false;
        state.error = action.payload as string;
      })
      
      // Fetch strategy parameters
      .addCase(fetchStrategyParams.pending, (state) => {
        state.loading = true;
        state.error = null;
      })
      .addCase(fetchStrategyParams.fulfilled, (state, action: PayloadAction<{name: string; params: any}>) => {
        state.strategyParams[action.payload.name] = action.payload.params;
        state.loading = false;
      })
      .addCase(fetchStrategyParams.rejected, (state, action) => {
        state.loading = false;
        state.error = action.payload as string;
      })
      
      // Update strategy parameters
      .addCase(updateStrategyParams.pending, (state) => {
        state.loading = true;
        state.error = null;
      })
      .addCase(updateStrategyParams.fulfilled, (state, action: PayloadAction<{name: string; params: any; success: boolean}>) => {
        state.strategyParams[action.payload.name] = action.payload.params;
        state.loading = false;
      })
      .addCase(updateStrategyParams.rejected, (state, action) => {
        state.loading = false;
        state.error = action.payload as string;
      })
      
      // Evaluate strategies
      .addCase(evaluateStrategies.pending, (state) => {
        state.loading = true;
        state.error = null;
      })
      .addCase(evaluateStrategies.fulfilled, (state, action: PayloadAction<StrategyEvaluation>) => {
        state.evaluationResults = action.payload;
        // If best strategy is different from active strategy, update active strategy
        if (action.payload.best_strategy && action.payload.best_strategy !== state.activeStrategy) {
          state.activeStrategy = action.payload.best_strategy;
        }
        state.loading = false;
      })
      .addCase(evaluateStrategies.rejected, (state, action) => {
        state.loading = false;
        state.error = action.payload as string;
      });
  },
});

export const { clearErrors } = strategySlice.actions;
export default strategySlice.reducer; 