import React, { useState, useEffect } from 'react';
import { ConceptVisual } from './ConceptVisual';
import {JourneyLessonActions} from './StudyJourney';
import {getStudyCatalog,applyStudyAction} from '../engine/axiomCore';
import { ArrowRight, HelpCircle, CheckCircle2, AlertTriangle, Lightbulb, BookOpen, Layers } from 'lucide-react';

export interface LessonDef {
  id: string;
  courseId: string;
  title: string;
  subtitle: string;
  templateId: string;
  challenge: string;
  predictionPrompt: string;
  predictionOptions: { label: string; value: string; isCorrect?: boolean }[];
  formalDefinition: string;
  workedExample: {
    steps: string[];
    result: string;
  };
  misconception: {
    mistake: string;
    correction: string;
  };
  reflection: string;
  prerequisiteCheck: string;
  representation: string;
  faded: { prompt: string; solution: string };
  independent: { prompt: string; solution: string }[];
  explanation: string;
  transfer: { prompt: string; solution: string };
  retrieval: string;
  reviewNote: string;
}

// Versioned content, not a second hard-coded curriculum in the UI.
const lessonFiles = import.meta.glob<LessonDef>('../../../../content/lessons/*/*/lesson.json', { eager: true, import: 'default' });
export const LESSONS: LessonDef[] = Object.values(lessonFiles).sort((left, right) => left.courseId.localeCompare(right.courseId) || left.title.localeCompare(right.title));

interface LessonViewerProps {
  onStartPractice: (templateId: string) => void;
  initialLessonId?: string;
}

