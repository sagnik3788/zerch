import { useAuth } from '../context/AuthContext';
import { useNavigate } from '../hooks/useNavigate';
import './LandingPage.css';

function LandingPage() {
  const { logout } = useAuth();
  const navigate = useNavigate();
  const userEmail = localStorage.getItem('zerch_user') || 'Admin';

  const handleLogout = () => {
    logout();
    navigate('/');
  };

  return (
    <div className="landing-container">
      {/* Navigation */}
      <nav className="landing-nav">
        <button 
          className="nav-brand" 
          onClick={() => navigate('/')}
          style={{ background: 'none', border: 'none', cursor: 'pointer' }}
        >
          <h1>Zerch</h1>
          <span className="nav-tagline">Vector Search Engine</span>
        </button>
        <div className="nav-links">
          <a href="#features" className="nav-link">Features</a>
          <a href="#about" className="nav-link">About</a>
          <a href="#tech" className="nav-link">Technology</a>
          <button 
            onClick={() => navigate('/live-logs')}
            className="nav-link nav-button"
          >
            Live Logs
          </button>
          <div className="user-menu">
            <span className="user-email">{userEmail}</span>
            <button onClick={handleLogout} className="logout-btn">Logout</button>
          </div>
        </div>
      </nav>

      {/* Hero Section */}
      <section className="hero-section">
        <div className="hero-content">
          <h2 className="hero-title">Vector Search Engine Built in Rust</h2>
          <p className="hero-subtitle">
            Lightning-fast semantic search for your logs
          </p>
          <div className="hero-cta">
            <button onClick={() => navigate('/zerch')} className="cta-primary">Go to Dashboard</button>
            <a href="#features" className="cta-secondary">Learn More</a>
          </div>
          <div className="hero-stats">
            <div className="stat">
              <span className="stat-value">10M+</span>
              <span className="stat-label">Vectors Indexed</span>
            </div>
            <div className="stat">
              <span className="stat-value">&lt;1ms</span>
              <span className="stat-label">Query Time</span>
            </div>
            <div className="stat">
              <span className="stat-value">100%</span>
              <span className="stat-label">Rust Native</span>
            </div>
          </div>
        </div>
      </section>

      {/* Features Section */}
      <section className="features-section" id="features">
        <h3 className="section-title">Powerful Features</h3>
        <div className="features-grid">
          <div className="feature-card">
            <div className="feature-icon">⚡</div>
            <h4>Ultra-Fast Search</h4>
            <p>Sub-millisecond query response times with advanced indexing algorithms</p>
          </div>
          <div className="feature-card">
            <div className="feature-icon">🔍</div>
            <h4>Semantic Understanding</h4>
            <p>Find what you mean, not just what you type with AI-powered embeddings</p>
          </div>
          <div className="feature-card">
            <div className="feature-icon">📊</div>
            <h4>Log Analysis</h4>
            <p>Search and analyze millions of log entries with natural language queries</p>
          </div>
          <div className="feature-card">
            <div className="feature-icon">🛡️</div>
            <h4>Enterprise Grade</h4>
            <p>Built for scalability, reliability, and production-ready performance</p>
          </div>
        </div>
      </section>

      {/* Technology Section */}
      <section className="tech-section" id="tech">
        <h3 className="section-title">Built with Modern Technology</h3>
        <div className="tech-stack">
          <div className="tech-item">
            <h4>Rust Backend</h4>
            <p>Memory-safe, blazing-fast performance with zero-cost abstractions</p>
          </div>
          <div className="tech-item">
            <h4>Vector Embeddings</h4>
            <p>Leverage sentence-transformers for semantic understanding of text</p>
          </div>
          <div className="tech-item">
            <h4>Efficient Storage</h4>
            <p>Optimized binary formats for compact storage and rapid access</p>
          </div>
          <div className="tech-item">
            <h4>React Frontend</h4>
            <p>Modern, responsive UI built with React and neobrutalism design</p>
          </div>
        </div>
      </section>

      {/* CTA Section */}
      <section className="cta-section">
        <h3>Ready to Search Your Logs Smarter?</h3>
        <p>Jump into the dashboard and start searching your log data with AI-powered semantics</p>
        <button onClick={() => navigate('/zerch')} className="cta-button">Access Dashboard Now</button>
      </section>

      {/* Footer */}
      <footer className="landing-footer">
        <p>&copy; 2024 Zerch - Vector Search Engine in Rust</p>
        <div className="footer-links">
          <a href="#privacy">Privacy</a>
          <a href="#terms">Terms</a>
          <a href="#contact">Contact</a>
        </div>
      </footer>
    </div>
  );
}

export default LandingPage;
