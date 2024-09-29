import React from 'react';
import { BrowserRouter as Router, Route, Switch } from 'react-router-dom';
import { ThemeProvider, createTheme } from '@material-ui/core/styles';
import CssBaseline from '@material-ui/core/CssBaseline';
import Dashboard from './components/Dashboard';
import SBOMUploader from './components/SBOMUploader';
import ProvenanceViewer from './components/ProvenanceViewer';
import ComplianceReporter from './components/ComplianceReporter';

const theme = createTheme({
  palette: {
    primary: {
      main: '#00152b',
    },
    secondary: {
      main: '#38a3a5',
    },
  },
  typography: {
    fontFamily: 'Roboto, Arial, sans-serif',
  },
});

const App: React.FC = () => {
  return (
    <ThemeProvider theme={theme}>
      <CssBaseline />
      <Router>
        <Switch>
          <Route exact path="/" component={Dashboard} />
          <Route path="/upload-sbom" component={SBOMUploader} />
          <Route path="/provenance" component={ProvenanceViewer} />
          <Route path="/compliance" component={ComplianceReporter} />
        </Switch>
      </Router>
    </ThemeProvider>
  );
};

export default App;