import React, { useState, useEffect } from 'react';
import { AppBar, Toolbar, Typography, Container } from '@material-ui/core';
import axios from 'axios';
import Auth from './Auth';
import Dashboard from './Dashboard';

function App() {
  const [token, setToken] = useState<string | null>(localStorage.getItem('token'));

  useEffect(() => {
    if (token) {
      localStorage.setItem('token', token);
      axios.defaults.headers.common['Authorization'] = `Bearer ${token}`;
    } else {
      localStorage.removeItem('token');
      delete axios.defaults.headers.common['Authorization'];
    }
  }, [token]);

  return (
    <div>
      <AppBar position="static">
        <Toolbar>
          <Typography variant="h6">TraceGuard Dashboard</Typography>
        </Toolbar>
      </AppBar>
      <Container>
        {token ? (
          <Dashboard setToken={setToken} />
        ) : (
          <Auth setToken={setToken} />
        )}
      </Container>
    </div>
  );
}

export default App;