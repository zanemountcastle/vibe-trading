import { configureStore } from '@reduxjs/toolkit';
import { TypedUseSelectorHook, useDispatch, useSelector } from 'react-redux';
import marketSlice from './slices/marketSlice';
import ordersSlice from './slices/ordersSlice';
import strategySlice from './slices/strategySlice';
import accountSlice from './slices/accountSlice';
import uiSlice from './slices/uiSlice';

export const store = configureStore({
  reducer: {
    market: marketSlice,
    orders: ordersSlice,
    strategy: strategySlice,
    account: accountSlice,
    ui: uiSlice,
  },
  middleware: (getDefaultMiddleware) =>
    getDefaultMiddleware({
      serializableCheck: false,
    }),
});

export type RootState = ReturnType<typeof store.getState>;
export type AppDispatch = typeof store.dispatch;

// Use throughout the app instead of plain `useDispatch` and `useSelector`
export const useAppDispatch = () => useDispatch<AppDispatch>();
export const useAppSelector: TypedUseSelectorHook<RootState> = useSelector; 