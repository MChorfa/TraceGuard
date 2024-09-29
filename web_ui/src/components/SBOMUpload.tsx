import React, { useState } from 'react';
import { Button, Input, message, Upload } from 'antd';
import { UploadOutlined } from '@ant-design/icons';
import { RcFile } from 'antd/lib/upload';
import { uploadSBOM } from '../api/sbom';

const SBOMUpload: React.FC = () => {
  const [file, setFile] = useState<RcFile | null>(null);
  const [uploading, setUploading] = useState(false);

  const handleUpload = async () => {
    if (!file) {
      message.error('Please select an SBOM file to upload');
      return;
    }

    setUploading(true);

    try {
      await uploadSBOM(file);
      message.success('SBOM uploaded successfully');
      setFile(null);
    } catch (error) {
      console.error('Error uploading SBOM:', error);
      message.error('Failed to upload SBOM');
    } finally {
      setUploading(false);
    }
  };

  const props = {
    onRemove: () => {
      setFile(null);
    },
    beforeUpload: (file: RcFile) => {
      setFile(file);
      return false;
    },
    file,
  };

  return (
    <div>
      <Upload {...props}>
        <Button icon={<UploadOutlined />}>Select SBOM File</Button>
      </Upload>
      <Button
        type="primary"
        onClick={handleUpload}
        disabled={!file}
        loading={uploading}
        style={{ marginTop: 16 }}
      >
        {uploading ? 'Uploading' : 'Start Upload'}
      </Button>
    </div>
  );
};

export default SBOMUpload;