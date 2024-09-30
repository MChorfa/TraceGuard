import axios from 'axios';
import { SBOM } from '../types';

const API_BASE_URL = process.env.REACT_APP_API_BASE_URL || 'http://localhost:8080/api';

export const fetchSBOMs = async (page: number, pageSize: number) => {
  const response = await axios.get(`${API_BASE_URL}/sboms`, {
    params: { page, per_page: pageSize },
  });
  return response.data;
};

export const uploadSBOM = async (formData: FormData) => {
  const response = await axios.post(`${API_BASE_URL}/sboms`, formData, {
    headers: { 'Content-Type': 'multipart/form-data' },
  });
  return response.data;
};

export const downloadSBOM = async (id: string) => {
  const response = await axios.get(`${API_BASE_URL}/sboms/${id}/download`);
  return response.data;
};