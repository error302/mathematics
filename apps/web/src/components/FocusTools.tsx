import React, { useEffect, useRef, useState } from 'react';
import { Timer, Play, Pause, X, Eye, EyeOff, Wind } from 'lucide-react';
import './focusTools.css';

interface FocusToolsProps {
  focusView: boolean;
  onToggleFocusView: () => void;
}

const DURATION_CHOICES = [10, 20, 30, 45];

const formatTime = (totalSeconds: number): string => {
  const minutes = Math.floor(totalSeconds / 60);
  const seconds = totalSeconds % 60;
  return `${minutes}:${String(seconds).padStart(2, '0')}`;
};

/**
 * Honest, opt-in focus supports (ADR-0006).
 * Nothing here auto-starts, interrupts typing, or claims a medical/ADHD effect.
 * Timers are learner-chosen preferences, never requirements or pressure.
 */
export const FocusTools: React.FC<FocusToolsProps> = ({ focusView, onToggleFocusView }) => {
  const [selectedMinutes, setSelectedMinutes] = useState(20);
  const [remainingSeconds, setRemainingSeconds] = useState<number | null>(null);
  const [paused, setPaused] = useState(false);
  const [sessionFinished, setSessionFinished] = useState(false);
  const [breathOpen, setBreathOpen] = useState(false);
  const [breath, setBreath] = useState<{ phase: 'in' | 'out'; seconds: number }>({ phase: 'in', seconds: 4 });
  const deadlineRef = useRef<number | null>(null);
  const pausedAtRef = useRef<number | null>(null);
  const closePauseRef = useRef<HTMLButtonElement | null>(null);

  const counting = remainingSeconds !== null && !paused && !sessionFinished;

  useEffect(() => {
    if (!counting || deadlineRef.current === null) return;
    const interval = window.setInterval(() => {
      if (deadlineRef.current === null) return;
      const left = Math.max(0, Math.ceil((deadlineRef.current - Date.now()) / 1000));
      setRemainingSeconds(left);
      if (left <= 0) {
        deadlineRef.current = null;
        setSessionFinished(true);
      }
    }, 250);
    return () => window.clearInterval(interval);
  }, [counting]);

  useEffect(() => {
    if (!breathOpen) return;
    setBreath({ phase: 'in', seconds: 4 });
    const interval = window.setInterval(() => {
      setBreath((current) => {
        if (current.seconds > 1) return { phase: current.phase, seconds: current.seconds - 1 };
        return current.phase === 'in' ? { phase: 'out', seconds: 6 } : { phase: 'in', seconds: 4 };
      });
    }, 1000);
    return () => window.clearInterval(interval);
  }, [breathOpen]);

  useEffect(() => {
    if (breathOpen) closePauseRef.current?.focus();
  }, [breathOpen]);

  const startSession = () => {
    deadlineRef.current = Date.now() + selectedMinutes * 60 * 1000;
    pausedAtRef.current = null;
    setRemainingSeconds(selectedMinutes * 60);
    setPaused(false);
    setSessionFinished(false);
  };

  const pauseSession = () => {
    pausedAtRef.current = Date.now();
    setPaused(true);
  };

  const resumeSession = () => {
    if (deadlineRef.current !== null && pausedAtRef.current !== null) {
      deadlineRef.current += Date.now() - pausedAtRef.current;
    }
    pausedAtRef.current = null;
    setPaused(false);
  };

  const endSession = () => {
    deadlineRef.current = null;
    pausedAtRef.current = null;
    setRemainingSeconds(null);
    setPaused(false);
    setSessionFinished(false);
  };

  return (
    <section className="axiom-focus-bar" aria-label="Study focus tools">
      <div className="axiom-focus-inner">
        <button
          type="button"
          className="axiom-focus-btn"
          aria-pressed={focusView}
          onClick={onToggleFocusView}
          title="Focus view hides XP details, the page footer and decorative highlights while you study"
        >
          {focusView ? <EyeOff size={14} aria-hidden="true" /> : <Eye size={14} aria-hidden="true" />}
          <span>Focus view{focusView ? ': on' : ''}</span>
        </button>

        {remainingSeconds === null && !sessionFinished && (
          <div className="axiom-focus-timer-setup">
            <label htmlFor="focus-duration-select">Focus session</label>
            <select
              id="focus-duration-select"
              className="axiom-select"
              value={selectedMinutes}
              onChange={(event) => setSelectedMinutes(Number(event.target.value))}
            >
              {DURATION_CHOICES.map((minutes) => (
                <option key={minutes} value={minutes}>{minutes} minutes</option>
              ))}
            </select>
            <button type="button" className="axiom-focus-btn" onClick={startSession}>
              <Play size={13} aria-hidden="true" />
              <span>Start session</span>
            </button>
          </div>
        )}

        {remainingSeconds !== null && !sessionFinished && (
          <div className="axiom-focus-timer-running" role="timer" aria-label="Focus session remaining">
            <Timer size={14} aria-hidden="true" />
            <span className="axiom-focus-time">{formatTime(remainingSeconds)}</span>
            {paused ? (
              <button type="button" className="axiom-focus-btn" onClick={resumeSession}>
                <Play size={13} aria-hidden="true" />
                <span>Resume</span>
              </button>
            ) : (
              <button type="button" className="axiom-focus-btn" onClick={pauseSession}>
                <Pause size={13} aria-hidden="true" />
                <span>Pause</span>
              </button>
            )}
            <button type="button" className="axiom-focus-btn" onClick={endSession}>End session</button>
          </div>
        )}

        {sessionFinished && (
          <div className="axiom-focus-finished" role="status">
            <span>Session finished. A short break may help, or keep going—either is fine.</span>
            <button type="button" className="axiom-focus-btn" onClick={endSession}>Keep going</button>
            <button type="button" className="axiom-focus-btn" onClick={startSession}>
              Start another {selectedMinutes}-minute session
            </button>
          </div>
        )}

        <button type="button" className="axiom-focus-btn" onClick={() => setBreathOpen(true)}>
          <Wind size={14} aria-hidden="true" />
          <span>Take a pause</span>
        </button>
      </div>
      <p className="axiom-focus-boundary">
        Optional study tools. A session length is your choice, never a requirement, and nothing here interrupts your work.
      </p>

      {breathOpen && (
        <div
          className="axiom-focus-overlay"
          role="dialog"
          aria-modal="true"
          aria-labelledby="focus-pause-title"
          aria-describedby="focus-pause-note"
          onKeyDown={(event) => { if (event.key === 'Escape') setBreathOpen(false); }}
        >
          <div className="axiom-focus-pause">
            <div className="axiom-focus-pause-header">
              <h3 id="focus-pause-title">Take a pause</h3>
              <button
                type="button"
                ref={closePauseRef}
                className="axiom-focus-btn"
                onClick={() => setBreathOpen(false)}
                aria-label="Close the pause panel"
              >
                <X size={16} aria-hidden="true" />
                <span>Close</span>
              </button>
            </div>
            <p id="focus-pause-note">
              Some people reset with a slow breath or a stretch before the next question. This is an everyday pause
              habit—not medical advice, treatment, or ADHD care—and it is never required before practice.
            </p>
            <p className="axiom-focus-breath-guide" aria-hidden="true">
              {breath.phase === 'in' ? 'Breathe in…' : 'Breathe out…'} {breath.seconds}
            </p>
            <p className="axiom-sr-only">Text pace: breathe in for four seconds, then out for six seconds, and repeat as you like.</p>
            <div className="axiom-focus-pause-footer">
              <button type="button" className="axiom-btn axiom-btn-primary" onClick={() => setBreathOpen(false)}>
                Back to my practice
              </button>
            </div>
          </div>
        </div>
      )}
    </section>
  );
};