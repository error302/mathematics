import React, { useState, useEffect } from 'react';
import { MathView } from './MathView';
import { generateExercise, checkAnswer, ProblemArtifact, GradeOutcome } from '../engine/axiomCore';
import { CheckCircle2, XCircle, AlertCircle, RefreshCw, ArrowRight, Award, Lightbulb } from 'lucide-react';

interface PracticeWorkspaceProps {
  onAwardXp: (instanceId: string) => { delta: number; reason: string } | null;
  onRecordMasteryEvidence: (family: string, success: boolean) => void;
  initialTemplateId?: string;
}

export const PracticeWorkspace: React.FC<PracticeWorkspaceProps> = ({
  onAwardXp,
  onRecordMasteryEvidence,
  initialTemplateId = 'fractions.compare.positive',
}) => {
  const [templateId, setTemplateId] = useState<string>(initialTemplateId);
  const [problem, setProblem] = useState<ProblemArtifact | null>(null);
  const [selectedOption, setSelectedOption] = useState<string>('');
  const [textInput, setTextInput] = useState<string>('');
  const [outcome, setOutcome] = useState<GradeOutcome | null>(null);
  const [seedCounter, setSeedCounter] = useState<number>(1);
  const [repairNeeded, setRepairNeeded] = useState<boolean>(false);
  const [lastAward, setLastAward] = useState<number | null>(null);

  useEffect(() => {
    if (initialTemplateId) {
      setTemplateId(initialTemplateId);
    }
  }, [initialTemplateId]);

  // Generate new problem on template or seed change
  useEffect(() => {
    loadNewProblem(templateId, seedCounter);
  }, [templateId, seedCounter]);

  const loadNewProblem = (tmpl: string, counter: number) => {
    // Generate 64-char hex seed deterministically
    const seedHex = (counter.toString(16).padStart(16, '0') + "abcdef0123456789abcdef0123456789abcdef0123456789").slice(0, 64);
    const p = generateExercise(tmpl, seedHex, 1);
    setProblem(p);
    setSelectedOption('');
    setTextInput('');
    setOutcome(null);
    setLastAward(null);
  };

  const handleSubmit = (e?: React.FormEvent) => {
    if (e) e.preventDefault();
    if (!problem) return;

    const answer = problem.answer_kind === 'relation' ? selectedOption : textInput;
    if (!answer.trim()) return;

    const res = checkAnswer(problem, answer);
    setOutcome(res);

    if (res.disposition === 'correct') {
      onRecordMasteryEvidence(problem.template_id, true);
      const instanceId = `${problem.template_id}:${problem.seed_hex}`;
      const award = onAwardXp(instanceId);
      if (award && award.delta > 0) {
        setLastAward(award.delta);
      }
      setRepairNeeded(false);
    } else if (res.disposition === 'incorrect') {
      onRecordMasteryEvidence(problem.template_id, false);
      if (problem.template_id === 'fractions.compare.positive') {
        setRepairNeeded(true);
      }
    }
  };

  const handleNextProblem = () => {
    setSeedCounter((c) => c + 1);
  };

  const handleStartRepair = () => {
    setTemplateId('fractions.equivalent.find');
    setRepairNeeded(false);
  };

  return (
    <div className="axiom-practice-container">
      <div className="axiom-practice-header">
        <div>
          <h2>Mathematical Practice Workspace</h2>
          <p className="axiom-subtext">Deterministic exercises with exact rational checking and justified feedback.</p>
        </div>

        <div className="axiom-template-selector">
          <label htmlFor="tmpl-select">Exercise Family: </label>
          <select
            id="tmpl-select"
            value={templateId}
            onChange={(e) => setTemplateId(e.target.value)}
            className="axiom-select"
          >
            <option value="fractions.compare.positive">Fraction Comparison (F04)</option>
            <option value="fractions.equivalent.find">Equivalent Fraction Construction (F04 Repair)</option>
            <option value="fractions.unit.identify">Unit Fraction Partitions (F04)</option>
            <option value="place_value.decompose">Base-10 Place Value Decomposition (F01)</option>
            <option value="arithmetic.whole.addition">Whole Number Addition (F02)</option>
            <option value="arithmetic.column.addition">Column Addition with Regrouping (F02)</option>
            <option value="arithmetic.column.subtraction">Column Subtraction with Borrowing (F02)</option>
            <option value="abacus.read.state">Soroban Abacus Rod Reading (A01)</option>
            <option value="abacus.target.setting">Soroban Abacus Target Setting (A01/A02)</option>
          </select>
        </div>
      </div>

      {problem && (
        <div className="axiom-practice-card axiom-card">
          <div className="axiom-exercise-meta">
            <span className="axiom-tag">Template: {problem.template_id}</span>
            <span className="axiom-tag axiom-tag-seed" title={`Seed: ${problem.seed_hex}`}>
              Seed: #{seedCounter}
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
          <form onSubmit={handleSubmit} className="axiom-answer-form">
            {problem.answer_kind === 'relation' && problem.options ? (
              <div className="axiom-options-grid">
                {problem.options.map((opt) => (
                  <button
                    type="button"
                    key={opt}
                    className={`axiom-option-btn ${selectedOption === opt ? 'selected' : ''}`}
                    onClick={() => setSelectedOption(opt)}
                  >
                    <span>
                      {opt === 'less' ? 'is less than (<)' : opt === 'equal' ? 'is equal to (=)' : 'is greater than (>)'}
                    </span>
                  </button>
                ))}
              </div>
            ) : (
              <div className="axiom-input-row">
                <input
                  type="text"
                  className="axiom-text-input"
                  placeholder={problem.answer_kind === 'rational' ? 'e.g. 6/8' : 'Enter number'}
                  value={textInput}
                  onChange={(e) => setTextInput(e.target.value)}
                  autoFocus
                />
              </div>
            )}

            <div className="axiom-action-row">
              <button
                type="submit"
                className="axiom-btn axiom-btn-primary"
                disabled={problem.answer_kind === 'relation' ? !selectedOption : !textInput.trim()}
              >
                Submit Answer
              </button>

              <button
                type="button"
                className="axiom-btn axiom-btn-outline"
                onClick={handleNextProblem}
                title="Generate another deterministic problem instance"
              >
                <RefreshCw size={14} />
                <span>Next Variant</span>
              </button>
            </div>
          </form>

          {/* Feedback & Award Display */}
          {outcome && (
            <div className={`axiom-feedback-banner ${outcome.disposition}`}>
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
                    ? 'Accepted & Verified'
                    : outcome.disposition === 'incorrect'
                    ? 'Mathematical Issue Identified'
                    : 'Input Syntax Notice'}
                </h4>
              </div>

              <p className="axiom-feedback-text">{outcome.feedback}</p>

              {lastAward && (
                <div className="axiom-award-pill">
                  <Award size={16} />
                  <span>+{lastAward} XP awarded to ledger (first unassisted success)!</span>
                </div>
              )}

              {/* Repair workflow if misconception detected */}
              {repairNeeded && (
                <div className="axiom-repair-suggestion">
                  <div className="axiom-repair-text">
                    <Lightbulb size={18} />
                    <span>
                      <strong>Targeted Repair Recommendation:</strong> Practice finding equivalent fractions with common denominators before re-attempting comparisons.
                    </span>
                  </div>
                  <button className="axiom-btn axiom-btn-secondary" onClick={handleStartRepair}>
                    <span>Start Equivalent Fractions Repair</span>
                    <ArrowRight size={14} />
                  </button>
                </div>
              )}
            </div>
          )}
        </div>
      )}
    </div>
  );
};
