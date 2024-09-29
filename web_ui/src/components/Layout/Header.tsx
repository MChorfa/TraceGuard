import React from 'react';
import { Layout, Menu } from 'antd';
import { Link } from 'react-router-dom';
import { useAuth } from '../../hooks/useAuth';
import LoginButton from '../Auth/LoginButton';
import UserProfile from '../Auth/UserProfile';

const { Header } = Layout;

const AppHeader: React.FC = () => {
  const { isAuthenticated } = useAuth();

  return (
    <Header className="app-header">
      <div className="logo">TraceGuard</div>
      <Menu theme="dark" mode="horizontal" defaultSelectedKeys={['1']}>
        <Menu.Item key="1">
          <Link to="/">Home</Link>
        </Menu.Item>
        <Menu.Item key="2">
          <Link to="/sboms">SBOMs</Link>
        </Menu.Item>
        <Menu.Item key="3">
          <Link to="/provenance">Provenance</Link>
        </Menu.Item>
        <Menu.Item key="4">
          <Link to="/compliance">Compliance</Link>
        </Menu.Item>
      </Menu>
      <div className="auth-section">
        {isAuthenticated ? <UserProfile /> : <LoginButton />}
      </div>
    </Header>
  );
};

export default AppHeader;