import React, { useState, useEffect, useRef } from 'react';
import { MathView } from './MathView';
import { Encouragement } from './Encouragement';
import { FeedbackSupport } from './FeedbackSupport';
import { generateExercise, checkAnswer, ProblemArtifact, GradeOutcome, getTopicReviewSchedule, saveTopicReviewSchedule, updateReviewSchedule, INTERVAL_LADDER_DAYS, ReviewSchedule, studyDay } from '../engine/axiomCore';
import { CheckCircle2, XCircle, AlertCircle, RefreshCw, ArrowRight, Award, Lightbulb, Zap, Target, Clock, Compass, CornerDownLeft } from 'lucide-react';

interface PracticeWorkspaceProps {
  onAwardXp: (instanceId: string) => { delta: number; reason: string } | null;
  onRecordMasteryEvidence: (family: string, success: boolean) => void;
  onAwardProvisionalMilestone?: (conceptId: string) => { delta: number; reason: string } | null;
  initialTemplateId?: string;
}

const DIAGNOSTIC_BATTERY = [
  { templateId: 'learning.problem_reading', band: 1, topic: 'Problem Reading & Unknowns', course: 'F00' },
  { templateId: 'place_value.decompose', band: 1, topic: 'Base-10 Place Value', course: 'F01' },
  { templateId: 'arithmetic.column.addition', band: 1, topic: 'Column Arithmetic & Regrouping', course: 'F02' },
  { templateId: 'divisibility.gcd_lcm', band: 1, topic: 'Divisibility, Primes & Factors', course: 'F03' },
  { templateId: 'fractions.compare.positive', band: 1, topic: 'Fraction Models & Common Denominators', course: 'F04' },
  { templateId: 'decimals.fraction_convert', band: 1, topic: 'Decimals & Power-of-10 Parts', course: 'F05' },
  { templateId: 'powers.integer_eval', band: 1, topic: 'Powers, Exponents & Radicals', course: 'F06' },
  { templateId: 'geometry.perimeter_and_area', band: 1, topic: 'Geometry, Area & Coordinates', course: 'F07' },
  { templateId: 'data.mean_median_calc', band: 1, topic: 'Data, Counting & Exact Probability', course: 'F08' },
  { templateId: 'algebra.linear_equation_solve', band: 1, topic: 'Algebraic Equations & Systems', course: 'S01' },
  { templateId: 'functions.evaluate_and_range', band: 1, topic: 'Functions, Composition & Inverses', course: 'S02' },
  { templateId: 'algebra2.quadratic_discriminant', band: 1, topic: 'Polynomials, Quadratics & Logarithms', course: 'S03' },
  { templateId: 'sec_geom.slope_and_lines', band: 1, topic: 'Analytic Lines, Circles & Vectors', course: 'S04' },
  { templateId: 'trig.radians_unit_circle', band: 1, topic: 'Trigonometry, Identities & Triangle Laws', course: 'S05' },
  { templateId: 'sequences.arithmetic_progression', band: 1, topic: 'Sequences, Series & Binomial Theorem', course: 'S06' },
  { templateId: 'calculus.limits_rational', band: 1, topic: 'Limits, Derivatives & Definite Integrals', course: 'S07' },
];

