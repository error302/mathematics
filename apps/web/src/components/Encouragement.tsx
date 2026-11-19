import React from 'react';
import {GradeOutcome} from '../engine/axiomCore';
export const Encouragement:React.FC<{outcome:GradeOutcome;mistakes:number;attempts:number;assisted?:boolean}>=({outcome,mistakes,attempts,assisted=false})=>{
 const positive=outcome.disposition==='correct';
 const message=positive
  ?assisted?mistakes>0?'Good recovery. You corrected your answer using support—try a fresh example when you feel ready.':'Good work using support to find the answer. Try a fresh example when you feel ready.'
   :mistakes>0?'That’s progress—you kept going and corrected your answer.'
   :['Nice work. You found a correct answer.','Well done. This answer checks out.','Good work. Keep building on this with another example.'][Math.max(0,attempts-1)%3]
  :outcome.disposition==='incorrect'
   ?['Good try. Let’s work through one useful step.','Keep going. You can use this attempt to choose your next step.','Take your time. A hint can help you decide what to change.'][Math.max(0,mistakes-1)%3]
   :outcome.disposition==='malformed'?'Keep your idea. Let’s make the way you wrote it clearer.'
   :'The checker could not decide this answer. It has not been marked wrong.';
 return <p className={`axiom-encouragement ${positive?'positive':'supportive'}`}><strong>{message}</strong>{positive&&mistakes>0&&<span>A corrected mistake is worth acknowledging. Now explain what changed.</span>}</p>;
};
