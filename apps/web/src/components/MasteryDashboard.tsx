import React, { useRef, useState } from 'react';
import { RewardEntry, getTopicReviewSchedule, INTERVAL_LADDER_DAYS, studyDay } from '../engine/axiomCore';
import { Clock, Download, Upload, ArrowRight } from 'lucide-react';
import { CertificatePreview } from './CertificatePreview';
import { FullStudyBackup } from './FullStudyBackup';
import { GamerProgressTracker } from './GamerProgressTracker';

interface MasteryDashboardProps {
  ledgerEntries: RewardEntry[];
  totalXp: number;
  todayPracticeXp: number;
  provisionalUnlocked: boolean;
  masteryUnits: number;
  onImportLedger: (entries: RewardEntry[]) => void;
  onStartPractice: () => void;
  onSelectPractice?: (templateId: string) => void;
}

const TOPICS = [
  { id: 'f00_reading', name: 'Problem Reading & Formulating Unknowns', course: 'F00', templateId: 'learning.problem_reading' },
  { id: 'f00_balance', name: 'Meaning of Equality & Balance', course: 'F00', templateId: 'learning.equality_balance' },
  { id: 'f00_bounds', name: 'Checking Bounds & Counterexamples', course: 'F00', templateId: 'learning.checking_bounds' },
  { id: 'f01_decompose', name: 'Base-10 Place Value', course: 'F01', templateId: 'place_value.decompose' },
  { id: 'f02_carry', name: 'Column Addition (Regrouping)', course: 'F02', templateId: 'arithmetic.column.addition' },
  { id: 'f02_borrow', name: 'Column Subtraction (Borrowing)', course: 'F02', templateId: 'arithmetic.column.subtraction' },
  { id: 'f02_mult', name: 'Multiplication & Arrays', course: 'F02', templateId: 'arithmetic.whole.multiplication' },
  { id: 'f02_div', name: 'Division & Equal Sharing', course: 'F02', templateId: 'arithmetic.whole.division' },
  { id: 'f03_order', name: 'Signed Integers & Order', course: 'F03', templateId: 'integers.order' },
  { id: 'f03_factors', name: 'Factors & Divisibility', course: 'F03', templateId: 'divisibility.factors.find' },
  { id: 'f03_gcd', name: 'Greatest Common Divisor & LCM', course: 'F03', templateId: 'divisibility.gcd_lcm' },
  { id: 'f04_compare', name: 'Fraction Comparison', course: 'F04', templateId: 'fractions.compare.positive' },
  { id: 'f04_equiv', name: 'Equivalent Fractions', course: 'F04', templateId: 'fractions.equivalent.find' },
  { id: 'f04_unit', name: 'Unit Fraction Partitions', course: 'F04', templateId: 'fractions.unit.identify' },
  { id: 'f05_decimal', name: 'Decimals & Power-of-10 Fractions', course: 'F05', templateId: 'decimals.fraction_convert' },
  { id: 'f05_percent', name: 'Percentages & Benchmark Reasoning', course: 'F05', templateId: 'percentages.benchmark_calc' },
  { id: 'f05_ratio', name: 'Ratios & Proportional Scaling', course: 'F05', templateId: 'ratios.scaling_solve' },
  { id: 'f06_powers', name: 'Integer Exponents & Reciprocal Powers', course: 'F06', templateId: 'powers.integer_eval' },
  { id: 'f06_scientific', name: 'Scientific Notation & Powers of 10', course: 'F06', templateId: 'powers.scientific_notation' },
  { id: 'f06_roots', name: 'Square Roots & Rational Radicals', course: 'F06', templateId: 'roots.radical_eval' },
  { id: 'f07_perimeter_area', name: 'Perimeter, Area & Spatial Conservation', course: 'F07', templateId: 'geometry.perimeter_and_area' },
  { id: 'f07_angles', name: 'Angles, Transversals & Triangle Sum', course: 'F07', templateId: 'geometry.angles_triangles' },
  { id: 'f07_coords', name: 'Cartesian Coordinates & Distance Formula', course: 'F07', templateId: 'geometry.coordinate_distance' },
  { id: 'f08_data', name: 'Data Center: Mean, Median & Balance', course: 'F08', templateId: 'data.mean_median_calc' },
  { id: 'f08_counting', name: 'Sample Spaces, Permutations & Combinations', course: 'F08', templateId: 'counting.permutations_combinations' },
  { id: 'f08_prob', name: 'Exact Probability, Complements & Draws', course: 'F08', templateId: 'probability.simple_exact' },
  { id: 's01_expr', name: 'Algebraic Expressions & Substitution', course: 'S01', templateId: 'algebra.expression_eval' },
  { id: 's01_linear', name: 'Linear Equations & Equivalence', course: 'S01', templateId: 'algebra.linear_equation_solve' },
  { id: 's01_systems', name: '2×2 Systems of Linear Equations', course: 'S01', templateId: 'algebra.linear_system_solve' },
  { id: 's02_eval', name: 'Function Mappings, Pre-images & Piecewise', course: 'S02', templateId: 'functions.evaluate_and_range' },
  { id: 's02_comp', name: 'Function Composition & Exact Inverses', course: 'S02', templateId: 'functions.composition_inverse' },
  { id: 's02_trans', name: 'Graph Transformations & Quadratic Vertex', course: 'S02', templateId: 'functions.transformations_vertex' },
  { id: 's03_poly', name: 'Polynomial Factoring, Vieta & Remainder Theorem', course: 'S03', templateId: 'algebra2.polynomial_factor_roots' },
  { id: 's03_disc', name: 'Quadratic Formula & Discriminant Δ', course: 'S03', templateId: 'algebra2.quadratic_discriminant' },
  { id: 's03_explog', name: 'Exponentials, Logarithms & Log Laws', course: 'S03', templateId: 'algebra2.exp_log_eval' },
  { id: 's04_lines', name: 'Analytic Lines, Perpendicularity & Section Ratio', course: 'S04', templateId: 'sec_geom.slope_and_lines' },
  { id: 's04_circles', name: 'Circle Loci, Inscribed Angles & Power of a Point', course: 'S04', templateId: 'sec_geom.circle_equations_theorems' },
  { id: 's04_vectors', name: 'Planar Vectors, Dot Product & Projections', course: 'S04', templateId: 'sec_geom.planar_vectors_dot' },
  { id: 's05_radians', name: 'Radians, Unit Circle Quadrants & Wave Period', course: 'S05', templateId: 'trig.radians_unit_circle' },
  { id: 's05_identities', name: 'Pythagorean, Addition & Double-Angle Identities', course: 'S05', templateId: 'trig.identities_double_angle' },
  { id: 's05_laws', name: 'Inverse Trig Branches, Law of Sines & Cosines', course: 'S05', templateId: 'trig.law_of_sines_cosines' },
  { id: 's06_arith', name: 'Arithmetic Progressions & Gauss Series Sums', course: 'S06', templateId: 'sequences.arithmetic_progression' },
  { id: 's06_geom', name: 'Geometric Sequences & Convergent Infinite Series', course: 'S06', templateId: 'sequences.geometric_and_infinite' },
  { id: 's06_binom', name: 'Recurrence Relations & The Binomial Theorem', course: 'S06', templateId: 'sequences.recursion_binomial' },
  { id: 's07_limits', name: 'Limits, Removable Discontinuities & Continuity', course: 'S07', templateId: 'calculus.limits_rational' },
  { id: 's07_deriv', name: 'Derivatives, Power Rule & Tangent Lines', course: 'S07', templateId: 'calculus.derivatives_tangent' },
  { id: 's07_int', name: 'Definite Integrals & Fundamental Theorem of Calculus', course: 'S07', templateId: 'calculus.definite_integral_ftc' },
  { id: 'a01_read', name: 'Soroban Bead Reading', course: 'A01', templateId: 'abacus.read.state' },
  { id: 'a01_target', name: 'Soroban Target Setting', course: 'A01', templateId: 'abacus.target.setting' },
];

