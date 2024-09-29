import React, { useState, useEffect } from 'react';
import { Paper, Typography, Table, TableBody, TableCell, TableContainer, TableHead, TableRow, CircularProgress } from '@material-ui/core';
import axios from 'axios';

interface ProvenanceRecord {
  id: number;
  artifact_id: string;
  slsa_level: number;
  metadata: Record<string, any>;
}

const ProvenanceViewer: React.FC = () => {
  const [records, setRecords] = useState<ProvenanceRecord[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    const fetchProvenanceRecords = async () => {
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

    fetchProvenanceRecords();
  }, []);

  if (loading) return <CircularProgress />;
  if (error) return <Typography color="error">{error}</Typography>;

  return (
    <Paper style={{ padding: '20px', marginTop: '20px' }}>
      <Typography variant="h6" gutterBottom>Provenance Records</Typography>
      <TableContainer>
        <Table>
          <TableHead>
            <TableRow>
              <TableCell>ID</TableCell>
              <TableCell>Artifact ID</TableCell>
              <TableCell>SLSA Level</TableCell>
              <TableCell>Metadata</TableCell>
            </TableRow>
          </TableHead>
          <TableBody>
            {records.map((record) => (
              <TableRow key={record.id}>
                <TableCell>{record.id}</TableCell>
                <TableCell>{record.artifact_id}</TableCell>
                <TableCell>{record.slsa_level}</TableCell>
                <TableCell>{JSON.stringify(record.metadata)}</TableCell>
              </TableRow>
            ))}
          </TableBody>
        </Table>
      </TableContainer>
    </Paper>
  );
};

export default ProvenanceViewer;