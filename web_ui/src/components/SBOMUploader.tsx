import React, { useState } from 'react';
import { Button, Paper, Typography, CircularProgress } from '@material-ui/core';
import axios from 'axios';

const SBOMUploader: React.FC = () => {
  const [file, setFile] = useState<File | null>(null);
  const [uploading, setUploading] = useState(false);
  const [uploadStatus, setUploadStatus] = useState<string | null>(null);

  const handleFileChange = (event: React.ChangeEvent<HTMLInputElement>) => {
    if (event.target.files) {
      setFile(event.target.files[0]);
    }
  };

  const handleUpload = async () => {
    if (!file) return;

    setUploading(true);
    const formData = new FormData();
    formData.append('sbom', file);

    try {
      const response = await axios.post('/api/sboms/upload', formData, {
        headers: {
          'Content-Type': 'multipart/form-data',
        },
      });
      setUploadStatus('SBOM uploaded successfully');
    } catch (error) {
      console.error('Error uploading SBOM:', error);
      setUploadStatus('Error uploading SBOM');
    } finally {
      setUploading(false);
    }
  };

  return (
    <Paper style={{ padding: '20px', marginTop: '20px' }}>
      <Typography variant="h6">Upload SBOM</Typography>
      <input
        accept=".json,.xml,.spdx"
        style={{ display: 'none' }}
        id="raised-button-file"
        type="file"
        onChange={handleFileChange}
      />
      <label htmlFor="raised-button-file">
        <Button variant="contained" component="span">
          Select SBOM File
        </Button>
      </label>
      {file && <Typography>{file.name}</Typography>}
      <Button
        onClick={handleUpload}
        disabled={!file || uploading}
        variant="contained"
        color="primary"
        style={{ marginTop: '10px' }}
      >
        {uploading ? <CircularProgress size={24} /> : 'Upload'}
      </Button>
      {uploadStatus && <Typography>{uploadStatus}</Typography>}
    </Paper>
  );
};

export default SBOMUploader;