export const MasteryDashboard: React.FC<MasteryDashboardProps> = ({
  ledgerEntries,
  totalXp,
  todayPracticeXp,
  onImportLedger,
  onStartPractice,
  onSelectPractice,
}) => {
  const fileInputRef = useRef<HTMLInputElement>(null);
  const [notice, setNotice] = useState<{ text: string; error: boolean } | null>(null);

  const handleExport = () => {
    const data = {
      export_version: 1,
      export_timestamp: new Date().toISOString(),
      app: 'axiom-academy',
      total_xp: totalXp,
      entries: ledgerEntries,
    };
    const blob = new Blob([JSON.stringify(data, null, 2)], { type: 'application/json' });
    const url = URL.createObjectURL(blob);
    const a = document.createElement('a');
    a.href = url;
    a.download = `axiom-backup-${new Date().toISOString().slice(0, 10)}.json`;
    a.click();
    URL.revokeObjectURL(url);
    setNotice({ text: 'Your rewards backup is ready to download. Lesson notes and unfinished answers are not included.', error: false });
  };

  const handleFileChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    const file = e.target.files?.[0];
    if (!file) return;
    e.target.value = '';
    if (file.size > 2 * 1024 * 1024) {
      setNotice({ text: 'This backup is too large. Choose an AXIOM rewards backup smaller than 2 MB.', error: true });
      return;
    }
    const reader = new FileReader();
    reader.onload = (event) => {
      try {
        const json = JSON.parse(event.target?.result as string);
        if (!Array.isArray(json.entries) || json.entries.length > 10000 || json.entries.some((entry: any) => !entry || typeof entry.award_key !== 'string' || typeof entry.reason !== 'string' || typeof entry.timestamp_utc !== 'string' || !Number.isSafeInteger(entry.delta) || Math.abs(entry.delta) > 1000)) {
          throw new Error('Invalid local reward history');
        }
        if (new Set(json.entries.map((entry: any) => entry.award_key)).size !== json.entries.length) throw new Error('Duplicate award keys');
        if (json.entries && Array.isArray(json.entries)) {
          onImportLedger(json.entries);
          setNotice({ text: 'Rewards restored. Existing entries were kept and duplicate rewards were not added.', error: false });
        }
      } catch (err) {
        setNotice({ text: 'This file is not a valid rewards backup. Choose a backup exported from AXIOM; your current history has been kept.', error: true });
      }
    };
    reader.onerror = () => setNotice({ text: 'The file could not be read. Try choosing it again; your history has been kept.', error: true });
    reader.readAsText(file);
  };

  return (
    <div className="axiom-mastery-container">
      <div className="axiom-mastery-header">
        <div>
          <h2>Your practice, over time</h2>
          <p className="axiom-subtext">
            Small steps add up. See your practice rewards, track topic fluency, and keep a copy of your history.
          </p>
        </div>

        <div className="axiom-backup-actions">
          <button className="axiom-btn axiom-btn-outline" onClick={handleExport}>
            <Download size={14} />
            <span>Export rewards</span>
          </button>
          <button className="axiom-btn axiom-btn-outline" onClick={() => fileInputRef.current?.click()}>
            <Upload size={14} />
            <span>Restore rewards</span>
          </button>
          <input
            ref={fileInputRef}
            type="file"
            accept=".json"
            style={{ display: 'none' }}
            onChange={handleFileChange}
          />
        </div>
      </div>

      {notice && <p className="axiom-import-notice" role={notice.error ? 'alert' : 'status'}>{notice.text}</p>}

      <div className="axiom-reward-summary">
        <div><strong>{totalXp} XP</strong><span>Practice rewards</span></div>
        <div><strong>{todayPracticeXp} / 100</strong><span>Today’s practice XP · UTC</span></div>
      </div>

      <GamerProgressTracker
        ledgerEntries={ledgerEntries}
        totalXp={totalXp}
        todayPracticeXp={todayPracticeXp}
        onStartPractice={onStartPractice}
        onSelectPractice={onSelectPractice}
      />

      {/* Topic Fluency and Confidence Grid */}
      <section className="axiom-card" aria-labelledby="fluency-heading">
        <div className="axiom-card-header">
          <div>
            <h3 id="fluency-heading">Topic fluency & practice confidence</h3>
            <p className="axiom-subtext">
              Track your clean solution streaks across foundational topics. Fluency reflects repeated independent checks on this device; formal certification requires independent review.
            </p>
          </div>
        </div>

        {(() => {
          const today = studyDay();
          const dueTopics = TOPICS.filter((t) => {
            const s = getTopicReviewSchedule(t.templateId);
            return s && today >= s.next_due_epoch_days;
          });
          if (dueTopics.length === 0) return null;
          return (
            <div className="axiom-review-due-alert" role="status" style={{ margin: '1rem 0' }}>
              <Clock size={20} className="axiom-accent" />
              <div>
                <strong>{dueTopics.length} Spaced Retrieval Review{dueTopics.length > 1 ? 's' : ''} Due Today</strong>
                <p className="axiom-subtext">
                  Under Section 16 of the Source of Truth, spaced retrieval reviews test retention after scheduled delays (1d, 3d, 7d, 14d, 30d, 60d, 120d).
                </p>
              </div>
              <button
                type="button"
                className="axiom-btn axiom-btn-primary"
                onClick={() => {
                  if (onSelectPractice) onSelectPractice(dueTopics[0].templateId);
                  else onStartPractice();
                }}
              >
                Practice {dueTopics[0].name}
              </button>
            </div>
          );
        })()}

        <div className="axiom-fluency-grid">
          {TOPICS.map((topic) => {
            const streak = parseInt(localStorage.getItem(`axiom_clean_streak:${topic.templateId}`) || '0', 10);
            const draft = localStorage.getItem(`axiom_practice_draft_v2:${topic.templateId}`);
            const isFluent = streak >= 3;
            const hasStarted = Boolean(draft) || streak > 0;
            const sched = getTopicReviewSchedule(topic.templateId);
            const today = studyDay();
            const isDue = sched ? today >= sched.next_due_epoch_days : false;

            return (
              <div key={topic.id} className="axiom-fluency-card">
                <div className="axiom-fluency-header">
                  <div>
                    <span className="axiom-course-id">[{topic.course}]</span>
                    <h4 className="axiom-fluency-title">{topic.name}</h4>
                  </div>
                  <span className={`axiom-fluency-badge ${isDue ? 'due' : isFluent ? 'fluent' : hasStarted ? 'progress' : 'new'}`}>
                    {isDue ? 'Review due' : isFluent ? '✓ Fluent' : hasStarted ? 'In progress' : 'Not started'}
                  </span>
                </div>

                <div className="axiom-fluency-body">
                  <p className="axiom-subtext">
                    {isFluent
                      ? `${streak} consecutive clean answers`
                      : hasStarted
                      ? `${streak}/3 clean streak`
                      : 'Try an exercise to begin'}
                  </p>
                  {sched && (
                    <p className="axiom-subtext" style={{ marginTop: '0.25rem', color: isDue ? '#e6c580' : '#94a3b8' }}>
                      <Clock size={12} style={{ display: 'inline', marginRight: '4px', verticalAlign: 'middle' }} />
                      Ladder {sched.ladder_step + 1}/7 ({INTERVAL_LADDER_DAYS[sched.ladder_step]}d): {isDue ? 'Due for retrieval' : `Check in ${sched.next_due_epoch_days - today}d`}
                    </p>
                  )}
                </div>

                <button
                  type="button"
                  className="axiom-text-link"
                  onClick={() => (onSelectPractice ? onSelectPractice(topic.templateId) : onStartPractice())}
                >
                  <span>Practice {topic.name}</span>
                  <ArrowRight size={14} aria-hidden="true" />
                </button>
              </div>
            );
          })}
        </div>
      </section>

      <div className="axiom-card">
        <h3>Practice history — mastery not yet assessed</h3>
        <p>Local rewards recognize practice. Conceptual understanding, reasoning, transfer, and delayed retention have not yet been independently assessed.</p>
        <p className="axiom-draft-note">Your history is saved on this device. Account backup is still being built. Exported rewards do not include lesson notes or unfinished answers.</p>
      </div>

      {/* Reward Ledger Table */}
      <FullStudyBackup />
      <CertificatePreview />
      <div className="axiom-card">
        <div className="axiom-card-header">
          <div>
            <h3>Practice rewards</h3>
            <p className="axiom-subtext">One reward per successful example. These local records recognize practice, not certified mastery.</p>
          </div>
        </div>

        {ledgerEntries.length === 0 ? (
          <div className="axiom-empty-ledger">
            <Clock size={24} />
            <h4>Your first step is an example</h4>
            <p>Try a practice problem. A checked answer earns XP, and feedback helps you decide what to try next.</p>
            <button className="axiom-text-link" onClick={onStartPractice}>Start practicing</button>
          </div>
        ) : (
          <div className="axiom-table-wrapper">
            <table className="axiom-table">
              <caption>Most recent rewards first. Dates are shown in UTC.</caption>
              <thead>
                <tr>
                  <th scope="col">Date</th>
                  <th scope="col">Activity</th>
                  <th scope="col">Reward</th>
                </tr>
              </thead>
              <tbody>
                {[...ledgerEntries].reverse().map((e, idx) => (
                  <tr key={`${e.award_key}-${idx}`}>
                    <td>{e.timestamp_utc.slice(0, 10)}</td>
                    <td>Practice answer checked<details className="axiom-reward-key"><summary>Record details</summary><p>{e.reason}</p><p>{e.award_key}</p><p>{e.timestamp_utc}</p></details></td>
                    <td className="axiom-delta-cell">
                      <span className={e.delta > 0 ? 'axiom-xp-plus' : 'axiom-xp-zero'}>
                        +{e.delta} XP
                      </span>
                    </td>
                  </tr>
                ))}
              </tbody>
            </table>
          </div>
        )}
      </div>
    </div>
  );
};
