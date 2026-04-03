import { useState } from 'react';
import { useLogs } from '../hooks/useLogs';
import { API_ENDPOINTS } from '../config/api';
import Topbar from '../components/Topbar';
import LogPanel from '../components/LogPanel';
import RightPanel from '../components/RightPanel';
import SearchPanel from '../components/SearchPanel';
import SearchResultsPanel from '../components/SearchResultsPanel';
import IncidentDetails from '../components/IncidentDetails';
import '../App.css';

function DashboardPage() {
  const { logs, setLogs, paused, connected, togglePause, toggleConnect, simulateEvent } = useLogs();
  const [selectedIncident, setSelectedIncident] = useState(null);
  const [semanticView, setSemanticView] = useState(true);
  const [isThinking, setIsThinking] = useState(false);
  const [searchResults, setSearchResults] = useState(null);
  const [searchQuery, setSearchQuery] = useState('');
  const [isSearching, setIsSearching] = useState(false);

  // Handle log upload
  const handleUploadLogs = (newLogs) => {
    setIsThinking(true);
    setSearchResults(null); // Clear search when uploading new logs
    setAiSummary(''); // Clear summary

    setTimeout(() => {
      setLogs && setLogs(prev => [...prev, ...newLogs]);
      setIsThinking(false);
    }, 1500);
  };

  // Handle search
  const handleSearch = async (query) => {
    setIsSearching(true);
    setSearchQuery(query);
    setAiSummary(''); // Clear summary

    try {
      const response = await fetch(`${API_ENDPOINTS.search}?q=${encodeURIComponent(query)}&limit=5`);
      if (!response.ok) {
        throw new Error(`Search failed: ${response.statusText}`);
      }

      const data = await response.json();

      if (data.success) {
        setSearchResults(data.results);
        setIsThinking(false);
      } else {
        console.error('Search error:', data.message);
        setIsThinking(false);
      }
    } catch (error) {
      console.error('Search error:', error);
      setIsThinking(false);
    } finally {
      setIsSearching(false);
    }
  };

  // Clear search results
  const handleClearSearch = () => {
    setSearchResults(null);
    setSearchQuery('');
  };

  const handleSimulate = () => {
    setIsThinking(true);
    simulateEvent();
    setTimeout(() => setIsThinking(false), 3000);
  };

  const [aiSummary, setAiSummary] = useState('');
  const [isSummarizing, setIsSummarizing] = useState(false);

  const handleSummarize = async (text) => {
    setIsSummarizing(true);
    try {
      const response = await fetch(API_ENDPOINTS.summarize, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ text }),
      });
      if (!response.ok) {
        throw new Error('Summarization failed');
      }
      const data = await response.json();
      setAiSummary(data.summary || '');
    } catch (error) {
      console.error('Summarize error:', error);
    } finally {
      setIsSummarizing(false);
    }
  };

  return (
    <div className="app-container">
      <Topbar thinking={isThinking} />

      <main className="app-main">
        <div className="layout-grid">
          {/* Main Column (Left) - Split into Upper (Search) and Lower (Logs or Results) */}
          <div className="panel-main-column">
            <SearchPanel
              semanticView={semanticView}
              onToggleSemantic={() => setSemanticView(!semanticView)}
              onSearch={handleSearch}
              isSearching={isSearching}
            />

            {searchResults !== null ? (
              <>
                <SearchResultsPanel
                  searchResults={searchResults}
                  query={searchQuery}
                  onSummarize={handleSummarize}
                />
                <div className="search-clear-banner">
                  <span>Showing search results for "{searchQuery}"</span>
                  <button
                    className="btn btn-sm btn-ghost"
                    onClick={handleClearSearch}
                  >
                    ✕ Clear
                  </button>
                </div>
              </>
            ) : (
              <LogPanel
                logs={logs}
                paused={paused}
                connected={connected}
                onTogglePause={togglePause}
                onToggleConnect={toggleConnect}
                onSimulate={handleSimulate}
                semanticView={semanticView}
              />
            )}
          </div>

          {/* Sidebar Column (Right) - Split into Upper (Upload) and Lower (Incidents) */}
          <div className="panel-side-column">
            <RightPanel
              onIncidentSelect={setSelectedIncident}
              onUploadLogs={handleUploadLogs}
              aiSummary={aiSummary}
              isSummarizing={isSummarizing}
            />
          </div>
        </div>
      </main>

      {/* Modal/Overlay */}
      <IncidentDetails
        incident={selectedIncident}
        onClose={() => setSelectedIncident(null)}
      />

      <footer className="status-bar">
        <div className="status-bar__left">
          <div className="status-item">
            <span className="status-label">Engine:</span>
            <span className="status-value status-value--ready">Running (Rust)</span>
          </div>
          <div className="status-item">
            <span className="status-label">Vectors:</span>
            <span className="status-value">{(logs.length * 1.5).toFixed(0)} and counting</span>
          </div>
        </div>
        <div className="status-bar__right">
          <span className="status-item">API: {API_ENDPOINTS.search.split('/search')[0]}</span>
        </div>
      </footer>
    </div>
  );
}

export default DashboardPage;
