import React from 'react';
import { Typography, Paper } from '@material-ui/core';

interface ErrorMessageProps {
  message: string;
}

const ErrorMessage: React.FC<ErrorMessageProps> = ({ message }) => {
  return (
    <Paper style={{ padding: '10px', backgroundColor: '#FFCCCB', marginTop: '10px' }}>
      <Typography color="error">{message}</Typography>
    </Paper>
  );
};

export default ErrorMessage;