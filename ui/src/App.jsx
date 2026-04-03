import { useState, useEffect } from 'react';
import { AuthProvider, useAuth } from './context/AuthContext';
import LoginPage from './pages/LoginPage';
import LandingPage from './pages/LandingPage';
import DashboardPage from './pages/DashboardPage';
import LiveLogsPage from './pages/LiveLogsPage';
import './App.css';

function AppContent() {
  const { isAuthenticated } = useAuth();
  const [currentPath, setCurrentPath] = useState(window.location.pathname);

  useEffect(() => {
    const handlePopState = () => {
      setCurrentPath(window.location.pathname);
    };

    window.addEventListener('popstate', handlePopState);
    
    // Also listen for custom navigation events
    const handleNavigate = () => {
      setCurrentPath(window.location.pathname);
    };
    window.addEventListener('popstate', handleNavigate);
    
    return () => {
      window.removeEventListener('popstate', handlePopState);
      window.removeEventListener('popstate', handleNavigate);
    };
  }, []);

  // If not authenticated, show login
  if (!isAuthenticated) {
    return <LoginPage />;
  }

  // Authenticated routes
  if (currentPath === '/zerch') {
    return <DashboardPage />;
  }

  if (currentPath === '/live-logs') {
    return <LiveLogsPage />;
  }

  // Default to landing page
  return <LandingPage />;
}

function App() {
  return (
    <AuthProvider>
      <AppContent />
    </AuthProvider>
  );
}

export default App;
