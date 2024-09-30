import React, { useState } from 'react';
import { Upload, message, Button } from 'antd';
import { UploadOutlined } from '@ant-design/icons';
import { RcFile, UploadProps } from 'antd/es/upload';
import { uploadSBOM } from '../../api/sbom';

const SBOMUpload: React.FC = () => {
  const [uploading, setUploading] = useState(false);

  const handleUpload = async (file: RcFile) => {
    setUploading(true);
    try {
      await uploadSBOM(file);
      message.success('SBOM uploaded successfully');
    } catch (error) {
      message.error('Failed to upload SBOM');
      console.error('Upload error:', error);
    } finally {
      setUploading(false);
    }
  };

  const uploadProps: UploadProps = {
    accept: '.json,.xml,.spdx',
    beforeUpload: (file) => {
      handleUpload(file);
      return false;
    },
    showUploadList: false,
  };

  return (
    <Upload {...uploadProps}>
      <Button icon={<UploadOutlined />} loading={uploading} disabled={uploading}>
        {uploading ? 'Uploading...' : 'Upload SBOM'}
      </Button>
    </Upload>
  );
};

export default SBOMUpload;