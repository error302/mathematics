import React, { useState } from 'react';
import { createAbacus, computeAbacusValue, describeAbacus, applyAbacusAction, AbacusState } from '../engine/axiomCore';
import { RotateCcw, Volume2, CheckCircle2, Sparkles } from 'lucide-react';

export const AbacusPlayground: React.FC = () => {
  const [abacus, setAbacus] = useState<AbacusState>(() => createAbacus(5, 0));
  const [selectedRod, setSelectedRod] = useState<number>(4); // Default to units rod (rightmost)
  const [challengeTarget] = useState<number>(15);

  const value = computeAbacusValue(abacus);
  const speechText = describeAbacus(abacus);
  const numValue = Number(value.n);
  const isChallengeComplete = numValue === challengeTarget;

  // Toggle upper bead on rod
  const toggleUpper = (rodIndex: number) => {
    setAbacus(prev => applyAbacusAction(prev, { type: 'toggle_upper', rod_index: rodIndex }));
    setSelectedRod(rodIndex);
  };

  // Set lower beads on rod
  const setLower = (rodIndex: number, count: number) => {
    setAbacus(prev => applyAbacusAction(prev, { type: 'set_lower', rod_index: rodIndex, count }));
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
    if (e.target !== e.currentTarget) return;
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
    <div className="axiom-abacus-studio" role="group" aria-label="Soroban keyboard workspace" onKeyDown={handleKeyDown} tabIndex={0}>
      <div className="axiom-studio-header">
        <div>
          <span className="axiom-tag">Soroban · a Japanese abacus</span>
          <h2>See numbers on the soroban</h2>
          <p className="axiom-subtext">
            Move beads toward the middle bar to count them. Each upper bead counts as five; each lower bead counts as one.
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
          <span>Try setting the number <strong>{challengeTarget}</strong></span>
        </div>
        {isChallengeComplete ? (
          <div className="axiom-challenge-success">
            <CheckCircle2 size={18} />
            <span>You’ve made 15. The Tens rod has 1 lower bead engaged, Ones rod has 1 upper bead engaged (10 + 5 = 15).</span>
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
                onKeyDown={(event) => { if (event.key === 'Enter' || event.key === ' ') { event.preventDefault(); event.stopPropagation(); toggleUpper(idx); } }}
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
      <div className="axiom-bead-accessible-controls">
        {abacus.rods.map((rod, index) => <label key={index}>
          {rodPlaceNames[index]} lower beads
          <select className="axiom-select" aria-label={`Lower beads on ${rodPlaceNames[index]}`} value={rod.lower} onChange={event => setLower(index, Number(event.target.value))}>
            {[0, 1, 2, 3, 4].map(count => <option key={count} value={count}>{count} engaged</option>)}
          </select>
        </label>)}
      </div>
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
