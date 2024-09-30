import React, { useState } from 'react';
import { Steps, Button, message, Upload, Spin } from 'antd';
import { UploadOutlined } from '@ant-design/icons';
import { RcFile } from 'antd/es/upload';
import { uploadSBOM } from '../../api/sbom';

const { Step } = Steps;

const GuidedSBOMUpload: React.FC = () => {
  const [currentStep, setCurrentStep] = useState(0);
  const [file, setFile] = useState<RcFile | null>(null);
  const [uploading, setUploading] = useState(false);

  const steps = [
    {
      title: 'Select File',
      content: (
        <Upload
          beforeUpload={(file) => {
            setFile(file);
            return false;
          }}
          accept=".json,.xml,.spdx"
        >
          <Button icon={<UploadOutlined />}>Select SBOM File</Button>
        </Upload>
      ),
    },
    {
      title: 'Confirm',
      content: file && (
        <div>
          <p>File selected: {file.name}</p>
          <p>Size: {(file.size / 1024).toFixed(2)} KB</p>
          <p>Type: {file.type || 'Unknown'}</p>
        </div>
      ),
    },
    {
      title: 'Upload',
      content: <p>Click 'Done' to upload the SBOM file.</p>,
    },
  ];

  const next = () => {
    if (currentStep === 0 && !file) {
      message.error('Please select a file before proceeding.');
      return;
    }
    setCurrentStep(currentStep + 1);
  };

  const prev = () => setCurrentStep(currentStep - 1);

  const handleUpload = async () => {
    if (!file) {
      message.error('Please select a file first');
      return;
    }

    setUploading(true);
    try {
      await uploadSBOM(file);
      message.success('SBOM uploaded successfully');
      resetForm();
    } catch (error) {
      console.error('Error uploading SBOM:', error);
      message.error('Failed to upload SBOM');
    } finally {
      setUploading(false);
    }
  };

  const resetForm = () => {
    setCurrentStep(0);
    setFile(null);
  };

  return (
    <Spin spinning={uploading}>
      <Steps current={currentStep}>
        {steps.map(item => <Step key={item.title} title={item.title} />)}
      </Steps>
      <div className="steps-content" style={{ marginTop: 16, marginBottom: 16 }}>
        {steps[currentStep].content}
      </div>
      <div className="steps-action">
        {currentStep < steps.length - 1 && (
          <Button type="primary" onClick={next} disabled={currentStep === 0 && !file}>
            Next
          </Button>
        )}
        {currentStep === steps.length - 1 && (
          <Button type="primary" onClick={handleUpload} loading={uploading}>
            Done
          </Button>
        )}
        {currentStep > 0 && (
          <Button style={{ margin: '0 8px' }} onClick={prev}>
            Previous
          </Button>
        )}
      </div>
    </Spin>
  );
};

export default GuidedSBOMUpload;