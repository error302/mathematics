import React from 'react';
import { AbacusState, applyAbacusAction, computeAbacusValue } from '../engine/axiomCore';

interface Props { state: AbacusState; onChange?: (state: AbacusState)=>void; highlightRod?: number; showValue?: boolean; disabled?: boolean }
export const BeadBoard: React.FC<Props> = ({state,onChange,highlightRod,showValue=true,disabled=false}) => {
  const change=(action: Record<string, unknown>)=>onChange?.(applyAbacusAction(state,action));
  const names=state.rod_count===1?['Ones']:['Tens','Ones'];
  return <div className="tutor-board-area">
    <div className={`tutor-board ${state.rod_count===1?'one-rod':''}`} role="group" aria-label={onChange?'Your practice abacus':'Worked example abacus'}>
      {state.rods.map((rod,index)=><fieldset key={index} className={`tutor-rod ${highlightRod===index?'highlighted':''}`}>
        <legend>{names[index]}</legend>
        <p className="tutor-place-weight">{state.rod_count===2 && index===0?'Lower = 10 · upper = 50':'Lower = 1 · upper = 5'}</p>
        <div className="tutor-upper-deck"><span className="tutor-wire" aria-hidden="true"/>
          {onChange?<button type="button" disabled={disabled} className={`tutor-bead upper ${rod.upper?'counted':''}`} aria-pressed={rod.upper} aria-label={`${names[index]} upper bead: ${rod.upper?'counting five':'not counting'}`} onClick={()=>change({type:'toggle_upper',rod_index:index})}><span aria-hidden="true">5</span></button>:<span className={`tutor-bead upper ${rod.upper?'counted':''}`} aria-hidden="true">5</span>}
        </div>
        <div className="tutor-middle-bar" aria-hidden="true"/>
        <div className="tutor-lower-deck"><span className="tutor-wire" aria-hidden="true"/>
          {[0,1,2,3].map(bead=>{const counted=bead<rod.lower;const position=counted?bead*28:64+bead*28;return onChange?<button type="button" disabled={disabled} key={bead} className={`tutor-bead lower ${counted?'counted':''}`} style={{transform:`translateY(${position}px)`}} aria-label={`${names[index]} lower bead ${bead+1}: ${counted?'counting':'not counting'}`} onClick={()=>change({type:'set_lower',rod_index:index,count:counted?bead:bead+1})}><span aria-hidden="true">1</span></button>:<span key={bead} className={`tutor-bead lower ${counted?'counted':''}`} style={{transform:`translateY(${position}px)`}} aria-hidden="true">1</span>})}
        </div>
        <p className="tutor-count-description">Upper: {rod.upper?'counting':'away'}<br/>Lower: {rod.lower} counting</p>
        {onChange && <label className="tutor-rod-selector">Lower beads on {names[index]}<select disabled={disabled} className="axiom-select" aria-label={`Lower beads on ${names[index]}`} value={rod.lower} onChange={event=>change({type:'set_lower',rod_index:index,count:Number(event.target.value)})}>{[0,1,2,3,4].map(value=><option key={value} value={value}>{value} counting</option>)}</select></label>}
      </fieldset>)}
    </div>
    <p className="tutor-bar-legend"><span aria-hidden="true">↥ ↧</span> The middle bar is the counting line. Only beads touching it count.</p>
    {showValue && <p className="tutor-visible-value">Shown value: <strong>{computeAbacusValue(state).toCanonical()}</strong></p>}
  </div>;
};
