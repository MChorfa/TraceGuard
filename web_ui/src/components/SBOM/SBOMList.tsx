import React, { useState, useEffect } from 'react';
import { Table, message, Spin, Button } from 'antd';
import { DownloadOutlined, EyeOutlined } from '@ant-design/icons';
import { fetchSBOMs, downloadSBOM } from '../../api/sbom';
import { SBOM } from '../../types';
import { ColumnsType } from 'antd/es/table';

const SBOMList: React.FC = () => {
  const [sboms, setSBOMs] = useState<SBOM[]>([]);
  const [loading, setLoading] = useState(true);
  const [page, setPage] = useState(1);
  const [pageSize, setPageSize] = useState(10);
  const [total, setTotal] = useState(0);

  useEffect(() => {
    loadSBOMs();
  }, [page, pageSize]);

  const loadSBOMs = async () => {
    try {
      setLoading(true);
      const data = await fetchSBOMs(page, pageSize);
      setSBOMs(data.sboms);
      setTotal(data.total);
    } catch (error) {
      message.error('Failed to load SBOMs');
    } finally {
      setLoading(false);
    }
  };

  const handleDownload = async (id: string) => {
    try {
      const sbomData = await downloadSBOM(id);
      const blob = new Blob([JSON.stringify(sbomData, null, 2)], { type: 'application/json' });
      const url = window.URL.createObjectURL(blob);
      const link = document.createElement('a');
      link.href = url;
      link.download = `sbom_${id}.json`;
      link.click();
    } catch (error) {
      message.error('Failed to download SBOM');
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
    {
      title: 'Actions',
      key: 'actions',
      render: (_, record) => (
        <>
          <Button icon={<EyeOutlined />} onClick={() => console.log('View details', record.id)}>
            View
          </Button>
          <Button icon={<DownloadOutlined />} onClick={() => handleDownload(record.id)}>
            Download
          </Button>
        </>
      ),
    },
  ];

  return (
    <Spin spinning={loading}>
      <Table 
        columns={columns} 
        dataSource={sboms} 
        rowKey="id" 
        pagination={{
          current: page,
          pageSize: pageSize,
          total: total,
          onChange: (page, pageSize) => {
            setPage(page);
            setPageSize(pageSize);
          },
        }}
      />
    </Spin>
  );
};

export default SBOMList;