import React, { useState, useEffect } from 'react';
import { Layout, Card, Row, Col, Select } from 'antd';
import { useAuth } from '../hooks/useAuth';
import SBOMList from '../components/SBOM/SBOMList';
import ProvenanceList from '../components/Provenance/ProvenanceList';
import ComplianceReport from '../components/Compliance/ComplianceReport';

const { Content } = Layout;
const { Option } = Select;

const Dashboard: React.FC = () => {
  const { user } = useAuth();
  const [layout, setLayout] = useState<string[]>([]);

  useEffect(() => {
    // Load user's dashboard layout preference
    const savedLayout = localStorage.getItem(`dashboard_layout_${user?.id}`);
    setLayout(savedLayout ? JSON.parse(savedLayout) : ['sboms', 'provenance', 'compliance']);
  }, [user]);

  const saveLayout = (newLayout: string[]) => {
    setLayout(newLayout);
    localStorage.setItem(`dashboard_layout_${user?.id}`, JSON.stringify(newLayout));
  };

  const renderComponent = (componentKey: string) => {
    switch (componentKey) {
      case 'sboms':
        return <SBOMList />;
      case 'provenance':
        return <ProvenanceList />;
      case 'compliance':
        return <ComplianceReport />;
      default:
        return null;
    }
  };

  return (
    <Content>
      <h1>Dashboard</h1>
      <Select
        mode="multiple"
        style={{ width: '100%', marginBottom: 16 }}
        placeholder="Customize your dashboard"
        value={layout}
        onChange={saveLayout}
      >
        <Option value="sboms">SBOMs</Option>
        <Option value="provenance">Provenance</Option>
        <Option value="compliance">Compliance</Option>
      </Select>
      <Row gutter={[16, 16]}>
        {layout.map((componentKey) => (
          <Col key={componentKey} xs={24} sm={12} lg={8}>
            <Card title={componentKey.toUpperCase()}>
              {renderComponent(componentKey)}
            </Card>
          </Col>
        ))}
      </Row>
    </Content>
  );
};

export default Dashboard;