export const LessonViewer: React.FC<LessonViewerProps> = ({
  onStartPractice,
  initialLessonId = "foundation.fractions.compare",
}) => {
  const [selectedId, setSelectedId] = useState<string>(initialLessonId);
  const [prediction, setPrediction] = useState<string | null>(null);
  const [showExplanation, setShowExplanation] = useState<boolean>(false);

  const [predictionMistakes,setPredictionMistakes]=useState(0);
  const [noteEncouragement,setNoteEncouragement]=useState(false);
  const [reasoningDraft, setReasoningDraft] = useState('');
  useEffect(() => { setSelectedId(initialLessonId); }, [initialLessonId]);
  useEffect(() => { setReasoningDraft(localStorage.getItem(`axiom_lesson_reasoning:${selectedId}`) || ''); }, [selectedId]);

  const lesson = LESSONS.find((l) => l.id === selectedId) || LESSONS[0];

  const handleSelectLesson = (id: string) => {
    setPredictionMistakes(0);setNoteEncouragement(false);
    setSelectedId(id);
    setPrediction(null);
    setShowExplanation(false);
  };

  const handlePredict = (value: string) => {
    if(!lesson.predictionOptions.find(option=>option.value===value)?.isCorrect)setPredictionMistakes(count=>count+1);
    setPrediction(value);
    setShowExplanation(true);
  };

  return (
    <div className="axiom-lesson-container">
      {/* Lesson Selector Bar */}
      <div className="axiom-lesson-nav-bar">
        <div className="axiom-lesson-nav-header">
          <BookOpen size={18} className="axiom-accent" />
          <span>Choose your lesson</span>
        </div>
        <select
          aria-label="Choose a lesson"
          value={lesson.id}
          onChange={(e) => handleSelectLesson(e.target.value)}
          className="axiom-select axiom-select-wide"
        >
          {LESSONS.map((l) => (
            <option key={l.id} value={l.id}>
              [{l.courseId}] {l.title}
            </option>
          ))}
        </select>
      </div>

      <div className="axiom-lesson-header">
        <span className="axiom-tag">Course {lesson.courseId} • Draft pilot lesson</span>
        <h2>{lesson.title}</h2>
        <p className="axiom-lead">{lesson.subtitle}</p>
        <p className="axiom-draft-note">{lesson.reviewNote}</p>
      </div>

      {/* 1. The Challenge */}
      <section className="axiom-section axiom-card">
        <div className="axiom-section-title">
          <HelpCircle size={18} className="axiom-accent" />
          <h3>1. Start with a question</h3>
        </div>
        <p className="axiom-challenge-text">{lesson.challenge}</p>

        {/* 2. Predict before calculating */}
        <div className="axiom-interactive-box">
          <h4>2. Make a prediction</h4>
          <p className="axiom-subtext">{lesson.predictionPrompt}</p>
          <div className="axiom-btn-group">
            {lesson.predictionOptions.map((opt) => (
              <button
                key={opt.value}
                className={`axiom-btn ${prediction === opt.value ? 'axiom-btn-selected' : 'axiom-btn-outline'}`}
                onClick={() => handlePredict(opt.value)}
                aria-pressed={prediction === opt.value}
              >
                <span>{opt.label}</span>
              </button>
            ))}
          </div>

          {showExplanation && (
            <div className="axiom-feedback-reveal" role="status">
              {lesson.predictionOptions.find(option=>option.value===prediction)?.isCorrect?<CheckCircle2 size={16} className="axiom-success"/>:<Lightbulb size={16} className="axiom-accent"/>}
              <span>
                {lesson.predictionOptions.find(option => option.value === prediction)?.isCorrect
                  ? predictionMistakes>0?'Nice recovery. You revised your prediction—now explain why it works.':'Good thinking. Your prediction agrees with the example. Read the explanation to understand why.'
                  : 'Good try. This prediction gives you a starting point. Compare the picture and explanation, then try again.'}
              </span>
            </div>
          )}
          {showExplanation && !lesson.predictionOptions.find(option=>option.value===prediction)?.isCorrect && <div className="axiom-prediction-hint"><p><strong>A useful hint:</strong> {lesson.representation}</p><button className="axiom-text-link" onClick={()=>document.getElementById('lesson-worked-example')?.scrollIntoView({block:'start'})}>Show me the worked example</button></div>}
        </div>
      </section>

      {/* 3. Formal Definition */}
      <section className="axiom-section axiom-card">
        <div className="axiom-section-title">
          <BookOpen size={18} className="axiom-accent" />
          <h3>3. Understand the idea</h3>
        </div>
        <p>{lesson.formalDefinition}</p>
        <h4>Check your starting knowledge</h4><p>{lesson.prerequisiteCheck}</p>
        <h4>Picture the idea</h4><p>{lesson.representation}</p>
        <ConceptVisual key={lesson.id} lessonId={lesson.id}/>
      </section>

      {/* 4. Worked Example */}
      <section id="lesson-worked-example" className="axiom-section axiom-card">
        <div className="axiom-section-title">
          <Lightbulb size={18} className="axiom-accent" />
          <h3>4. Follow a worked example</h3>
        </div>
        <div className="axiom-steps-list">
          {lesson.workedExample.steps.map((st, i) => (
            <div key={i} className="axiom-step-item">
              <span className="axiom-step-num">{i + 1}</span>
              <p>{st}</p>
            </div>
          ))}
        </div>
        <div className="axiom-result-box">
          <strong>Conclusion:</strong> {lesson.workedExample.result}
        </div>
      </section>

      {/* 5. Misconception Trap */}
      <section className="axiom-section axiom-card axiom-card-warning">
        <div className="axiom-section-title">
          <AlertTriangle size={18} className="axiom-warning" />
          <h3>5. Check a common mistake</h3>
        </div>
        <div className="axiom-misconception-content">
          <p className="axiom-error-quote"><em>"{lesson.misconception.mistake}"</em></p>
          <p className="axiom-correct-explanation">
            <strong>Why it works:</strong> {lesson.misconception.correction}
          </p>
        </div>
      </section>

      <section className="axiom-section axiom-card">
        <h3>6. Complete a partly worked problem</h3>
        <p>{lesson.faded.prompt}</p>
        <details><summary>Show the self-check after trying</summary><p>{lesson.faded.solution}</p><p className="axiom-supported-note">Good choice checking your work. Compare your reasoning with the explanation, then try the next question.</p></details>
      </section>
      <section className="axiom-section axiom-card">
        <h3>7. Try independently</h3>
        {lesson.independent.map((activity, index) => <div key={index}>
          <p>{index + 1}. {activity.prompt}</p>
          <details><summary>Compare with a solution</summary><p>{activity.solution}</p><p className="axiom-supported-note">Good choice checking your work. Compare your reasoning with the explanation, then try the next question.</p></details>
        </div>)}
        <p>These self-checks help you learn; they do not establish independently reviewed mastery.</p>
      </section>
      <section className="axiom-section axiom-card">
        <h3>8. Explain the reasoning</h3><p>{lesson.explanation}</p>
        <label htmlFor="lesson-reasoning">Your explanation (saved on this device)</label>
        <textarea id="lesson-reasoning" className="axiom-text-input" rows={5} maxLength={16000} value={reasoningDraft} onBlur={()=>setNoteEncouragement(Boolean(reasoningDraft.trim()))}
          onChange={event => { setNoteEncouragement(false); setReasoningDraft(event.target.value); localStorage.setItem(`axiom_lesson_reasoning:${selectedId}`, event.target.value); }} />
        {noteEncouragement && <p className="axiom-note-encouragement" role="status">You’ve recorded your thinking. Good work putting the idea into words—compare it with the explanation and refine it if needed.</p>}
      </section>
      <section className="axiom-section axiom-card">
        <h3>9. Transfer to a new situation</h3><p>{lesson.transfer.prompt}</p>
        <details><summary>Show the explanation after attempting</summary><p>{lesson.transfer.solution}</p><p className="axiom-supported-note">Good choice checking your work. Compare your reasoning with the explanation, then try the next question.</p></details>
      </section>
      <section className="axiom-section axiom-card">
        <h3>10. Retrieve after a delay</h3><p>{lesson.retrieval}</p>
        <p>These are study prompts you can use tomorrow and next week. Automatic review scheduling and trusted retention assessments are still being built.</p>
      </section>
      {/* 6. Reflection & Action */}
      <section className="axiom-section axiom-card">
        <div className="axiom-section-title">
          <Layers size={18} className="axiom-accent" />
          <h3>Reflection and next practice</h3>
        </div>
        <p>{lesson.reflection}</p>
        <div className="axiom-lesson-actions">
          <button className="axiom-btn axiom-btn-outline" onClick={()=>onStartPractice('quest:reasoning.analysis')}>Apply it in a reasoning challenge</button>
          <button
            className="axiom-btn axiom-btn-primary axiom-btn-large"
            onClick={() => {const step=getStudyCatalog().find(s=>s.id===lesson.id);if(step){try{applyStudyAction({kind:'open',step_id:step.id})}catch{}}onStartPractice(step?.template_id||lesson.templateId)}}
          >
            <span>Practice this idea</span>
            <ArrowRight size={18} />
          </button>
        </div>
      </section>
      <JourneyLessonActions lessonId={lesson.id} notes={reasoningDraft} onPractice={onStartPractice}/>
    </div>
  );
};
