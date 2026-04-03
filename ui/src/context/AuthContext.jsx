import { createContext, useState, useContext } from 'react';

const AuthContext = createContext();

export function AuthProvider({ children }) {
  const [isAuthenticated, setIsAuthenticated] = useState(() => {
    // Check if user is already logged in
    return localStorage.getItem('zerch_auth') === 'true';
  });

  const login = (email, password) => {
    // Simple authentication check
    if (email === 'admin@123.com' && password === 'admin123') {
      setIsAuthenticated(true);
      localStorage.setItem('zerch_auth', 'true');
      localStorage.setItem('zerch_user', email);
      return true;
    }
    return false;
  };

  const logout = () => {
    setIsAuthenticated(false);
    localStorage.removeItem('zerch_auth');
    localStorage.removeItem('zerch_user');
  };

  return (
    <AuthContext.Provider value={{ isAuthenticated, login, logout }}>
      {children}
    </AuthContext.Provider>
  );
}

export function useAuth() {
  const context = useContext(AuthContext);
  if (!context) {
    throw new Error('useAuth must be used within AuthProvider');
  }
  return context;
}
