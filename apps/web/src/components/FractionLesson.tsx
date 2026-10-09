import React, { useState } from 'react';
import { MathView } from './MathView';
import { ArrowRight, HelpCircle, CheckCircle2, AlertTriangle, Lightbulb, BookOpen } from 'lucide-react';

interface FractionLessonProps {
  onStartPractice: () => void;
}

export const FractionLesson: React.FC<FractionLessonProps> = ({ onStartPractice }) => {
  const [prediction, setPrediction] = useState<string | null>(null);
  const [showExplanation, setShowExplanation] = useState(false);
  const [showVisualAlign, setShowVisualAlign] = useState(false);

  const handlePredict = (choice: string) => {
    setPrediction(choice);
    setShowExplanation(true);
  };

  return (
    <div className="axiom-lesson-container">
      <div className="axiom-lesson-header">
        <span className="axiom-tag">Course F04 • Unit: Fractions</span>
        <h2>Comparing Fractions: The Common Denominator</h2>
        <p className="axiom-lead">
          Why comparing fractions requires equal-sized units, and how to find them using rational equivalence.
        </p>
      </div>

      {/* 1. Puzzle / Intelligible Question */}
      <section className="axiom-section axiom-card">
        <div className="axiom-section-title">
          <HelpCircle size={18} className="axiom-accent" />
          <h3>1. The Purpose Challenge</h3>
        </div>
        <p>
          Imagine two identical graduated cylinders in a chemistry laboratory. Cylinder A contains{' '}
          <MathView math="\frac{3}{4}" /> liter of liquid. Cylinder B contains{' '}
          <MathView math="\frac{5}{8}" /> liter.
        </p>
        <p>
          <strong>Which cylinder contains more liquid?</strong>
        </p>

        {/* 2. Predict before calculating */}
        <div className="axiom-interactive-box">
          <h4>Test Your Intuition (Make a Prediction):</h4>
          <p className="axiom-subtext">It is completely normal to be uncertain — reasoning begins by confronting our assumptions.</p>
          <div className="axiom-btn-group">
            <button
              className={`axiom-btn ${prediction === 'less' ? 'axiom-btn-selected' : 'axiom-btn-outline'}`}
              onClick={() => handlePredict('less')}
            >
              <MathView math="\frac{3}{4} < \frac{5}{8}" />
            </button>
            <button
              className={`axiom-btn ${prediction === 'equal' ? 'axiom-btn-selected' : 'axiom-btn-outline'}`}
              onClick={() => handlePredict('equal')}
            >
              <MathView math="\frac{3}{4} = \frac{5}{8}" />
            </button>
            <button
              className={`axiom-btn ${prediction === 'greater' ? 'axiom-btn-selected' : 'axiom-btn-outline'}`}
              onClick={() => handlePredict('greater')}
            >
              <MathView math="\frac{3}{4} > \frac{5}{8}" />
            </button>
          </div>

          {showExplanation && (
            <div className="axiom-prediction-feedback">
              {prediction === 'greater' ? (
                <div className="axiom-feedback-positive">
                  <CheckCircle2 size={18} />
                  <span><strong>Spot on!</strong> Indeed, <MathView math="\frac{3}{4}" /> is strictly greater than <MathView math="\frac{5}{8}" />. Let's see the mathematical justification.</span>
                </div>
              ) : (
                <div className="axiom-feedback-instructive">
                  <Lightbulb size={18} />
                  <span><strong>A common intuitive guess!</strong> While 5 is bigger than 3, each eighth is smaller than each fourth. Let's explore why below.</span>
                </div>
              )}
            </div>
          )}
        </div>
      </section>

      {/* 3. Definition & Domain */}
      <section className="axiom-section axiom-card">
        <div className="axiom-section-title">
          <BookOpen size={18} className="axiom-accent" />
          <h3>2. Mathematical Definition of a Fraction</h3>
        </div>
        <p>
          A positive rational number is written as <MathView math="\frac{a}{b}" /> where <MathView math="a, b \in \mathbb{Z}^+" /> and <MathView math="b \neq 0" />:
        </p>
        <div className="axiom-callout">
          <ul>
            <li><strong>Denominator (<MathView math="b" />):</strong> Divides the whole unit into <MathView math="b" /> equal parts. <em>The larger <MathView math="b" /> is, the smaller each individual piece becomes.</em></li>
            <li><strong>Numerator (<MathView math="a" />):</strong> Selects <MathView math="a" /> of those equal pieces.</li>
          </ul>
        </div>
      </section>

      {/* 4. Concrete visual number line */}
      <section className="axiom-section axiom-card">
        <div className="axiom-section-title">
          <Lightbulb size={18} className="axiom-accent" />
          <h3>3. Visual Number Line Alignment</h3>
        </div>
        <p>
          Observe the interval from <MathView math="0" /> to <MathView math="1" /> partitioned into fourths and eighths:
        </p>

        <div className="axiom-numberline-container">
          <div className="axiom-nl-track">
            <div className="axiom-nl-label">Fourths (<MathView math="\frac{1}{4}" /> per step):</div>
            <div className="axiom-nl-bar">
              <div className="axiom-nl-segment quarter" style={{ width: '75%' }}>
                <span className="axiom-nl-mark">3/4 (75%)</span>
              </div>
            </div>
          </div>

          <div className="axiom-nl-track">
            <div className="axiom-nl-label">Eighths (<MathView math="\frac{1}{8}" /> per step):</div>
            <div className="axiom-nl-bar">
              <div className="axiom-nl-segment eighth" style={{ width: '62.5%' }}>
                <span className="axiom-nl-mark">5/8 (62.5%)</span>
              </div>
            </div>
          </div>
        </div>

        <button
          className="axiom-btn axiom-btn-outline"
          onClick={() => setShowVisualAlign(!showVisualAlign)}
        >
          {showVisualAlign ? "Hide partition subdivision" : "Show: Subdivide each fourth into 2 eighths"}
        </button>

        {showVisualAlign && (
          <div className="axiom-subdivision-visual">
            <p className="axiom-note">
              Since <MathView math="\frac{1}{4} = \frac{2}{8}" />, exactly 3 fourths equals <MathView math="3 \times 2 = 6" /> eighths!
            </p>
          </div>
        )}
      </section>

      {/* 5. Worked Example & Common Denominator */}
      <section className="axiom-section axiom-card">
        <div className="axiom-section-title">
          <CheckCircle2 size={18} className="axiom-accent" />
          <h3>4. Step-by-Step Derivation</h3>
        </div>
        <div className="axiom-steps-list">
          <div className="axiom-step">
            <span className="axiom-step-num">Step 1</span>
            <div>
              <strong>Find a Common Denominator:</strong> The least common multiple of 4 and 8 is 8 (<MathView math="\text{lcm}(4, 8) = 8" />).
            </div>
          </div>
          <div className="axiom-step">
            <span className="axiom-step-num">Step 2</span>
            <div>
              <strong>Convert by Multiplying by 1:</strong> Scale <MathView math="\frac{3}{4}" /> by <MathView math="\frac{2}{2}" />:
              <div className="axiom-math-display">
                <MathView math="\frac{3}{4} = \frac{3 \times 2}{4 \times 2} = \frac{6}{8}" display />
              </div>
            </div>
          </div>
          <div className="axiom-step">
            <span className="axiom-step-num">Step 3</span>
            <div>
              <strong>Compare Numerators Over the Common Unit:</strong>
              <div className="axiom-math-display">
                <MathView math="\frac{6}{8} > \frac{5}{8} \implies \frac{3}{4} > \frac{5}{8}" display />
              </div>
            </div>
          </div>
        </div>
      </section>

      {/* 6. Non-Example / Misconception Challenge */}
      <section className="axiom-section axiom-card axiom-card-warning">
        <div className="axiom-section-title">
          <AlertTriangle size={18} className="axiom-warning" />
          <h3>5. The False-Heuristic Trap</h3>
        </div>
        <p>
          <strong>Common Mistake:</strong> <em>"5 is greater than 3, and 8 is greater than 4, so 5/8 must be larger than 3/4."</em>
        </p>
        <p>
          <strong>Why this fails:</strong> The fraction <MathView math="\frac{5}{8}" /> has more pieces, but each eighth is smaller (<MathView math="\frac{1}{8} < \frac{1}{4}" />). Numerator comparison is valid <strong>only when the denominators are identical</strong>!
        </p>
      </section>

      {/* CTA to Practice Workspace */}
      <div className="axiom-lesson-footer">
        <button className="axiom-btn axiom-btn-primary axiom-btn-lg" onClick={onStartPractice}>
          <span>Ready to Practice: Compare Unseen Fractions</span>
          <ArrowRight size={18} />
        </button>
      </div>
    </div>
  );
};
