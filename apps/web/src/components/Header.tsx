import React from 'react';
import { Award, Compass, BookOpen, Layers, Calculator } from 'lucide-react';

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
  isOnline = true,
}) => {
  const tabs = [
    { id: 'atlas', label: 'Explore', icon: Compass },
    { id: 'lesson', label: 'Lessons', icon: BookOpen },
    { id: 'practice', label: 'Practice', icon: Calculator },
    { id: 'abacus', label: 'Abacus', icon: Layers },
    { id: 'mastery', label: 'Progress', icon: Award },
  ];

  return (
    <header className="axiom-header" role="banner">
      <div className="axiom-header-top">
        <button className="axiom-brand" onClick={() => onTabChange('atlas')} aria-label="AXIOM Mathematics Academy Home">
          <img
            src="/logo.png"
            alt=""
            className="axiom-brand-logo-img"
          />
          <div>
            <span className="axiom-title">AXIOM</span>
            <span className="axiom-subtitle">Mathematics Academy</span>
          </div>
        </button>

        <div className="axiom-status-bar">
          <span className="axiom-local-status">Saved on this device <span className="axiom-network-status">{isOnline ? 'Connection available' : 'No connection'}</span></span>

          {/* XP & Ledger status */}
          <div className="axiom-badge axiom-badge-xp" title="Total experience points earned from locally checked practice; not certified mastery">
            <Award size={14} />
            <span><strong>{totalXp}</strong> XP</span>
            <span className="axiom-xp-cap">{todayPracticeXp}/100 today</span>
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
