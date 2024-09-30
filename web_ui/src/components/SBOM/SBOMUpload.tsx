import React, { useState } from 'react';
import { Upload, message, Button, Form, Input, Select } from 'antd';
import { UploadOutlined } from '@ant-design/icons';
import { RcFile } from 'antd/es/upload';
import { uploadSBOM } from '../../api/sbom';

const { Option } = Select;

const SBOMUpload: React.FC = () => {
  const [form] = Form.useForm();
  const [uploading, setUploading] = useState(false);
  const [fileList, setFileList] = useState<RcFile[]>([]);

  const handleUpload = async () => {
    try {
      const values = await form.validateFields();
      if (fileList.length === 0) {
        message.error('Please select a file to upload');
        return;
      }

      setUploading(true);
      const formData = new FormData();
      formData.append('file', fileList[0]);
      formData.append('name', values.name);
      formData.append('format', values.format);
      formData.append('version', values.version);

      await uploadSBOM(formData);
      message.success('SBOM uploaded successfully');
      form.resetFields();
      setFileList([]);
    } catch (error) {
      message.error('Failed to upload SBOM');
      console.error('Upload error:', error);
    } finally {
      setUploading(false);
    }
  };

  const uploadProps = {
    accept: '.json,.xml,.spdx',
    beforeUpload: (file: RcFile) => {
      setFileList([file]);
      return false;
    },
    fileList,
  };

  return (
    <Form form={form} layout="vertical">
      <Form.Item name="name" label="SBOM Name" rules={[{ required: true }]}>
        <Input />
      </Form.Item>
      <Form.Item name="format" label="SBOM Format" rules={[{ required: true }]}>
        <Select>
          <Option value="CycloneDX">CycloneDX</Option>
          <Option value="SPDX">SPDX</Option>
          <Option value="SWID">SWID</Option>
        </Select>
      </Form.Item>
      <Form.Item name="version" label="SBOM Version" rules={[{ required: true }]}>
        <Input />
      </Form.Item>
      <Form.Item>
        <Upload {...uploadProps}>
          <Button icon={<UploadOutlined />}>Select File</Button>
        </Upload>
      </Form.Item>
      <Form.Item>
        <Button type="primary" onClick={handleUpload} loading={uploading} disabled={fileList.length === 0}>
          Upload SBOM
        </Button>
      </Form.Item>
    </Form>
  );
};

export default SBOMUpload;