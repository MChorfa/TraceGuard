import React, { useCallback } from 'react';
import { Button, message, Upload } from 'antd';
import { UploadOutlined } from '@ant-design/icons';
import { RcFile } from 'antd/lib/upload';
import { uploadSBOM } from '../../api/sbom';

const SBOMUpload: React.FC = () => {
  const handleUpload = useCallback(async (file: RcFile) => {
    try {
      await uploadSBOM(file);
      message.success('SBOM uploaded successfully');
    } catch (error) {
      console.error('Error uploading SBOM:', error);
      message.error('Failed to upload SBOM');
    }
  }, []);

  return (
    <Upload
      accept=".json,.xml"
      beforeUpload={(file) => {
        handleUpload(file);
        return false;
      }}
    >
      <Button icon={<UploadOutlined />}>Upload SBOM</Button>
    </Upload>
  );
};

export default SBOMUpload;