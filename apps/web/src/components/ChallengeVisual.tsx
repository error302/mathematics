import React from 'react';
import {ProblemArtifact} from '../engine/axiomCore';
export const ChallengeVisual:React.FC<{problem:ProblemArtifact}>=({problem})=>{
 const m=problem.metadata;
 if(m.kind==='rectangle'){
  const scale=280/Math.max(m.width,m.height),w=m.width*scale,h=m.height*scale;
  return <figure className="axiom-challenge-visual"><svg viewBox="0 0 380 390" role="img" aria-label={`Rectangle, width ${m.width} units and height ${m.height} units. Opposite sides are equal; all four angles are right angles.`}>
   <rect x="55" y="45" width={w} height={h} fill="#29394b" stroke="#e6c580" strokeWidth="2"/>
   {Array.from({length:m.width-1},(_,i)=><line key={`x${i}`} x1={55+(i+1)*scale} x2={55+(i+1)*scale} y1="45" y2={45+h} stroke="#6a7d92"/>)}
   {Array.from({length:m.height-1},(_,i)=><line key={`y${i}`} y1={45+(i+1)*scale} y2={45+(i+1)*scale} x1="55" x2={55+w} stroke="#6a7d92"/>)}
   <path d="M55 59h14v-14" stroke="#edf1f5" fill="none"/><text x={55+w/2} y="29" textAnchor="middle">{m.width} units</text><text x="35" y={45+h/2} textAnchor="middle" transform={`rotate(-90 35 ${45+h/2})`}>{m.height} units</text>
  </svg><figcaption>Each small square is 1 square unit. Lengths share the same scale. Area covers the inside; perimeter follows the edge.</figcaption></figure>
 }
 if(m.kind!=='graph')return null;
 const left=50,top=30,width=300,height=280;
 const px=(x:number)=>left+(x-m.x_min)/(m.x_max-m.x_min)*width;
 const py=(y:number)=>top+height-(y-m.y_min)/(m.y_max-m.y_min)*height;
 const points=m.points as [number,number][];
 // Coordinate conversion is display-only. Coefficients, points and Bézier control are supplied by Rust.
 const path=m.quadratic?`M${px(points[0][0])},${py(points[0][1])} Q${px(m.control[0])},${py(m.control[1])} ${px(points[6][0])},${py(points[6][1])}`:`M${px(points[0][0])},${py(points[0][1])} L${px(points[6][0])},${py(points[6][1])}`;
 const step=m.y_max-m.y_min>35?10:m.y_max-m.y_min>16?5:2;
 const ticks=Array.from({length:Math.floor(m.y_max/step)-Math.ceil(m.y_min/step)+1},(_,i)=>(Math.ceil(m.y_min/step)+i)*step);
 return <figure className="axiom-challenge-visual"><svg viewBox="0 0 400 365" role="img" aria-label={`Coordinate graph of y equals ${m.a} times x${m.quadratic?' squared':''} plus ${m.b}. x ranges from −3 to 3. Exact point table follows.`}>
  {points.map(([x])=><g key={x}><line x1={px(x)} x2={px(x)} y1={top} y2={top+height} stroke="#29394b"/><text x={px(x)} y={top+height+20} textAnchor="middle">{x}</text></g>)}
  {ticks.map(y=><g key={y}><line x1={left} x2={left+width} y1={py(y)} y2={py(y)} stroke="#29394b"/><text x={left-10} y={py(y)+5} textAnchor="end">{y}</text></g>)}
  <line x1={left} x2={left+width} y1={py(0)} y2={py(0)} stroke="#b5c1cd"/><line x1={px(0)} x2={px(0)} y1={top} y2={top+height} stroke="#b5c1cd"/>
  <path d={path} stroke="#e6c580" strokeWidth="3" fill="none"/>{points.map(([x,y])=><circle key={x} cx={px(x)} cy={py(y)} r="4" fill="#edf1f5"/>)}
  <text x={left+width+18} y={py(0)+5}>x</text><text x={px(0)+8} y="18">y</text><text x="200" y="350" textAnchor="middle">Horizontal axis: x · Vertical axis: y</text>
 </svg><figcaption>Dots are exact sample points. The curve represents the equation throughout the shown x range. Axis scales are labeled independently.</figcaption>
 <details><summary>Read the exact point table</summary><table className="axiom-table"><caption>Coordinates calculated by the mathematics engine</caption><thead><tr><th scope="col">x</th><th scope="col">y</th></tr></thead><tbody>{points.map(([x,y])=><tr key={x}><td>{x}</td><td>{y}</td></tr>)}</tbody></table></details></figure>
};
