import React, { useState } from 'react';
import { createSBOM } from '../services/grpcClient';

const SBOMUploader: React.FC = () => {
  const [file, setFile] = useState<File | null>(null);
  const [uploading, setUploading] = useState(false);
  const [message, setMessage] = useState('');

  const handleFileChange = (event: React.ChangeEvent<HTMLInputElement>) => {
    if (event.target.files) {
      setFile(event.target.files[0]);
    }
  };

  const handleUpload = async () => {
    if (!file) {
      setMessage('Please select a file to upload');
      return;
    }

    setUploading(true);
    try {
      const content = await file.text();
      const sbomId = await createSBOM(file.name, '1.0', 'json', content);
      setMessage(`SBOM uploaded successfully. ID: ${sbomId}`);
    } catch (error) {
      setMessage('Error uploading SBOM. Please try again.');
      console.error('Error uploading SBOM:', error);
    } finally {
      setUploading(false);
    }
  };

  return (
    <div className="sbom-uploader">
      <h2>Upload SBOM</h2>
      <input type="file" onChange={handleFileChange} accept=".json,.xml,.spdx,.cdx" />
      <button onClick={handleUpload} disabled={uploading}>
        {uploading ? 'Uploading...' : 'Upload SBOM'}
      </button>
      {message && <p>{message}</p>}
    </div>
  );
};

export default SBOMUploader;