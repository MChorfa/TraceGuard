import React, { useMemo } from 'react';
import { Table } from 'antd';
import { SBOM } from '../../types';

interface SBOMListProps {
  sboms: SBOM[];
}

const SBOMList: React.FC<SBOMListProps> = ({ sboms }) => {
  const columns = useMemo(() => [
    {
      title: 'ID',
      dataIndex: 'id',
      key: 'id',
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
  ], []);

  return <Table columns={columns} dataSource={sboms} rowKey="id" />;
};

export default React.memo(SBOMList);