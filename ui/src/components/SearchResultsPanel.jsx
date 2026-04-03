import { useRef, useEffect, useState } from 'react';
import './SearchResultsPanel.css';

export default function SearchResultsPanel({ searchResults, query, onSummarize }) {
    const bottomRef = useRef(null);

    // Scroll to bottom when new results arrive
    useEffect(() => {
        if (searchResults && searchResults.length > 0) {
            bottomRef.current?.scrollIntoView({ behavior: 'smooth' });
        }
    }, [searchResults]);

    if (!searchResults) {
        return null;
    }

    return (
        <section className="search-results-panel log-panel--main">
            <div className="log-panel__header">
                <div className="log-panel__title-row">
                    <div className="log-panel__title">
                        <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2">
                            <circle cx="11" cy="11" r="8" /><line x1="21" y1="21" x2="16.65" y2="16.65" />
                        </svg>
                        Semantic Search Results
                    </div>
                    <div className="search-query-info">
                        Query: <span className="query-text">"{query}"</span>
                    </div>
                </div>
            </div>

            <div className="search-results-stream">
                {searchResults.length === 0 ? (
                    <div className="results-empty">
                        <span className="no-results-icon">🔍</span>
                        <span className="results-empty__text">No matching logs found</span>
                        <p className="results-empty__subtitle">Try a different search query</p>
                    </div>
                ) : (
                    <>
                        <div className="results-summary">
                            <div className="summary-badge">
                                <span className="badge-label">Results</span>
                                <span className="badge-value">{searchResults.length}</span>
                            </div>
                        </div>
                        {searchResults.map((result, idx) => (
                            <SearchResultEntry
                                key={result.id}
                                result={result}
                                rank={idx + 1}
                                onSummarize={onSummarize}
                            />
                        ))}
                        <div ref={bottomRef} />
                    </>
                )}
            </div>
        </section>
    );
}

function SearchResultEntry({ result, rank, onSummarize }) {
    const handleClick = async () => {
        if (onSummarize) {
            await onSummarize(result.text);
        }
    };
    return (
        <div className="search-result-entry" onClick={handleClick} style={{ cursor: 'pointer' }}>
            <div className="result-rank">#{rank}</div>
            <div className="result-content">
                <div className="result-header">
                    <div className="result-score">
                        <div className="score-label">Score</div>
                        <div className="score-value">{(result.score * 100).toFixed(1)}%</div>
                        <ScoreBar value={result.score} />
                    </div>
                </div>
                <div className="result-text">
                    {result.text}
                </div>
            </div>
        </div>
    );
}

function ScoreBar({ value }) {
    const color = value > 0.85 ? '#10b981' : value > 0.7 ? '#f59e0b' : '#ef4444';
    const height = Math.min(value * 3, 3) + 'px';
    return (
        <div className="score-bar">
            <div className="score-bar-fill" style={{ width: `${value * 100}%`, background: color, height }} />
        </div>
    );
}
