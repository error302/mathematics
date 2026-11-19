import React, { useState, useEffect } from 'react';
import { Header } from './components/Header';
import { CurriculumAtlas } from './components/CurriculumAtlas';
import { LessonViewer } from './components/LessonViewer';
import { PracticeHub } from './components/PracticeHub';
import { AbacusStudio } from './components/AbacusStudio';
import { MasteryDashboard } from './components/MasteryDashboard';
import { ClientRewardLedger, RewardEntry,applyStudyAction,getStudyView } from './engine/axiomCore';
import {StudyJourney,StudyPracticeBanner} from './components/StudyJourney';
import { FocusTools } from './components/FocusTools';
import './index.css';
import './study.css';

export const App: React.FC = () => {
  const [currentTab, setCurrentTab] = useState<string>('atlas');
  const navigate = (tab: string) => {
    setCurrentTab(tab);
    requestAnimationFrame(() => {
      window.scrollTo({top: 0, behavior: 'instant'});
      document.getElementById('study-content')?.focus({preventScroll: true});
    });
  };
  const [isOnline, setIsOnline] = useState(navigator.onLine);
  useEffect(() => {
    const update = () => setIsOnline(navigator.onLine);
    window.addEventListener('online', update); window.addEventListener('offline', update);
    return () => { window.removeEventListener('online', update); window.removeEventListener('offline', update); };
  }, []);
  const [activeLessonId, setActiveLessonId] = useState<string>('foundation.fractions.compare');
  const [activeTemplateId, setActiveTemplateId] = useState<string>('fractions.compare.positive');
  const [practiceRouteRevision,setPracticeRouteRevision]=useState(0);
  const [syncMode, setSyncMode] = useState<'local' | 'cloud'>(() => {
    return 'local';
  });

  const [focusView, setFocusView] = useState<boolean>(() => {
    try { return localStorage.getItem('axiom_focus_view') === 'true'; } catch { return false; }
  });
  useEffect(() => {
    try { localStorage.setItem('axiom_focus_view', String(focusView)); } catch { /* Preference is best-effort. */ }
  }, [focusView]);

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
  useEffect(()=>{const restored=()=>{try{const value=JSON.parse(localStorage.getItem('axiom_reward_ledger')||'[]');if(Array.isArray(value))setLedgerEntries(value);setPracticeRouteRevision(n=>n+1)}catch{}};window.addEventListener('axiom-backup-restored',restored);return()=>window.removeEventListener('axiom-backup-restored',restored)},[]);

  const [masteryUnits] = useState<number>(() => {
    return parseInt(localStorage.getItem('axiom_mastery_units') || '0', 10);
  });

  const [provisionalUnlocked] = useState<boolean>(() => {
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

  useEffect(() => {
    localStorage.setItem('axiom_sync_mode', syncMode);
  }, [syncMode]);

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

  const handleAwardProvisionalMilestone = (conceptId: string) => {
    const entry = ledger.awardProvisionalMilestone(conceptId);
    if (entry) {
      setLedgerEntries([...ledger.entries]);
    }
    return entry;
  };

  const handleRecordMasteryEvidence = (_family: string, _success: boolean) => {
    // Local numeric practice is not multidimensional or delayed mastery evidence.
    // Keep legacy evidence stored, but do not issue unsupported mastery claims.
  };

  const handleImportLedger = (newEntries: RewardEntry[]) => {
    setLedgerEntries(existing => {
      const byKey = new Map(existing.map(entry => [entry.award_key, entry]));
      for (const entry of newEntries) if (!byKey.has(entry.award_key)) byKey.set(entry.award_key, entry);
      return [...byKey.values()];
    });
  };

  const handleSelectCourse = (courseId: string) => {
    const courseLessonMap: Record<string, string> = {
      'F00': 'foundation.learning.problem_reading',
      'F01': 'foundation.place_value.decimal_expansion',
      'F02': 'foundation.arithmetic.addition_disjoint',
      'F03': 'foundation.integers.number_line',
      'F04': 'foundation.fractions.compare',
      'F05': 'foundation.decimals.place_value_fractions',
      'F06': 'foundation.powers.integer_exponents',
      'F07': 'foundation.geometry.perimeter_and_area',
      'F08': 'foundation.data.center_and_spread',
      'S01': 'secondary.algebra.expressions_and_substitution',
      'S02': 'secondary.functions.mappings_and_evaluation',
      'S03': 'secondary.algebra2.polynomials_and_factoring',
      'S04': 'secondary.geometry.lines_slopes_and_perpendicularity',
      'S05': 'secondary.trig.radians_and_unit_circle',
      'S06': 'secondary.sequences.arithmetic_and_series',
      'S07': 'secondary.calculus.limits_and_continuity',
      'A01': 'abacus.orientation.place_value',
      'A02': 'abacus.direct.addition',
    };

    if (courseLessonMap[courseId]) {
      setActiveLessonId(courseLessonMap[courseId]);
      navigate('lesson');
    } else {
      navigate('practice');
    }
  };

  const handleStartPracticeFromLesson = (tmpl?: string) => {
    setPracticeRouteRevision(n=>n+1);
    if (tmpl) {
      setActiveTemplateId(tmpl);
    }
    navigate('practice');
  };
  const handleGuidedLesson=(id:string)=>{try{applyStudyAction({kind:'open',step_id:id})}catch{/* The journey surface displays unreadable-history guidance; lessons stay usable. */}setActiveLessonId(id);navigate('lesson')};
  const handleContinueStudy=()=>{const {view}=getStudyView();if(!view){navigate('mastery');return}const next=view.recommendation;const step=view.steps.find(s=>s.id===next.step_id)!;if(['practice','review'].includes(next.action)){try{applyStudyAction({kind:'open',step_id:step.id})}catch{}setActiveTemplateId(step.template_id);setPracticeRouteRevision(n=>n+1);navigate('practice')}else if(next.action==='enrichment'){handleStartPracticeFromLesson('quest:puzzles.arithmetic.grid')}else{handleGuidedLesson(step.id)}};

  const handleToggleSyncMode = () => {
    setSyncMode('local');
  };

  return (
    <div className={`axiom-app${focusView ? ' axiom-focus-view' : ''}`}>
      <a className="axiom-skip-link" href="#study-content">Skip to study content</a>
      <Header
        currentTab={currentTab}
        onTabChange={navigate}
        totalXp={totalXp}
        todayPracticeXp={todayPracticeXp}
        syncMode={syncMode}
        onToggleSyncMode={handleToggleSyncMode}
        isOnline={isOnline}
      />

      <FocusTools focusView={focusView} onToggleFocusView={() => setFocusView((value) => !value)} />

      <main className="axiom-main-content" id="study-content" tabIndex={-1}>
        {currentTab === 'atlas' && (
          <><StudyJourney onContinue={handleContinueStudy} onLesson={handleGuidedLesson}/><CurriculumAtlas onSelectCourse={handleSelectCourse} onStartPractice={handleStartPracticeFromLesson} onOpenAbacus={()=>navigate('abacus')} /></>
        )}

        {currentTab === 'lesson' && (
          <LessonViewer
            key={activeLessonId}
            initialLessonId={activeLessonId}
            onStartPractice={handleStartPracticeFromLesson}
          />
        )}

        {currentTab === 'practice' && (
          <><StudyPracticeBanner onContinue={handleContinueStudy}/><PracticeHub key={`${activeTemplateId}:${practiceRouteRevision}`}
            onAwardXp={handleAwardXp}
            onRecordMasteryEvidence={handleRecordMasteryEvidence}
            onAwardProvisionalMilestone={handleAwardProvisionalMilestone}
            initialTemplateId={activeTemplateId}
          /></>
        )}

        {currentTab === 'abacus' && <AbacusStudio onAwardXp={handleAwardXp} />}

        {currentTab === 'mastery' && (
          <><StudyJourney full onContinue={handleContinueStudy} onLesson={handleGuidedLesson}/><MasteryDashboard
            ledgerEntries={ledgerEntries}
            totalXp={totalXp}
            todayPracticeXp={todayPracticeXp}
            provisionalUnlocked={false}
            masteryUnits={masteryUnits}
            onImportLedger={handleImportLedger}
            onStartPractice={() => navigate('practice')}
            onSelectPractice={(tmpl) => handleStartPracticeFromLesson(tmpl)}
          /></>
        )}
      </main>

      <footer className="axiom-footer">
        <p>
          <strong>AXIOM Mathematics Academy</strong><span>Draft academy · Your work stays on this device. Account backup and independent course review are still in development.</span>
        </p>
      </footer>
    </div>
  );
};
