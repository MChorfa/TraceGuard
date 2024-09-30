import { createSlice, PayloadAction } from '@reduxjs/toolkit';
import { SBOM } from '../types';

interface SBOMState {
  sboms: SBOM[];
  loading: boolean;
  error: string | null;
}

const initialState: SBOMState = {
  sboms: [],
  loading: false,
  error: null,
};

export const sbomSlice = createSlice({
  name: 'sbom',
  initialState,
  reducers: {
    setSBOMs: (state, action: PayloadAction<SBOM[]>) => {
      state.sboms = action.payload;
    },
    setLoading: (state, action: PayloadAction<boolean>) => {
      state.loading = action.payload;
    },
    setError: (state, action: PayloadAction<string | null>) => {
      state.error = action.payload;
    },
  },
});

export const { setSBOMs, setLoading, setError } = sbomSlice.actions;

export default sbomSlice.reducer;