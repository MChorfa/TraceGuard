import React, { useState, useEffect } from 'react';
import { Layout, Card, Row, Col, Select, message } from 'antd';
import { useAuth } from '../hooks/useAuth';
import SBOMList from '../components/SBOM/SBOMList';
import SBOMGraph from '../components/SBOM/SBOMGraph';
import ProvenanceList from '../components/Provenance/ProvenanceList';
import ComplianceReport from '../components/Compliance/ComplianceReport';
import { fetchSBOMs, fetchSBOMRelationships } from '../api/sbom';
import { SBOM } from '../types';

const { Content } = Layout;
const { Option } = Select;

const Dashboard: React.FC = () => {
  const { user } = useAuth();
  const [layout, setLayout] = useState<string[]>(['sbom', 'sbomGraph']);
  const [sboms, setSBOMs] = useState<SBOM[]>([]);
  const [relationships, setRelationships] = useState<[string, string][]>([]);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    loadSBOMData();
  }, []);

  const loadSBOMData = async () => {
    try {
      setLoading(true);
      const [sbomData, relationshipData] = await Promise.all([
        fetchSBOMs(),
        fetchSBOMRelationships()
      ]);
      setSBOMs(sbomData);
      setRelationships(relationshipData);
    } catch (error) {
      message.error('Failed to load SBOM data');
    } finally {
      setLoading(false);
    }
  };

  const renderComponent = (key: string) => {
    switch (key) {
      case 'sbom':
        return <SBOMList sboms={sboms} loading={loading} />;
      case 'sbomGraph':
        return <SBOMGraph sboms={sboms} relationships={relationships} loading={loading} />;
      case 'provenance':
        return <ProvenanceList />;
      case 'compliance':
        return <ComplianceReport />;
      default:
        return null;
    }
  };

  return (
    <Content style={{ padding: '0 50px' }}>
      <h1>Welcome, {user?.name}!</h1>
      <Select
        mode="multiple"
        style={{ width: '100%', marginBottom: '20px' }}
        placeholder="Select dashboard components"
        value={layout}
        onChange={setLayout}
      >
        <Option value="sbom">SBOM List</Option>
        <Option value="sbomGraph">SBOM Graph</Option>
        <Option value="provenance">Provenance</Option>
        <Option value="compliance">Compliance</Option>
      </Select>
      <Row gutter={[16, 16]}>
        {layout.map((key) => (
          <Col key={key} xs={24} sm={24} md={12} lg={12} xl={6}>
            <Card title={key.toUpperCase()} style={{ height: '100%' }}>
              {renderComponent(key)}
            </Card>
          </Col>
        ))}
      </Row>
    </Content>
  );
};

export default Dashboard;