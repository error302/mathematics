import React from 'react';
import { Award, Compass, BookOpen, Layers, Calculator, ShieldCheck, Cloud, CloudOff } from 'lucide-react';

interface HeaderProps {
  currentTab: string;
  onTabChange: (tab: string) => void;
  totalXp: number;
  todayPracticeXp: number;
  syncMode?: 'local' | 'cloud';
  onToggleSyncMode?: () => void;
  isOnline?: boolean;
}

export const Header: React.FC<HeaderProps> = ({
  currentTab,
  onTabChange,
  totalXp,
  todayPracticeXp,
  syncMode = 'local',
  onToggleSyncMode,
  isOnline = true,
}) => {
  const tabs = [
    { id: 'atlas', label: 'Curriculum Atlas', icon: Compass },
    { id: 'lesson', label: 'Lessons (12 Pilot Units)', icon: BookOpen },
    { id: 'practice', label: 'Practice Workspace (9 Families)', icon: Calculator },
    { id: 'abacus', label: 'Soroban Studio (A01/A02)', icon: Layers },
    { id: 'mastery', label: 'Mastery & Ledger', icon: Award },
  ];

  return (
    <header className="axiom-header" role="banner">
      <div className="axiom-header-top">
        <div className="axiom-brand" onClick={() => onTabChange('atlas')} role="button" tabIndex={0} aria-label="AXIOM Mathematics Academy Home">
          <img
            src="/logo.png"
            alt="AXIOM Mathematics Academy"
            className="axiom-brand-logo-img"
          />
          <div>
            <h1 className="axiom-title">AXIOM</h1>
            <span className="axiom-subtitle">Mathematics Academy</span>
          </div>
        </div>

        <div className="axiom-status-bar">
          {/* Dual Mode Switcher */}
          <button
            className={`axiom-badge axiom-badge-guest ${syncMode === 'cloud' ? 'axiom-badge-cloud' : ''}`}
            onClick={onToggleSyncMode}
            title={syncMode === 'cloud' ? "Cloud API Mode: Attempts and ledger synchronize with backend" : "Local Mode: Pure offline Rust WASM evaluation with client persistence"}
            style={{ cursor: 'pointer', border: '1px solid rgba(255,255,255,0.1)' }}
          >
            {syncMode === 'cloud' ? (
              <>
                <Cloud size={14} className="axiom-accent" />
                <span>Cloud API Sync</span>
              </>
            ) : (
              <>
                <ShieldCheck size={14} />
                <span>Local Engine (Guest)</span>
              </>
            )}
          </button>

          {/* Network Indicator */}
          <div
            className="axiom-badge"
            title={isOnline ? "Network Connected" : "Operating in Offline Mode"}
            style={{ opacity: 0.85 }}
          >
            {isOnline ? <span style={{ width: 8, height: 8, borderRadius: '50%', background: '#10b981' }} /> : <CloudOff size={14} className="axiom-warning" />}
            <span>{isOnline ? "PWA Ready" : "Offline Cache"}</span>
          </div>

          {/* XP & Ledger status */}
          <div className="axiom-badge axiom-badge-xp" title="Total experience points earned from verified attempts">
            <Award size={14} />
            <span><strong>{totalXp}</strong> XP</span>
            <span className="axiom-xp-cap">({todayPracticeXp}/100 today)</span>
          </div>
        </div>
      </div>

      <nav className="axiom-nav" role="navigation" aria-label="Main Navigation">
        {tabs.map((tab) => {
          const Icon = tab.icon;
          const active = currentTab === tab.id;
          return (
            <button
              key={tab.id}
              className={`axiom-nav-btn ${active ? 'active' : ''}`}
              onClick={() => onTabChange(tab.id)}
              aria-current={active ? 'page' : undefined}
            >
              <Icon size={16} />
              <span>{tab.label}</span>
            </button>
          );
        })}
      </nav>
    </header>
  );
};
