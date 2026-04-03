import UploadSection from './UploadSection';
import './RightPanel.css';

export default function RightPanel({ onUploadLogs, aiSummary, isSummarizing }) {
    return (
        <aside className="right-panel">
            {/* Upload Section (Upper Right) */}
            <UploadSection onUpload={onUploadLogs} />

            {/* AI Summary Section (Lower Right) */}
            {(isSummarizing || aiSummary) && (
                <section className={`summary-panel ${isSummarizing ? 'is-loading' : ''}`} style={{ marginTop: '20px' }}>
                    <div className="panel__header">
                        <h3 style={{ margin: 0, fontSize: '14px', color: 'var(--accent-blue)', display: 'flex', alignItems: 'center', gap: '8px' }}>
                            <span style={{ fontSize: '18px' }}>✨</span> AI LOG INSIGHTS
                        </h3>
                    </div>
                    <div className="panel__content" style={{ padding: '15px 0' }}>
                        {isSummarizing ? (
                            <div className="summary-loading">
                                <div className="shimmer-line"></div>
                                <div className="shimmer-line shorter"></div>
                                <div className="shimmer-line"></div>
                            </div>
                        ) : (
                            <div className="summary-text" style={{ fontSize: '13px', lineHeight: '1.6', color: 'var(--text-primary)', whiteSpace: 'pre-wrap' }}>
                                {aiSummary}
                            </div>
                        )}
                    </div>
                </section>
            )}
        </aside>
    );
}
