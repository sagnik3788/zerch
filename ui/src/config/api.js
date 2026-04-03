const getApiUrl = () => {
  if (import.meta.env.VITE_API_URL) {
    return import.meta.env.VITE_API_URL;
  }
  
  const protocol = window.location.protocol;
  const hostname = window.location.hostname;
  const port = 8080;
  
  if (hostname === 'localhost' || hostname === '127.0.0.1') {
    return `${protocol}//${hostname}:${port}`;
  }
  
  return `${protocol}//${hostname}/api`;
};

export const API_BASE_URL = getApiUrl();

export const API_ENDPOINTS = {
  search: `${API_BASE_URL}/search`,
  upload: `${API_BASE_URL}/upload`,
  summarize: `${API_BASE_URL}/summarize`,
  health: `${API_BASE_URL}/health`,
};
