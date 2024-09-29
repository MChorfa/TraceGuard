import React, { useState, useEffect } from 'react';
import { Button, Paper, Table, TableBody, TableCell, TableContainer, TableHead, TableRow, Typography, TextField, Tab, Tabs } from '@material-ui/core';
import axios from 'axios';

interface DashboardProps {
  setToken: (token: string | null) => void;
}

interface SBOM {
  id: string;
  format: string;
  version: string;
  content: string;
}

interface ProvenanceRecord {
  id: string;
  artifact_id: string;
  slsa_level: number;
  metadata: Record<string, any>;
}

const Dashboard: React.FC<DashboardProps> = ({ setToken }) => {
  const [sboms, setSboms] = useState<SBOM[]>([]);
  const [provenanceRecords, setProvenanceRecords] = useState<ProvenanceRecord[]>([]);
  const [newSbom, setNewSbom] = useState<Partial<SBOM>>({});
  const [activeTab, setActiveTab] = useState(0);
  const [complianceReport, setComplianceReport] = useState<string>('');

  useEffect(() => {
    fetchSboms();
    fetchProvenanceRecords();
  }, []);

  const fetchSboms = async () => {
    try {
      const response = await axios.get('/api/sboms');
      setSboms(response.data);
    } catch (error) {
      console.error('Error fetching SBOMs:', error);
    }
  };

  const fetchProvenanceRecords = async () => {
    try {
      const response = await axios.get('/api/provenance');
      setProvenanceRecords(response.data);
    } catch (error) {
      console.error('Error fetching provenance records:', error);
    }
  };

  const handleUploadSbom = async () => {
    try {
      await axios.post('/api/sboms', newSbom);
      setNewSbom({});
      fetchSboms();
    } catch (error) {
      console.error('Error uploading SBOM:', error);
    }
  };

  const handleLogout = () => {
    setToken(null);
  };

  const handleGenerateComplianceReport = async () => {
    try {
      const response = await axios.get('/api/compliance/report');
      setComplianceReport(response.data);
    } catch (error) {
      console.error('Error generating compliance report:', error);
    }
  };

  return (
    <div>
      <Typography variant="h4">TraceGuard Dashboard</Typography>
      <Button onClick={handleLogout}>Logout</Button>

      <Tabs value={activeTab} onChange={(_, newValue) => setActiveTab(newValue)}>
        <Tab label="SBOMs" />
        <Tab label="Provenance" />
        <Tab label="Compliance" />
      </Tabs>

      {activeTab === 0 && (
        <Paper>
          <Typography variant="h6">SBOMs</Typography>
          <TableContainer>
            <Table>
              <TableHead>
                <TableRow>
                  <TableCell>ID</TableCell>
                  <TableCell>Format</TableCell>
                  <TableCell>Version</TableCell>
                </TableRow>
              </TableHead>
              <TableBody>
                {sboms.map((sbom) => (
                  <TableRow key={sbom.id}>
                    <TableCell>{sbom.id}</TableCell>
                    <TableCell>{sbom.format}</TableCell>
                    <TableCell>{sbom.version}</TableCell>
                  </TableRow>
                ))}
              </TableBody>
            </Table>
          </TableContainer>
          <Typography variant="h6">Upload New SBOM</Typography>
          <TextField
            label="Format"
            value={newSbom.format || ''}
            onChange={(e) => setNewSbom({ ...newSbom, format: e.target.value })}
          />
          <TextField
            label="Version"
            value={newSbom.version || ''}
            onChange={(e) => setNewSbom({ ...newSbom, version: e.target.value })}
          />
          <TextField
            label="Content"
            multiline
            rows={4}
            value={newSbom.content || ''}
            onChange={(e) => setNewSbom({ ...newSbom, content: e.target.value })}
          />
          <Button onClick={handleUploadSbom}>Upload SBOM</Button>
        </Paper>
      )}

      {activeTab === 1 && (
        <Paper>
          <Typography variant="h6">Provenance Records</Typography>
          <TableContainer>
            <Table>
              <TableHead>
                <TableRow>
                  <TableCell>ID</TableCell>
                  <TableCell>Artifact ID</TableCell>
                  <TableCell>SLSA Level</TableCell>
                </TableRow>
              </TableHead>
              <TableBody>
                {provenanceRecords.map((record) => (
                  <TableRow key={record.id}>
                    <TableCell>{record.id}</TableCell>
                    <TableCell>{record.artifact_id}</TableCell>
                    <TableCell>{record.slsa_level}</TableCell>
                  </TableRow>
                ))}
              </TableBody>
            </Table>
          </TableContainer>
        </Paper>
      )}

      {activeTab === 2 && (
        <Paper>
          <Typography variant="h6">Compliance</Typography>
          <Button onClick={handleGenerateComplianceReport}>Generate Compliance Report</Button>
          {complianceReport && (
            <TextField
              label="Compliance Report"
              multiline
              rows={10}
              value={complianceReport}
              fullWidth
              variant="outlined"
              margin="normal"
            />
          )}
        </Paper>
      )}
    </div>
  );
};

export default Dashboard;