export const PracticeWorkspace: React.FC<PracticeWorkspaceProps> = ({
  onAwardXp,
  onRecordMasteryEvidence,
  onAwardProvisionalMilestone,
  initialTemplateId = 'fractions.compare.positive',
}) => {
  const [templateId, setTemplateId] = useState<string>(initialTemplateId);
  const [band, setBand] = useState<number>(1);
  const [problem, setProblem] = useState<ProblemArtifact | null>(null);
  const [selectedOption, setSelectedOption] = useState<string>('');
  const [textInput, setTextInput] = useState<string>('');
  const [outcome, setOutcome] = useState<GradeOutcome | null>(null);
  const [seedCounter, setSeedCounter] = useState<number>(1);
  const [repairNeeded, setRepairNeeded] = useState<boolean>(false);
  const [busy, setBusy] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const evidenceRecorded = useRef(new Set<string>());
  const loadRevision = useRef(0);
  const [mistakes, setMistakes] = useState(0);
  const [attempts, setAttempts] = useState(0);
  const [assisted, setAssisted] = useState(false);
  const [lastAward, setLastAward] = useState<number | null>(null);
  const [cleanStreak, setCleanStreak] = useState<number>(0);
  const [explanation, setExplanation] = useState<string>('');
  const [scratch, setScratch] = useState<string>('');
  const [workoutActive, setWorkoutActive] = useState<boolean>(false);
  const [workoutIndex, setWorkoutIndex] = useState<number>(1);
  const [workoutCorrect, setWorkoutCorrect] = useState<number>(0);
  const [workoutDone, setWorkoutDone] = useState<boolean>(false);
  const [schedule, setSchedule] = useState<ReviewSchedule | null>(null);
  const [reviewAdvanceNotice, setReviewAdvanceNotice] = useState<string | null>(null);
  const [diagnosticActive, setDiagnosticActive] = useState<boolean>(false);
  const [diagnosticIndex, setDiagnosticIndex] = useState<number>(0);
  const [diagnosticResults, setDiagnosticResults] = useState<Array<{ topic: string; course: string; correct: boolean; templateId: string }>>([]);
  const [diagnosticDone, setDiagnosticDone] = useState<boolean>(false);

  useEffect(() => {
    if (initialTemplateId) {
      setTemplateId(initialTemplateId);
    }
  }, [initialTemplateId]);

  // Persist the current problem and unfinished answer; stale worker results cannot replace it.
  useEffect(() => {
    let restored = false;
    setOutcome(null); setLastAward(null); setRepairNeeded(false); setError(null); setMistakes(0); setAttempts(0); setAssisted(false);
    setReviewAdvanceNotice(null);
    setSchedule(getTopicReviewSchedule(templateId));
    try {
      const saved = JSON.parse(localStorage.getItem(`axiom_practice_draft_v2:${templateId}`) || 'null');
      if (saved?.problem?.template_id === templateId && saved.problem.replay_identity) {
        setProblem(saved.problem);
        setTextInput(typeof saved.answer === 'string' ? saved.answer : '');
        setSelectedOption(typeof saved.option === 'string' ? saved.option : '');
        setMistakes(Number.isSafeInteger(saved.mistakes) ? Math.max(0, Math.min(saved.mistakes, 1000)) : 0);
        setAttempts(Number.isSafeInteger(saved.attempts) ? Math.max(0, Math.min(saved.attempts, 1000)) : 0);
        setAssisted(Boolean(saved.assisted));
        if (Number.isSafeInteger(saved.problem.replay_identity.difficulty_band)) {
          setBand(Math.max(1, Math.min(3, saved.problem.replay_identity.difficulty_band)));
        }
        setBusy(false);
        restored = true;
      }
      const savedStreak = parseInt(localStorage.getItem(`axiom_clean_streak:${templateId}`) || '0', 10);
      setCleanStreak(Number.isSafeInteger(savedStreak) ? Math.max(0, savedStreak) : 0);
      const savedExp = localStorage.getItem(`axiom_practice_explanation:${templateId}`) || '';
      setExplanation(savedExp);
      setScratch(typeof saved.scratch === 'string' ? saved.scratch.slice(0, 4000) : '');
    } catch { /* Preserve other saved data when a draft is unreadable. */ }
    if (!restored) void loadNewProblem(templateId, band);
    return () => { loadRevision.current += 1; };
  }, [templateId]);

  useEffect(() => {
    if (problem) {
      localStorage.setItem(`axiom_practice_draft_v2:${problem.template_id}`, JSON.stringify({
        problem,
        answer: textInput,
        option: selectedOption,
        mistakes,
        attempts,
        assisted,
        scratch,
      }));
    }
  }, [problem, textInput, selectedOption, mistakes, attempts, assisted, scratch]);

  const loadNewProblem = async (tmpl: string, targetBand = band) => {
    const revision = ++loadRevision.current;
    setBusy(true); setError(null);
    const bytes = crypto.getRandomValues(new Uint8Array(32));
    const seedHex = Array.from(bytes, byte => byte.toString(16).padStart(2, '0')).join('');
    try {
      const next = await generateExercise(tmpl, seedHex, targetBand);
      if (revision !== loadRevision.current) return;
      setProblem(next);
      setSelectedOption('');
      setTextInput('');
      setOutcome(null);
      setLastAward(null);
      setMistakes(0);
      setAttempts(0);
      setAssisted(false);
      setRepairNeeded(false);
    } catch (failure) {
      if (revision === loadRevision.current) setError(String(failure));
    } finally {
      if (revision === loadRevision.current) setBusy(false);
    }
  };

  const handleSubmit = async (e?: React.FormEvent) => {
    if (e) e.preventDefault();
    if (!problem || busy) return;

    const answer = (problem.answer_kind === 'relation' || problem.answer_kind === 'enum') ? selectedOption : textInput;
    if (!answer.trim()) return;

    setBusy(true); setError(null);
    const revision = loadRevision.current;
    let res: GradeOutcome;
    try { res = await checkAnswer(problem, answer, assisted); }
    catch (failure) { setError(String(failure)); setBusy(false); return; }
    if (revision !== loadRevision.current) return;
    setBusy(false);
    setAttempts(value => value + 1);
    setOutcome(res);

    if (res.disposition === 'correct') {
      const key = problem.problem_hash;
      if (!evidenceRecorded.current.has(key)) {
        onRecordMasteryEvidence(problem.template_id, true);
        evidenceRecorded.current.add(key);
      }
      const instanceId = `${problem.template_id}:${problem.seed_hex}`;
      const award = assisted ? null : onAwardXp(instanceId);
      if (award && award.delta > 0) {
        setLastAward(award.delta);
      }
      setRepairNeeded(false);

      if (!assisted) {
        const nextStreak = cleanStreak + 1;
        setCleanStreak(nextStreak);
        try { localStorage.setItem(`axiom_clean_streak:${problem.template_id}`, nextStreak.toString()); } catch {}
        if (nextStreak >= 6 && onAwardProvisionalMilestone) {
          const award = onAwardProvisionalMilestone(problem.template_id);
          if (award && award.delta > 0) {
            setLastAward(award.delta);
          }
        }

        // Spaced delayed retention review schedule update (Section 16.3)
        const today = studyDay();
        const currentSched = getTopicReviewSchedule(problem.template_id);
        if (currentSched && today >= currentSched.next_due_epoch_days) {
          const nextSched = updateReviewSchedule(currentSched, true, today);
          saveTopicReviewSchedule(problem.template_id, nextSched);
          setSchedule(nextSched);
          const intervalDays = INTERVAL_LADDER_DAYS[nextSched.ladder_step];
          setReviewAdvanceNotice(
            `Spaced Retrieval Check Passed! Advanced to step ${nextSched.ladder_step + 1} of 7 (${intervalDays} day${intervalDays > 1 ? 's' : ''} interval).`
          );
        } else if (nextStreak >= 6 && !currentSched) {
          const initialSched = updateReviewSchedule(null, true, today);
          saveTopicReviewSchedule(problem.template_id, initialSched);
          setSchedule(initialSched);
        }

        if (workoutActive) {
          setWorkoutCorrect(c => c + 1);
        }

        if (diagnosticActive) {
          const currentItem = DIAGNOSTIC_BATTERY[diagnosticIndex];
          setDiagnosticResults(prev => {
            const next = [...prev];
            next[diagnosticIndex] = {
              topic: currentItem.topic,
              course: currentItem.course,
              correct: true,
              templateId: currentItem.templateId,
            };
            return next;
          });
        }
      }
    } else if (res.disposition === 'incorrect') {
      setMistakes(value => value + 1);
      setCleanStreak(0);
      try { localStorage.setItem(`axiom_clean_streak:${problem.template_id}`, '0'); } catch {}
      if (!evidenceRecorded.current.has(problem.problem_hash)) {
        onRecordMasteryEvidence(problem.template_id, false);
        evidenceRecorded.current.add(problem.problem_hash);
      }

      if (diagnosticActive) {
        const currentItem = DIAGNOSTIC_BATTERY[diagnosticIndex];
        setDiagnosticResults(prev => {
          const next = [...prev];
          next[diagnosticIndex] = {
            topic: currentItem.topic,
            course: currentItem.course,
            correct: false,
            templateId: currentItem.templateId,
          };
          return next;
        });
      }

      // On incorrect review, reset ladder to step 0 (1 day) and recommend repair (Section 16.3)
      const today = studyDay();
      const currentSched = getTopicReviewSchedule(problem.template_id);
      if (currentSched) {
        const resetSched = updateReviewSchedule(currentSched, false, today);
        saveTopicReviewSchedule(problem.template_id, resetSched);
        setSchedule(resetSched);
      }

      if (problem.template_id === 'fractions.compare.positive') {
        setRepairNeeded(true);
      }
    }
  };

  const handleNextProblem = () => {
    setSeedCounter((c) => c + 1);
    if (diagnosticActive) {
      if (diagnosticIndex >= DIAGNOSTIC_BATTERY.length - 1) {
        setDiagnosticDone(true);
        return;
      }
      const nextIndex = diagnosticIndex + 1;
      setDiagnosticIndex(nextIndex);
      const nextItem = DIAGNOSTIC_BATTERY[nextIndex];
      setTemplateId(nextItem.templateId);
      setBand(nextItem.band);
      void loadNewProblem(nextItem.templateId, nextItem.band);
      return;
    }
    if (workoutActive) {
      if (workoutIndex >= 5) {
        setWorkoutDone(true);
        return;
      }
      const nextIndex = workoutIndex + 1;
      setWorkoutIndex(nextIndex);
      // Workout schedule: Q1-2: Band 1, Q3-4: Band 2, Q5: Band 3
      const targetBand = nextIndex <= 2 ? 1 : nextIndex <= 4 ? 2 : 3;
      setBand(targetBand);
      void loadNewProblem(templateId, targetBand);
    } else {
      void loadNewProblem(templateId, band);
    }
  };

  const handleStartDiagnostic = () => {
    setDiagnosticActive(true);
    setDiagnosticDone(false);
    setDiagnosticIndex(0);
    setDiagnosticResults([]);
    setWorkoutActive(false);
    setWorkoutDone(false);
    const first = DIAGNOSTIC_BATTERY[0];
    setTemplateId(first.templateId);
    setBand(first.band);
    void loadNewProblem(first.templateId, first.band);
  };

  const handleExitDiagnostic = () => {
    setDiagnosticActive(false);
    setDiagnosticDone(false);
    setDiagnosticIndex(0);
    setDiagnosticResults([]);
    void loadNewProblem(templateId, band);
  };

  const handleStartWorkout = () => {
    setWorkoutActive(true);
    setWorkoutIndex(1);
    setWorkoutCorrect(0);
    setWorkoutDone(false);
    setDiagnosticActive(false);
    setDiagnosticDone(false);
    setBand(1);
    void loadNewProblem(templateId, 1);
  };

  const handleExitWorkout = () => {
    setWorkoutActive(false);
    setWorkoutDone(false);
    setWorkoutIndex(1);
    setWorkoutCorrect(0);
    void loadNewProblem(templateId, band);
  };

  const handleBandChange = (newBand: number) => {
    setBand(newBand);
    setCleanStreak(0);
    setWorkoutActive(false);
    setWorkoutDone(false);
    void loadNewProblem(templateId, newBand);
  };

  const handleStartRepair = () => {
    setTemplateId('fractions.equivalent.find');
    setBand(1);
    setRepairNeeded(false);
  };

  return (
    <div className="axiom-practice-container">
      <div className="axiom-practice-header">
        <div>
          <h2>A little practice, a clearer idea</h2>
          <p className="axiom-subtext">Try an answer, learn from the feedback, and build confidence through tiered practice.</p>
        </div>

        <div className="axiom-template-selector">
          <label htmlFor="tmpl-select">Choose an exercise </label>
          <select
            id="tmpl-select"
            aria-label="Choose an exercise"
            disabled={busy}
            value={templateId}
            onChange={(e) => {
              setTemplateId(e.target.value);
              setWorkoutActive(false);
              setWorkoutDone(false);
            }}
            className="axiom-select"
          >
            <option value="learning.problem_reading">Problem Reading & Formulating Unknowns (F00)</option>
            <option value="learning.equality_balance">Meaning of Equality & Equation Balance (F00)</option>
            <option value="learning.checking_bounds">Checking Answers, Bounds & Counterexamples (F00)</option>
            <option value="fractions.compare.positive">Fraction Comparison (F04)</option>
            <option value="fractions.equivalent.find">Equivalent Fraction Construction (F04)</option>
            <option value="fractions.unit.identify">Unit Fraction Partitions (F04)</option>
            <option value="place_value.decompose">Base-10 Place Value Decomposition (F01)</option>
            <option value="arithmetic.whole.addition">Whole Number Addition (F02)</option>
            <option value="arithmetic.column.addition">Column Addition with Regrouping (F02)</option>
            <option value="arithmetic.column.subtraction">Column Subtraction with Borrowing (F02)</option>
            <option value="arithmetic.whole.multiplication">Multiplication & Array Models (F02)</option>
            <option value="arithmetic.whole.division">Division & Equal Partitioning (F02)</option>
            <option value="integers.order">Signed Integer Number Line & Ordering (F03)</option>
            <option value="divisibility.factors.find">Divisibility & Greatest Proper Divisors (F03)</option>
            <option value="divisibility.gcd_lcm">Greatest Common Divisor & LCM (F03)</option>
            <option value="decimals.fraction_convert">Decimals & Power-of-10 Fractions (F05)</option>
            <option value="percentages.benchmark_calc">Percentages & Benchmark Calculations (F05)</option>
            <option value="ratios.scaling_solve">Ratios, Unit Rates & Proportions (F05)</option>
            <option value="powers.integer_eval">Integer Exponents & Reciprocal Powers (F06)</option>
            <option value="powers.scientific_notation">Scientific Notation & Powers of 10 (F06)</option>
            <option value="roots.radical_eval">Square Roots & Rational Radicals (F06)</option>
            <option value="geometry.perimeter_and_area">Perimeter, Area & Spatial Conservation (F07)</option>
            <option value="geometry.angles_triangles">Angles, Transversals & Triangle Sum (F07)</option>
            <option value="geometry.coordinate_distance">Cartesian Coordinates & Distance Formula (F07)</option>
            <option value="data.mean_median_calc">Data Center: Mean, Median & Missing Values (F08)</option>
            <option value="counting.permutations_combinations">Sample Spaces: Permutations & Combinations (F08)</option>
            <option value="probability.simple_exact">Exact Probability, Complements & Compound Draws (F08)</option>
            <option value="algebra.expression_eval">Algebraic Expressions, Substitution & Expansion (S01)</option>
            <option value="algebra.linear_equation_solve">Linear Equations & Equivalence Transformations (S01)</option>
            <option value="algebra.linear_system_solve">2×2 Systems of Linear Equations (S01)</option>
            <option value="functions.evaluate_and_range">Functions: Pre-images, Piecewise & Rate of Change (S02)</option>
            <option value="functions.composition_inverse">Function Composition (f ∘ g) & Exact Inverses (S02)</option>
            <option value="functions.transformations_vertex">Function Transformations & Quadratic Vertex Form (S02)</option>
            <option value="algebra2.polynomial_factor_roots">Polynomial Factoring, Vieta & Remainder Theorem (S03)</option>
            <option value="algebra2.quadratic_discriminant">Quadratic Formula, Discriminant Δ & Repeated Roots (S03)</option>
            <option value="algebra2.exp_log_eval">Exponentials, Logarithms & Log Laws (S03)</option>
            <option value="sec_geom.slope_and_lines">Analytic Lines, Perpendicularity & Section Formula (S04)</option>
            <option value="sec_geom.circle_equations_theorems">Circle Loci, Inscribed Angles & Power of a Point (S04)</option>
            <option value="sec_geom.planar_vectors_dot">Planar Vectors, Dot Product & Projections (S04)</option>
            <option value="trig.radians_unit_circle">Radians, Unit Circle Quadrants & Wave Period (S05)</option>
            <option value="trig.identities_double_angle">Pythagorean, Addition & Double-Angle Identities (S05)</option>
            <option value="trig.law_of_sines_cosines">Inverse Trig Branches, Law of Sines & Cosines (S05)</option>
            <option value="sequences.arithmetic_progression">Arithmetic Progressions & Gauss Series Sums (S06)</option>
            <option value="sequences.geometric_and_infinite">Geometric Sequences & Convergent Infinite Series (S06)</option>
            <option value="sequences.recursion_binomial">Recurrence Relations & The Binomial Theorem (S06)</option>
            <option value="calculus.limits_rational">Limits, Removable Discontinuities & Continuity (S07)</option>
            <option value="calculus.derivatives_tangent">Derivatives, Power Rule & Tangent Lines (S07)</option>
            <option value="calculus.definite_integral_ftc">Definite Integrals & Fundamental Theorem of Calculus (S07)</option>
            <option value="abacus.read.state">Soroban Abacus Rod Reading (A01)</option>
            <option value="abacus.target.setting">Soroban Abacus Target Setting (A01/A02)</option>
          </select>
        </div>
      </div>

      {/* Tiered Difficulty & Mastery Mode Selector */}
      <section className="axiom-card axiom-practice-modes" aria-label="Practice difficulty and goals">
        <div className="axiom-mode-row">
          <div className="axiom-band-picker-wrap" role="group" aria-label="Difficulty band">
            <span className="axiom-label-caption">Challenge band:</span>
            <div className="axiom-band-buttons">
              {[1, 2, 3].map((b) => (
                <button
                  key={b}
                  type="button"
                  className={`axiom-chip ${band === b && !workoutActive ? 'active' : ''}`}
                  aria-pressed={band === b && !workoutActive}
                  disabled={busy}
                  onClick={() => handleBandChange(b)}
                >
                  {b === 1 ? '1 · Find your footing' : b === 2 ? '2 · Build connections' : '3 · Stretch thinking'}
                </button>
              ))}
            </div>
          </div>

          <div className="axiom-workout-toggle" style={{ display: 'flex', gap: '0.6rem', flexWrap: 'wrap' }}>
            {!workoutActive && !diagnosticActive ? (
              <>
                <button
                  type="button"
                  className="axiom-btn axiom-btn-outline"
                  disabled={busy}
                  onClick={handleStartWorkout}
                  title="Complete a structured 5-problem sequence across all difficulty tiers"
                >
                  <Target size={15} aria-hidden="true" />
                  <span>Start 5-Problem Mastery Workout</span>
                </button>
                <button
                  type="button"
                  className="axiom-btn axiom-btn-outline"
                  disabled={busy}
                  onClick={handleStartDiagnostic}
                  title="Take a non-punitive diagnostic across 6 foundational topics"
                >
                  <Compass size={15} aria-hidden="true" />
                  <span>Diagnostic Placement Check</span>
                </button>
              </>
            ) : workoutActive ? (
              <button
                type="button"
                className="axiom-btn axiom-btn-outline"
                disabled={busy}
                onClick={handleExitWorkout}
              >
                <span>Exit Workout (Free Mode)</span>
              </button>
            ) : (
              <button
                type="button"
                className="axiom-btn axiom-btn-outline"
                disabled={busy}
                onClick={handleExitDiagnostic}
              >
                <span>Exit Diagnostic (Free Mode)</span>
              </button>
            )}
          </div>
        </div>

        {/* Workout Progress Bar if Active */}
        {workoutActive && !workoutDone && (
          <div className="axiom-workout-status-bar" role="status">
            <div className="axiom-workout-info">
              <Zap size={16} className="axiom-accent" />
              <strong>Mastery Workout: Question {workoutIndex} of 5</strong>
              <span className="axiom-subtext">· Band {band} · Current score: {workoutCorrect} / {workoutIndex - (outcome ? 0 : 1)}</span>
            </div>
            <div className="axiom-progress-track">
              <div
                className="axiom-progress-fill"
                style={{ width: `${((workoutIndex - (outcome ? 0 : 1)) / 5) * 100}%` }}
              />
            </div>
          </div>
        )}

        {/* Workout Completion Card */}
        {workoutDone && (
          <div className="axiom-workout-summary" role="status">
            <CheckCircle2 size={24} className="axiom-success-icon" />
            <div>
              <h3>Mastery Workout Complete!</h3>
              <p>You solved <strong>{workoutCorrect} of 5</strong> problems accurately across all three difficulty bands.</p>
              <p className="axiom-subtext">Solving unseen problems across foundational and stretch bands builds genuine mathematical fluency and confidence.</p>
            </div>
            <div className="axiom-support-actions">
              <button className="axiom-btn axiom-btn-primary" onClick={handleStartWorkout}>
                Try another workout
              </button>
              <button className="axiom-btn axiom-btn-outline" onClick={handleExitWorkout}>
                Return to free practice
              </button>
            </div>
          </div>
        )}

        {/* Diagnostic Progress Bar if Active */}
        {diagnosticActive && !diagnosticDone && (
          <div className="axiom-workout-status-bar" role="status">
            <div className="axiom-workout-info">
              <Compass size={16} className="axiom-accent" />
              <strong>Diagnostic Placement: Item {diagnosticIndex + 1} of {DIAGNOSTIC_BATTERY.length}</strong>
              <span className="axiom-subtext">· {DIAGNOSTIC_BATTERY[diagnosticIndex].course}: {DIAGNOSTIC_BATTERY[diagnosticIndex].topic}</span>
            </div>
            <div className="axiom-progress-track">
              <div
                className="axiom-progress-fill"
                style={{ width: `${((diagnosticIndex + (outcome ? 1 : 0)) / DIAGNOSTIC_BATTERY.length) * 100}%` }}
              />
            </div>
          </div>
        )}

        {/* Diagnostic Completion Summary Card */}
        {diagnosticDone && (
          <div className="axiom-workout-summary" role="status">
            <div style={{ display: 'flex', alignItems: 'center', gap: '0.75rem' }}>
              <Compass size={28} className="axiom-accent" />
              <div>
                <h3>Foundations Diagnostic & Placement Profile</h3>
                <p className="axiom-subtext">
                  Non-punitive diagnostic assessment per Section 5.1 & Section 11.3 of the Source of Truth.
                </p>
              </div>
            </div>

            <p>
              You completed all {DIAGNOSTIC_BATTERY.length} diagnostic items. Below is your verified foundational profile:
            </p>

            <div style={{ display: 'grid', gridTemplateColumns: 'repeat(auto-fit, minmax(260px, 1fr))', gap: '0.75rem', marginTop: '0.5rem' }}>
              {diagnosticResults.map((res, i) => (
                <div
                  key={i}
                  style={{
                    padding: '0.85rem 1rem',
                    background: 'var(--bg-surface)',
                    borderRadius: '6px',
                    border: '1px solid var(--border-color)',
                    display: 'flex',
                    flexDirection: 'column',
                    justifyContent: 'space-between',
                    gap: '0.5rem',
                  }}
                >
                  <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center' }}>
                    <span className="axiom-course-id">[{res.course}] {res.topic}</span>
                    <span className={`axiom-fluency-badge ${res.correct ? 'fluent' : 'progress'}`}>
                      {res.correct ? '✓ Fluent' : 'Practice'}
                    </span>
                  </div>
                  <button
                    type="button"
                    className="axiom-btn axiom-btn-outline axiom-btn-sm"
                    onClick={() => {
                      setDiagnosticActive(false);
                      setDiagnosticDone(false);
                      setTemplateId(res.templateId);
                      setBand(res.correct ? 2 : 1);
                      void loadNewProblem(res.templateId, res.correct ? 2 : 1);
                    }}
                  >
                    {res.correct ? 'Practice Band 2' : 'Focus on this topic'}
                  </button>
                </div>
              ))}
            </div>

            <div className="axiom-support-actions" style={{ marginTop: '1rem' }}>
              <button
                type="button"
                className="axiom-btn axiom-btn-primary"
                onClick={handleStartDiagnostic}
              >
                Retake Diagnostic
              </button>
              <button
                type="button"
                className="axiom-btn axiom-btn-outline"
                onClick={handleExitDiagnostic}
              >
                Return to Free Practice
              </button>
            </div>
          </div>
        )}

        {/* Spaced Retention Review Indicator & Alert */}
        {schedule && (
          <div className="axiom-schedule-banner">
            <div className="axiom-schedule-status">
              <Clock size={16} className="axiom-accent" />
              <span>
                <strong>Spaced Retention Ladder: Step {schedule.ladder_step + 1} of 7 ({INTERVAL_LADDER_DAYS[schedule.ladder_step]}d interval)</strong>
                {studyDay() >= schedule.next_due_epoch_days ? (
                  <span className="axiom-review-badge due"> · Review Due Now</span>
                ) : (
                  <span className="axiom-review-badge scheduled"> · Next check in {schedule.next_due_epoch_days - studyDay()} day(s)</span>
                )}
              </span>
            </div>
            {studyDay() >= schedule.next_due_epoch_days && (
              <p className="axiom-subtext">
                This retrieval review is due! Solving this problem unassisted will advance your interval on the retention ladder toward Secure Mastery.
              </p>
            )}
          </div>
        )}

        {reviewAdvanceNotice && (
          <div className="axiom-streak-congrats" role="status">
            <Award size={18} className="axiom-accent" />
            <span><strong>{reviewAdvanceNotice}</strong></span>
          </div>
        )}

        {/* Clean Streak & Confidence Meter */}
        {!workoutActive && (
          <div className="axiom-confidence-bar">
            <div className="axiom-confidence-text">
              <span>Band {band} Clean Streak: </span>
              <strong>{cleanStreak} / {cleanStreak >= 6 ? '6 (Provisional Milestone Unlocked!)' : cleanStreak >= 3 ? '6 toward Provisional Milestone' : '3 consecutive unassisted correct'}</strong>
            </div>
            {cleanStreak >= 6 ? (
              <div className="axiom-streak-congrats" role="status">
                <Award size={18} className="axiom-accent" />
                <span><strong>Provisional Milestone Achieved (+50 XP)!</strong> 6 consecutive unassisted solutions demonstrate confident initial mastery (posterior <i>m</i> = 7/8 = 0.875). A retrieval check will be scheduled.</span>
              </div>
            ) : cleanStreak >= 3 ? (
              <div className="axiom-streak-congrats" role="status">
                <CheckCircle2 size={16} className="axiom-success" />
                <span>Consistent fluency in Band {band}! {6 - cleanStreak} more for Provisional Milestone (+50 XP).</span>
                {band < 3 && (
                  <button
                    type="button"
                    className="axiom-btn axiom-btn-primary axiom-btn-sm"
                    onClick={() => handleBandChange(band + 1)}
                  >
                    Try Band {band + 1}
                  </button>
                )}
              </div>
            ) : null}
          </div>
        )}
      </section>

      {error && <p role="alert">{error}</p>}
      {busy && <p role="status">Working on your example…</p>}
      {problem && !workoutDone && !diagnosticDone && (
        <div className="axiom-practice-card axiom-card">
          <div className="axiom-exercise-meta">
            <span className="axiom-tag">Band {String(problem.replay_identity?.difficulty_band ?? band)} · saved on this device</span>
            <span className="axiom-tag axiom-tag-seed" title={`Seed: ${problem.seed_hex}`}>
              {workoutActive ? `Workout Item #${workoutIndex}` : diagnosticActive ? `Diagnostic Item #${diagnosticIndex + 1} of ${DIAGNOSTIC_BATTERY.length}` : `Example #${seedCounter}`}
            </span>
          </div>

          <div className="axiom-exercise-prompt">
            <h3>{problem.prompt_text}</h3>
            {problem.prompt_latex && (
              <div className="axiom-math-large">
                <MathView math={problem.prompt_latex} display />
              </div>
            )}
          </div>

          {/* Interactive Input Form */}
          {problem.answer_kind === 'soroban_state' && (
            <p>A numerical equivalent can be entered here. Explore bead controls in the Soroban Studio; a typed number does not assess physical technique.</p>
          )}
          {problem.metadata.abacus_state && (
            <div className="axiom-card">
              <h4>Soroban bead positions</h4>
              <p>Only engaged beads count. Each upper bead contributes five units of its rod; each lower bead contributes one.</p>
              <table className="axiom-table">
                <thead><tr><th>Place</th><th>Upper bead</th><th>Lower beads touching the bar</th></tr></thead>
                <tbody>{problem.metadata.abacus_state.rods.map((rod: { upper: boolean; lower: number }, index: number) => (
                  <tr key={index}><th>{index === 0 ? 'Tens' : 'Ones'}</th><td>{rod.upper ? 'Engaged' : 'Away from bar'}</td><td>{rod.lower} of 4</td></tr>
                ))}</tbody>
              </table>
            </div>
          )}
          <form
            onSubmit={handleSubmit}
            className="axiom-answer-form"
            onKeyDown={(e) => {
              if (e.key === 'Enter' && !busy) {
                const canSubmit = (problem.answer_kind === 'relation' || problem.answer_kind === 'enum')
                  ? Boolean(selectedOption)
                  : Boolean(textInput.trim());
                if (canSubmit && (e.target as HTMLElement).tagName !== 'TEXTAREA') {
                  // Submit immediately
                  handleSubmit(e);
                }
              }
            }}
          >
            {(problem.answer_kind === 'relation' || problem.answer_kind === 'enum') && problem.options ? (
              <>
                <div className="axiom-options-grid">
                  {problem.options.map((opt) => (
                    <button
                      type="button"
                      key={opt}
                      className={`axiom-option-btn ${selectedOption === opt ? 'selected' : ''}`}
                      disabled={busy}
                      onClick={() => { setSelectedOption(opt); setOutcome(null); setLastAward(null); }}
                      onKeyDown={(e) => {
                        if (e.key === 'Enter') {
                          if (selectedOption === opt) {
                            e.preventDefault();
                            handleSubmit(e);
                          } else {
                            setSelectedOption(opt);
                            setOutcome(null);
                            setLastAward(null);
                          }
                        }
                      }}
                      aria-pressed={selectedOption === opt}
                    >
                      <span>
                        {opt === 'less' ? 'is less than (<)' : opt === 'equal' ? 'is equal to (=)' : 'is greater than (>)'}
                      </span>
                      {selectedOption === opt && (
                        <span className="axiom-enter-badge" aria-hidden="true">↵ Enter</span>
                      )}
                    </button>
                  ))}
                </div>
                {selectedOption && (
                  <p className="axiom-input-hint">
                    <CornerDownLeft size={13} aria-hidden="true" />
                    <span>Option selected. Press <kbd>Enter ↵</kbd> to submit immediately.</span>
                  </p>
                )}
              </>
            ) : (
              <div className="axiom-input-row">
                <label htmlFor="practice-answer">Your answer</label>
                <input
                  type="text"
                  className="axiom-text-input"
                  id="practice-answer"
                  aria-label="Your answer"
                  maxLength={8192}
                  placeholder={problem.answer_kind === 'rational' ? 'e.g. 6/8' : 'Enter number'}
                  value={textInput}
                  disabled={busy}
                  onChange={(e) => { setTextInput(e.target.value); setOutcome(null); setLastAward(null); }}
                  autoFocus
                />
                <p className="axiom-input-hint">
                  <CornerDownLeft size={13} aria-hidden="true" />
                  <span>Press <kbd>Enter ↵</kbd> to automatically submit your answer</span>
                </p>
              </div>
            )}

            <div className="axiom-action-row">
              <button
                type="submit"
                className="axiom-btn axiom-btn-primary"
                aria-label="Check answer"
                disabled={busy || (problem.answer_kind === 'relation' ? !selectedOption : !textInput.trim())}
              >
                <span>Check answer</span>
                <span className="axiom-enter-badge" aria-hidden="true">↵ Enter</span>
              </button>

              <button
                type="button"
                className="axiom-btn axiom-btn-outline"
                disabled={busy}
                onClick={handleNextProblem}
                title="Generate another deterministic problem instance"
              >
                <RefreshCw size={14} />
                <span>{workoutActive ? (workoutIndex >= 5 ? 'Finish workout' : 'Next problem') : 'New example'}</span>
              </button>
            </div>
          </form>

          {/* Working notes: a collapsed scratchpad keeps one task in focus while holding working memory. */}
          <details className="axiom-scratchpad">
            <summary>Working notes</summary>
            <label htmlFor="practice-scratchpad">Working notes (kept on this device; never graded)</label>
            <textarea
              id="practice-scratchpad"
              className="axiom-text-input"
              rows={3}
              maxLength={4000}
              value={scratch}
              onChange={(e) => setScratch(e.target.value)}
              placeholder="Jot steps, patterns, or questions here. Notes never affect checking or rewards."
            />
          </details>

          {/* Feedback & Award Display */}
          {outcome && (
            <div role="status" className={`axiom-feedback-banner ${outcome.disposition}`}>
              <div className="axiom-feedback-header">
                {outcome.disposition === 'correct' ? (
                  <CheckCircle2 size={20} className="axiom-success-icon" />
                ) : outcome.disposition === 'incorrect' ? (
                  <XCircle size={20} className="axiom-error-icon" />
                ) : (
                  <AlertCircle size={20} className="axiom-warn-icon" />
                )}
                <h4>
                  {outcome.disposition === 'correct'
                    ? 'Answer checked locally'
                    : outcome.disposition === 'incorrect'
                    ? 'Try another approach'
                    : 'Check how your answer is written'}
                </h4>
              </div>

              <Encouragement outcome={outcome} mistakes={mistakes} attempts={attempts} assisted={assisted} />
              <p className="axiom-feedback-text">
                {outcome.disposition === 'incorrect'
                  ? 'This answer does not match yet. Use the hint below to choose a next step, then check your revised answer.'
                  : outcome.feedback}
              </p>

              {lastAward && (
                <div className="axiom-award-pill">
                  <Award size={16} aria-hidden="true" />
                  <span>+{lastAward} practice XP</span>
                </div>
              )}

              {/* Repair workflow if misconception detected */}
              {repairNeeded && (
                <div className="axiom-repair-suggestion">
                  <div className="axiom-repair-text">
                    <Lightbulb size={18} />
                    <span>
                      <strong>A useful next step:</strong> Practice finding equivalent fractions with common denominators before re-attempting comparisons.
                    </span>
                  </div>
                  <button className="axiom-btn axiom-btn-secondary" onClick={handleStartRepair}>
                    <span>Practice equivalent fractions</span>
                    <ArrowRight size={14} />
                  </button>
                </div>
              )}
            </div>
          )}
          <FeedbackSupport
            key={problem.problem_hash}
            problem={problem}
            afterMistake={outcome?.disposition === 'incorrect'}
            onSupport={() => { if (outcome?.disposition !== 'correct') setAssisted(true); }}
          />

          {/* Self-Explanation Reflection */}
          <div className="axiom-practice-reflection">
            <label className="axiom-reflection-label" htmlFor="practice-explanation">
              Explain your thinking (saved on this device)
            </label>
            <textarea
              id="practice-explanation"
              className="axiom-text-input"
              rows={2}
              maxLength={8000}
              value={explanation}
              onChange={(e) => {
                setExplanation(e.target.value);
                try { localStorage.setItem(`axiom_practice_explanation:${templateId}`, e.target.value); } catch {}
              }}
              placeholder="Why does your answer work? What mathematical rule or property did you use?"
            />
            <p className="axiom-subtext">Putting your reasoning into words deepens understanding and builds lasting confidence.</p>
          </div>
        </div>
      )}
    </div>
  );
};
