import React,{useEffect,useState} from 'react';
import {CoachingMaterial,getExerciseCoaching,ProblemArtifact} from '../engine/axiomCore';
import {BeadBoard} from './BeadBoard';
export const FeedbackSupport:React.FC<{problem:ProblemArtifact;afterMistake:boolean;onSupport:()=>void}>=({problem,afterMistake,onSupport})=>{
 const [material,setMaterial]=useState<CoachingMaterial|null>(null);
 const [error,setError]=useState(false);
 const [level,setLevel]=useState<'none'|'hint'|'worked'>('none');
 useEffect(()=>{let active=true;setMaterial(null);setError(false);getExerciseCoaching(problem).then(value=>{if(active)setMaterial(value)}).catch(()=>{if(active)setError(true)});return()=>{active=false}},[problem.problem_hash]);
 useEffect(()=>{if(afterMistake)setLevel(current=>current==='worked'?'worked':'hint')},[afterMistake]);
 const visual=material?.visual;
 return <section className="axiom-feedback-support" aria-label="Hints and worked explanation">
  <div className="axiom-support-actions"><button type="button" className="axiom-text-link" disabled={!material} onClick={()=>{setLevel('hint');onSupport()}}>Give me a hint</button><button type="button" className="axiom-text-link" disabled={!material} onClick={()=>{setLevel('worked');onSupport()}}>Show me how</button>{level!=='none'&&<button type="button" className="axiom-text-link" onClick={()=>setLevel('none')}>Close help</button>}</div>
  {error&&<p role="status">Worked help could not load for this example. Your answer is saved; try the related lesson or a new example.</p>}
  {level!=='none'&&material&&<div className="axiom-help-content"><h4>{level==='hint'?'A useful hint':'Let’s work it through'}</h4><p>{material.hint}</p>
   {level==='worked'&&<><ol>{material.steps.map((step,index)=><li key={index}>{step}</li>)}</ol><p className="axiom-help-conclusion">{material.conclusion}</p>
    {visual?.kind==='columns'&&<pre className="axiom-column-visual" aria-label="Worked arithmetic columns">{visual.rows.join('\n')}</pre>}
    {visual?.kind==='fractions'&&<figure className="axiom-coaching-strips"><figcaption>Equal-length wholes, with equal-sized parts</figcaption>{visual.rows.map((row:any,index:number)=><div key={index}><p className="axiom-strip-label">{row.label}</p><svg viewBox="0 0 460 45" role="img" aria-label={row.label}><rect x="8" y="8" width="440" height="30" rx="2" fill="#29394b"/><rect x="8" y="8" width={440*row.numerator/row.denominator} height="30" rx="2" fill="#e6c580"/>{row.denominator<=64&&Array.from({length:row.denominator-1},(_,i)=><path key={i} d={`M${8+440*(i+1)/row.denominator} 8v30`} stroke="#0d1521" strokeWidth="2"/>)}</svg></div>)}</figure>}
    {visual?.kind==='abacus'&&<BeadBoard state={visual.state}/>}
    <p className="axiom-supported-note">This is supported practice: you’re learning with a worked example. Try the moves, then choose a fresh challenge to see what you can do independently.</p>
   </>}
  </div>}
 </section>;
};
