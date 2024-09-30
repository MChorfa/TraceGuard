import axios from 'axios';
import { SBOM, ProvenanceRecord } from '../types';

const API_BASE_URL = 'http://localhost:8080/api';

export const api = {
  // Existing SBOM methods...

  // New Provenance methods
  async createProvenance(slsaProvenance: any): Promise<ProvenanceRecord> {
    const response = await axios.post(`${API_BASE_URL}/provenance`, slsaProvenance);
    return response.data;
  },

  async getProvenance(id: string): Promise<ProvenanceRecord> {
    const response = await axios.get(`${API_BASE_URL}/provenance/${id}`);
    return response.data;
  },

  async updateProvenance(id: string, record: ProvenanceRecord): Promise<ProvenanceRecord> {
    const response = await axios.put(`${API_BASE_URL}/provenance/${id}`, record);
    return response.data;
  },

  async deleteProvenance(id: string): Promise<void> {
    await axios.delete(`${API_BASE_URL}/provenance/${id}`);
  },

  async listProvenance(page: number = 1, pageSize: number = 10): Promise<{ records: ProvenanceRecord[], total: number }> {
    const response = await axios.get(`${API_BASE_URL}/provenance`, { params: { page, page_size: pageSize } });
    return response.data;
  },

  async verifySlsaProvenance(id: string): Promise<boolean> {
    const response = await axios.get(`${API_BASE_URL}/provenance/${id}/verify`);
    return response.data;
  }
};