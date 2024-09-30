import { configureStore } from '@reduxjs/toolkit';
import sbomReducer from './sbomSlice';

export const store = configureStore({
  reducer: {
    sbom: sbomReducer,
  },
});

export type RootState = ReturnType<typeof store.getState>;
export type AppDispatch = typeof store.dispatch;