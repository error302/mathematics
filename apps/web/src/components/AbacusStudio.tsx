import React, { useState } from 'react';
import { createAbacus, computeAbacusValue, describeAbacus, AbacusState } from '../engine/axiomCore';
import { RotateCcw, Volume2, CheckCircle2, Sparkles } from 'lucide-react';

export const AbacusStudio: React.FC = () => {
  const [abacus, setAbacus] = useState<AbacusState>(() => createAbacus(5, 0));
  const [selectedRod, setSelectedRod] = useState<number>(4); // Default to units rod (rightmost)
  const [challengeTarget] = useState<number>(15);

  const value = computeAbacusValue(abacus);
  const speechText = describeAbacus(abacus);
  const numValue = Number(value.n);
  const isChallengeComplete = numValue === challengeTarget;

  // Toggle upper bead on rod
  const toggleUpper = (rodIndex: number) => {
    setAbacus((prev) => {
      const nextRods = prev.rods.map((r, i) =>
        i === rodIndex ? { ...r, upper: !r.upper } : r
      );
      return { ...prev, rods: nextRods };
    });
    setSelectedRod(rodIndex);
  };

  // Set lower beads on rod
  const setLower = (rodIndex: number, count: number) => {
    setAbacus((prev) => {
      const nextRods = prev.rods.map((r, i) =>
        i === rodIndex ? { ...r, lower: count } : r
      );
      return { ...prev, rods: nextRods };
    });
    setSelectedRod(rodIndex);
  };

  const handleClear = () => {
    setAbacus(createAbacus(5, 0));
  };

  const setTarget15 = () => {
    // 5 rods: Rod 3 (Tens) = 1 lower, Rod 4 (Ones) = 1 upper
    setAbacus({
      schema_version: 1,
      rod_count: 5,
      rightmost_exponent: 0,
      sign: 1,
      rods: [
        { upper: false, lower: 0 },
        { upper: false, lower: 0 },
        { upper: false, lower: 0 },
        { upper: false, lower: 1 }, // Tens = 1
        { upper: true, lower: 0 },  // Ones = 5 (Upper down)
      ],
    });
  };

  // Keyboard navigation handler (Section 10.3 & 25.1)
  const handleKeyDown = (e: React.KeyboardEvent) => {
    if (e.key === 'ArrowLeft') {
      setSelectedRod((r) => Math.max(0, r - 1));
      e.preventDefault();
    } else if (e.key === 'ArrowRight') {
      setSelectedRod((r) => Math.min(abacus.rod_count - 1, r + 1));
      e.preventDefault();
    } else if (e.key === ' ' || e.key === 'ArrowUp' || e.key === 'ArrowDown') {
      toggleUpper(selectedRod);
      e.preventDefault();
    } else if (['0', '1', '2', '3', '4'].includes(e.key)) {
      setLower(selectedRod, parseInt(e.key, 10));
      e.preventDefault();
    } else if (e.key === 'c' || e.key === 'C') {
      handleClear();
      e.preventDefault();
    }
  };

  const rodPlaceNames = ['10,000s', '1,000s', '100s', 'Tens', 'Ones'];

  return (
    <div className="axiom-abacus-studio" onKeyDown={handleKeyDown} tabIndex={0}>
      <div className="axiom-studio-header">
        <div>
          <span className="axiom-tag">Course A01 • Modern Japanese Soroban</span>
          <h2>Virtual Soroban Studio</h2>
          <p className="axiom-subtext">
            Standard 1:4 bead instrument. Upper heavenly bead represents 5; lower earthly beads represent 1 each.
          </p>
        </div>

        <div className="axiom-studio-actions">
          <button className="axiom-btn axiom-btn-outline" onClick={handleClear} title="Reset all rods to zero">
            <RotateCcw size={14} />
            <span>Reset (C)</span>
          </button>
          <button className="axiom-btn axiom-btn-secondary" onClick={setTarget15}>
            <span>Demo 15</span>
          </button>
        </div>
      </div>

      {/* Challenge Card */}
      <div className={`axiom-challenge-card ${isChallengeComplete ? 'complete' : ''}`}>
        <div className="axiom-challenge-header">
          <Sparkles size={16} />
          <span>Practice Objective: Set the number <strong>{challengeTarget}</strong></span>
        </div>
        {isChallengeComplete ? (
          <div className="axiom-challenge-success">
            <CheckCircle2 size={18} />
            <span>Success! Tens rod has 1 lower bead engaged, Ones rod has 1 upper bead engaged (10 + 5 = 15).</span>
          </div>
        ) : (
          <p className="axiom-challenge-hint">
            Hint: Push 1 lower bead up on the <strong>Tens</strong> rod, and push 1 upper bead down on the <strong>Ones</strong> rod.
          </p>
        )}
      </div>

      {/* Live Value Display */}
      <div className="axiom-abacus-value-banner">
        <div className="axiom-value-box">
          <span className="axiom-value-label">Current Value</span>
          <span className="axiom-value-display">{value.toCanonical()}</span>
        </div>
        <div className="axiom-speech-box" aria-live="polite">
          <Volume2 size={16} />
          <span>{speechText}</span>
        </div>
      </div>

      {/* Soroban Physical Frame Graphic */}
      <div className="axiom-soroban-frame" role="region" aria-label="Interactive Soroban Abacus">
        {/* Top Upper Deck */}
        <div className="axiom-deck axiom-upper-deck">
          {abacus.rods.map((rod, idx) => {
            const isSelected = selectedRod === idx;
            return (
              <div
                key={`upper-${idx}`}
                className={`axiom-rod ${isSelected ? 'selected' : ''}`}
                onClick={() => toggleUpper(idx)}
                role="button"
                tabIndex={0}
                aria-label={`${rodPlaceNames[idx]} Upper Bead: ${rod.upper ? 'Engaged (5)' : 'Idle'}`}
              >
                <div className="axiom-rod-wire" />
                <div
                  className={`axiom-bead axiom-upper-bead ${rod.upper ? 'engaged' : 'idle'}`}
                  title={`${rodPlaceNames[idx]} upper bead (${rod.upper ? '5 engaged' : '0'})`}
                />
              </div>
            );
          })}
        </div>

        {/* Reckoning Bar (Beam) */}
        <div className="axiom-reckoning-bar">
          <div className="axiom-beam-line" />
          {abacus.rods.map((_, idx) => (
            <div key={`dot-${idx}`} className={`axiom-beam-marker ${idx === 4 ? 'unit-dot' : ''}`} />
          ))}
        </div>

        {/* Bottom Lower Deck */}
        <div className="axiom-deck axiom-lower-deck">
          {abacus.rods.map((rod, idx) => {
            const isSelected = selectedRod === idx;
            return (
              <div
                key={`lower-${idx}`}
                className={`axiom-rod ${isSelected ? 'selected' : ''}`}
                role="region"
                aria-label={`${rodPlaceNames[idx]} Lower Beads: ${rod.lower} of 4 engaged`}
              >
                <div className="axiom-rod-wire" />
                {/* 4 lower beads */}
                {[0, 1, 2, 3].map((bIndex) => {
                  const isEngaged = bIndex < rod.lower;
                  return (
                    <div
                      key={`bead-${idx}-${bIndex}`}
                      className={`axiom-bead axiom-lower-bead ${isEngaged ? 'engaged' : 'idle'}`}
                      onClick={() => setLower(idx, isEngaged ? bIndex : bIndex + 1)}
                      title={`Click to set lower beads to ${bIndex + 1}`}
                    />
                  );
                })}
              </div>
            );
          })}
        </div>

        {/* Rod Place Labels */}
        <div className="axiom-rod-labels">
          {rodPlaceNames.map((name, idx) => (
            <div
              key={name}
              className={`axiom-rod-label ${selectedRod === idx ? 'selected' : ''}`}
              onClick={() => setSelectedRod(idx)}
            >
              <span>{name}</span>
              <span className="axiom-rod-digit">
                {(abacus.rods[idx].upper ? 5 : 0) + abacus.rods[idx].lower}
              </span>
            </div>
          ))}
        </div>
      </div>

      {/* Keyboard Controls Guide */}
      <div className="axiom-keyboard-guide">
        <h4>Keyboard Shortcuts:</h4>
        <div className="axiom-shortcuts-list">
          <span><kbd>←</kbd> <kbd>→</kbd> Select Rod</span>
          <span><kbd>Space</kbd> or <kbd>↑</kbd> Toggle Upper Bead (5)</span>
          <span><kbd>1</kbd>–<kbd>4</kbd> Set Lower Beads</span>
          <span><kbd>0</kbd> Clear Lower Beads</span>
          <span><kbd>C</kbd> Reset All</span>
        </div>
      </div>
    </div>
  );
};
