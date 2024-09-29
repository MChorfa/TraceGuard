import React, { useState } from 'react';
import axios from 'axios';
import { Button, TextField, Typography, Container } from '@material-ui/core';

interface AuthProps {
  setToken: (token: string) => void;
}

const Auth: React.FC<AuthProps> = ({ setToken }) => {
  const [username, setUsername] = useState('');
  const [password, setPassword] = useState('');
  const [isLogin, setIsLogin] = useState(true);

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    try {
      const endpoint = isLogin ? '/api/login' : '/api/register';
      const response = await axios.post(endpoint, { username, password });
      if (response.data.token) {
        setToken(response.data.token);
      }
    } catch (error) {
      console.error('Authentication error:', error);
    }
  };

  return (
    <Container maxWidth="sm">
      <Typography variant="h4">{isLogin ? 'Login' : 'Register'}</Typography>
      <form onSubmit={handleSubmit}>
        <TextField
          label="Username"
          value={username}
          onChange={(e) => setUsername(e.target.value)}
          fullWidth
          margin="normal"
        />
        <TextField
          label="Password"
          type="password"
          value={password}
          onChange={(e) => setPassword(e.target.value)}
          fullWidth
          margin="normal"
        />
        <Button type="submit" variant="contained" color="primary">
          {isLogin ? 'Login' : 'Register'}
        </Button>
      </form>
      <Button onClick={() => setIsLogin(!isLogin)}>
        {isLogin ? 'Need to register?' : 'Already have an account?'}
      </Button>
    </Container>
  );
};

export default Auth;