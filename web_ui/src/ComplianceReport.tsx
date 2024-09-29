import React, { useState } from 'react';
import { Button, Paper, Typography, TextField } from '@material-ui/core';
import axios from 'axios';

const ComplianceReport: React.FC = () => {
  const [report, setReport] = useState<string>('');
  const [loading, setLoading] = useState<boolean>(false);

  const generateReport = async () => {
    setLoading(true);
    try {
      const response = await axios.get('/api/compliance/report');
      setReport(response.data);
    } catch (error) {
      console.error('Error generating compliance report:', error);
    } finally {
      setLoading(false);
    }
  };

  return (
    <Paper style={{ padding: '20px', marginTop: '20px' }}>
      <Typography variant="h6">Compliance Report</Typography>
      <Button onClick={generateReport} disabled={loading}>
        {loading ? 'Generating...' : 'Generate Report'}
      </Button>
      {report && (
        <TextField
          label="Compliance Report"
          multiline
          rows={10}
          value={report}
          fullWidth
          variant="outlined"
          margin="normal"
          InputProps={{ readOnly: true }}
        />
      )}
    </Paper>
  );
};

export default ComplianceReport;