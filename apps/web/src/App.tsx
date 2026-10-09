import React, { useState, useEffect } from 'react';
import { Header } from './components/Header';
import { CurriculumAtlas } from './components/CurriculumAtlas';
import { FractionLesson } from './components/FractionLesson';
import { PracticeWorkspace } from './components/PracticeWorkspace';
import { AbacusStudio } from './components/AbacusStudio';
import { MasteryDashboard } from './components/MasteryDashboard';
import { ClientRewardLedger, RewardEntry } from './engine/axiomCore';
import './index.css';

export const App: React.FC = () => {
  const [currentTab, setCurrentTab] = useState<string>('atlas');
  const [ledgerEntries, setLedgerEntries] = useState<RewardEntry[]>(() => {
    const saved = localStorage.getItem('axiom_reward_ledger');
    if (saved) {
      try {
        return JSON.parse(saved);
      } catch {
        return [];
      }
    }
    return [];
  });

  const [masteryUnits, setMasteryUnits] = useState<number>(() => {
    return parseInt(localStorage.getItem('axiom_mastery_units') || '0', 10);
  });

  const [provisionalUnlocked, setProvisionalUnlocked] = useState<boolean>(() => {
    return localStorage.getItem('axiom_provisional_unlocked') === 'true';
  });

  // Keep localStorage synchronized
  useEffect(() => {
    localStorage.setItem('axiom_reward_ledger', JSON.stringify(ledgerEntries));
  }, [ledgerEntries]);

  useEffect(() => {
    localStorage.setItem('axiom_mastery_units', masteryUnits.toString());
  }, [masteryUnits]);

  useEffect(() => {
    localStorage.setItem('axiom_provisional_unlocked', provisionalUnlocked.toString());
  }, [provisionalUnlocked]);

  const ledger = new ClientRewardLedger(ledgerEntries);
  const totalXp = ledger.getTotalXp();
  const todayStr = new Date().toISOString().slice(0, 10);
  const todayPracticeXp = ledger.getTodayPracticeXp(todayStr);

  const handleAwardXp = (instanceId: string) => {
    const entry = ledger.awardInstanceSuccess(instanceId);
    if (entry) {
      setLedgerEntries([...ledger.entries]);
    }
    return entry;
  };

  const handleRecordMasteryEvidence = (_family: string, success: boolean) => {
    if (success) {
      setMasteryUnits((prev) => {
        const next = prev + 1;
        if (next >= 6 && !provisionalUnlocked) {
          // Trigger provisional milestone award
          const milestone = ledger.awardProvisionalMilestone('foundation.fractions.compare');
          if (milestone) {
            setLedgerEntries([...ledger.entries]);
          }
          setProvisionalUnlocked(true);
        }
        return next;
      });
    }
  };

  const handleImportLedger = (newEntries: RewardEntry[]) => {
    setLedgerEntries(newEntries);
  };

  const handleSelectCourse = (courseId: string) => {
    if (courseId === 'F04') {
      setCurrentTab('lesson');
    } else if (courseId === 'A01' || courseId === 'A02') {
      setCurrentTab('abacus');
    } else {
      setCurrentTab('practice');
    }
  };

  return (
    <div className="axiom-app">
      <Header
        currentTab={currentTab}
        onTabChange={setCurrentTab}
        totalXp={totalXp}
        todayPracticeXp={todayPracticeXp}
      />

      <main className="axiom-main-content">
        {currentTab === 'atlas' && (
          <CurriculumAtlas onSelectCourse={handleSelectCourse} />
        )}

        {currentTab === 'lesson' && (
          <FractionLesson onStartPractice={() => setCurrentTab('practice')} />
        )}

        {currentTab === 'practice' && (
          <PracticeWorkspace
            onAwardXp={handleAwardXp}
            onRecordMasteryEvidence={handleRecordMasteryEvidence}
          />
        )}

        {currentTab === 'abacus' && <AbacusStudio />}

        {currentTab === 'mastery' && (
          <MasteryDashboard
            ledgerEntries={ledgerEntries}
            totalXp={totalXp}
            todayPracticeXp={todayPracticeXp}
            provisionalUnlocked={provisionalUnlocked}
            masteryUnits={masteryUnits}
            onImportLedger={handleImportLedger}
          />
        )}
      </main>

      <footer className="axiom-footer">
        <p>
          <strong>AXIOM Mathematics Academy</strong> • Grounded in <a href="#sot" onClick={(e) => { e.preventDefault(); setCurrentTab('atlas'); }}>AXIOM-SOURCE-OF-TRUTH</a> • WCAG 2.2 AA Target • Exact Rational Arithmetic over $\mathbb&#123;Q&#125;$
        </p>
      </footer>
    </div>
  );
};
