import React, { useState, useEffect } from 'react';
import { Button, Paper, Table, TableBody, TableCell, TableContainer, TableHead, TableRow, Typography, TextField } from '@material-ui/core';
import axios from 'axios';

interface DashboardProps {
  setToken: (token: string | null) => void;
}

interface SBOM {
  id: string;
  format: string;
  version: string;
  components: string;
}

interface ProvenanceRecord {
  id: string;
  artifact_id: string;
  timestamp: string;
  slsa_level: number;
}

const Dashboard: React.FC<DashboardProps> = ({ setToken }) => {
  const [sboms, setSboms] = useState<SBOM[]>([]);
  const [provenanceRecords, setProvenanceRecords] = useState<ProvenanceRecord[]>([]);
  const [sbomFile, setSbomFile] = useState<File | null>(null);
  const [complianceReport, setComplianceReport] = useState<string | null>(null);

  useEffect(() => {
    fetchSBOMs();
    fetchProvenanceRecords();
  }, []);

  const fetchSBOMs = async () => {
    try {
      const response = await axios.get<SBOM[]>('/api/sboms');
      setSboms(response.data);
    } catch (error) {
      console.error('Error fetching SBOMs:', error);
    }
  };

  const fetchProvenanceRecords = async () => {
    try {
      const response = await axios.get<ProvenanceRecord[]>('/api/provenance');
      setProvenanceRecords(response.data);
    } catch (error) {
      console.error('Error fetching provenance records:', error);
    }
  };

  const handleSbomUpload = async (event: React.ChangeEvent<HTMLInputElement>) => {
    const file = event.target.files?.[0];
    if (file) {
      setSbomFile(file);
      const formData = new FormData();
      formData.append('sbom', file);
      try {
        await axios.post('/api/sboms', formData, {
          headers: {
            'Content-Type': 'multipart/form-data',
          },
        });
        fetchSBOMs();
      } catch (error) {
        console.error('Error uploading SBOM:', error);
      }
    }
  };

  const generateComplianceReport = async () => {
    try {
      const response = await axios.get('/api/compliance/report');
      setComplianceReport(response.data);
    } catch (error) {
      console.error('Error generating compliance report:', error);
    }
  };

  const handleLogout = () => {
    setToken(null);
  };

  return (
    <div>
      <Button onClick={handleLogout}>Logout</Button>
      <Paper style={{ margin: '20px 0', padding: '20px' }}>
        <Typography variant="h5">Upload SBOM</Typography>
        <input
          accept=".json,.xml"
          style={{ display: 'none' }}
          id="sbom-file-upload"
          type="file"
          onChange={handleSbomUpload}
        />
        <label htmlFor="sbom-file-upload">
          <Button variant="contained" component="span">
            Upload SBOM
          </Button>
        </label>
        {sbomFile && <Typography>{sbomFile.name}</Typography>}
      </Paper>
      <Paper style={{ margin: '20px 0', padding: '20px' }}>
        <Typography variant="h5">SBOMs</Typography>
        <TableContainer>
          <Table>
            <TableHead>
              <TableRow>
                <TableCell>ID</TableCell>
                <TableCell>Format</TableCell>
                <TableCell>Version</TableCell>
                <TableCell>Components</TableCell>
              </TableRow>
            </TableHead>
            <TableBody>
              {sboms.map((sbom) => (
                <TableRow key={sbom.id}>
                  <TableCell>{sbom.id}</TableCell>
                  <TableCell>{sbom.format}</TableCell>
                  <TableCell>{sbom.version}</TableCell>
                  <TableCell>{sbom.components}</TableCell>
                </TableRow>
              ))}
            </TableBody>
          </Table>
        </TableContainer>
      </Paper>
      <Paper style={{ margin: '20px 0', padding: '20px' }}>
        <Typography variant="h5">Provenance Records</Typography>
        <TableContainer>
          <Table>
            <TableHead>
              <TableRow>
                <TableCell>ID</TableCell>
                <TableCell>Artifact ID</TableCell>
                <TableCell>Timestamp</TableCell>
                <TableCell>SLSA Level</TableCell>
              </TableRow>
            </TableHead>
            <TableBody>
              {provenanceRecords.map((record) => (
                <TableRow key={record.id}>
                  <TableCell>{record.id}</TableCell>
                  <TableCell>{record.artifact_id}</TableCell>
                  <TableCell>{record.timestamp}</TableCell>
                  <TableCell>{record.slsa_level}</TableCell>
                </TableRow>
              ))}
            </TableBody>
          </Table>
        </TableContainer>
      </Paper>
      <Paper style={{ margin: '20px 0', padding: '20px' }}>
        <Typography variant="h5">Compliance Report</Typography>
        <Button onClick={generateComplianceReport} variant="contained" color="primary">
          Generate Compliance Report
        </Button>
        {complianceReport && (
          <TextField
            multiline
            fullWidth
            rows={10}
            value={complianceReport}
            variant="outlined"
            margin="normal"
          />
        )}
      </Paper>
    </div>
  );
};

export default Dashboard;