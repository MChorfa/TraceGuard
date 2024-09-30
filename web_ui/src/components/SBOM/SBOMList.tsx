import React, { useState, useEffect } from 'react';
import { Table, message, Spin } from 'antd';
import { fetchSBOMs } from '../../api/sbom';
import { SBOM } from '../../types';
import { ColumnsType } from 'antd/es/table';

const SBOMList: React.FC = () => {
  const [sboms, setSBOMs] = useState<SBOM[]>([]);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    loadSBOMs();
  }, []);

  const loadSBOMs = async () => {
    try {
      const data = await fetchSBOMs();
      setSBOMs(data);
    } catch (error) {
      message.error('Failed to load SBOMs');
    } finally {
      setLoading(false);
    }
  };

  const columns: ColumnsType<SBOM> = [
    {
      title: 'ID',
      dataIndex: 'id',
      key: 'id',
    },
    {
      title: 'Name',
      dataIndex: 'name',
      key: 'name',
    },
    {
      title: 'Format',
      dataIndex: 'format',
      key: 'format',
    },
    {
      title: 'Version',
      dataIndex: 'version',
      key: 'version',
    },
    {
      title: 'Created At',
      dataIndex: 'created_at',
      key: 'created_at',
      render: (date: string) => new Date(date).toLocaleString(),
    },
  ];

  return (
    <Spin spinning={loading}>
      <Table columns={columns} dataSource={sboms} rowKey="id" />
    </Spin>
  );
};

export default SBOMList;