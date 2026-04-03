import { useState } from 'react';
import './SearchPanel.css';

export default function SearchPanel({ semanticView, onToggleSemantic, onSearch, isSearching }) {
    const [query, setQuery] = useState('');

    const handleSearch = async (e) => {
        e.preventDefault();
        if (query.trim()) {
            onSearch(query.trim());
            setQuery('');
        }
    };

    return (
        <div className="search-panel card">
            <div className="card-header">
                <h2 className="card-title">Search & Filters</h2>
            </div>
            <div className="search-container">
                <form onSubmit={handleSearch}>
                    <div className="search-input-wrapper">
                        <svg className="search-icon" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2.5">
                            <circle cx="11" cy="11" r="8" /><line x1="21" y1="21" x2="16.65" y2="16.65" />
                        </svg>
                        <input 
                            type="text" 
                            placeholder="Search logs or describe issue..." 
                            className="search-input" 
                            value={query}
                            onChange={(e) => setQuery(e.target.value)}
                            disabled={isSearching}
                        />
                        <button 
                            type="submit" 
                            className="search-btn"
                            disabled={isSearching || !query.trim()}
                            title="Search"
                        >
                            {isSearching ? (
                                <svg className="spinner" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2">
                                    <circle cx="12" cy="12" r="10" />
                                </svg>
                            ) : (
                                <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2.5">
                                    <line x1="5" y1="12" x2="19" y2="12" /><polyline points="12 5 19 12 12 19" />
                                </svg>
                            )}
                        </button>
                    </div>
                </form>
                <div className="filters-grid">
                    <select className="filter-select" defaultValue="">
                        <option value="" disabled>Severity</option>
                        <option value="INFO">INFO</option>
                        <option value="WARN">WARN</option>
                        <option value="ERROR">ERROR</option>
                    </select>
                    <input type="text" placeholder="Service (e.g. AuthService)" className="filter-input" />
                    <select className="filter-select" defaultValue="">
                        <option value="" disabled>Time range</option>
                        <option value="5m">Last 5 mins</option>
                        <option value="1h">Last 1 hour</option>
                        <option value="24h">Last 24 hours</option>
                    </select>
                    <button
                        className={`btn btn-sm ${semanticView ? 'btn-primary' : 'btn-ghost'}`}
                        onClick={onToggleSemantic}
                        title="Toggle AI-powered semantic search"
                    >
                        <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2.5">
                            <path d="M12 2a10 10 0 1 0 10 10H12V2z" /><path d="M12 2a10 10 0 0 1 10 10h-10V2z" strokeOpacity="0.3" />
                        </svg>
                        Semantic
                    </button>
                </div>
            </div>
        </div>
    );
}
