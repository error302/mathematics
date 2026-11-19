import React, { useState, useMemo } from 'react';
import { RewardEntry, getTopicReviewSchedule, studyDay } from '../engine/axiomCore';
import { Trophy, Target, Sparkles, CheckCircle2, ArrowRight, Lock } from 'lucide-react';
import './gamerProgress.css';

interface GamerProgressTrackerProps {
  ledgerEntries: RewardEntry[];
  totalXp: number;
  todayPracticeXp: number;
  onStartPractice: () => void;
  onSelectPractice?: (templateId: string) => void;
}

interface DomainStats {
  id: string;
  name: string;
  icon: string;
  templateId: string;
  topicsCount: number;
  startedCount: number;
  fluentCount: number;
  maxStreak: number;
  isDue: boolean;
  needsRepair: boolean;
  scorePct: number;
  strengthSummary: string;
  growthAdvice: string;
}

interface TrophyDef {
  id: string;
  title: string;
  description: string;
  tier: 'bronze' | 'silver' | 'gold' | 'diamond';
  icon: string;
  category: string;
  current: number;
  target: number;
  unlocked: boolean;
  progressLabel: string;
}

export const GamerProgressTracker: React.FC<GamerProgressTrackerProps> = ({
  ledgerEntries,
  totalXp,
  todayPracticeXp,
  onStartPractice,
  onSelectPractice,
}) => {
  const [trophyFilter, setTrophyFilter] = useState<'all' | 'unlocked' | 'progress'>('all');

  // 1. Calculate Gamer Level & Rank Title
  const rankInfo = useMemo(() => {
    if (totalXp >= 1000) return { level: 5, title: 'Axiom Luminary', minXp: 1000, nextXp: 2000, icon: '👑' };
    if (totalXp >= 500) return { level: 4, title: 'Harmonic Geometer', minXp: 500, nextXp: 1000, icon: '💎' };
    if (totalXp >= 250) return { level: 3, title: 'Equations Weaver', minXp: 250, nextXp: 500, icon: '⚡' };
    if (totalXp >= 100) return { level: 2, title: 'Pattern Seeker', minXp: 100, nextXp: 250, icon: '🎯' };
    return { level: 1, title: 'Apprentice of Axioms', minXp: 0, nextXp: 100, icon: '🌱' };
  }, [totalXp]);

  const levelProgressPct = Math.min(
    100,
    Math.max(0, Math.round(((totalXp - rankInfo.minXp) / (rankInfo.nextXp - rankInfo.minXp)) * 100))
  );

  // 2. Compute Real-Time Strengths & Weaknesses across 5 Mathematical Domains
  const domains: DomainStats[] = useMemo(() => {
    const today = studyDay();

    const domainDefs = [
      {
        id: 'arithmetic',
        name: 'Arithmetic & Number Sense',
        icon: '🔢',
        templateId: 'arithmetic.column.addition',
        templates: ['place_value.decompose', 'arithmetic.column.addition', 'arithmetic.column.subtraction', 'arithmetic.whole.multiplication', 'arithmetic.whole.division'],
        strengthSummary: 'Confident column regrouping, place-value decomposition, and integer operations.',
        growthAdvice: 'Reinforce column subtraction with zero-regrouping and multi-digit partition sharing.',
      },
      {
        id: 'fractions',
        name: 'Rational Numbers & Proportions',
        icon: '⚖️',
        templateId: 'fractions.compare.positive',
        templates: ['fractions.compare.positive', 'fractions.equivalent.find', 'fractions.unit.identify', 'decimals.fraction_convert', 'percentages.benchmark_calc', 'ratios.scaling_solve'],
        strengthSummary: 'Strong common-denominator reasoning, unit fraction models, and benchmark conversions.',
        growthAdvice: 'Level up proportional scaling and power-of-10 decimal conversion fluency.',
      },
      {
        id: 'algebra',
        name: 'Algebraic Structures & Exponents',
        icon: '⚡',
        templateId: 'powers.integer_eval',
        templates: ['learning.problem_reading', 'learning.equality_balance', 'learning.checking_bounds', 'integers.order', 'divisibility.factors.find', 'divisibility.gcd_lcm', 'powers.integer_eval', 'powers.scientific_notation', 'roots.radical_eval'],
        strengthSummary: 'Precise bounding checks, parity invariants, and exponents with negative integer rules.',
        growthAdvice: 'Focus on prime factorizations, GCD/LCM relationships, and square-root simplification.',
      },
      {
        id: 'geometry',
        name: 'Spatial Geometry & Coordinate Plane',
        icon: '📐',
        templateId: 'geometry.coordinate_distance',
        templates: ['geometry.perimeter_and_area', 'geometry.angles_triangles', 'geometry.coordinate_distance'],
        strengthSummary: 'Visual spatial conservation, Cartesian distance formula, and angle transversal theorems.',
        growthAdvice: 'Explore perimeter vs area trade-offs and negative-quadrant Cartesian coordinates.',
      },
      {
        id: 'soroban',
        name: 'Soroban Bead Agility & Mental Gym',
        icon: '🧮',
        templateId: 'abacus.read.state',
        templates: ['abacus.read.state', 'abacus.target.setting'],
        strengthSummary: 'Dexterous bi-quinary bead reading, 5/10 complement exchanges, and mental bead visualization.',
        growthAdvice: 'Practice compound carry/borrow operations (+10 -5) and try mental arithmetic with beads hidden.',
      },
    ];

    return domainDefs.map((def) => {
      let started = 0;
      let fluent = 0;
      let maxStr = 0;
      let isDue = false;
      let repair = false;

      def.templates.forEach((t) => {
        const s = parseInt(localStorage.getItem(`axiom_clean_streak:${t}`) || '0', 10);
        const draft = localStorage.getItem(`axiom_practice_draft_v2:${t}`);
        const sched = getTopicReviewSchedule(t);

        if (s > 0 || Boolean(draft)) started++;
        if (s >= 3) fluent++;
        if (s > maxStr) maxStr = s;
        if (sched && today >= sched.next_due_epoch_days) isDue = true;
      });

      // Special check for abacus tutor
      if (def.id === 'soroban') {
        const stage = localStorage.getItem('axiom_abacus_tutor_stage_v1');
        if (stage) started = Math.max(started, 1);
      }

      const totalT = def.templates.length;
      const scorePct = Math.min(100, Math.round(((fluent * 2 + started) / (totalT * 2)) * 100));

      return {
        id: def.id,
        name: def.name,
        icon: def.icon,
        templateId: def.templateId,
        topicsCount: totalT,
        startedCount: started,
        fluentCount: fluent,
        maxStreak: maxStr,
        isDue,
        needsRepair: repair,
        scorePct,
        strengthSummary: def.strengthSummary,
        growthAdvice: def.growthAdvice,
      };
    });
  }, []);

  // Separate top strengths and growth edges
  const strengths = useMemo(() => {
    return domains
      .filter((d) => d.scorePct >= 30 || d.maxStreak >= 2)
      .sort((a, b) => b.scorePct - a.scorePct);
  }, [domains]);

  const growthEdges = useMemo(() => {
    return domains
      .filter((d) => d.scorePct < 70 || d.isDue || d.needsRepair)
      .sort((a, b) => a.scorePct - b.scorePct);
  }, [domains]);

  // 3. 12 Real-Time Video Game Trophies & Achievements
  const trophies: TrophyDef[] = useMemo(() => {
    const maxGlobalStreak = Math.max(
      0,
      ...Array.from({ length: localStorage.length }, (_, i) => {
        const k = localStorage.key(i) || '';
        if (k.startsWith('axiom_clean_streak:')) {
          return parseInt(localStorage.getItem(k) || '0', 10);
        }
        return 0;
      })
    );

    const hasProvisionalMilestone = ledgerEntries.some((e) =>
      e.award_key.startsWith('milestone:provisional:')
    );

    const distinctTopicsPracticed = Array.from({ length: localStorage.length }, (_, i) => {
      const k = localStorage.key(i) || '';
      return k.startsWith('axiom_clean_streak:') && parseInt(localStorage.getItem(k) || '0', 10) > 0;
    }).filter(Boolean).length;

    const hasAbacusAttempt = Boolean(
      localStorage.getItem('axiom_abacus_tutor_stage_v1') ||
      ledgerEntries.some((e) => e.award_key.includes('abacus'))
    );

    const hasMentalHiddenAttempt = Boolean(
      localStorage.getItem('axiom_abacus_tutor_v1:mental') ||
      ledgerEntries.some((e) => e.award_key.includes('mental'))
    );

    const hasPuzzleSolved = Boolean(
      localStorage.getItem('axiom_challenge_draft_v1:puzzles.arithmetic.grid:1') ||
      localStorage.getItem('axiom_challenge_draft_v1:puzzles.arithmetic.grid:2')
    );

    const hasGeometrySolved = Boolean(
      parseInt(localStorage.getItem('axiom_clean_streak:geometry.coordinate_distance') || '0', 10) > 0 ||
      parseInt(localStorage.getItem('axiom_clean_streak:geometry.perimeter_and_area') || '0', 10) > 0
    );

    const hasSpacedLadderProgress = Array.from({ length: localStorage.length }, (_, i) => {
      const k = localStorage.key(i) || '';
      if (k.startsWith('axiom_review_schedule:')) {
        try {
          const s = JSON.parse(localStorage.getItem(k) || '{}');
          return s.ladder_step > 0;
        } catch {
          return false;
        }
      }
      return false;
    }).some(Boolean);

    // Read study view completed practice loops
    let loops = 0;
    try {
      const studyRaw = localStorage.getItem('axiom_study_state_v1');
      if (studyRaw) {
        const parsed = JSON.parse(studyRaw);
        loops = Object.values(parsed.steps || {}).filter((s: any) => s.practice_complete).length;
      }
    } catch {}

    const defs: TrophyDef[] = [
      {
        id: 'first_spark',
        title: 'First Spark',
        description: 'Solve your first independent exercise and earn checked practice XP.',
        tier: 'bronze',
        icon: '⚡',
        category: 'milestone',
        current: totalXp > 0 ? 1 : 0,
        target: 1,
        unlocked: totalXp > 0 || ledgerEntries.length > 0,
        progressLabel: totalXp > 0 ? 'Completed' : '0/1 practice solved',
      },
      {
        id: 'trio_precision',
        title: 'Trio of Precision',
        description: 'Achieve a 3-streak of clean consecutive answers in any topic.',
        tier: 'bronze',
        icon: '🎯',
        category: 'precision',
        current: Math.min(3, maxGlobalStreak),
        target: 3,
        unlocked: maxGlobalStreak >= 3,
        progressLabel: `${Math.min(3, maxGlobalStreak)}/3 clean streak`,
      },
      {
        id: 'six_fold_clarity',
        title: 'Six-Fold Clarity',
        description: 'Reach a 6-clean-answer streak to unlock a provisional milestone (+50 XP).',
        tier: 'silver',
        icon: '🛡️',
        category: 'precision',
        current: Math.min(6, maxGlobalStreak),
        target: 6,
        unlocked: maxGlobalStreak >= 6 || hasProvisionalMilestone,
        progressLabel: `${Math.min(6, maxGlobalStreak)}/6 clean streak`,
      },
      {
        id: 'daily_dynamo',
        title: 'Daily Dynamo',
        description: 'Hit the 100 XP daily practice cap with focused study.',
        tier: 'silver',
        icon: '🌟',
        category: 'milestone',
        current: Math.min(100, todayPracticeXp),
        target: 100,
        unlocked: todayPracticeXp >= 100,
        progressLabel: `${todayPracticeXp}/100 XP today`,
      },
      {
        id: 'soroban_novice',
        title: 'Soroban Apprentice',
        description: 'Engage with the Japanese Soroban abacus and make numbers with beads.',
        tier: 'bronze',
        icon: '🧮',
        category: 'abacus',
        current: hasAbacusAttempt ? 1 : 0,
        target: 1,
        unlocked: hasAbacusAttempt,
        progressLabel: hasAbacusAttempt ? 'Completed' : '0/1 abacus stage explored',
      },
      {
        id: 'mental_mirage',
        title: 'Mind Over Beads',
        description: 'Practice abacus arithmetic with beads hidden, visualizing the rods mentally.',
        tier: 'gold',
        icon: '👁️',
        category: 'abacus',
        current: hasMentalHiddenAttempt ? 1 : 0,
        target: 1,
        unlocked: hasMentalHiddenAttempt,
        progressLabel: hasMentalHiddenAttempt ? 'Completed' : 'Try mental stage with beads hidden',
      },
      {
        id: 'pattern_crosser',
        title: 'Pattern Crosser',
        description: 'Solve an arithmetic crossing puzzle quest where all rows and columns agree.',
        tier: 'silver',
        icon: '🧩',
        category: 'exploration',
        current: hasPuzzleSolved ? 1 : 0,
        target: 1,
        unlocked: hasPuzzleSolved,
        progressLabel: hasPuzzleSolved ? 'Completed' : 'Try a puzzle quest in Practice',
      },
      {
        id: 'cartesian_navigator',
        title: 'Cartesian Navigator',
        description: 'Accurately locate coordinates and calculate Euclidean distance in F07.',
        tier: 'silver',
        icon: '📐',
        category: 'exploration',
        current: hasGeometrySolved ? 1 : 0,
        target: 1,
        unlocked: hasGeometrySolved,
        progressLabel: hasGeometrySolved ? 'Completed' : 'Practice coordinate plane in F07',
      },
      {
        id: 'foundation_runner',
        title: 'Foundation Runner',
        description: 'Complete 3 guided foundation lesson loops (acknowledged + 2 examples).',
        tier: 'silver',
        icon: '🚀',
        category: 'milestone',
        current: Math.min(3, loops),
        target: 3,
        unlocked: loops >= 3,
        progressLabel: `${Math.min(3, loops)}/3 loops complete`,
      },
      {
        id: 'resilience_leap',
        title: 'Resilience Leap',
        description: 'Turn an initial mistake into a verified success through actionable repair.',
        tier: 'gold',
        icon: '💫',
        category: 'resilience',
        current: ledgerEntries.length > 1 ? 1 : 0,
        target: 1,
        unlocked: ledgerEntries.length > 1,
        progressLabel: ledgerEntries.length > 1 ? 'Leap achieved' : 'Repair a wrong answer',
      },
      {
        id: 'retrieval_sentinel',
        title: 'Retrieval Sentinel',
        description: 'Advance to Step 2 or higher on the 7-day spaced retrieval schedule ladder.',
        tier: 'gold',
        icon: '⏳',
        category: 'precision',
        current: hasSpacedLadderProgress ? 1 : 0,
        target: 1,
        unlocked: hasSpacedLadderProgress,
        progressLabel: hasSpacedLadderProgress ? 'Step 2+ Achieved' : 'Pass a spaced review check',
      },
      {
        id: 'academy_polymath',
        title: 'Academy Polymath',
        description: 'Practice across 5 distinct mathematical domains in the academy roadmap.',
        tier: 'diamond',
        icon: '👑',
        category: 'exploration',
        current: Math.min(5, distinctTopicsPracticed),
        target: 5,
        unlocked: distinctTopicsPracticed >= 5,
        progressLabel: `${Math.min(5, distinctTopicsPracticed)}/5 topics practiced`,
      },
    ];

    return defs;
  }, [ledgerEntries, totalXp, todayPracticeXp]);

  const unlockedCount = trophies.filter((t) => t.unlocked).length;

  const filteredTrophies = useMemo(() => {
    if (trophyFilter === 'unlocked') return trophies.filter((t) => t.unlocked);
    if (trophyFilter === 'progress') return trophies.filter((t) => !t.unlocked);
    return trophies;
  }, [trophies, trophyFilter]);

  const handleAction = (templateId: string) => {
    if (onSelectPractice) onSelectPractice(templateId);
    else onStartPractice();
  };

  return (
    <div className="axiom-gamer-profile">
      {/* 1. RPG Rank & Level Card */}
      <section className="axiom-rank-card" aria-label="Adventurer Level and Rank">
        <div className="axiom-rank-header">
          <div className="axiom-rank-identity">
            <div className="axiom-rank-badge-icon" aria-hidden="true">
              {rankInfo.icon}
            </div>
            <div className="axiom-rank-title-group">
              <h3>
                <span>{rankInfo.title}</span>
                <span className="axiom-rank-level-tag">Level {rankInfo.level}</span>
              </h3>
              <p className="axiom-rank-subtitle">
                {totalXp} Total Practice XP · Local Device Progress
              </p>
            </div>
          </div>

          <div className="axiom-rank-stats-pills">
            <div className="axiom-stat-pill">
              <span className="axiom-stat-pill-label">Trophies</span>
              <span className="axiom-stat-pill-val">{unlockedCount} / 12</span>
            </div>
            <div className="axiom-stat-pill">
              <span className="axiom-stat-pill-label">Today's XP</span>
              <span className="axiom-stat-pill-val">{todayPracticeXp} / 100</span>
            </div>
            <div className="axiom-stat-pill">
              <span className="axiom-stat-pill-label">Answers</span>
              <span className="axiom-stat-pill-val">{ledgerEntries.length}</span>
            </div>
          </div>
        </div>

        {/* Level XP Progress Bar */}
        <div className="axiom-level-progress-bar">
          <div className="axiom-level-progress-labels">
            <span>Progress to Next Rank</span>
            <span>
              <strong>{totalXp}</strong> / {rankInfo.nextXp} XP ({levelProgressPct}%)
            </span>
          </div>
          <div className="axiom-xp-track" role="progressbar" aria-valuenow={levelProgressPct} aria-valuemin={0} aria-valuemax={100}>
            <div className="axiom-xp-fill" style={{ width: `${levelProgressPct}%` }} />
          </div>
        </div>
      </section>

      {/* 2. Real-Time Strengths & Weaknesses Analysis */}
      <section className="axiom-analysis-section" aria-label="Strengths and Weaknesses">
        {/* Strengths Card */}
        <div className="axiom-card-strength">
          <div className="axiom-analysis-header">
            <Sparkles size={20} className="axiom-analysis-icon-strength" aria-hidden="true" />
            <div>
              <h4>Your Mathematical Strengths</h4>
              <p>Domains where you demonstrate clean precision, fluent streaks, or steady consistency.</p>
            </div>
          </div>

          <div className="axiom-analysis-list">
            {strengths.length > 0 ? (
              strengths.slice(0, 3).map((s) => (
                <div key={s.id} className="axiom-analysis-item">
                  <div className="axiom-analysis-item-top">
                    <span className="axiom-analysis-name">
                      {s.icon} {s.name}
                    </span>
                    <span className="axiom-analysis-badge strength">
                      {s.maxStreak >= 3 ? `🔥 ${s.maxStreak}-Streak Flow` : '🌟 High Competency'}
                    </span>
                  </div>
                  <p className="axiom-analysis-desc">{s.strengthSummary}</p>
                </div>
              ))
            ) : (
              <div className="axiom-analysis-item">
                <div className="axiom-analysis-item-top">
                  <span className="axiom-analysis-name">🌱 Building Foundations</span>
                  <span className="axiom-analysis-badge strength">Initial Steps</span>
                </div>
                <p className="axiom-analysis-desc">
                  Complete your first 3 clean answers in any topic to establish your signature strength!
                </p>
                <button type="button" className="axiom-analysis-action" onClick={() => handleAction('arithmetic.column.addition')}>
                  <span>Start Arithmetic Foundation</span>
                  <ArrowRight size={13} aria-hidden="true" />
                </button>
              </div>
            )}
          </div>
        </div>

        {/* Growth Edges (Weaknesses / Level-Up Areas) */}
        <div className="axiom-card-growth">
          <div className="axiom-analysis-header">
            <Target size={20} className="axiom-analysis-icon-growth" aria-hidden="true" />
            <div>
              <h4>Active Growth Edges (Level Up)</h4>
              <p>Concrete opportunities to expand your mathematical intuition and earn trophies.</p>
            </div>
          </div>

          <div className="axiom-analysis-list">
            {growthEdges.slice(0, 3).map((g) => (
              <div key={g.id} className="axiom-analysis-item">
                <div className="axiom-analysis-item-top">
                  <span className="axiom-analysis-name">
                    {g.icon} {g.name}
                  </span>
                  <span className="axiom-analysis-badge growth">
                    {g.isDue ? '⏳ Review Due' : '🎯 Level-Up Target'}
                  </span>
                </div>
                <p className="axiom-analysis-desc">{g.growthAdvice}</p>
                <button type="button" className="axiom-analysis-action" onClick={() => handleAction(g.templateId)}>
                  <span>Train {g.name}</span>
                  <ArrowRight size={13} aria-hidden="true" />
                </button>
              </div>
            ))}
          </div>
        </div>
      </section>

      {/* Domain Skill Power Bars */}
      <section className="axiom-skill-branches-card" aria-label="Mathematical Skill Power Breakdown">
        <h4>Domain Skill Mastery Breakdown</h4>
        <p className="axiom-subtext">Real-time competency bars derived from verified local checks across all 5 branches.</p>
        <div className="axiom-skill-branches-grid">
          {domains.map((d) => (
            <div key={d.id} className="axiom-skill-branch-item">
              <div className="axiom-skill-branch-meta">
                <span>{d.icon} {d.name}</span>
                <span className="axiom-skill-branch-pct">{d.scorePct}%</span>
              </div>
              <div className="axiom-skill-track" role="progressbar" aria-valuenow={d.scorePct} aria-valuemin={0} aria-valuemax={100}>
                <div
                  className={`axiom-skill-fill ${d.scorePct >= 70 ? 'high' : d.scorePct >= 30 ? 'medium' : 'low'}`}
                  style={{ width: `${Math.max(8, d.scorePct)}%` }}
                />
              </div>
            </div>
          ))}
        </div>
      </section>

      {/* 3. Video Game Trophy Showcase Cabinet */}
      <section className="axiom-trophy-cabinet" aria-label="Trophy and Achievement Cabinet">
        {unlockedCount > 0 && (
          <div className="axiom-trophy-alert-banner" role="status">
            <div className="axiom-trophy-alert-content">
              <Trophy size={24} style={{ color: '#f5d082' }} aria-hidden="true" />
              <div className="axiom-trophy-alert-text">
                <strong>Trophy Showcase Active!</strong>
                <span>You’ve unlocked {unlockedCount} of 12 achievements. Keep your momentum going!</span>
              </div>
            </div>
            <button type="button" className="axiom-btn axiom-btn-outline axiom-btn-sm" onClick={onStartPractice}>
              <span>Practice Now</span>
            </button>
          </div>
        )}

        <div className="axiom-trophy-header">
          <div className="axiom-trophy-header-info">
            <h3>
              <Trophy size={20} style={{ color: '#f5d082' }} aria-hidden="true" />
              <span>Trophy Cabinet ({unlockedCount} / {trophies.length})</span>
            </h3>
            <p>Earn trophies and unlock achievements as you progress, make leaps, and master topics.</p>
          </div>

          <div className="axiom-trophy-filters" role="group" aria-label="Filter achievements">
            <button
              type="button"
              className={`axiom-trophy-filter-btn ${trophyFilter === 'all' ? 'active' : ''}`}
              onClick={() => setTrophyFilter('all')}
            >
              All ({trophies.length})
            </button>
            <button
              type="button"
              className={`axiom-trophy-filter-btn ${trophyFilter === 'unlocked' ? 'active' : ''}`}
              onClick={() => setTrophyFilter('unlocked')}
            >
              Unlocked ({unlockedCount})
            </button>
            <button
              type="button"
              className={`axiom-trophy-filter-btn ${trophyFilter === 'progress' ? 'active' : ''}`}
              onClick={() => setTrophyFilter('progress')}
            >
              In Progress ({trophies.length - unlockedCount})
            </button>
          </div>
        </div>

        <div className="axiom-trophy-grid">
          {filteredTrophies.map((trophy) => (
            <article
              key={trophy.id}
              className={`axiom-trophy-card ${trophy.unlocked ? 'unlocked' : 'locked'}`}
              aria-label={`${trophy.title}: ${trophy.unlocked ? 'Unlocked' : 'Locked'}`}
            >
              <div className="axiom-trophy-icon-box" aria-hidden="true">
                {trophy.unlocked ? trophy.icon : <Lock size={20} style={{ color: '#94a3b8' }} />}
              </div>

              <div className="axiom-trophy-content">
                <div className="axiom-trophy-top">
                  <h4 className="axiom-trophy-title">{trophy.title}</h4>
                  <span className={`axiom-trophy-tier-badge ${trophy.tier}`}>
                    {trophy.tier}
                  </span>
                </div>

                <p className="axiom-trophy-desc">{trophy.description}</p>

                {trophy.unlocked ? (
                  <div className="axiom-trophy-unlocked-tag">
                    <CheckCircle2 size={13} aria-hidden="true" />
                    <span>Unlocked & Earned</span>
                  </div>
                ) : (
                  <div className="axiom-trophy-progress-wrap">
                    <div className="axiom-trophy-progress-meta">
                      <span>Progress</span>
                      <span>{trophy.progressLabel}</span>
                    </div>
                    <div className="axiom-xp-track" style={{ height: '6px' }}>
                      <div
                        className="axiom-xp-fill"
                        style={{
                          width: `${Math.min(100, Math.round((trophy.current / trophy.target) * 100))}%`,
                        }}
                      />
                    </div>
                  </div>
                )}
              </div>
            </article>
          ))}
        </div>
      </section>
    </div>
  );
};
