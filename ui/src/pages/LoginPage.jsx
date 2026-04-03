import { useState } from 'react';
import { useAuth } from '../context/AuthContext';
import './LoginPage.css';

function LoginPage() {
  const [email, setEmail] = useState('');
  const [password, setPassword] = useState('');
  const [error, setError] = useState('');
  const [isLoading, setIsLoading] = useState(false);
  const { login } = useAuth();

  const handleSubmit = (e) => {
    e.preventDefault();
    setError('');
    setIsLoading(true);

    // Simulate API call delay
    setTimeout(() => {
      if (login(email, password)) {
        // Login successful - redirect will happen in App component
        setIsLoading(false);
      } else {
        setError('Invalid email or password');
        setIsLoading(false);
      }
    }, 500);
  };

  return (
    <div className="login-container">
      <div className="login-wrapper">
        {/* Left side - Branding */}
        <div className="login-brand">
          <div className="brand-content">
            <h1 className="brand-title">Zerch</h1>
            <p className="brand-subtitle">Vector Search Engine in Rust</p>
            <p className="brand-description">
              Fast, powerful semantic search for your logs and data
            </p>
          </div>
        </div>

        {/* Right side - Login Form */}
        <div className="login-form-wrapper">
          <div className="login-form-container">
            <h2>Login to Zerch</h2>
            <p className="login-subtitle">Enter your credentials to access the dashboard</p>

            <form onSubmit={handleSubmit} className="login-form">
              <div className="form-group">
                <label htmlFor="email">Email</label>
                <input
                  id="email"
                  type="email"
                  value={email}
                  onChange={(e) => setEmail(e.target.value)}
                  placeholder="admin@123.com"
                  required
                  disabled={isLoading}
                />
              </div>

              <div className="form-group">
                <label htmlFor="password">Password</label>
                <input
                  id="password"
                  type="password"
                  value={password}
                  onChange={(e) => setPassword(e.target.value)}
                  placeholder="Enter your password"
                  required
                  disabled={isLoading}
                />
              </div>

              {error && <div className="error-message">{error}</div>}

              <button type="submit" className="login-button" disabled={isLoading}>
                {isLoading ? 'Logging in...' : 'Login'}
              </button>
            </form>

            <div className="demo-credentials">
              <p className="demo-title">Demo Credentials:</p>
              <p><code>Email:</code> <span>admin@123.com</span></p>
              <p><code>Password:</code> <span>admin123</span></p>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}

export default LoginPage;
