import React, { Suspense, lazy } from 'react';
import { BrowserRouter as Router, Route, Switch } from 'react-router-dom';
import { Layout, Spin } from 'antd';
import { AuthProvider } from './context/AuthContext';
import AppHeader from './components/Layout/Header';
import Sidebar from './components/Layout/Sidebar';
import Footer from './components/Layout/Footer';
import ErrorBoundary from './components/Common/ErrorBoundary';
import Home from './pages/Home';
import Dashboard from './pages/Dashboard';
import SBOMs from './pages/SBOMs';
import Provenance from './pages/Provenance';
import Compliance from './pages/Compliance';
import OAuthCallback from './components/Auth/OAuthCallback';

const { Content } = Layout;

const Home = lazy(() => import('./pages/Home'));
const Dashboard = lazy(() => import('./pages/Dashboard'));
const SBOMs = lazy(() => import('./pages/SBOMs'));
const Provenance = lazy(() => import('./pages/Provenance'));
const Compliance = lazy(() => import('./pages/Compliance'));
const OAuthCallback = lazy(() => import('./components/Auth/OAuthCallback'));

const App: React.FC = () => {
  return (
    <AuthProvider>
      <Router>
        <Layout style={{ minHeight: '100vh' }}>
          <AppHeader />
          <Layout>
            <Sidebar />
            <Layout style={{ padding: '0 24px 24px' }}>
              <ErrorBoundary>
                <Content style={{ padding: 24, margin: 0, minHeight: 280 }}>
                  <Suspense fallback={<Spin size="large" />}>
                    <Switch>
                      <Route exact path="/" component={Home} />
                      <Route path="/dashboard" component={Dashboard} />
                      <Route path="/sboms" component={SBOMs} />
                      <Route path="/provenance" component={Provenance} />
                      <Route path="/compliance" component={Compliance} />
                      <Route path="/auth/callback" component={OAuthCallback} />
                    </Switch>
                  </Suspense>
                </Content>
              </ErrorBoundary>
            </Layout>
          </Layout>
          <Footer />
        </Layout>
      </Router>
    </AuthProvider>
  );
};

export default App;