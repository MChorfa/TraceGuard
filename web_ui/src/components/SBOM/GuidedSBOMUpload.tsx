import React, { useState } from 'react';
import { Steps, Button, message, Upload } from 'antd';
import { UploadOutlined } from '@ant-design/icons';
import { RcFile } from 'antd/lib/upload';
import { uploadSBOM } from '../../api/sbom';

const { Step } = Steps;

const GuidedSBOMUpload: React.FC = () => {
  const [currentStep, setCurrentStep] = useState(0);
  const [file, setFile] = useState<RcFile | null>(null);

  const steps = [
    {
      title: 'Select File',
      content: (
        <Upload
          beforeUpload={(file) => {
            setFile(file);
            return false;
          }}
        >
          <Button icon={<UploadOutlined />}>Select SBOM File</Button>
        </Upload>
      ),
    },
    {
      title: 'Confirm',
      content: (
        <div>
          <p>File selected: {file?.name}</p>
          <p>Size: {file?.size} bytes</p>
          <p>Type: {file?.type}</p>
        </div>
      ),
    },
    {
      title: 'Upload',
      content: 'Click "Done" to upload your SBOM.',
    },
  ];

  const next = () => {
    setCurrentStep(currentStep + 1);
  };

  const prev = () => {
    setCurrentStep(currentStep - 1);
  };

  const handleUpload = async () => {
    if (!file) {
      message.error('Please select an SBOM file to upload');
      return;
    }

    try {
      await uploadSBOM(file);
      message.success('SBOM uploaded successfully');
      setCurrentStep(0);
      setFile(null);
    } catch (error) {
      console.error('Error uploading SBOM:', error);
      message.error('Failed to upload SBOM');
    }
  };

  return (
    <div>
      <Steps current={currentStep}>
        {steps.map(item => <Step key={item.title} title={item.title} />)}
      </Steps>
      <div className="steps-content">{steps[currentStep].content}</div>
      <div className="steps-action">
        {currentStep < steps.length - 1 && (
          <Button type="primary" onClick={next}>
            Next
          </Button>
        )}
        {currentStep === steps.length - 1 && (
          <Button type="primary" onClick={handleUpload}>
            Done
          </Button>
        )}
        {currentStep > 0 && (
          <Button style={{ margin: '0 8px' }} onClick={prev}>
            Previous
          </Button>
        )}
      </div>
    </div>
  );
};

export default GuidedSBOMUpload;