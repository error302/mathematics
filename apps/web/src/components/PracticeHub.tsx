import React,{useState} from 'react';
import {PracticeWorkspace} from './PracticeWorkspace';
import {ChallengeWorkshop} from './ChallengeWorkshop';
import '../challenges.css';
type Props=React.ComponentProps<typeof PracticeWorkspace>;
export const PracticeHub:React.FC<Props>=(props)=>{
 const routed=props.initialTemplateId?.startsWith('quest:')?props.initialTemplateId.slice(6):null;
 const [mode,setMode]=useState<'foundation'|'quest'|'visual'>(routed?.startsWith('visual.')?'visual':routed?'quest':'foundation');
 return <div className="axiom-practice-hub"><div className="axiom-practice-switch" role="group" aria-label="Practice paths">
 {([['foundation','Foundation practice'],['quest','Puzzle quests'],['visual','Visual lab']] as const).map(([id,title])=><button key={id} className={`axiom-btn ${mode===id?'axiom-btn-primary':'axiom-btn-outline'}`} aria-pressed={mode===id} onClick={()=>setMode(id)}>{title}</button>)}</div>
 {mode==='foundation'?<PracticeWorkspace {...props} initialTemplateId={routed?undefined:props.initialTemplateId}/>:<ChallengeWorkshop key={mode} mode={mode} initialTemplate={routed||undefined} onAwardXp={props.onAwardXp}/>}
 </div>
};
