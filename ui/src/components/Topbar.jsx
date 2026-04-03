import { useNavigate } from '../hooks/useNavigate';
import './Topbar.css';

export default function Topbar({ thinking }) {
    const navigate = useNavigate();

    const handleLogoClick = () => {
        navigate('/');
    };

    return (
        <header className="topbar">
            <div className="topbar-left">
                <button 
                    className="topbar-logo"
                    onClick={handleLogoClick}
                >
                    <svg width="22" height="22" viewBox="0 0 24 24" fill="none">
                        <path d="M12 2L2 7l10 5 10-5-10-5z" stroke="#4f8ef7" strokeWidth="1.8" strokeLinejoin="round" />
                        <path d="M2 17l10 5 10-5" stroke="#8b5cf6" strokeWidth="1.8" strokeLinejoin="round" />
                        <path d="M2 12l10 5 10-5" stroke="#22d3ee" strokeWidth="1.8" strokeLinejoin="round" />
                    </svg>
                    <span className="topbar-brand">Zerch</span>
                </button>
                <div className="topbar-sep" />
                <span className="topbar-subtitle">AI Log Debugger</span>
            </div>

            <div className="topbar-center">
                <div className="topbar-stats">
                    <Stat label="Indexed" value="15.2K" color="blue" />
                    <Stat label="Incidents" value="2" color="red" />
                    <Stat label="P99 Latency" value="982ms" color="warn" />
                    <Stat label="Services" value="5" color="green" />
                </div>
            </div>

            <div className="topbar-right">
                {thinking && (
                    <div className="thinking-pill">
                        <span className="thinking-spinner" />
                        <span>AI Thinking</span>
                        <ThinkingDots />
                    </div>
                )}
                <div className="topbar-time" id="topbar-time">
                    {new Date().toLocaleTimeString('en-US', { hour12: false })}
                </div>
                <div className="topbar-avatar">SG</div>
            </div>
        </header>
    );
}

function Stat({ label, value, color }) {
    return (
        <div className={`topbar-stat topbar-stat--${color}`}>
            <span className="topbar-stat-val">{value}</span>
            <span className="topbar-stat-lbl">{label}</span>
        </div>
    );
}

function ThinkingDots() {
    return (
        <span className="thinking-dots">
            <span /><span /><span />
        </span>
    );
}
