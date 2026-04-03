import { useAuth } from '../context/AuthContext';
import { useNavigate } from '../hooks/useNavigate';
import './LiveLogsPage.css';

function LiveLogsPage() {
  const { logout } = useAuth();
  const navigate = useNavigate();
  const userEmail = localStorage.getItem('zerch_user') || 'Admin';

  const handleLogout = () => {
    logout();
    navigate('/');
  };

  // Mock live logs data
  const mockLogs = [
    '[2024-04-04 10:23:45] INFO Starting deployment to production',
    '[2024-04-04 10:23:46] INFO Pulling latest image from registry',
    '[2024-04-04 10:23:48] INFO Creating new container instance',
    '[2024-04-04 10:23:50] INFO Health check passed',
    '[2024-04-04 10:23:52] INFO Database migrations completed',
    '[2024-04-04 10:23:54] INFO Cache warmed up',
    '[2024-04-04 10:23:56] INFO Service endpoints registered',
    '[2024-04-04 10:23:58] INFO Load balancer updated',
    '[2024-04-04 10:24:00] WARN Request timeout detected',
    '[2024-04-04 10:24:02] INFO Retrying failed connection',
    '[2024-04-04 10:24:04] INFO Connection restored',
    '[2024-04-04 10:24:06] INFO Processing 1,234 queued events',
  ];

  const mockThought = [
    'Analyzing log patterns...',
    'Detected: Multiple timeout errors in database layer',
    'Checking: Connection pool exhaustion',
    'Found: 3 slow queries blocking operations',
    'Hypothesis: Need to optimize table indexes',
    'Recommendation: Scale database read replicas',
    'Action: Running optimization queries...',
    'Status: 45% complete',
  ];

  const mockActions = [
    { id: 1, action: 'Increasing connection pool', status: 'running', progress: 45 },
    { id: 2, action: 'Adding database indexes', status: 'running', progress: 65 },
    { id: 3, action: 'Scaling read replicas', status: 'queued', progress: 0 },
    { id: 4, action: 'Monitoring query performance', status: 'running', progress: 80 },
    { id: 5, action: 'Cleaning up stale connections', status: 'completed', progress: 100 },
  ];

  return (
    <div className="live-logs-container">
      {/* Navigation */}
      <nav className="live-logs-nav">
        <button 
          className="nav-brand" 
          onClick={() => navigate('/')}
          style={{ background: 'none', border: 'none', cursor: 'pointer' }}
        >
          <h1>Zerch</h1>
          <span className="nav-tagline">Vector Search Engine</span>
        </button>
        <div className="nav-links">
          <button 
            onClick={() => navigate('/zerch')}
            className="nav-link active"
          >
            Dashboard
          </button>
          <a href="#" className="nav-link">Live Logs</a>
          <a href="#" className="nav-link">Analytics</a>
          <div className="user-menu">
            <span className="user-email">{userEmail}</span>
            <button onClick={handleLogout} className="logout-btn">Logout</button>
          </div>
        </div>
      </nav>

      {/* Main Content */}
      <main className="live-logs-main">
        <div className="three-panel-layout">
          {/* Left Panel - Log Stream */}
          <div className="panel left-panel">
            <div className="panel-header">
              <h3>📡 Live Log Stream</h3>
              <span className="status-badge live">● Live</span>
            </div>
            <div className="panel-content log-stream">
              {mockLogs.map((log, idx) => (
                <div key={idx} className="log-line">
                  {log}
                </div>
              ))}
              <div className="log-line typing">▌</div>
            </div>
          </div>

          {/* Middle Panel - AI Agent Chain of Thought */}
          <div className="panel middle-panel">
            <div className="panel-header">
              <h3>🧠 AI Agent Chain of Thought</h3>
              <span className="thinking-badge">Analyzing...</span>
            </div>
            <div className="panel-content thought-stream">
              {mockThought.map((thought, idx) => (
                <div key={idx} className="thought-step">
                  <div className="thought-icon">→</div>
                  <div className="thought-text">{thought}</div>
                </div>
              ))}
              <div className="thought-step thinking">
                <div className="thought-spinner">⟳</div>
                <div className="thought-text">Processing recommendations...</div>
              </div>
            </div>
          </div>

          {/* Right Panel - Actions & Fixes */}
          <div className="panel right-panel">
            <div className="panel-header">
              <h3>⚙️ Actions & Fixes</h3>
              <span className="action-count">{mockActions.length}</span>
            </div>
            <div className="panel-content actions-stream">
              {mockActions.map((action) => (
                <div key={action.id} className={`action-item status-${action.status}`}>
                  <div className="action-info">
                    <div className="action-title">{action.action}</div>
                    <div className="action-status">{action.status}</div>
                  </div>
                  <div className="action-progress">
                    <div className="progress-bar">
                      <div 
                        className="progress-fill" 
                        style={{ width: `${action.progress}%` }}
                      />
                    </div>
                    <span className="progress-text">{action.progress}%</span>
                  </div>
                </div>
              ))}
            </div>
          </div>
        </div>
      </main>
    </div>
  );
}

export default LiveLogsPage;
