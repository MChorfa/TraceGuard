import React, { useEffect, useState } from 'react';
import { Table, message, Spin, Button, Modal } from 'antd';
import { DownloadOutlined, EyeOutlined } from '@ant-design/icons';
import { api } from '../services/api';
import { ProvenanceRecord } from '../types';
import { ColumnsType } from 'antd/es/table';
import { useAuth } from '../hooks/useAuth';

const ProvenanceList: React.FC = () => {
  const [provenanceRecords, setProvenanceRecords] = useState<ProvenanceRecord[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [selectedRecord, setSelectedRecord] = useState<ProvenanceRecord | null>(null);
  const [modalVisible, setModalVisible] = useState(false);
  const { user } = useAuth();

  useEffect(() => {
    fetchProvenanceRecords();
  }, []);

  const fetchProvenanceRecords = async () => {
    try {
      setLoading(true);
      const result = await api.listProvenance();
      setProvenanceRecords(result.records);
    } catch (err) {
      setError('Failed to fetch provenance records');
      message.error('Failed to fetch provenance records');
    } finally {
      setLoading(false);
    }
  };

  const handleVerify = async (id: string) => {
    try {
      await api.verifySlsaProvenance(id);
      message.success('Provenance verified successfully');
      fetchProvenanceRecords(); // Refresh the list
    } catch (err) {
      message.error('Failed to verify provenance');
    }
  };

  const handleDownload = async (id: string) => {
    try {
      const data = await api.downloadProvenance(id);
      const blob = new Blob([JSON.stringify(data, null, 2)], { type: 'application/json' });
      const url = window.URL.createObjectURL(blob);
      const link = document.createElement('a');
      link.href = url;
      link.download = `provenance_${id}.json`;
      link.click();
    } catch (err) {
      message.error('Failed to download provenance');
    }
  };

  const showDetails = (record: ProvenanceRecord) => {
    setSelectedRecord(record);
    setModalVisible(true);
  };

  const columns: ColumnsType<ProvenanceRecord> = [
    {
      title: 'ID',
      dataIndex: 'id',
      key: 'id',
    },
    {
      title: 'Build Type',
      dataIndex: ['slsa_provenance', 'build_type'],
      key: 'build_type',
    },
    {
      title: 'Created At',
      dataIndex: 'created_at',
      key: 'created_at',
      render: (date: string) => new Date(date).toLocaleString(),
    },
    {
      title: 'Actions',
      key: 'actions',
      render: (_, record) => (
        <>
          <Button icon={<EyeOutlined />} onClick={() => showDetails(record)}>
            Details
          </Button>
          <Button icon={<DownloadOutlined />} onClick={() => handleDownload(record.id)}>
            Download
          </Button>
          {user?.role === 'admin' && (
            <Button onClick={() => handleVerify(record.id)}>Verify</Button>
          )}
        </>
      ),
    },
  ];

  if (loading) return <Spin size="large" />;
  if (error) return <div>Error: {error}</div>;

  return (
    <div>
      <h2>Provenance Records</h2>
      <Table columns={columns} dataSource={provenanceRecords} rowKey="id" />
      <Modal
        title="Provenance Details"
        visible={modalVisible}
        onCancel={() => setModalVisible(false)}
        footer={null}
        width={800}
      >
        {selectedRecord && (
          <pre>{JSON.stringify(selectedRecord.slsa_provenance, null, 2)}</pre>
        )}
      </Modal>
    </div>
  );
};

export default ProvenanceList;