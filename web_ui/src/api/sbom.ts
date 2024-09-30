import axios from 'axios';

// ... existing code ...

export const fetchSBOMRelationships = async (): Promise<[string, string][]> => {
  try {
    const response = await axios.get('/api/sboms/relationships');
    return response.data;
  } catch (error) {
    console.error('Error fetching SBOM relationships:', error);
    throw error;
  }
};