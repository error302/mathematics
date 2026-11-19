import React, {useEffect, useMemo, useRef, useState} from 'react';
import {ArrowLeft, ArrowRight, EyeOff} from 'lucide-react';
import {BeadBoard} from './BeadBoard';
import {Encouragement} from './Encouragement';
import {FeedbackSupport} from './FeedbackSupport';
import {AbacusPlayground} from './AbacusPlayground';
import {AbacusState, ProblemArtifact, GradeOutcome, getAbacusTutorLessons, generateExercise, checkAnswer, computeAbacusValue, createAbacus} from '../engine/axiomCore';
import './abacusTutor.css';

export const AbacusStudio: React.FC<{onAwardXp:(id:string)=>{delta:number;reason:string}|null}> = ({onAwardXp}) => {
 const lessons=useMemo(()=>getAbacusTutorLessons(),[]);
 const [stage,setStage]=useState(()=>{try{const saved=localStorage.getItem('axiom_abacus_tutor_stage_v1');return lessons.some(l=>l.id===saved)?saved!:'number'}catch{return 'number'}});
 const lesson=lessons.find(l=>l.id===stage)!;
 const [view,setView]=useState<'learn'|'free'>('learn');
 const [phase,setPhase]=useState<'watch'|'practice'>('watch');
 const [demoIndex,setDemoIndex]=useState(0);
 const [problem,setProblem]=useState<ProblemArtifact|null>(null);
 const [board,setBoard]=useState<AbacusState|null>(null);
 const [answer,setAnswer]=useState('');
 const [support,setSupport]=useState<'full'|'outline'|'hidden'>('full');
 const [assisted,setAssisted]=useState(false);
 const [showHelp,setShowHelp]=useState(false);
 const [showValue,setShowValue]=useState(false);
 const [outcome,setOutcome]=useState<GradeOutcome|null>(null);
 const [busy,setBusy]=useState(false);
 const [error,setError]=useState<string|null>(null);
 const [saveError,setSaveError]=useState(false);
 const [mistakes,setMistakes]=useState(0);
 const [attempts,setAttempts]=useState(0);
 const [award,setAward]=useState<number|null>(null);
 const revision=useRef(0);
 const mental=stage==='mental';

 const fresh=async()=>{
  const id=++revision.current;setBusy(true);setError(null);
  try{
   const seed=Array.from(crypto.getRandomValues(new Uint8Array(32)),b=>b.toString(16).padStart(2,'0')).join('');
   const next=await generateExercise(lesson.template_id,seed);
   if(id!==revision.current)return;
   setMistakes(0);setAttempts(0);setProblem(next);setBoard(next.metadata.initial_state);setAnswer('');setOutcome(null);setAward(null);setAssisted(false);setShowHelp(false);setShowValue(false);setSupport('full');
  }catch(e){if(id===revision.current)setError(String(e))}finally{if(id===revision.current)setBusy(false)}
 };
 useEffect(()=>{
  setMistakes(0);setAttempts(0);setPhase('watch');setDemoIndex(0);setProblem(null);setBoard(null);setOutcome(null);setAward(null);setShowHelp(false);setShowValue(false);setAssisted(false);setError(null);
  let restored=false;
  try{
   localStorage.setItem('axiom_abacus_tutor_stage_v1',stage);
   const raw=localStorage.getItem(`axiom_abacus_tutor_v1:${stage}`);
   const saved=raw && raw.length<150000?JSON.parse(raw):null;
   if(saved?.problem?.replay_identity?.template_id===lesson.template_id && saved.board?.rod_count===saved.problem.metadata.initial_state.rod_count){
    computeAbacusValue(saved.board);setProblem(saved.problem);setBoard(saved.board);setAnswer(typeof saved.answer==='string'?saved.answer:'');setPhase(saved.phase==='practice'?'practice':'watch');setSupport(['full','outline','hidden'].includes(saved.support)?saved.support:'full');setMistakes(Number.isSafeInteger(saved.mistakes)?Math.max(0,Math.min(saved.mistakes,1000)):0);setAttempts(Number.isSafeInteger(saved.attempts)?Math.max(0,Math.min(saved.attempts,1000)):0);setAssisted(Boolean(saved.assisted));setShowHelp(Boolean(saved.showHelp));setShowValue(Boolean(saved.showValue));setDemoIndex(Math.max(0,Math.min(lesson.trace.length-1,Number(saved.demoIndex)||0)));setBusy(false);restored=true;
   }
  }catch{setSaveError(true)}
  if(!restored)void fresh();
  return()=>{revision.current+=1};
 },[stage]);
 useEffect(()=>{
  if(!problem||!board||problem.template_id!==lesson.template_id)return;
  try{localStorage.setItem(`axiom_abacus_tutor_v1:${stage}`,JSON.stringify({problem,board,answer,phase,support,assisted,showHelp,showValue,demoIndex,mistakes,attempts}));setSaveError(false)}catch{setSaveError(true)}
 },[problem,board,answer,phase,support,assisted,showHelp,showValue,demoIndex,stage,mistakes,attempts]);

 const check=async(event:React.FormEvent)=>{
  event.preventDefault();if(!problem||!board||busy)return;
  const id=revision.current;setBusy(true);setError(null);
  try{const result=await checkAnswer(problem,mental?answer:JSON.stringify(board),assisted);if(id===revision.current){setAttempts(value=>value+1);if(result.disposition==='incorrect')setMistakes(value=>value+1);setOutcome(result);if(result.disposition==='correct'&&!assisted){const earned=onAwardXp(`${problem.template_id}:${problem.seed_hex}`);if(earned)setAward(earned.delta)}}}catch(e){if(id===revision.current)setError(String(e))}finally{if(id===revision.current)setBusy(false)}
 };
 const frame=lesson.trace[Math.min(demoIndex,lesson.trace.length-1)];
 const nextLesson=lessons[lessons.findIndex(l=>l.id===stage)+1];
 const stepFocus=()=>document.getElementById('tutor-stage-heading')?.scrollIntoView({block:'start'});
 const changeStage=(id:string)=>{setStage(id);requestAnimationFrame(stepFocus)};

 return <div className="axiom-abacus-studio tutor-academy" role={view==='learn'?'group':undefined} aria-label={view==='learn'?'Soroban keyboard workspace':undefined}>
  <div className="tutor-heading"><span className="axiom-tag">From beads to mental arithmetic</span><h2>Learn it with your hands.<br/>Picture it in your mind.</h2><p>Start with one rod. Watch each move, try it yourself, then gradually practise without seeing the beads.</p></div>
  <div className="tutor-view-switch" role="group" aria-label="Abacus workspace"><button className={`axiom-chip ${view==='learn'?'active':''}`} aria-pressed={view==='learn'} onClick={()=>setView('learn')}>Guided learning</button><button className={`axiom-chip ${view==='free'?'active':''}`} aria-pressed={view==='free'} onClick={()=>setView('free')}>Free exploration</button></div>
  {view==='free'?<AbacusPlayground/>:<>
   <nav className="tutor-path" aria-label="Route to mental arithmetic">{lessons.map((item,index)=><button key={item.id} className={stage===item.id?'current':''} disabled={busy} aria-current={stage===item.id?'step':undefined} onClick={()=>changeStage(item.id)}><span>{index+1}</span>{item.title}</button>)}</nav>
   <section className="tutor-lesson" aria-labelledby="tutor-stage-heading"><p className="tutor-stage-count">Stage {lessons.findIndex(l=>l.id===stage)+1} of {lessons.length}</p><h3 id="tutor-stage-heading">{lesson.title}</h3><p>{lesson.intro}</p>
    <div className="tutor-phase-switch" role="group" aria-label="Learn and practise"><button className="axiom-btn axiom-btn-outline" aria-pressed={phase==='watch'} onClick={()=>setPhase('watch')}>Watch the example</button><button className="axiom-btn axiom-btn-outline" aria-pressed={phase==='practice'} onClick={()=>setPhase('practice')}>Try it yourself</button></div>
    {saveError && <p role="status">Your browser could not save this practice. You can continue, but this attempt may not survive a reload.</p>}
    {error && <p role="alert">{error}<button className="axiom-text-link" onClick={()=>void fresh()}>Try loading another example</button></p>}
    {phase==='watch'?<div className="tutor-demonstration">
     <div><p className="tutor-example-equation">{lesson.equation}</p><BeadBoard state={frame.state} highlightRod={frame.highlight_rod}/></div>
     <div className="tutor-coach"><p className="tutor-stage-count">Move {demoIndex+1} of {lesson.trace.length}</p><p className="tutor-move-caption" role="status">{frame.caption}</p><p className="tutor-coach-note">Gold beads touch the bar and count. Beads away from the bar do not count. Follow the highlighted rod.</p>
      <div className="tutor-move-buttons"><button className="axiom-btn axiom-btn-outline" disabled={demoIndex===0} onClick={()=>setDemoIndex(i=>i-1)}><ArrowLeft size={16}/>Previous move</button>{demoIndex<lesson.trace.length-1?<button className="axiom-btn axiom-btn-primary" onClick={()=>setDemoIndex(i=>i+1)}>Next move<ArrowRight size={16}/></button>:<button className="axiom-btn axiom-btn-primary" onClick={()=>setPhase('practice')}>Now try it yourself<ArrowRight size={16}/></button>}</div>
      {demoIndex===lesson.trace.length-1 && <p className="axiom-learning-acknowledgement" role="status">Good work following the example. Now try a fresh challenge and explain the moves yourself.</p>}<button className="axiom-text-link" onClick={()=>setDemoIndex(0)}>Replay from the beginning</button>
     </div>
    </div>:<div className="tutor-practice">
     {busy && <p role="status">Preparing or checking your practice…</p>}
     {problem && board && <>
      <h4 className="tutor-practice-prompt">{problem.prompt_text}</h4>
      {mental && <div className="tutor-fading-controls"><p>Look at the starting number, then choose how much help you want.</p><div role="group" aria-label="Visual support">{[['full','Starting beads'],['outline','Empty frame'],['hidden','Hide the beads']].map(([id,label])=><button key={id} className="axiom-chip" aria-pressed={support===id} disabled={busy} onClick={()=>{setSupport(id as typeof support);setOutcome(null)}}>{label}</button>)}</div></div>}
      {mental?(support==='hidden'?<div className="tutor-hidden-beads" role="status"><EyeOff size={24} aria-hidden="true"/><h4>The beads are hidden</h4><p>Picture the starting number. Imagine moving the beads for each instruction. What number remains?</p></div>:<BeadBoard state={support==='full'?problem.metadata.initial_state:createAbacus(board.rod_count)} showValue={false}/>):<BeadBoard state={board} disabled={busy} onChange={next=>{if(!busy){setBoard(next);setOutcome(null)}}} showValue={showValue||outcome?.disposition==='correct'}/>}
      <form onSubmit={check} className="tutor-answer-form">
       {mental?<><label htmlFor="mental-answer">Your final number<input id="mental-answer" className="axiom-text-input" maxLength={8192} inputMode="numeric" disabled={busy} value={answer} onChange={event=>{setAnswer(event.target.value);setOutcome(null)}} onKeyDown={e=>{if(e.key==="Enter"&&answer.trim()){e.currentTarget.form?.requestSubmit()}}}/></label><p className="axiom-input-hint"><span aria-hidden="true">↵</span> Press <kbd>Enter ↵</kbd> to automatically check your number</p></>:<div className="tutor-bead-guidance"><p>Make your final answer on the abacus, then check it. Tap the beads or use the lower-bead selectors.</p><p className="axiom-input-hint"><span aria-hidden="true">↵</span> Press <kbd>Enter ↵</kbd> or click below to check immediately</p></div>}
       <div className="tutor-move-buttons"><button type="submit" className="axiom-btn axiom-btn-primary" aria-label="Check my answer" disabled={busy||(mental&&!answer.trim())}><span>Check my answer</span><span className="axiom-enter-badge" aria-hidden="true">↵ Enter</span></button><button type="button" className="axiom-btn axiom-btn-outline" disabled={busy} onClick={()=>void fresh()}>New challenge</button></div>
      </form>
      {outcome && <div className={`tutor-result ${outcome.disposition}`} role="status"><h4>{outcome.disposition==='correct'?'Your answer checks out':outcome.disposition==='incorrect'?'Try the moves once more':'Check your input'}</h4><Encouragement outcome={outcome} mistakes={mistakes} attempts={attempts} assisted={assisted}/><p>{outcome.disposition==='incorrect'?'The final number does not match yet. Try the hint below, then adjust your answer or beads.':outcome.feedback}</p>{outcome.disposition==='correct' && <p>{assisted?'You used a worked example or value hint. Try a new challenge with less support when you feel comfortable.':mental?`Answer checked with ${support==='hidden'?'the beads hidden':support==='outline'?'an empty frame':'starting beads visible'}. This records your chosen support, not how you calculated internally.`:'You made the right final number. Now explain which beads changed.'}</p>}{nextLesson && outcome.disposition==='correct' && <button className="axiom-text-link" onClick={()=>changeStage(nextLesson.id)}>Explore the next stage<ArrowRight size={15}/></button>}</div>}
      {award!==null && outcome?.disposition==='correct' && <p className="tutor-practice-xp">+{award} local practice XP · not a mastery certificate</p>}
      <FeedbackSupport key={problem.problem_hash} problem={problem} afterMistake={outcome?.disposition==='incorrect'} onSupport={()=>{if(outcome?.disposition!=='correct')setAssisted(true)}}/>
      {!mental && <div className="tutor-hint-actions"><button className="axiom-text-link" onClick={()=>{setShowValue(true);if(outcome?.disposition!=='correct')setAssisted(true)}}>Show the current value (hint)</button></div>}
     </>}
    </div>}
   </section>
   <details className="tutor-readiness"><summary>How to move toward mental arithmetic</summary><p>Use the beads until you can read and make numbers comfortably. Explain direct addition and subtraction, then practise the five and ten exchanges. Next, look at a starting number and hide the beads yourself. Begin with short sequences; return to visible beads whenever the picture is unclear.</p><p>These are untimed draft learning activities, not a speed test or a certificate of mental technique. The mental stage currently practises short, one-rod addition/subtraction; longer sequences and more rods come later.</p></details>
  </>}
 </div>;
};
