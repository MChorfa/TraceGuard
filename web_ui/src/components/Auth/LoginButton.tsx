import React from 'react';
import { Button, Menu, Dropdown } from 'antd';
import { LoginOutlined } from '@ant-design/icons';
import { useAuth } from '../../hooks/useAuth';

const providers = ['Azure', 'GitHub', 'Google', 'GitLab'];

const LoginButton: React.FC = () => {
  const { login } = useAuth();

  const handleLogin = (provider: string) => {
    login(provider.toLowerCase());
  };

  const menu = (
    <Menu>
      {providers.map((provider) => (
        <Menu.Item key={provider} onClick={() => handleLogin(provider)}>
          {provider}
        </Menu.Item>
      ))}
    </Menu>
  );

  return (
    <Dropdown overlay={menu} placement="bottomRight">
      <Button type="primary" icon={<LoginOutlined />}>
        Login
      </Button>
    </Dropdown>
  );
};

export default LoginButton;