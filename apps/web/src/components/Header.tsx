import React from 'react';
import { Award, Compass, BookOpen, Layers, Calculator, ShieldCheck } from 'lucide-react';

interface HeaderProps {
  currentTab: string;
  onTabChange: (tab: string) => void;
  totalXp: number;
  todayPracticeXp: number;
}

export const Header: React.FC<HeaderProps> = ({
  currentTab,
  onTabChange,
  totalXp,
  todayPracticeXp,
}) => {
  const tabs = [
    { id: 'atlas', label: 'Curriculum Atlas', icon: Compass },
    { id: 'lesson', label: 'Lesson: Fractions (F04)', icon: BookOpen },
    { id: 'practice', label: 'Practice Workspace', icon: Calculator },
    { id: 'abacus', label: 'Soroban Studio (A01)', icon: Layers },
    { id: 'mastery', label: 'Mastery & Ledger', icon: Award },
  ];

  return (
    <header className="axiom-header" role="banner">
      <div className="axiom-header-top">
        <div className="axiom-brand" onClick={() => onTabChange('atlas')} role="button" tabIndex={0}>
          <div className="axiom-brand-symbol">ΑΞ</div>
          <div>
            <h1 className="axiom-title">AXIOM</h1>
            <span className="axiom-subtitle">Mathematics Academy</span>
          </div>
        </div>

        <div className="axiom-status-bar">
          <div className="axiom-badge axiom-badge-guest" title="Local guest study mode with client-side persistence">
            <ShieldCheck size={14} />
            <span>Guest Mode (Local)</span>
          </div>
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
