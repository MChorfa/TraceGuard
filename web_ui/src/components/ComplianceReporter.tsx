import React, { useState } from 'react';
import { Paper, Typography, Button, CircularProgress, TextField } from '@material-ui/core';
import axios from 'axios';

interface ComplianceReport {
  id: number;
  report_type: string;
  content: string;
  generated_at: string;
}

const ComplianceReporter: React.FC = () => {
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [report, setReport] = useState<ComplianceReport | null>(null);
  const [tenantId, setTenantId] = useState('');

  const generateReport = async () => {
    setLoading(true);
    setError(null);
    try {
      const response = await axios.post<ComplianceReport>('/api/compliance/report', { tenant_id: tenantId });
      setReport(response.data);
    } catch (err) {
      setError('Failed to generate compliance report');
      console.error('Error generating compliance report:', err);
    } finally {
      setLoading(false);
    }
  };

  return (
    <Paper style={{ padding: '20px', marginTop: '20px' }}>
      <Typography variant="h6" gutterBottom>Compliance Reporter</Typography>
      <TextField
        label="Tenant ID"
        value={tenantId}
        onChange={(e) => setTenantId(e.target.value)}
        fullWidth
        margin="normal"
      />
      <Button
        variant="contained"
        color="primary"
        onClick={generateReport}
        disabled={loading || !tenantId}
      >
        Generate Report
      </Button>
      {loading && <CircularProgress style={{ marginLeft: '10px' }} />}
      {error && <Typography color="error">{error}</Typography>}
      {report && (
        <div style={{ marginTop: '20px' }}>
          <Typography variant="subtitle1">Report ID: {report.id}</Typography>
          <Typography variant="subtitle1">Type: {report.report_type}</Typography>
          <Typography variant="subtitle1">Generated At: {report.generated_at}</Typography>
          <Typography variant="subtitle1">Content:</Typography>
          <pre style={{ whiteSpace: 'pre-wrap', wordBreak: 'break-all' }}>
            {JSON.stringify(JSON.parse(report.content), null, 2)}
          </pre>
        </div>
      )}
    </Paper>
  );
};

export default ComplianceReporter;