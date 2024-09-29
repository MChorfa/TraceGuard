import React, { useState, useEffect } from 'react';
import { Paper, Typography, Table, TableBody, TableCell, TableContainer, TableHead, TableRow, CircularProgress, Button, TextField } from '@material-ui/core';
import { useTheme } from '@material-ui/core/styles';
import axios from 'axios';
import ErrorMessage from './ErrorMessage';

interface ProvenanceRecord {
  id: number;
  artifact_id: string;
  slsa_level: number;
  metadata: Record<string, any>;
  created_at: string;
}

const ProvenanceViewer: React.FC = () => {
  const [records, setRecords] = useState<ProvenanceRecord[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [filter, setFilter] = useState('');
  const theme = useTheme();

  useEffect(() => {
    fetchProvenanceRecords();
  }, []);

  const fetchProvenanceRecords = async () => {
    setLoading(true);
    setError(null);
    try {
      const response = await axios.get<ProvenanceRecord[]>('/api/provenance');
      setRecords(response.data);
    } catch (err) {
      setError('Failed to fetch provenance records');
      console.error('Error fetching provenance records:', err);
    } finally {
      setLoading(false);
    }
  };

  const filteredRecords = records.filter(record =>
    record.artifact_id.toLowerCase().includes(filter.toLowerCase())
  );

  const handleFilterChange = (event: React.ChangeEvent<HTMLInputElement>) => {
    setFilter(event.target.value);
  };

  if (loading) return <CircularProgress />;
  if (error) return <ErrorMessage message={error} />;

  return (
    <Paper style={{ padding: '20px', marginTop: '20px' }}>
      <Typography variant="h6" gutterBottom>Provenance Records</Typography>
      <TextField
        label="Filter by Artifact ID"
        variant="outlined"
        value={filter}
        onChange={handleFilterChange}
        style={{ marginBottom: '20px' }}
      />
      <TableContainer>
        <Table>
          <TableHead>
            <TableRow>
              <TableCell>ID</TableCell>
              <TableCell>Artifact ID</TableCell>
              <TableCell>SLSA Level</TableCell>
              <TableCell>Created At</TableCell>
              <TableCell>Actions</TableCell>
            </TableRow>
          </TableHead>
          <TableBody>
            {filteredRecords.map((record) => (
              <TableRow key={record.id}>
                <TableCell>{record.id}</TableCell>
                <TableCell>{record.artifact_id}</TableCell>
                <TableCell>{record.slsa_level}</TableCell>
                <TableCell>{new Date(record.created_at).toLocaleString()}</TableCell>
                <TableCell>
                  <Button
                    variant="contained"
                    color="primary"
                    size="small"
                    onClick={() => alert(`View details for ${record.artifact_id}`)}
                  >
                    View Details
                  </Button>
                </TableCell>
              </TableRow>
            ))}
          </TableBody>
        </Table>
      </TableContainer>
    </Paper>
  );
};

export default ProvenanceViewer;