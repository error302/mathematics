import React, { useState } from 'react';

export const ConceptVisual: React.FC<{ lessonId: string }> = ({ lessonId }) => {
  const [split, setSplit] = useState(false);
  const [bundled, setBundled] = useState(false);
  const [rotated, setRotated] = useState(false);
  const [divisionMode, setDivisionMode] = useState<'three' | 'four'>('three');
  const [equivScale, setEquivScale] = useState<1 | 2 | 4>(1);
  const [numLineVal, setNumLineVal] = useState<number>(-3);
  const [factorShape, setFactorShape] = useState<'3x4' | '2x6' | '1x12' | 'prime7'>('3x4');
  const [gcdMode, setGcdMode] = useState<'gcd' | 'lcm'>('gcd');
  const [balanceX, setBalanceX] = useState<number>(14);
  const [checkMode, setCheckMode] = useState<'bounds' | 'parity' | 'counterexample'>('bounds');
  const [decimalShade, setDecimalShade] = useState<'0.3' | '0.35' | '0.7'>('0.35');
  const [percentBenchmark, setPercentBenchmark] = useState<'25' | '50' | '75' | '10'>('75');
  const [ratioScale, setRatioScale] = useState<number>(8);
  const [powerExp, setPowerExp] = useState<number>(-2);
  const [sciChoice, setSciChoice] = useState<'45000' | '0.0072' | '300000000'>('45000');
  const [rootDemo, setRootDemo] = useState<'fraction' | 'fallacy'>('fraction');
  const [geomShape, setGeomShape] = useState<'6x6_square' | '2x10_rectangle' | 'triangle_8x5'>('6x6_square');
  const [angleDemo, setAngleDemo] = useState<'triangle_sum' | 'parallel_transversal'>('triangle_sum');
  const [coordTarget, setCoordTarget] = useState<'point_3_4' | 'point_5_12' | 'axis_aligned'>('point_3_4');
  const [dataOutlier, setDataOutlier] = useState<boolean>(false);
  const [countingMode, setCountingMode] = useState<'permutation' | 'combination'>('permutation');
  const [probMode, setProbMode] = useState<'complement' | 'without_replacement'>('complement');
  const [algEvalMode, setAlgEvalMode] = useState<'negative_square' | 'distributive_minus'>('negative_square');
  const [eqStep, setEqStep] = useState<1 | 2 | 3>(1);
  const [sysMode, setSysMode] = useState<'intersection' | 'elimination'>('intersection');
  const [fnMapMode, setFnMapMode] = useState<'preimage' | 'piecewise'>('preimage');
  const [compMode, setCompMode] = useState<'chain' | 'inverse'>('chain');
  const [transMode, setTransMode] = useState<'vertex' | 'point_shift'>('vertex');
  const [polyMode, setPolyMode] = useState<'vieta' | 'remainder'>('vieta');
  const [discMode, setDiscMode] = useState<'two_roots' | 'tangent_zero'>('two_roots');
  const [expLogMode, setExpLogMode] = useState<'log_laws' | 'common_base'>('log_laws');
  const [secLineMode, setSecLineMode] = useState<'perp_slope' | 'section_ratio'>('perp_slope');
  const [circleMode, setCircleMode] = useState<'inscribed_angle' | 'chord_power'>('inscribed_angle');
  const [vecMode, setVecMode] = useState<'orthogonal' | 'projection'>('orthogonal');
  const [radMode, setRadMode] = useState<'quadrant_ii' | 'wave_period'>('quadrant_ii');
  const [trigIdMode, setTrigIdMode] = useState<'double_angle' | 'angle_sum'>('double_angle');
  const [triLawMode, setTriLawMode] = useState<'cosine_law' | 'principal_arcsin'>('cosine_law');
  const [arithSeqMode, setArithSeqMode] = useState<'nth_step' | 'gauss_sum'>('nth_step');
  const [geomSeqMode, setGeomSeqMode] = useState<'finite_telescope' | 'infinite_sum'>('finite_telescope');
  const [binomMode, setBinomMode] = useState<'recurrence' | 'binomial_term'>('recurrence');
  const [limitMode, setLimitMode] = useState<'removable_hole' | 'infinity_lead'>('removable_hole');
  const [derivMode, setDerivMode] = useState<'tangent_line' | 'critical_point'>('tangent_line');
  const [intMode, setIntMode] = useState<'ftc_area' | 'net_change'>('ftc_area');

  if (lessonId === 'foundation.fractions.compare') {
    return (
      <figure className="concept-visual">
        <figcaption>Compare equal-sized parts of the same whole</figcaption>
        <svg viewBox="0 0 480 160" role="img" aria-label={split ? 'Three quarters is six eighths. Six eighths is more than five eighths.' : 'Three quarters and five eighths shown on equal-length strips.'}>
          <text x="4" y="42" fill="#edf1f5" fontSize="18">{split ? '6/8' : '3/4'}</text>
          <rect x="65" y="16" width="400" height="40" rx="3" fill="#29394b" />
          <rect x="65" y="16" width="300" height="40" rx="3" fill="#e6c580" />
          {Array.from({ length: split ? 7 : 3 }, (_, i) => (
            <path key={i} d={`M${65 + (i + 1) * (split ? 50 : 100)} 16v40`} stroke="#0d1521" strokeWidth="2" />
          ))}
          <text x="4" y="114" fill="#edf1f5" fontSize="18">5/8</text>
          <rect x="65" y="88" width="400" height="40" rx="3" fill="#29394b" />
          <rect x="65" y="88" width="250" height="40" rx="3" fill="#e6c580" />
          {Array.from({ length: 7 }, (_, i) => (
            <path key={i} d={`M${65 + (i + 1) * 50} 88v40`} stroke="#0d1521" strokeWidth="2" />
          ))}
        </svg>
        <button className="axiom-btn axiom-btn-outline" aria-pressed={split} onClick={() => setSplit(v => !v)}>
          {split ? 'Return to quarters' : 'Split each quarter into two'}
        </button>
        <p role="status">
          {split
            ? 'The shaded amount did not change. Three quarters is six eighths, so it is larger than five eighths.'
            : 'The whole strips have equal lengths. Try splitting each quarter so both strips use eighths.'}
        </p>
      </figure>
    );
  }

  if (lessonId === 'foundation.place_value.regrouping') {
    return (
      <figure className="concept-visual">
        <figcaption>Fourteen ones, or one ten and four ones</figcaption>
        <svg viewBox="0 0 480 156" role="img" aria-label={bundled ? 'Ten unit squares grouped into one ten rod, with four unit squares remaining. Total fourteen.' : 'Fourteen individual unit squares. Total fourteen.'}>
          {Array.from({ length: 14 }, (_, i) => {
            const x = i < 10 ? 20 + i * 30 : 335 + (i - 10) * 30;
            return <rect key={i} x={x} y="47" width="26" height="26" rx="2" fill="#e6c580" />;
          })}
          {bundled && (
            <>
              <rect x="15" y="42" width="300" height="36" rx="4" fill="none" stroke="#edf1f5" strokeWidth="2" />
              <text x="115" y="110" fill="#edf1f5" fontSize="17">1 ten</text>
              <text x="337" y="110" fill="#edf1f5" fontSize="17">4 ones</text>
            </>
          )}
          <text x="20" y="142" fill="#b5c1cd" fontSize="16">{bundled ? '10 + 4 = 14' : '14 ones = 14'}</text>
        </svg>
        <button className="axiom-btn axiom-btn-outline" aria-pressed={bundled} onClick={() => setBundled(v => !v)}>
          {bundled ? 'Unbundle the ten' : 'Bundle ten ones'}
        </button>
        <p role="status">
          {bundled
            ? 'Ten ones are grouped as one ten. The total stays fourteen.'
            : 'Count the squares, then group ten of them. Regrouping changes the description, not the amount.'}
        </p>
      </figure>
    );
  }

  if (lessonId === 'foundation.arithmetic.multiplication_array') {
    const rows = rotated ? 5 : 3;
    const cols = rotated ? 3 : 5;
    return (
      <figure className="concept-visual">
        <figcaption>Rectangular array: {rows} rows of {cols} units</figcaption>
        <svg viewBox="0 0 480 170" role="img" aria-label={`${rows} rows and ${cols} columns of gold squares. Total fifteen units.`}>
          {Array.from({ length: rows }, (_, r) =>
            Array.from({ length: cols }, (_, c) => {
              const x = 50 + c * 42;
              const y = 20 + r * 26;
              return <rect key={`${r}-${c}`} x={x} y={y} width="32" height="20" rx="3" fill="#e6c580" />;
            })
          )}
          <text x="280" y="70" fill="#edf1f5" fontSize="18">
            {rows} × {cols} = 15
          </text>
          <text x="280" y="100" fill="#b5c1cd" fontSize="14">
            {rotated ? '5 groups of 3' : '3 groups of 5'}
          </text>
        </svg>
        <button className="axiom-btn axiom-btn-outline" aria-pressed={rotated} onClick={() => setRotated(v => !v)}>
          {rotated ? 'Rotate to 3 × 5' : 'Rotate to 5 × 3 (demonstrates commutativity)'}
        </button>
        <p role="status">
          Rotating the rectangle swaps rows and columns without changing the total count: 3 × 5 = 5 × 3 = 15.
        </p>
      </figure>
    );
  }

  if (lessonId === 'foundation.arithmetic.division_partition') {
    const groups = divisionMode === 'three' ? 3 : 4;
    const size = divisionMode === 'three' ? 4 : 3;
    return (
      <figure className="concept-visual">
        <figcaption>12 units partitioned into {groups} equal groups of {size}</figcaption>
        <svg viewBox="0 0 480 160" role="img" aria-label={`Twelve items split into ${groups} equal groups of ${size}.`}>
          {Array.from({ length: groups }, (_, g) => (
            <g key={g}>
              <rect x={20 + g * (440 / groups)} y="20" width={440 / groups - 15} height="80" rx="6" fill="#162231" stroke="#334457" strokeWidth="2" />
              {Array.from({ length: size }, (_, i) => (
                <circle key={i} cx={35 + g * (440 / groups) + (i % 2) * 30} cy={45 + Math.floor(i / 2) * 30} r="10" fill="#e6c580" />
              ))}
              <text x={30 + g * (440 / groups)} y="125" fill="#b5c1cd" fontSize="14">
                Group {g + 1}: {size}
              </text>
            </g>
          ))}
          <text x="20" y="150" fill="#edf1f5" fontSize="16">
            12 ÷ {groups} = {size} (Check: {groups} × {size} = 12)
          </text>
        </svg>
        <button
          className="axiom-btn axiom-btn-outline"
          onClick={() => setDivisionMode(m => (m === 'three' ? 'four' : 'three'))}
        >
          {divisionMode === 'three' ? 'Partition into 4 groups of 3' : 'Partition into 3 groups of 4'}
        </button>
        <p role="status">
          Division splits the whole into equal subsets. Every division can be confirmed by multiplication.
        </p>
      </figure>
    );
  }

  if (lessonId === 'foundation.fractions.equivalent') {
    const totalParts = 3 * equivScale;
    const shadedParts = 2 * equivScale;
    const partWidth = 360 / totalParts;
    return (
      <figure className="concept-visual">
        <figcaption>
          Equivalent fraction: {shadedParts}/{totalParts} of the same unit strip
        </figcaption>
        <svg viewBox="0 0 480 150" role="img" aria-label={`Strip with ${shadedParts} out of ${totalParts} equal parts shaded.`}>
          <text x="10" y="55" fill="#edf1f5" fontSize="20">
            {shadedParts}/{totalParts}
          </text>
          <rect x="70" y="25" width="360" height="50" rx="4" fill="#29394b" />
          <rect x="70" y="25" width={shadedParts * partWidth} height="50" rx="4" fill="#e6c580" />
          {Array.from({ length: totalParts - 1 }, (_, i) => (
            <path key={i} d={`M${70 + (i + 1) * partWidth} 25v50`} stroke="#0d1521" strokeWidth="2" />
          ))}
          <text x="70" y="105" fill="#b5c1cd" fontSize="15">
            Original: 2/3 · Scaling factor: ×{equivScale} top and bottom
          </text>
        </svg>
        <div className="axiom-btn-group" role="group" aria-label="Subdivision scale">
          <button
            type="button"
            className={`axiom-chip ${equivScale === 1 ? 'active' : ''}`}
            onClick={() => setEquivScale(1)}
          >
            Original 2/3
          </button>
          <button
            type="button"
            className={`axiom-chip ${equivScale === 2 ? 'active' : ''}`}
            onClick={() => setEquivScale(2)}
          >
            Subdivide ×2 (4/6)
          </button>
          <button
            type="button"
            className={`axiom-chip ${equivScale === 4 ? 'active' : ''}`}
            onClick={() => setEquivScale(4)}
          >
            Subdivide ×4 (8/12)
          </button>
        </div>
        <p role="status">
          Multiplying numerator and denominator by {equivScale} creates smaller pieces, but the total shaded amount remains identical: 2/3 = {shadedParts}/{totalParts}.
        </p>
      </figure>
    );
  }

  if (lessonId === 'foundation.integers.number_line') {
    const originX = 240;
    const unitPx = 32;
    const currentX = originX + numLineVal * unitPx;
    return (
      <figure className="concept-visual">
        <figcaption>Interactive Signed Number Line: {numLineVal > 0 ? `+${numLineVal}` : numLineVal}</figcaption>
        <svg viewBox="0 0 480 150" role="img" aria-label={`Number line showing integer ${numLineVal}. Distance from zero is ${Math.abs(numLineVal)} units.`}>
          <line x1="20" y1="70" x2="460" y2="70" stroke="#718096" strokeWidth="3" />
          <polygon points="15,70 25,65 25,75" fill="#718096" />
          <polygon points="465,70 455,65 455,75" fill="#718096" />
          {[-6, -5, -4, -3, -2, -1, 0, 1, 2, 3, 4, 5, 6].map(n => {
            const x = originX + n * unitPx;
            const isZero = n === 0;
            return (
              <g key={n}>
                <line x1={x} y1={isZero ? 55 : 62} x2={x} y2={isZero ? 85 : 78} stroke={isZero ? '#e6c580' : '#4a5568'} strokeWidth={isZero ? 3 : 2} />
                <text x={x} y="105" textAnchor="middle" fill={isZero ? '#e6c580' : n === numLineVal ? '#edf1f5' : '#a0aec0'} fontSize={isZero ? '15' : '13'} fontWeight={isZero ? 'bold' : 'normal'}>
                  {n}
                </text>
              </g>
            );
          })}
          {numLineVal !== 0 && (
            <path
              d={`M${originX} 55 Q${(originX + currentX) / 2} ${55 - Math.abs(numLineVal) * 6} ${currentX} 55`}
              fill="none"
              stroke="#e6c580"
              strokeWidth="2"
              strokeDasharray="4 2"
            />
          )}
          <circle cx={currentX} cy="70" r="8" fill="#e6c580" stroke="#0d1521" strokeWidth="2" />
          <text x={currentX} y="35" textAnchor="middle" fill="#e6c580" fontSize="14" fontWeight="bold">
            {numLineVal > 0 ? `+${numLineVal}` : numLineVal}
          </text>
        </svg>
        <div className="axiom-btn-group" role="group" aria-label="Choose integer position">
          {[-5, -3, -1, 0, 2, 4].map(val => (
            <button
              key={val}
              type="button"
              className={`axiom-chip ${numLineVal === val ? 'active' : ''}`}
              onClick={() => setNumLineVal(val)}
            >
              {val > 0 ? `+${val}` : val}
            </button>
          ))}
        </div>
        <p role="status">
          {numLineVal < 0
            ? `${numLineVal} lies ${Math.abs(numLineVal)} units to the LEFT of 0. Magnitude |${numLineVal}| = ${Math.abs(numLineVal)}.`
            : numLineVal > 0
            ? `${numLineVal} lies ${numLineVal} units to the RIGHT of 0. Magnitude |${numLineVal}| = ${numLineVal}.`
            : '0 is the neutral origin dividing negative integers (left) from positive integers (right).'}
        </p>
      </figure>
    );
  }

  if (lessonId === 'foundation.divisibility.factors_primes') {
    const isPrime7 = factorShape === 'prime7';
    const rows = isPrime7 ? 1 : factorShape === '3x4' ? 3 : factorShape === '2x6' ? 2 : 1;
    const cols = isPrime7 ? 7 : factorShape === '3x4' ? 4 : factorShape === '2x6' ? 6 : 12;
    const count = isPrime7 ? 7 : 12;
    return (
      <figure className="concept-visual">
        <figcaption>Factor Rectangles: {count} units arranged as {rows} × {cols}</figcaption>
        <svg viewBox="0 0 480 150" role="img" aria-label={`${count} unit squares in ${rows} rows of ${cols}.`}>
          {Array.from({ length: rows }, (_, r) =>
            Array.from({ length: cols }, (_, c) => {
              const size = isPrime7 ? 28 : factorShape === '1x12' ? 24 : 26;
              const x = 30 + c * (size + 4);
              const y = 25 + r * (size + 4);
              return <rect key={`${r}-${c}`} x={x} y={y} width={size} height={size} rx="3" fill={isPrime7 ? '#63b3ed' : '#e6c580'} />;
            })
          )}
          <text x="350" y="60" fill="#edf1f5" fontSize="18" fontWeight="bold">
            {rows} × {cols} = {count}
          </text>
          <text x="350" y="85" fill="#a0aec0" fontSize="14">
            {isPrime7 ? '7 is Prime' : '12 is Composite'}
          </text>
        </svg>
        <div className="axiom-btn-group" role="group" aria-label="Factor arrangements">
          <button
            type="button"
            className={`axiom-chip ${factorShape === '3x4' ? 'active' : ''}`}
            onClick={() => setFactorShape('3x4')}
          >
            12 as 3 × 4
          </button>
          <button
            type="button"
            className={`axiom-chip ${factorShape === '2x6' ? 'active' : ''}`}
            onClick={() => setFactorShape('2x6')}
          >
            12 as 2 × 6
          </button>
          <button
            type="button"
            className={`axiom-chip ${factorShape === '1x12' ? 'active' : ''}`}
            onClick={() => setFactorShape('1x12')}
          >
            12 as 1 × 12
          </button>
          <button
            type="button"
            className={`axiom-chip ${factorShape === 'prime7' ? 'active' : ''}`}
            onClick={() => setFactorShape('prime7')}
          >
            Prime 7 (1 × 7 only)
          </button>
        </div>
        <p role="status">
          {isPrime7
            ? 'Prime numbers like 7 can only form a single 1 × p line because their only positive divisors are 1 and themselves.'
            : 'Composite numbers like 12 can be factored into multiple distinct rectangular arrangements (proper divisors).'}
        </p>
      </figure>
    );
  }

  if (lessonId === 'foundation.divisibility.gcd_lcm') {
    return (
      <figure className="concept-visual">
        <figcaption>Prime Factor Overlap: 24 and 36</figcaption>
        <svg viewBox="0 0 480 160" role="img" aria-label="Venn diagram showing prime factors of 24 and 36 with shared factors in the intersection.">
          <circle cx="180" cy="75" r="65" fill="#2d3748" fillOpacity="0.6" stroke="#4a5568" strokeWidth="2" />
          <circle cx="280" cy="75" r="65" fill="#2d3748" fillOpacity="0.6" stroke="#4a5568" strokeWidth="2" />
          <text x="140" y="35" fill="#edf1f5" fontSize="15" fontWeight="bold">24 only</text>
          <text x="215" y="25" fill="#e6c580" fontSize="15" fontWeight="bold">Shared (GCD)</text>
          <text x="300" y="35" fill="#edf1f5" fontSize="15" fontWeight="bold">36 only</text>
          <text x="145" y="80" fill="#cbd5e0" fontSize="18" fontWeight="bold">2</text>
          <rect x="210" y="48" width="60" height="55" rx="6" fill={gcdMode === 'gcd' ? '#e6c580' : 'none'} fillOpacity={gcdMode === 'gcd' ? '0.2' : '0'} stroke={gcdMode === 'gcd' ? '#e6c580' : 'none'} strokeWidth="1.5" />
          <text x="225" y="70" fill="#e6c580" fontSize="16" fontWeight="bold">2 · 2</text>
          <text x="233" y="92" fill="#e6c580" fontSize="16" fontWeight="bold">3</text>
          <text x="315" y="80" fill="#cbd5e0" fontSize="18" fontWeight="bold">3</text>
          <text x="240" y="145" textAnchor="middle" fill="#edf1f5" fontSize="15">
            {gcdMode === 'gcd'
              ? 'GCD = Product of shared factors = 2 × 2 × 3 = 12'
              : 'LCM = Product of all factors = 2 × (2 × 2 × 3) × 3 = 72'}
          </text>
        </svg>
        <div className="axiom-btn-group" role="group" aria-label="Highlight view">
          <button
            type="button"
            className={`axiom-chip ${gcdMode === 'gcd' ? 'active' : ''}`}
            onClick={() => setGcdMode('gcd')}
          >
            Show GCD (Intersection: 12)
          </button>
          <button
            type="button"
            className={`axiom-chip ${gcdMode === 'lcm' ? 'active' : ''}`}
            onClick={() => setGcdMode('lcm')}
          >
            Show LCM (Union: 72)
          </button>
        </div>
        <p role="status">
          {gcdMode === 'gcd'
            ? 'The GCD (12) is the product of common prime factors (the overlap). It is the largest integer dividing both 24 and 36.'
            : 'The LCM (72) combines all prime factors across both sets. Note that GCD × LCM = 12 × 72 = 864 = 24 × 36.'}
        </p>
      </figure>
    );
  }

  if (lessonId === 'foundation.learning.problem_reading') {
    return (
      <figure className="concept-visual">
        <figcaption>Part-Whole Model: 35 litres current + x unknown = 80 litres total capacity</figcaption>
        <svg viewBox="0 0 480 150" role="img" aria-label="Part-whole strip showing known amount 35 and unknown amount x combining to total 80.">
          <rect x="40" y="35" width="400" height="42" rx="4" fill="#1e293b" stroke="#475569" strokeWidth="2" />
          <rect x="40" y="35" width="175" height="42" rx="4" fill="#3b82f6" />
          <rect x="215" y="35" width="225" height="42" rx="4" fill="#e6c580" fillOpacity="0.85" />
          <text x="127" y="62" textAnchor="middle" fill="#edf1f5" fontSize="15" fontWeight="bold">Known: 35 L</text>
          <text x="327" y="62" textAnchor="middle" fill="#0d1521" fontSize="15" fontWeight="bold">Unknown: x = 45 L</text>
          <line x1="40" y1="92" x2="440" y2="92" stroke="#cbd5e0" strokeWidth="2" />
          <line x1="40" y1="87" x2="40" y2="97" stroke="#cbd5e0" strokeWidth="2" />
          <line x1="440" y1="87" x2="440" y2="97" stroke="#cbd5e0" strokeWidth="2" />
          <text x="240" y="112" textAnchor="middle" fill="#edf1f5" fontSize="14">Total Capacity = 80 L (Equation: 35 + x = 80)</text>
        </svg>
        <p role="status">
          Breaking word problems into explicit parts and wholes reveals the algebraic relationship directly: 35 + x = 80 &implies; x = 80 - 35 = 45 litres.
        </p>
      </figure>
    );
  }

  if (lessonId === 'foundation.learning.equality_balance') {
    const leftTotal = 14 + balanceX;
    const rightTotal = 28;
    const isBalanced = leftTotal === rightTotal;
    const tiltAngle = Math.max(-12, Math.min(12, (leftTotal - rightTotal) * 1.5));
    const leftY = 60 + tiltAngle;
    const rightY = 60 - tiltAngle;

    return (
      <figure className="concept-visual">
        <figcaption>The Equality Balance: Left Pan (14 + x) vs Right Pan (20 + 8 = 28)</figcaption>
        <svg viewBox="0 0 480 170" role="img" aria-label={`Balance scale currently ${isBalanced ? 'in horizontal equilibrium' : 'tilted'}. Left pan has weight ${leftTotal}, right pan has weight 28.`}>
          {/* Fulcrum base */}
          <polygon points="240,65 225,125 255,125" fill="#475569" stroke="#64748b" strokeWidth="1.5" />
          <line x1="200" y1="125" x2="280" y2="125" stroke="#64748b" strokeWidth="3" />

          {/* Balance beam */}
          <line x1="80" y1={leftY} x2="400" y2={rightY} stroke={isBalanced ? '#e6c580' : '#94a3b8'} strokeWidth="4" />
          <circle cx="240" cy="65" r="5" fill="#e6c580" />

          {/* Left pan strings and pan */}
          <line x1="80" y1={leftY} x2="80" y2={leftY + 30} stroke="#64748b" strokeWidth="1.5" />
          <rect x="35" y={leftY + 30} width="90" height="10" rx="3" fill="#334155" stroke="#64748b" strokeWidth="1" />
          <text x="80" y={leftY + 22} textAnchor="middle" fill="#edf1f5" fontSize="13" fontWeight="bold">14 + {balanceX} = {leftTotal}</text>

          {/* Right pan strings and pan */}
          <line x1="400" y1={rightY} x2="400" y2={rightY + 30} stroke="#64748b" strokeWidth="1.5" />
          <rect x="355" y={rightY + 30} width="90" height="10" rx="3" fill="#334155" stroke="#64748b" strokeWidth="1" />
          <text x="400" y={rightY + 22} textAnchor="middle" fill="#edf1f5" fontSize="13" fontWeight="bold">28</text>

          {/* Balance status readout */}
          <text x="240" y="152" textAnchor="middle" fill={isBalanced ? '#e6c580' : '#f87171'} fontSize="15" fontWeight="bold">
            {isBalanced ? '✓ Balanced Equilibrium: 14 + 14 = 28' : leftTotal < rightTotal ? `Tilted Right: ${leftTotal} < 28 (Needs more weight on left)` : `Tilted Left: ${leftTotal} > 28 (Too heavy on left)`}
          </text>
        </svg>

        <div className="axiom-btn-group" role="group" aria-label="Adjust unknown x">
          <button
            type="button"
            className={`axiom-chip ${balanceX === 10 ? 'active' : ''}`}
            onClick={() => setBalanceX(10)}
          >
            Try x = 10 (Left: 24)
          </button>
          <button
            type="button"
            className={`axiom-chip ${balanceX === 14 ? 'active' : ''}`}
            onClick={() => setBalanceX(14)}
          >
            Try x = 14 (Balanced: 28)
          </button>
          <button
            type="button"
            className={`axiom-chip ${balanceX === 18 ? 'active' : ''}`}
            onClick={() => setBalanceX(18)}
          >
            Try x = 18 (Left: 32)
          </button>
        </div>
        <p role="status">
          {isBalanced
            ? 'When x = 14, both sides equal 28 exactly. The equals sign asserts this state of equilibrium.'
            : 'When the two expressions do not evaluate to the same value, the scale tilts and the statement of equality is false.'}
        </p>
      </figure>
    );
  }

  if (lessonId === 'foundation.learning.checking_bounds') {
    return (
      <figure className="concept-visual">
        <figcaption>Answer Checking: Parity Invariant and Bounding Interval</figcaption>
        <svg viewBox="0 0 480 150" role="img" aria-label="Visualization of checking methods: parity rules and bounding intervals.">
          {checkMode === 'bounds' && (
            <>
              <line x1="50" y1="65" x2="430" y2="65" stroke="#64748b" strokeWidth="2" />
              {/* Lower bound */}
              <circle cx="90" cy="65" r="5" fill="#60a5fa" />
              <text x="90" y="45" textAnchor="middle" fill="#60a5fa" fontSize="13" fontWeight="bold">2,000</text>
              <text x="90" y="90" textAnchor="middle" fill="#94a3b8" fontSize="12">40 × 50 (Lower)</text>

              {/* Exact answer */}
              <circle cx="270" cy="65" r="7" fill="#e6c580" />
              <text x="270" y="40" textAnchor="middle" fill="#e6c580" fontSize="15" fontWeight="bold">2,491</text>
              <text x="270" y="90" textAnchor="middle" fill="#e6c580" fontSize="12">47 × 53 (Exact)</text>

              {/* Upper bound */}
              <circle cx="390" cy="65" r="5" fill="#60a5fa" />
              <text x="390" y="45" textAnchor="middle" fill="#60a5fa" fontSize="13" fontWeight="bold">3,000</text>
              <text x="390" y="90" textAnchor="middle" fill="#94a3b8" fontSize="12">50 × 60 (Upper)</text>

              <rect x="90" y="60" width="300" height="10" fill="#60a5fa" fillOpacity="0.15" />
              <text x="240" y="135" textAnchor="middle" fill="#edf1f5" fontSize="13">Placing the product inside [2,000, 3,000] catches order-of-magnitude errors</text>
            </>
          )}

          {checkMode === 'parity' && (
            <>
              <rect x="40" y="25" width="400" height="85" rx="6" fill="#1e293b" stroke="#475569" strokeWidth="1.5" />
              <text x="60" y="55" fill="#edf1f5" fontSize="14">Factor 1: 47 (Odd) &ensp;×&ensp; Factor 2: 53 (Odd)</text>
              <text x="60" y="80" fill="#e6c580" fontSize="15" fontWeight="bold">Odd × Odd = ALWAYS ODD &implies; Ends in 1, 3, 5, 7, 9</text>
              <text x="60" y="100" fill="#f87171" fontSize="13">An alleged answer of 2,492 is rejected immediately without calculating!</text>
              <text x="240" y="135" textAnchor="middle" fill="#edf1f5" fontSize="13">Parity is an algebraic invariant that cannot be violated</text>
            </>
          )}

          {checkMode === 'counterexample' && (
            <>
              <rect x="40" y="25" width="400" height="85" rx="6" fill="#1e293b" stroke="#475569" strokeWidth="1.5" />
              <text x="60" y="52" fill="#edf1f5" fontSize="14">Conjecture: P(n) = n² - n + 11 is prime for all n.</text>
              <text x="60" y="74" fill="#94a3b8" fontSize="13">For n = 1, 2, 3, 4, 5 &implies; 11, 13, 17, 23, 31 (all prime!).</text>
              <text x="60" y="96" fill="#e6c580" fontSize="14" fontWeight="bold">At n = 11: 11² - 11 + 11 = 121 = 11 × 11 (Composite!).</text>
              <text x="240" y="135" textAnchor="middle" fill="#f87171" fontSize="13">One counterexample definitively disproves a universal conjecture</text>
            </>
          )}
        </svg>

        <div className="axiom-btn-group" role="group" aria-label="Select check method">
          <button
            type="button"
            className={`axiom-chip ${checkMode === 'bounds' ? 'active' : ''}`}
            onClick={() => setCheckMode('bounds')}
          >
            Bounds [2000, 3000]
          </button>
          <button
            type="button"
            className={`axiom-chip ${checkMode === 'parity' ? 'active' : ''}`}
            onClick={() => setCheckMode('parity')}
          >
            Parity Invariant
          </button>
          <button
            type="button"
            className={`axiom-chip ${checkMode === 'counterexample' ? 'active' : ''}`}
            onClick={() => setCheckMode('counterexample')}
          >
            Counterexample (n = 11)
          </button>
        </div>
        <p role="status">
          {checkMode === 'bounds'
            ? 'Bounding clamps values within a known range before computing individual digits.'
            : checkMode === 'parity'
            ? 'Parity testing uses structural properties of odd and even numbers to rule out arithmetic errors.'
            : 'In mathematics, no amount of positive evidence replaces proof. A single counterexample settles the truth.'}
        </p>
      </figure>
    );
  }

  if (lessonId === 'foundation.decimals.place_value_fractions') {
    const shadedCount = decimalShade === '0.3' ? 30 : decimalShade === '0.35' ? 35 : 70;
    return (
      <figure className="concept-visual">
        <figcaption>10×10 Hundredths Grid: Decimal Place Value Representation</figcaption>
        <svg viewBox="0 0 480 180" role="img" aria-label={`A 10 by 10 grid of 100 hundredths squares, with ${shadedCount} squares shaded representing ${decimalShade}.`}>
          <g transform="translate(40, 10)">
            <rect x="0" y="0" width="150" height="150" fill="#1e293b" stroke="#475569" strokeWidth="2" rx="4" />
            {Array.from({ length: 100 }, (_, i) => {
              const row = Math.floor(i / 10);
              const col = i % 10;
              const isShaded = i < shadedCount;
              return (
                <rect
                  key={i}
                  x={col * 15}
                  y={row * 15}
                  width="14"
                  height="14"
                  rx="1.5"
                  fill={isShaded ? '#e6c580' : '#0d1521'}
                  opacity={isShaded ? 1 : 0.6}
                />
              );
            })}
          </g>
          <g transform="translate(220, 20)">
            <text x="0" y="24" fill="#edf1f5" fontSize="18" fontWeight="bold">
              Decimal: {decimalShade}
            </text>
            <text x="0" y="52" fill="#94a3b8" fontSize="14">
              Hundredths Shaded: <tspan fill="#e6c580" fontWeight="bold">{shadedCount} / 100</tspan>
            </text>
            <text x="0" y="78" fill="#94a3b8" fontSize="14">
              Equivalent Fraction: <tspan fill="#edf1f5">{shadedCount === 30 ? '3/10' : shadedCount === 35 ? '7/20' : '7/10'}</tspan>
            </text>
            <rect x="0" y="96" width="230" height="42" rx="4" fill="#0d1521" stroke="#334155" />
            <text x="12" y="122" fill="#7ad0b1" fontSize="13">
              {decimalShade === '0.35'
                ? '35 hundredths < 70 hundredths (0.7)'
                : decimalShade === '0.3'
                ? '30 hundredths = 3 tenths'
                : '70 hundredths > 35 hundredths (0.35)'}
            </text>
          </g>
        </svg>

        <div className="axiom-btn-group" role="group" aria-label="Select decimal value to shade">
          <button
            type="button"
            className={`axiom-chip ${decimalShade === '0.3' ? 'active' : ''}`}
            onClick={() => setDecimalShade('0.3')}
          >
            0.3 (30 hundredths)
          </button>
          <button
            type="button"
            className={`axiom-chip ${decimalShade === '0.35' ? 'active' : ''}`}
            onClick={() => setDecimalShade('0.35')}
          >
            0.35 (35 hundredths)
          </button>
          <button
            type="button"
            className={`axiom-chip ${decimalShade === '0.7' ? 'active' : ''}`}
            onClick={() => setDecimalShade('0.7')}
          >
            0.7 (70 hundredths)
          </button>
        </div>
        <p role="status">
          {decimalShade === '0.35'
            ? '0.35 is 3 tenths and 5 hundredths (35/100 = 7/20). Note that 0.7 has 70 shaded squares, while 0.35 has only 35.'
            : decimalShade === '0.3'
            ? '0.3 fills exactly 3 full columns (30 squares). Each full column is 1 tenth (10/100 = 1/10).'
            : '0.7 fills 7 full columns (70 squares). It is twice as large as 0.35, proving that decimal magnitude depends on place value, not digit count.'}
        </p>
      </figure>
    );
  }

  if (lessonId === 'foundation.percentages.benchmarks_and_parts') {
    const total = 240;
    const pct = parseInt(percentBenchmark, 10);
    const part = (total * pct) / 100;
    return (
      <figure className="concept-visual">
        <figcaption>Benchmark Percentage Bar (Total: 240 units)</figcaption>
        <svg viewBox="0 0 480 160" role="img" aria-label={`A percentage bar showing ${percentBenchmark}% of 240, which equals ${part} units.`}>
          <rect x="50" y="35" width="380" height="40" rx="4" fill="#1e293b" stroke="#475569" strokeWidth="2" />
          <rect x="50" y="35" width={(380 * pct) / 100} height="40" rx="4" fill="#e6c580" />
          {[0, 25, 50, 75, 100].map((b) => (
            <g key={b} transform={`translate(${50 + (380 * b) / 100}, 0)`}>
              <line x1="0" y1="30" x2="0" y2="80" stroke="#94a3b8" strokeWidth="1.5" strokeDasharray="3 3" />
              <text x="0" y="24" textAnchor="middle" fill="#94a3b8" fontSize="12">
                {b}%
              </text>
              <text x="0" y="98" textAnchor="middle" fill="#edf1f5" fontSize="12">
                {(total * b) / 100}
              </text>
            </g>
          ))}
          <text x="240" y="132" textAnchor="middle" fill="#7ad0b1" fontSize="15" fontWeight="bold">
            {percentBenchmark}% of 240 = {part} units
          </text>
        </svg>

        <div className="axiom-btn-group" role="group" aria-label="Select benchmark percentage">
          <button
            type="button"
            className={`axiom-chip ${percentBenchmark === '25' ? 'active' : ''}`}
            onClick={() => setPercentBenchmark('25')}
          >
            25% (1/4 = 60)
          </button>
          <button
            type="button"
            className={`axiom-chip ${percentBenchmark === '50' ? 'active' : ''}`}
            onClick={() => setPercentBenchmark('50')}
          >
            50% (1/2 = 120)
          </button>
          <button
            type="button"
            className={`axiom-chip ${percentBenchmark === '75' ? 'active' : ''}`}
            onClick={() => setPercentBenchmark('75')}
          >
            75% (3/4 = 180)
          </button>
          <button
            type="button"
            className={`axiom-chip ${percentBenchmark === '10' ? 'active' : ''}`}
            onClick={() => setPercentBenchmark('10')}
          >
            10% (1/10 = 24)
          </button>
        </div>
        <p role="status">
          {percentBenchmark === '75'
            ? '75% is composed of 50% (120) + 25% (60) = 180 units. Mentally decomposing percentages turns difficult calculations into quick additions.'
            : percentBenchmark === '50'
            ? '50% divides the whole directly by 2: 240 ÷ 2 = 120 units.'
            : percentBenchmark === '25'
            ? '25% divides the whole by 4: 240 ÷ 4 = 60 units (half of 50%).'
            : '10% shifts the decimal point left: 240 ÷ 10 = 24 units. 5% is half of that: 12 units.'}
        </p>
      </figure>
    );
  }

  if (lessonId === 'foundation.ratios.proportional_scaling') {
    const a = 3;
    const b = 8;
    const c = a * ratioScale;
    const x = b * ratioScale;
    return (
      <figure className="concept-visual">
        <figcaption>Double Number Line: Proportional Scaling (Ratio 3 : 8)</figcaption>
        <svg viewBox="0 0 480 170" role="img" aria-label={`Double number line showing scale factor ${ratioScale}, mapping 3 to ${c} and 8 to ${x}.`}>
          <line x1="50" y1="45" x2="430" y2="45" stroke="#94a3b8" strokeWidth="2.5" />
          <text x="35" y="49" textAnchor="end" fill="#edf1f5" fontSize="13">
            Drive:
          </text>
          <line x1="50" y1="105" x2="430" y2="105" stroke="#94a3b8" strokeWidth="2.5" />
          <text x="35" y="109" textAnchor="end" fill="#edf1f5" fontSize="13">
            Driven:
          </text>

          <line x1="60" y1="38" x2="60" y2="52" stroke="#edf1f5" strokeWidth="2" />
          <text x="60" y="30" textAnchor="middle" fill="#94a3b8" fontSize="12">0</text>
          <line x1="60" y1="98" x2="60" y2="112" stroke="#edf1f5" strokeWidth="2" />
          <text x="60" y="125" textAnchor="middle" fill="#94a3b8" fontSize="12">0</text>

          <line x1="140" y1="38" x2="140" y2="52" stroke="#e6c580" strokeWidth="2" />
          <text x="140" y="30" textAnchor="middle" fill="#e6c580" fontSize="13" fontWeight="bold">3</text>
          <line x1="140" y1="98" x2="140" y2="112" stroke="#e6c580" strokeWidth="2" />
          <text x="140" y="125" textAnchor="middle" fill="#e6c580" fontSize="13" fontWeight="bold">8</text>

          <line x1="380" y1="38" x2="380" y2="52" stroke="#7ad0b1" strokeWidth="2.5" />
          <text x="380" y="30" textAnchor="middle" fill="#7ad0b1" fontSize="14" fontWeight="bold">{c}</text>
          <line x1="380" y1="98" x2="380" y2="112" stroke="#7ad0b1" strokeWidth="2.5" />
          <text x="380" y="125" textAnchor="middle" fill="#7ad0b1" fontSize="14" fontWeight="bold">{x}</text>

          <path d="M 140 20 Q 260 5 380 20" fill="none" stroke="#7ad0b1" strokeWidth="1.5" strokeDasharray="4 3" />
          <text x="260" y="14" textAnchor="middle" fill="#7ad0b1" fontSize="12" fontWeight="bold">
            × {ratioScale} (Scale Factor)
          </text>
          <path d="M 140 135 Q 260 150 380 135" fill="none" stroke="#7ad0b1" strokeWidth="1.5" strokeDasharray="4 3" />
          <text x="260" y="158" textAnchor="middle" fill="#7ad0b1" fontSize="12" fontWeight="bold">
            × {ratioScale} (Scale Factor)
          </text>
        </svg>

        <div className="axiom-btn-group" role="group" aria-label="Select scale multiplier">
          <button
            type="button"
            className={`axiom-chip ${ratioScale === 2 ? 'active' : ''}`}
            onClick={() => setRatioScale(2)}
          >
            Scale ×2 (6 : 16)
          </button>
          <button
            type="button"
            className={`axiom-chip ${ratioScale === 4 ? 'active' : ''}`}
            onClick={() => setRatioScale(4)}
          >
            Scale ×4 (12 : 32)
          </button>
          <button
            type="button"
            className={`axiom-chip ${ratioScale === 8 ? 'active' : ''}`}
            onClick={() => setRatioScale(8)}
          >
            Scale ×8 (24 : 64)
          </button>
        </div>
        <p role="status">
          Scale factor k = {ratioScale}: 3 × {ratioScale} = {c}, and 8 × {ratioScale} = {x}. Both quantities are multiplied by the same constant factor, preserving the exact 3:8 proportion.
        </p>
      </figure>
    );
  }

  if (lessonId === 'foundation.powers.integer_exponents') {
    const powers = [
      { exp: 3, val: '8', note: '2 × 2 × 2' },
      { exp: 2, val: '4', note: '2 × 2' },
      { exp: 1, val: '2', note: '2' },
      { exp: 0, val: '1', note: 'Multiplicative identity' },
      { exp: -1, val: '1/2', note: 'Reciprocal: 1 / 2¹' },
      { exp: -2, val: '1/4', note: 'Reciprocal: 1 / 2²' },
      { exp: -3, val: '1/8', note: 'Reciprocal: 1 / 2³' },
    ];
    const current = powers.find(p => p.exp === powerExp) || powers[3];
    return (
      <figure className="concept-visual">
        <figcaption>Powers of 2: Moving left divides by 2; moving right multiplies by 2</figcaption>
        <svg viewBox="0 0 520 180" role="img" aria-label={`Power 2 to the power of ${powerExp} is ${current.val}. ${current.note}`}>
          <rect x="10" y="10" width="500" height="160" rx="8" fill="#131b26" stroke="#29394b" strokeWidth="1.5" />
          <line x1="30" y1="90" x2="490" y2="90" stroke="#485c74" strokeWidth="2" />
          {powers.map((p, idx) => {
            const cx = 50 + idx * 70;
            const isSelected = p.exp === powerExp;
            return (
              <g key={p.exp}>
                <line x1={cx} y1="80" x2={cx} y2="100" stroke={isSelected ? '#e6c580' : '#485c74'} strokeWidth={isSelected ? 3 : 1.5} />
                <circle cx={cx} cy="90" r={isSelected ? 9 : 5} fill={isSelected ? '#e6c580' : '#29394b'} stroke={isSelected ? '#f3ddab' : '#64748b'} strokeWidth="2" />
                <text x={cx} y="55" textAnchor="middle" fill={isSelected ? '#e6c580' : '#edf1f5'} fontSize={isSelected ? "14" : "12"} fontWeight={isSelected ? "bold" : "normal"}>
                  2^{p.exp >= 0 ? p.exp : `(${p.exp})`}
                </text>
                <text x={cx} y="130" textAnchor="middle" fill={isSelected ? '#7ad0b1' : '#94a3b8'} fontSize={isSelected ? "14" : "12"} fontWeight={isSelected ? "bold" : "normal"}>
                  {p.val}
                </text>
              </g>
            );
          })}
          <path d="M 40 155 L 480 155" stroke="#7ad0b1" strokeWidth="1" strokeDasharray="3 3" />
          <text x="260" y="170" textAnchor="middle" fill="#7ad0b1" fontSize="11">
            ← Divide by 2 (decrement exponent) | Multiply by 2 (increment exponent) →
          </text>
        </svg>

        <div className="axiom-btn-group" role="group" aria-label="Select power of 2 exponent">
          {powers.map(p => (
            <button
              key={p.exp}
              type="button"
              className={`axiom-chip ${powerExp === p.exp ? 'active' : ''}`}
              onClick={() => setPowerExp(p.exp)}
            >
              2^{p.exp >= 0 ? p.exp : `(${p.exp})`} = {p.val}
            </button>
          ))}
        </div>
        <p role="status">
          <strong>2^{powerExp >= 0 ? powerExp : `(${powerExp})`} = {current.val}</strong>: {current.note}. Notice that 2^0 = 1 because dividing 2 by 2 gives 1. Continuing to divide by 2 yields unit fractions, never negative values!
        </p>
      </figure>
    );
  }

  if (lessonId === 'foundation.powers.scientific_notation') {
    const configs = {
      '45000': { raw: '45,000', m: '4.5', k: 4, places: 4, dir: 'left', formatted: '4.5 × 10⁴' },
      '0.0072': { raw: '0.0072', m: '7.2', k: -3, places: 3, dir: 'right', formatted: '7.2 × 10⁻³' },
      '300000000': { raw: '300,000,000', m: '3.0', k: 8, places: 8, dir: 'left', formatted: '3.0 × 10⁸' },
    };
    const c = configs[sciChoice];
    return (
      <figure className="concept-visual">
        <figcaption>Standard Scientific Notation: Normalizing Significand 1 ≤ |m| &lt; 10</figcaption>
        <svg viewBox="0 0 480 160" role="img" aria-label={`${c.raw} normalized to ${c.formatted}`}>
          <rect x="10" y="10" width="460" height="140" rx="8" fill="#131b26" stroke="#29394b" strokeWidth="1.5" />
          <text x="240" y="45" textAnchor="middle" fill="#94a3b8" fontSize="13">
            Original Value: <tspan fill="#edf1f5" fontWeight="bold">{c.raw}</tspan>
          </text>
          
          <rect x="50" y="65" width="160" height="42" rx="5" fill="#1e293b" stroke="#7ad0b1" strokeWidth="1.5" />
          <text x="130" y="84" textAnchor="middle" fill="#7ad0b1" fontSize="11" fontWeight="bold">Significand m</text>
          <text x="130" y="100" textAnchor="middle" fill="#edf1f5" fontSize="15" fontWeight="bold">{c.m}</text>

          <text x="240" y="93" textAnchor="middle" fill="#edf1f5" fontSize="18" fontWeight="bold">×</text>

          <rect x="270" y="65" width="160" height="42" rx="5" fill="#1e293b" stroke="#e6c580" strokeWidth="1.5" />
          <text x="350" y="84" textAnchor="middle" fill="#e6c580" fontSize="11" fontWeight="bold">Scale Factor 10^k</text>
          <text x="350" y="100" textAnchor="middle" fill="#edf1f5" fontSize="15" fontWeight="bold">10^{c.k >= 0 ? c.k : `(${c.k})`}</text>

          <text x="240" y="135" textAnchor="middle" fill="#38bdf8" fontSize="12">
            Decimal point moved {c.places} places {c.dir} → {c.formatted}
          </text>
        </svg>

        <div className="axiom-btn-group" role="group" aria-label="Select sample number to convert">
          <button
            type="button"
            className={`axiom-chip ${sciChoice === '45000' ? 'active' : ''}`}
            onClick={() => setSciChoice('45000')}
          >
            45,000 (Macro)
          </button>
          <button
            type="button"
            className={`axiom-chip ${sciChoice === '0.0072' ? 'active' : ''}`}
            onClick={() => setSciChoice('0.0072')}
          >
            0.0072 (Micro)
          </button>
          <button
            type="button"
            className={`axiom-chip ${sciChoice === '300000000' ? 'active' : ''}`}
            onClick={() => setSciChoice('300000000')}
          >
            Speed of light (300,000,000 m/s)
          </button>
        </div>
        <p role="status">
          <strong>{c.raw} = {c.formatted}</strong>: The single non-zero leading digit isolates precision ({c.m}), while the power of 10 ({c.k}) records the order of magnitude.
        </p>
      </figure>
    );
  }

  if (lessonId === 'foundation.roots.perfect_squares_and_roots') {
    return (
      <figure className="concept-visual">
        <figcaption>Square Roots: Geometric Area and Non-Distributivity</figcaption>
        {rootDemo === 'fraction' ? (
          <svg viewBox="0 0 480 180" role="img" aria-label="Square of area 49/64 with side length 7/8 inside an 8 by 8 grid">
            <rect x="10" y="10" width="460" height="160" rx="8" fill="#131b26" stroke="#29394b" strokeWidth="1.5" />
            <g transform="translate(180, 25)">
              {Array.from({ length: 8 }).map((_, r) => (
                Array.from({ length: 8 }).map((_, col) => {
                  const isInner = r < 7 && col < 7;
                  return (
                    <rect
                      key={`${r}-${col}`}
                      x={col * 16}
                      y={r * 16}
                      width="15"
                      height="15"
                      rx="1"
                      fill={isInner ? '#e6c580' : '#1e293b'}
                      stroke="#0f172a"
                      strokeWidth="1"
                    />
                  );
                })
              ))}
              <rect x="0" y="0" width="127" height="127" fill="none" stroke="#64748b" strokeWidth="1.5" />
              <rect x="0" y="0" width="111" height="111" fill="none" stroke="#f3ddab" strokeWidth="2" />
            </g>

            <text x="95" y="65" textAnchor="middle" fill="#edf1f5" fontSize="14" fontWeight="bold">
              Area = 49 / 64
            </text>
            <text x="95" y="85" textAnchor="middle" fill="#94a3b8" fontSize="12">
              (49 shaded out of 64)
            </text>
            <text x="95" y="115" textAnchor="middle" fill="#7ad0b1" fontSize="15" fontWeight="bold">
              Side length = 7 / 8
            </text>
            <text x="95" y="135" textAnchor="middle" fill="#7ad0b1" fontSize="11">
              √(49/64) = √49 / √64
            </text>
          </svg>
        ) : (
          <svg viewBox="0 0 480 180" role="img" aria-label="Comparison showing square root of 9+16 equals 5, while root 9 plus root 16 equals 7">
            <rect x="10" y="10" width="460" height="160" rx="8" fill="#131b26" stroke="#29394b" strokeWidth="1.5" />
            
            <rect x="40" y="35" width="80" height="80" rx="4" fill="#29394b" stroke="#7ad0b1" strokeWidth="2" />
            <text x="80" y="70" textAnchor="middle" fill="#7ad0b1" fontSize="13" fontWeight="bold">Area = 25</text>
            <text x="80" y="90" textAnchor="middle" fill="#edf1f5" fontSize="12">(9 + 16)</text>
            <text x="80" y="135" textAnchor="middle" fill="#7ad0b1" fontSize="14" fontWeight="bold">Side = √25 = 5</text>

            <text x="200" y="80" textAnchor="middle" fill="#ef4444" fontSize="24" fontWeight="bold">≠</text>

            <rect x="250" y="45" width="50" height="50" rx="3" fill="#1e293b" stroke="#e6c580" strokeWidth="1.5" />
            <text x="275" y="75" textAnchor="middle" fill="#e6c580" fontSize="11">√9 = 3</text>

            <text x="315" y="75" textAnchor="middle" fill="#edf1f5" fontSize="16" fontWeight="bold">+</text>

            <rect x="335" y="35" width="70" height="70" rx="3" fill="#1e293b" stroke="#e6c580" strokeWidth="1.5" />
            <text x="370" y="75" textAnchor="middle" fill="#e6c580" fontSize="11">√16 = 4</text>

            <text x="340" y="135" textAnchor="middle" fill="#f59e0b" fontSize="14" fontWeight="bold">Sum = 3 + 4 = 7</text>
          </svg>
        )}

        <div className="axiom-btn-group" role="group" aria-label="Select root visual mode">
          <button
            type="button"
            className={`axiom-chip ${rootDemo === 'fraction' ? 'active' : ''}`}
            onClick={() => setRootDemo('fraction')}
          >
            Rational Radical: √(49/64) = 7/8
          </button>
          <button
            type="button"
            className={`axiom-chip ${rootDemo === 'fallacy' ? 'active' : ''}`}
            onClick={() => setRootDemo('fallacy')}
          >
            Additive Trap: √(9+16) ≠ √9 + √16
          </button>
        </div>
        <p role="status">
          {rootDemo === 'fraction'
            ? 'The square of side 7/8 contains exactly 7 × 7 = 49 unit cells out of an 8 × 8 = 64 grid. Thus √(49/64) = 7/8 exactly.'
            : 'Radicals do NOT distribute over addition! √(9 + 16) = √25 = 5, which is strictly less than √9 + √16 = 3 + 4 = 7.'}
        </p>
      </figure>
    );
  }

  if (lessonId === 'foundation.geometry.perimeter_and_area') {
    return (
      <figure className="concept-visual">
        <figcaption>Spatial conservation: boundary length vs surface area</figcaption>
        {geomShape === '6x6_square' && (
          <svg viewBox="0 0 480 200" role="img" aria-label="Square of side 6 with perimeter 24 and area 36">
            <rect x="20" y="20" width="440" height="160" rx="6" fill="#131b26" stroke="#29394b" strokeWidth="1.5" />
            <g transform="translate(60, 40)">
              <rect x="0" y="0" width="120" height="120" rx="3" fill="#1e293b" stroke="#e6c580" strokeWidth="2.5" />
              {Array.from({ length: 5 }, (_, i) => (
                <line key={`v${i}`} x1={(i + 1) * 20} y1="0" x2={(i + 1) * 20} y2="120" stroke="#0f172a" strokeWidth="1" />
              ))}
              {Array.from({ length: 5 }, (_, i) => (
                <line key={`h${i}`} x1="0" y1={(i + 1) * 20} x2="120" y2={(i + 1) * 20} stroke="#0f172a" strokeWidth="1" />
              ))}
              <text x="60" y="-8" textAnchor="middle" fill="#edf1f5" fontSize="12">width = 6</text>
              <text x="-10" y="65" textAnchor="end" fill="#edf1f5" fontSize="12">height = 6</text>
            </g>
            <g transform="translate(230, 60)">
              <text x="0" y="20" fill="#f3ddab" fontSize="16" fontWeight="bold">6 × 6 Square</text>
              <text x="0" y="50" fill="#edf1f5" fontSize="13">Perimeter: 2 × (6 + 6) = 24 units</text>
              <text x="0" y="75" fill="#7ad0b1" fontSize="15" fontWeight="bold">Area: 6 × 6 = 36 square units</text>
              <text x="0" y="100" fill="#94a3b8" fontSize="11">Maximum area enclosed for perimeter 24.</text>
            </g>
          </svg>
        )}
        {geomShape === '2x10_rectangle' && (
          <svg viewBox="0 0 480 200" role="img" aria-label="Elongated rectangle 10 by 2 with perimeter 24 and area 20">
            <rect x="20" y="20" width="440" height="160" rx="6" fill="#131b26" stroke="#29394b" strokeWidth="1.5" />
            <g transform="translate(40, 75)">
              <rect x="0" y="0" width="160" height="32" rx="2" fill="#1e293b" stroke="#e6c580" strokeWidth="2" />
              {Array.from({ length: 9 }, (_, i) => (
                <line key={`v${i}`} x1={(i + 1) * 16} y1="0" x2={(i + 1) * 16} y2="32" stroke="#0f172a" strokeWidth="1" />
              ))}
              <line x1="0" y1="16" x2="160" y2="16" stroke="#0f172a" strokeWidth="1" />
              <text x="80" y="-8" textAnchor="middle" fill="#edf1f5" fontSize="12">width = 10</text>
              <text x="-8" y="20" textAnchor="end" fill="#edf1f5" fontSize="12">height = 2</text>
            </g>
            <g transform="translate(230, 60)">
              <text x="0" y="20" fill="#f3ddab" fontSize="16" fontWeight="bold">10 × 2 Rectangle</text>
              <text x="0" y="50" fill="#edf1f5" fontSize="13">Perimeter: 2 × (10 + 2) = 24 units (Same!)</text>
              <text x="0" y="75" fill="#f59e0b" fontSize="15" fontWeight="bold">Area: 10 × 2 = 20 square units</text>
              <text x="0" y="100" fill="#94a3b8" fontSize="11">Same perimeter, but 44% less area!</text>
            </g>
          </svg>
        )}
        {geomShape === 'triangle_8x5' && (
          <svg viewBox="0 0 480 200" role="img" aria-label="Triangle with base 8 and altitude 5 showing area half of bounding rectangle">
            <rect x="20" y="20" width="440" height="160" rx="6" fill="#131b26" stroke="#29394b" strokeWidth="1.5" />
            <g transform="translate(50, 45)">
              <rect x="0" y="0" width="160" height="100" fill="none" stroke="#64748b" strokeWidth="1.5" strokeDasharray="4 4" />
              <polygon points="0,100 100,0 160,100" fill="rgba(230, 197, 128, 0.25)" stroke="#e6c580" strokeWidth="2.5" />
              <line x1="100" y1="0" x2="100" y2="100" stroke="#7ad0b1" strokeWidth="2" strokeDasharray="3 3" />
              <rect x="92" y="92" width="8" height="8" fill="none" stroke="#7ad0b1" strokeWidth="1" />
              <text x="80" y="118" textAnchor="middle" fill="#edf1f5" fontSize="12">base = 8</text>
              <text x="110" y="50" fill="#7ad0b1" fontSize="12">h = 5</text>
            </g>
            <g transform="translate(240, 60)">
              <text x="0" y="20" fill="#f3ddab" fontSize="16" fontWeight="bold">Triangle Dissection</text>
              <text x="0" y="50" fill="#edf1f5" fontSize="13">Bounding Box Area: 8 × 5 = 40</text>
              <text x="0" y="75" fill="#7ad0b1" fontSize="15" fontWeight="bold">Triangle Area: ½ × 8 × 5 = 20</text>
              <text x="0" y="100" fill="#94a3b8" fontSize="11">Every triangle covers exactly half its bounding box.</text>
            </g>
          </svg>
        )}

        <div className="axiom-btn-group" role="group" aria-label="Select shape comparison">
          <button
            type="button"
            className={`axiom-chip ${geomShape === '6x6_square' ? 'active' : ''}`}
            onClick={() => setGeomShape('6x6_square')}
          >
            6 × 6 Square (P=24, A=36)
          </button>
          <button
            type="button"
            className={`axiom-chip ${geomShape === '2x10_rectangle' ? 'active' : ''}`}
            onClick={() => setGeomShape('2x10_rectangle')}
          >
            10 × 2 Rectangle (P=24, A=20)
          </button>
          <button
            type="button"
            className={`axiom-chip ${geomShape === 'triangle_8x5' ? 'active' : ''}`}
            onClick={() => setGeomShape('triangle_8x5')}
          >
            Triangle (Base 8, Height 5, A=20)
          </button>
        </div>
        <p role="status">
          {geomShape === '6x6_square' && 'The 6 × 6 square has perimeter 24 and encloses 36 square units. The square maximizes area for a fixed perimeter.'}
          {geomShape === '2x10_rectangle' && 'The 10 × 2 rectangle also has perimeter 24, but encloses only 20 square units. Perimeter does not determine area!'}
          {geomShape === 'triangle_8x5' && 'A triangle of base 8 and height 5 occupies exactly half the 40-unit area of the enclosing rectangle, giving Area = 20.'}
        </p>
      </figure>
    );
  }

  if (lessonId === 'foundation.geometry.angles_and_triangles') {
    return (
      <figure className="concept-visual">
        <figcaption>Angle invariants: parallel transversals and triangle angle sum</figcaption>
        {angleDemo === 'triangle_sum' ? (
          <svg viewBox="0 0 480 200" role="img" aria-label="Triangle with top parallel line showing three angles summing to a 180 degree straight line">
            <rect x="20" y="20" width="440" height="160" rx="6" fill="#131b26" stroke="#29394b" strokeWidth="1.5" />
            <line x1="50" y1="50" x2="430" y2="50" stroke="#f59e0b" strokeWidth="1.5" strokeDasharray="5 5" />
            <text x="435" y="54" fill="#f59e0b" fontSize="10">Line ∥ Base</text>

            <polygon points="240,50 110,160 370,160" fill="rgba(230, 197, 128, 0.15)" stroke="#e6c580" strokeWidth="2.5" />

            <circle cx="240" cy="50" r="14" fill="none" stroke="#fbbf24" strokeWidth="2" />
            <text x="240" y="78" textAnchor="middle" fill="#fbbf24" fontSize="12" fontWeight="bold">α = 50°</text>

            <circle cx="110" cy="160" r="16" fill="none" stroke="#60a5fa" strokeWidth="2" />
            <text x="135" y="152" fill="#60a5fa" fontSize="12" fontWeight="bold">β = 65°</text>

            <circle cx="370" cy="160" r="16" fill="none" stroke="#34d399" strokeWidth="2" />
            <text x="330" y="152" fill="#34d399" fontSize="12" fontWeight="bold">γ = 65°</text>

            <path d="M 215,50 A 25 25 0 0 1 228,32" fill="none" stroke="#60a5fa" strokeWidth="2" />
            <text x="195" y="42" fill="#60a5fa" fontSize="11" fontWeight="bold">β = 65°</text>

            <path d="M 252,32 A 25 25 0 0 1 265,50" fill="none" stroke="#34d399" strokeWidth="2" />
            <text x="275" y="42" fill="#34d399" fontSize="11" fontWeight="bold">γ = 65°</text>

            <text x="240" y="186" textAnchor="middle" fill="#edf1f5" fontSize="13" fontWeight="bold">
              Along straight line: 65° + 50° + 65° = 180°
            </text>
          </svg>
        ) : (
          <svg viewBox="0 0 480 200" role="img" aria-label="Two parallel lines cut by a transversal showing equal alternate interior angles">
            <rect x="20" y="20" width="440" height="160" rx="6" fill="#131b26" stroke="#29394b" strokeWidth="1.5" />
            <line x1="40" y1="70" x2="440" y2="70" stroke="#94a3b8" strokeWidth="2" />
            <line x1="40" y1="140" x2="440" y2="140" stroke="#94a3b8" strokeWidth="2" />
            <text x="445" y="74" fill="#94a3b8" fontSize="11">L₁</text>
            <text x="445" y="144" fill="#94a3b8" fontSize="11">L₂ ∥ L₁</text>

            <line x1="120" y1="170" x2="360" y2="40" stroke="#e6c580" strokeWidth="2.5" />

            <circle cx="280" cy="70" r="14" fill="none" stroke="#34d399" strokeWidth="2" />
            <text x="250" y="90" fill="#34d399" fontSize="12" fontWeight="bold">65°</text>

            <circle cx="200" cy="140" r="14" fill="none" stroke="#34d399" strokeWidth="2" />
            <text x="225" y="130" fill="#34d399" fontSize="12" fontWeight="bold">65°</text>

            <circle cx="280" cy="70" r="18" fill="none" stroke="#60a5fa" strokeWidth="2" strokeDasharray="3 3" />
            <text x="305" y="65" fill="#60a5fa" fontSize="11">115°</text>

            <text x="240" y="185" textAnchor="middle" fill="#edf1f5" fontSize="13">
              Alternate interior angles are equal: 65° = 65°. Linear pair: 65° + 115° = 180°.
            </text>
          </svg>
        )}

        <div className="axiom-btn-group" role="group" aria-label="Select angle visual mode">
          <button
            type="button"
            className={`axiom-chip ${angleDemo === 'triangle_sum' ? 'active' : ''}`}
            onClick={() => setAngleDemo('triangle_sum')}
          >
            Triangle Sum: α + β + γ = 180°
          </button>
          <button
            type="button"
            className={`axiom-chip ${angleDemo === 'parallel_transversal' ? 'active' : ''}`}
            onClick={() => setAngleDemo('parallel_transversal')}
          >
            Parallel Lines & Transversal
          </button>
        </div>
        <p role="status">
          {angleDemo === 'triangle_sum'
            ? 'Drawing a line through the top vertex parallel to the base creates alternate interior angles equal to the base angles. Together they line up along the straight line to equal 180°.'
            : 'When a transversal cuts two parallel lines, alternate interior angles are congruent (65° = 65°), and consecutive angles form a supplementary pair summing to 180°.'}
        </p>
      </figure>
    );
  }

  if (lessonId === 'foundation.geometry.coordinate_plane') {
    return (
      <figure className="concept-visual">
        <figcaption>The Cartesian grid: coordinates and Euclidean distance</figcaption>
        <svg viewBox="0 0 480 200" role="img" aria-label="Cartesian plane showing distance triangle on lattice grid">
          <rect x="20" y="20" width="440" height="160" rx="6" fill="#131b26" stroke="#29394b" strokeWidth="1.5" />
          
          <g transform="translate(140, 110)">
            {Array.from({ length: 13 }, (_, i) => (
              <line key={`gx${i}`} x1={(i - 6) * 20} y1="-80" x2={(i - 6) * 20} y2="60" stroke="#1e293b" strokeWidth="1" />
            ))}
            {Array.from({ length: 8 }, (_, i) => (
              <line key={`gy${i}`} x1="-120" y1={(i - 4) * 20} x2="120" y2={(i - 4) * 20} stroke="#1e293b" strokeWidth="1" />
            ))}

            <line x1="-120" y1="0" x2="120" y2="0" stroke="#64748b" strokeWidth="1.5" />
            <line x1="0" y1="-80" x2="0" y2="60" stroke="#64748b" strokeWidth="1.5" />
            <text x="110" y="-5" fill="#94a3b8" fontSize="10">x</text>
            <text x="5" y="-70" fill="#94a3b8" fontSize="10">y</text>
            <circle cx="0" cy="0" r="3" fill="#edf1f5" />
            <text x="-12" y="12" fill="#94a3b8" fontSize="10">(0,0)</text>

            {coordTarget === 'point_3_4' && (
              <>
                <line x1="0" y1="0" x2="60" y2="0" stroke="#60a5fa" strokeWidth="2.5" />
                <line x1="60" y1="0" x2="60" y2="-80" stroke="#f59e0b" strokeWidth="2.5" />
                <line x1="0" y1="0" x2="60" y2="-80" stroke="#7ad0b1" strokeWidth="3" />
                <circle cx="60" cy="-80" r="4" fill="#7ad0b1" />
                <text x="68" y="-82" fill="#7ad0b1" fontSize="12" fontWeight="bold">(3, 4)</text>
                <text x="30" y="14" textAnchor="middle" fill="#60a5fa" fontSize="11">Δx = 3</text>
                <text x="75" y="-40" fill="#f59e0b" fontSize="11">Δy = 4</text>
              </>
            )}

            {coordTarget === 'point_5_12' && (
              <>
                <line x1="0" y1="0" x2="100" y2="0" stroke="#60a5fa" strokeWidth="2" />
                <line x1="100" y1="0" x2="100" y2="-60" stroke="#f59e0b" strokeWidth="2" />
                <line x1="0" y1="0" x2="100" y2="-60" stroke="#7ad0b1" strokeWidth="2.5" />
                <circle cx="100" cy="-60" r="4" fill="#7ad0b1" />
                <text x="65" y="-68" fill="#7ad0b1" fontSize="12" fontWeight="bold">(5, 12)</text>
                <text x="50" y="14" textAnchor="middle" fill="#60a5fa" fontSize="11">Δx = 5</text>
                <text x="108" y="-30" fill="#f59e0b" fontSize="11">Δy = 12</text>
              </>
            )}

            {coordTarget === 'axis_aligned' && (
              <>
                <line x1="-60" y1="-40" x2="100" y2="-40" stroke="#7ad0b1" strokeWidth="3" />
                <circle cx="-60" cy="-40" r="4" fill="#7ad0b1" />
                <circle cx="100" cy="-40" r="4" fill="#7ad0b1" />
                <text x="-60" y="-48" textAnchor="middle" fill="#edf1f5" fontSize="11">(−3, 2)</text>
                <text x="100" y="-48" textAnchor="middle" fill="#edf1f5" fontSize="11">(5, 2)</text>
                <text x="20" y="-28" textAnchor="middle" fill="#7ad0b1" fontSize="12" fontWeight="bold">d = |5 − (−3)| = 8</text>
              </>
            )}
          </g>

          <g transform="translate(290, 50)">
            <text x="0" y="20" fill="#f3ddab" fontSize="15" fontWeight="bold">Distance Formula</text>
            {coordTarget === 'point_3_4' && (
              <>
                <text x="0" y="45" fill="#edf1f5" fontSize="13">d = √(3² + 4²)</text>
                <text x="0" y="70" fill="#edf1f5" fontSize="13">= √(9 + 16) = √25</text>
                <text x="0" y="100" fill="#7ad0b1" fontSize="16" fontWeight="bold">d = 5 units exactly</text>
                <text x="0" y="125" fill="#94a3b8" fontSize="11">Pythagorean triple (3, 4, 5)</text>
              </>
            )}
            {coordTarget === 'point_5_12' && (
              <>
                <text x="0" y="45" fill="#edf1f5" fontSize="13">d = √(5² + 12²)</text>
                <text x="0" y="70" fill="#edf1f5" fontSize="13">= √(25 + 144) = √169</text>
                <text x="0" y="100" fill="#7ad0b1" fontSize="16" fontWeight="bold">d = 13 units exactly</text>
                <text x="0" y="125" fill="#94a3b8" fontSize="11">Pythagorean triple (5, 12, 13)</text>
              </>
            )}
            {coordTarget === 'axis_aligned' && (
              <>
                <text x="0" y="45" fill="#edf1f5" fontSize="13">Same y-coordinate (y = 2)</text>
                <text x="0" y="70" fill="#edf1f5" fontSize="13">d = |x₂ − x₁| = |5 − (−3)|</text>
                <text x="0" y="100" fill="#7ad0b1" fontSize="16" fontWeight="bold">d = 8 units</text>
                <text x="0" y="125" fill="#94a3b8" fontSize="11">Direct horizontal absolute displacement</text>
              </>
            )}
          </g>
        </svg>

        <div className="axiom-btn-group" role="group" aria-label="Select coordinate target">
          <button
            type="button"
            className={`axiom-chip ${coordTarget === 'point_3_4' ? 'active' : ''}`}
            onClick={() => setCoordTarget('point_3_4')}
          >
            (3, 4) Distance = 5
          </button>
          <button
            type="button"
            className={`axiom-chip ${coordTarget === 'point_5_12' ? 'active' : ''}`}
            onClick={() => setCoordTarget('point_5_12')}
          >
            (5, 12) Distance = 13
          </button>
          <button
            type="button"
            className={`axiom-chip ${coordTarget === 'axis_aligned' ? 'active' : ''}`}
            onClick={() => setCoordTarget('axis_aligned')}
          >
            Axis-Aligned (−3, 2) to (5, 2)
          </button>
        </div>
        <p role="status">
          {coordTarget === 'point_3_4' && 'Horizontal displacement 3 and vertical displacement 4 form the legs of a right triangle. The Euclidean hypotenuse is √(3² + 4²) = 5.'}
          {coordTarget === 'point_5_12' && 'With displacements 5 and 12, the sum of squares is 25 + 144 = 169. The square root yields an exact Euclidean distance of 13.'}
          {coordTarget === 'axis_aligned' && 'When two points share a coordinate line, distance is simply the absolute difference of their coordinates: |5 − (−3)| = 8.'}
        </p>
      </figure>
    );
  }

  if (lessonId === 'foundation.data.center_and_spread') {
    const pts = dataOutlier ? [8, 10, 12, 14, 66] : [8, 10, 12, 14, 16];
    const meanVal = dataOutlier ? 22 : 12;
    const medianVal = 12;
    const scaleX = (v: number) => 40 + ((v - 6) / 64) * 400;
    return (
      <figure className="concept-visual">
        <figcaption>Mean as a physical balance fulcrum vs. rank-ordered Median</figcaption>
        <svg viewBox="0 0 480 185" role="img" aria-label={dataOutlier ? 'Number line with outlier 66 shifting mean fulcrum to 22 while median stays at 12' : 'Symmetric dataset 8, 10, 12, 14, 16 balanced at mean 12 and median 12'}>
          <rect x="15" y="15" width="450" height="155" rx="6" fill="#131b26" stroke="#29394b" strokeWidth="1.5" />
          <line x1="40" y1="105" x2="440" y2="105" stroke="#94a3b8" strokeWidth="3" />
          {pts.map((v, idx) => {
            const cx = scaleX(v);
            const isMed = idx === 2;
            return (
              <g key={idx}>
                <circle cx={cx} cy="92" r="9" fill={isMed ? '#7ad0b1' : '#e6c580'} stroke="#0f172a" strokeWidth="1.5" />
                <text x={cx} y="74" textAnchor="middle" fill={isMed ? '#7ad0b1' : '#edf1f5'} fontSize="11" fontWeight="bold">{v}</text>
              </g>
            );
          })}
          {/* Fulcrum triangle at Mean */}
          <polygon
            points={`${scaleX(meanVal)},107 ${scaleX(meanVal) - 12},132 ${scaleX(meanVal) + 12},132`}
            fill="#f59e0b"
          />
          <text x={scaleX(meanVal)} y="148" textAnchor="middle" fill="#f59e0b" fontSize="12" fontWeight="bold">
            Mean Fulcrum = {meanVal}
          </text>
          <text x={scaleX(medianVal)} y="46" textAnchor="middle" fill="#7ad0b1" fontSize="12" fontWeight="bold">
            Median (3rd sorted) = {medianVal}
          </text>
        </svg>
        <div className="axiom-btn-group" role="group" aria-label="Toggle dataset outlier">
          <button
            type="button"
            className={`axiom-chip ${!dataOutlier ? 'active' : ''}`}
            onClick={() => setDataOutlier(false)}
          >
            Balanced: {'{8, 10, 12, 14, 16}'}
          </button>
          <button
            type="button"
            className={`axiom-chip ${dataOutlier ? 'active' : ''}`}
            onClick={() => setDataOutlier(true)}
          >
            With Outlier: {'{8, 10, 12, 14, 66}'}
          </button>
        </div>
        <p role="status">
          {dataOutlier
            ? 'Replacing 16 with 66 increases the total sum from 60 to 110, pulling the Mean balance fulcrum to 110 ÷ 5 = 22. The middle sorted value (Median = 12) is unaffected!'
            : 'In {8, 10, 12, 14, 16}, signed deviations from 12 are (−4) + (−2) + 0 + 2 + 4 = 0. Both Mean and Median equal 12.'}
        </p>
      </figure>
    );
  }

  if (lessonId === 'foundation.counting.sample_spaces') {
    return (
      <figure className="concept-visual">
        <figcaption>Permutations vs. Combinations: Why dividing by k! collapses orderings</figcaption>
        <svg viewBox="0 0 480 185" role="img" aria-label={countingMode === 'permutation' ? 'Ordered podium slots showing 6 times 5 times 4 equals 120 permutations' : 'Six ordered arrangements of A, B, C collapsing into 1 unordered committee'}>
          <rect x="15" y="15" width="450" height="155" rx="6" fill="#131b26" stroke="#29394b" strokeWidth="1.5" />
          {countingMode === 'permutation' ? (
            <>
              <rect x="45" y="45" width="110" height="65" rx="5" fill="#1e293b" stroke="#e6c580" strokeWidth="2" />
              <text x="100" y="68" textAnchor="middle" fill="#e6c580" fontSize="12" fontWeight="bold">1st Place (Gold)</text>
              <text x="100" y="92" textAnchor="middle" fill="#edf1f5" fontSize="16" fontWeight="bold">6 choices</text>

              <text x="172" y="83" textAnchor="middle" fill="#94a3b8" fontSize="18" fontWeight="bold">×</text>

              <rect x="190" y="45" width="110" height="65" rx="5" fill="#1e293b" stroke="#60a5fa" strokeWidth="2" />
              <text x="245" y="68" textAnchor="middle" fill="#60a5fa" fontSize="12" fontWeight="bold">2nd Place (Silver)</text>
              <text x="245" y="92" textAnchor="middle" fill="#edf1f5" fontSize="16" fontWeight="bold">5 remaining</text>

              <text x="317" y="83" textAnchor="middle" fill="#94a3b8" fontSize="18" fontWeight="bold">×</text>

              <rect x="335" y="45" width="110" height="65" rx="5" fill="#1e293b" stroke="#7ad0b1" strokeWidth="2" />
              <text x="390" y="68" textAnchor="middle" fill="#7ad0b1" fontSize="12" fontWeight="bold">3rd Place (Bronze)</text>
              <text x="390" y="92" textAnchor="middle" fill="#edf1f5" fontSize="16" fontWeight="bold">4 remaining</text>

              <text x="240" y="145" textAnchor="middle" fill="#f3ddab" fontSize="14" fontWeight="bold">
                Ordered Permutations P(6, 3) = 6 × 5 × 4 = 120 podiums
              </text>
            </>
          ) : (
            <>
              <g transform="translate(35, 35)">
                {['ABC', 'ACB', 'BAC', 'BCA', 'CAB', 'CBA'].map((perm, idx) => (
                  <g key={perm} transform={`translate(${(idx % 3) * 72}, ${Math.floor(idx / 3) * 42})`}>
                    <rect x="0" y="0" width="62" height="32" rx="4" fill="#1e293b" stroke="#64748b" strokeWidth="1.5" />
                    <text x="31" y="21" textAnchor="middle" fill="#edf1f5" fontSize="13" fontWeight="bold">{perm}</text>
                  </g>
                ))}
                <text x="102" y="105" textAnchor="middle" fill="#94a3b8" fontSize="11">3! = 6 ordered lists of the same 3 people</text>
              </g>
              <text x="272" y="82" textAnchor="middle" fill="#e6c580" fontSize="20" fontWeight="bold">÷ 3! →</text>
              <rect x="315" y="45" width="130" height="68" rx="6" fill="#1e293b" stroke="#7ad0b1" strokeWidth="2.5" />
              <text x="380" y="72" textAnchor="middle" fill="#7ad0b1" fontSize="13" fontWeight="bold">1 Unordered Set</text>
              <text x="380" y="95" textAnchor="middle" fill="#edf1f5" fontSize="15" fontWeight="bold">{'{A, B, C}'}</text>
              <text x="240" y="152" textAnchor="middle" fill="#7ad0b1" fontSize="14" fontWeight="bold">
                Combinations C(6, 3) = 120 ÷ 6 = 20 distinct committees
              </text>
            </>
          )}
        </svg>
        <div className="axiom-btn-group" role="group" aria-label="Select counting structure">
          <button
            type="button"
            className={`axiom-chip ${countingMode === 'permutation' ? 'active' : ''}`}
            onClick={() => setCountingMode('permutation')}
          >
            Ordered Podium: P(6, 3) = 120
          </button>
          <button
            type="button"
            className={`axiom-chip ${countingMode === 'combination' ? 'active' : ''}`}
            onClick={() => setCountingMode('combination')}
          >
            Unordered Committee: C(6, 3) = 20
          </button>
        </div>
        <p role="status">
          {countingMode === 'permutation'
            ? 'When order matters (Gold, Silver, Bronze), each stage multiplies the remaining choices: 6 × 5 × 4 = 120.'
            : 'When order does not matter, all 3! = 6 rearrangements of {A, B, C} form the same committee, so we divide 120 by 6 to get 20.'}
        </p>
      </figure>
    );
  }

  if (lessonId === 'foundation.chance.exact_probability') {
    return (
      <figure className="concept-visual">
        <figcaption>Exact Rational Probability over ℚ: Complements and Conditional Draws</figcaption>
        <svg viewBox="0 0 480 185" role="img" aria-label={probMode === 'complement' ? 'Sample space bar of 8 marbles partitioned into 3 red and 5 blue' : 'Two-step draw without replacement from 4 gold and 6 silver coins'}>
          <rect x="15" y="15" width="450" height="155" rx="6" fill="#131b26" stroke="#29394b" strokeWidth="1.5" />
          {probMode === 'complement' ? (
            <>
              <text x="240" y="42" textAnchor="middle" fill="#edf1f5" fontSize="13" fontWeight="bold">
                Sample Space Ω: 8 Equally Likely Marbles (3 Red, 5 Blue)
              </text>
              <g transform="translate(40, 58)">
                {Array.from({ length: 8 }, (_, i) => (
                  <rect
                    key={i}
                    x={i * 50}
                    y="0"
                    width="48"
                    height="44"
                    rx="4"
                    fill={i < 3 ? '#ef4444' : '#3b82f6'}
                    stroke="#0f172a"
                    strokeWidth="1.5"
                  />
                ))}
              </g>
              <text x="115" y="125" textAnchor="middle" fill="#f87171" fontSize="13" fontWeight="bold">
                P(Red) = 3/8
              </text>
              <text x="315" y="125" textAnchor="middle" fill="#60a5fa" fontSize="13" fontWeight="bold">
                P(not Red) = 1 − 3/8 = 5/8
              </text>
              <text x="240" y="152" textAnchor="middle" fill="#7ad0b1" fontSize="12">
                Exact Partition Identity: 3/8 + 5/8 = 8/8 = 1
              </text>
            </>
          ) : (
            <>
              <rect x="40" y="42" width="165" height="72" rx="5" fill="#1e293b" stroke="#e6c580" strokeWidth="2" />
              <text x="122" y="65" textAnchor="middle" fill="#e6c580" fontSize="12" fontWeight="bold">1st Draw (Initial Pouch)</text>
              <text x="122" y="86" textAnchor="middle" fill="#edf1f5" fontSize="13">4 Gold out of 10 Total</text>
              <text x="122" y="105" textAnchor="middle" fill="#7ad0b1" fontSize="13" fontWeight="bold">P(G₁) = 4/10 = 2/5</text>

              <text x="240" y="82" textAnchor="middle" fill="#edf1f5" fontSize="18" fontWeight="bold">×</text>

              <rect x="275" y="42" width="165" height="72" rx="5" fill="#1e293b" stroke="#60a5fa" strokeWidth="2" />
              <text x="357" y="65" textAnchor="middle" fill="#60a5fa" fontSize="12" fontWeight="bold">2nd Draw (No Replacement)</text>
              <text x="357" y="86" textAnchor="middle" fill="#edf1f5" fontSize="13">3 Gold out of 9 Remaining</text>
              <text x="357" y="105" textAnchor="middle" fill="#7ad0b1" fontSize="13" fontWeight="bold">P(G₂ | G₁) = 3/9 = 1/3</text>

              <text x="240" y="148" textAnchor="middle" fill="#f3ddab" fontSize="14" fontWeight="bold">
                P(Both Gold) = (4/10) × (3/9) = 12/90 = 2/15
              </text>
            </>
          )}
        </svg>
        <div className="axiom-btn-group" role="group" aria-label="Select probability model">
          <button
            type="button"
            className={`axiom-chip ${probMode === 'complement' ? 'active' : ''}`}
            onClick={() => setProbMode('complement')}
          >
            Complement Rule: P(E) + P(not E) = 1
          </button>
          <button
            type="button"
            className={`axiom-chip ${probMode === 'without_replacement' ? 'active' : ''}`}
            onClick={() => setProbMode('without_replacement')}
          >
            Without Replacement: (4/10) × (3/9)
          </button>
        </div>
        <p role="status">
          {probMode === 'complement'
            ? 'The denominator of a probability is always the entire sample space (3 + 5 = 8), never just the unfavorable outcomes.'
            : 'Without replacement, removing 1 gold coin on the first draw reduces both the remaining gold coins (4 → 3) and total coins (10 → 9).'}
        </p>
      </figure>
    );
  }

  if (lessonId === 'secondary.algebra.expressions_and_substitution') {
    return (
      <figure className="concept-visual">
        <figcaption>Symbolic Placeholders: Negative Substitution &amp; Distributive Sign Rules</figcaption>
        <svg viewBox="0 0 480 185" role="img" aria-label={algEvalMode === 'negative_square' ? 'Comparison of (-3)^2 = +9 versus -3^2 = -9' : 'Distributive expansion of 4(2x + 3) - 3(x - 5) = 5x + 27'}>
          <rect x="15" y="15" width="450" height="155" rx="6" fill="#131b26" stroke="#29394b" strokeWidth="1.5" />
          {algEvalMode === 'negative_square' ? (
            <>
              <rect x="35" y="38" width="185" height="92" rx="6" fill="#1e293b" stroke="#7ad0b1" strokeWidth="2" />
              <text x="127" y="62" textAnchor="middle" fill="#7ad0b1" fontSize="13" fontWeight="bold">Correct Substitution (x = −3)</text>
              <text x="127" y="88" textAnchor="middle" fill="#edf1f5" fontSize="15" fontWeight="bold">x² → (−3)² = (−3)(−3)</text>
              <text x="127" y="114" textAnchor="middle" fill="#7ad0b1" fontSize="16" fontWeight="bold">= +9</text>

              <rect x="260" y="38" width="185" height="92" rx="6" fill="#1e293b" stroke="#ef4444" strokeWidth="2" />
              <text x="352" y="62" textAnchor="middle" fill="#f87171" fontSize="13" fontWeight="bold">Unparenthesized Trap</text>
              <text x="352" y="88" textAnchor="middle" fill="#edf1f5" fontSize="15" fontWeight="bold">−3² = −(3 × 3)</text>
              <text x="352" y="114" textAnchor="middle" fill="#f87171" fontSize="16" fontWeight="bold">= −9 (Sign Error!)</text>

              <text x="240" y="154" textAnchor="middle" fill="#e6c580" fontSize="12">
                A variable x is a grouped box: squaring x squares its sign as well.
              </text>
            </>
          ) : (
            <>
              <text x="240" y="48" textAnchor="middle" fill="#edf1f5" fontSize="15" fontWeight="bold">
                4(2x + 3) − 3(x − 5)
              </text>
              <rect x="45" y="64" width="175" height="50" rx="5" fill="#1e293b" stroke="#60a5fa" strokeWidth="1.5" />
              <text x="132" y="85" textAnchor="middle" fill="#60a5fa" fontSize="12">First Group: 4 × 2x + 4 × 3</text>
              <text x="132" y="104" textAnchor="middle" fill="#edf1f5" fontSize="14" fontWeight="bold">= 8x + 12</text>

              <rect x="260" y="64" width="175" height="50" rx="5" fill="#1e293b" stroke="#e6c580" strokeWidth="1.5" />
              <text x="347" y="85" textAnchor="middle" fill="#e6c580" fontSize="12">Second Group: −3 × x + (−3)(−5)</text>
              <text x="347" y="104" textAnchor="middle" fill="#edf1f5" fontSize="14" fontWeight="bold">= −3x + 15</text>

              <text x="240" y="145" textAnchor="middle" fill="#7ad0b1" fontSize="15" fontWeight="bold">
                Combine Like Terms: (8 − 3)x + (12 + 15) = 5x + 27
              </text>
            </>
          )}
        </svg>
        <div className="axiom-btn-group" role="group" aria-label="Select algebraic expression concept">
          <button
            type="button"
            className={`axiom-chip ${algEvalMode === 'negative_square' ? 'active' : ''}`}
            onClick={() => setAlgEvalMode('negative_square')}
          >
            Negative Base: (−3)² = +9
          </button>
          <button
            type="button"
            className={`axiom-chip ${algEvalMode === 'distributive_minus' ? 'active' : ''}`}
            onClick={() => setAlgEvalMode('distributive_minus')}
          >
            Distributing Minus: −3(x − 5) = −3x + 15
          </button>
        </div>
        <p role="status">
          {algEvalMode === 'negative_square'
            ? 'Substituting x = −3 into 2x² + 3x − 1 gives 2(−3)² + 3(−3) − 1 = 2(9) − 9 − 1 = 8.'
            : 'Distributing −3 across (x − 5) multiplies both terms by −3, turning −5 into +15.'}
        </p>
      </figure>
    );
  }

  if (lessonId === 'secondary.algebra.linear_equations') {
    return (
      <figure className="concept-visual">
        <figcaption>Equivalence Transformations: Isolating x While Preserving Balance</figcaption>
        <svg viewBox="0 0 480 185" role="img" aria-label={`Step ${eqStep} of solving 7x + 4 = 3x + 20`}>
          <rect x="15" y="15" width="450" height="155" rx="6" fill="#131b26" stroke="#29394b" strokeWidth="1.5" />
          <line x1="90" y1="125" x2="390" y2="125" stroke="#e6c580" strokeWidth="4" />
          <polygon points="240,125 225,152 255,152" fill="#e6c580" />
          <text x="240" y="116" textAnchor="middle" fill="#f3ddab" fontSize="18" fontWeight="bold">=</text>

          <rect x="75" y="55" width="140" height="58" rx="6" fill="#1e293b" stroke="#60a5fa" strokeWidth="2" />
          <text x="145" y="78" textAnchor="middle" fill="#94a3b8" fontSize="11">Left Pan</text>
          <text x="145" y="100" textAnchor="middle" fill="#edf1f5" fontSize="16" fontWeight="bold">
            {eqStep === 1 ? '7x + 4' : eqStep === 2 ? '4x + 4' : 'x'}
          </text>

          <rect x="265" y="55" width="140" height="58" rx="6" fill="#1e293b" stroke="#7ad0b1" strokeWidth="2" />
          <text x="335" y="78" textAnchor="middle" fill="#94a3b8" fontSize="11">Right Pan</text>
          <text x="335" y="100" textAnchor="middle" fill="#edf1f5" fontSize="16" fontWeight="bold">
            {eqStep === 1 ? '3x + 20' : eqStep === 2 ? '20' : '4'}
          </text>

          <text x="240" y="38" textAnchor="middle" fill="#7ad0b1" fontSize="13" fontWeight="bold">
            {eqStep === 1 && 'Step 1: Original Balanced Equation 7x + 4 = 3x + 20'}
            {eqStep === 2 && 'Step 2: Subtract 3x from BOTH pans → 4x + 4 = 20'}
            {eqStep === 3 && 'Step 3: Subtract 4, then divide both pans by 4 → x = 4'}
          </text>
        </svg>
        <div className="axiom-btn-group" role="group" aria-label="Select equation solving step">
          <button
            type="button"
            className={`axiom-chip ${eqStep === 1 ? 'active' : ''}`}
            onClick={() => setEqStep(1)}
          >
            1 · 7x + 4 = 3x + 20
          </button>
          <button
            type="button"
            className={`axiom-chip ${eqStep === 2 ? 'active' : ''}`}
            onClick={() => setEqStep(2)}
          >
            2 · Subtract 3x → 4x + 4 = 20
          </button>
          <button
            type="button"
            className={`axiom-chip ${eqStep === 3 ? 'active' : ''}`}
            onClick={() => setEqStep(3)}
          >
            3 · Isolate x → x = 4
          </button>
        </div>
        <p role="status">
          {eqStep === 1 && 'Variables appear on both sides. Performing the exact same operation on both pans preserves the solution set.'}
          {eqStep === 2 && 'Removing 3x from both sides collects all variable terms on the left pan: (7 − 3)x + 4 = 20.'}
          {eqStep === 3 && 'Subtracting 4 gives 4x = 16, and dividing by 4 reveals x = 4. Check: 7(4) + 4 = 32 and 3(4) + 20 = 32.'}
        </p>
      </figure>
    );
  }

  if (lessonId === 'secondary.algebra.systems_of_equations') {
    return (
      <figure className="concept-visual">
        <figcaption>2×2 Linear Systems: Geometric Intersection &amp; Algebraic Elimination</figcaption>
        <svg viewBox="0 0 480 190" role="img" aria-label={sysMode === 'intersection' ? 'Two lines x + y = 10 and x - y = 2 intersecting at (6, 4)' : 'Algebraic elimination adding x + y = 10 and x - y = 2 to get 2x = 12'}>
          <rect x="15" y="15" width="450" height="160" rx="6" fill="#131b26" stroke="#29394b" strokeWidth="1.5" />
          {sysMode === 'intersection' ? (
            <>
              <g transform="translate(55, 150)">
                <line x1="0" y1="0" x2="180" y2="0" stroke="#64748b" strokeWidth="1.5" />
                <line x1="0" y1="0" x2="0" y2="-120" stroke="#64748b" strokeWidth="1.5" />
                {/* Line 1: x + y = 10 => (0,10) to (10,0), scale 12px per unit */}
                <line x1="0" y1="-120" x2="120" y2="0" stroke="#60a5fa" strokeWidth="2.5" />
                {/* Line 2: x - y = 2 => (2,0) to (10,8) */}
                <line x1="24" y1="0" x2="120" y2="-96" stroke="#e6c580" strokeWidth="2.5" />
                {/* Intersection (6, 4) => (72, -48) */}
                <circle cx="72" cy="-48" r="5" fill="#7ad0b1" stroke="#0f172a" strokeWidth="1.5" />
                <text x="82" y="-52" fill="#7ad0b1" fontSize="12" fontWeight="bold">(6, 4)</text>
              </g>
              <g transform="translate(255, 50)">
                <text x="0" y="15" fill="#60a5fa" fontSize="13" fontWeight="bold">Line 1: x + y = 10</text>
                <text x="0" y="40" fill="#e6c580" fontSize="13" fontWeight="bold">Line 2: x − y = 2</text>
                <text x="0" y="72" fill="#7ad0b1" fontSize="14" fontWeight="bold">Intersection: (x, y) = (6, 4)</text>
                <text x="0" y="96" fill="#edf1f5" fontSize="12">Check 1: 6 + 4 = 10 ✓</text>
                <text x="0" y="114" fill="#edf1f5" fontSize="12">Check 2: 6 − 4 = 2 ✓</text>
              </g>
            </>
          ) : (
            <>
              <rect x="65" y="32" width="350" height="125" rx="6" fill="#1e293b" stroke="#7ad0b1" strokeWidth="1.5" />
              <text x="240" y="58" textAnchor="middle" fill="#60a5fa" fontSize="15" fontWeight="bold">
                (1)   x + y = 10
              </text>
              <text x="240" y="82" textAnchor="middle" fill="#e6c580" fontSize="15" fontWeight="bold">
                + (2)   x − y =  2
              </text>
              <line x1="130" y1="92" x2="350" y2="92" stroke="#94a3b8" strokeWidth="1.5" />
              <text x="240" y="115" textAnchor="middle" fill="#7ad0b1" fontSize="16" fontWeight="bold">
                2x + 0y = 12  ⇒  x = 6
              </text>
              <text x="240" y="140" textAnchor="middle" fill="#edf1f5" fontSize="13">
                Substitute x = 6 into (1): 6 + y = 10 ⇒ y = 4
              </text>
            </>
          )}
        </svg>
        <div className="axiom-btn-group" role="group" aria-label="Select linear system view">
          <button
            type="button"
            className={`axiom-chip ${sysMode === 'intersection' ? 'active' : ''}`}
            onClick={() => setSysMode('intersection')}
          >
            Geometric Crossing: (6, 4)
          </button>
          <button
            type="button"
            className={`axiom-chip ${sysMode === 'elimination' ? 'active' : ''}`}
            onClick={() => setSysMode('elimination')}
          >
            Algebraic Elimination (+y − y = 0)
          </button>
        </div>
        <p role="status">
          {sysMode === 'intersection'
            ? 'Every point on Line 1 satisfies x + y = 10; every point on Line 2 satisfies x − y = 2. Only their crossing point (6, 4) satisfies both!'
            : 'Adding the two equations cancels +y and −y immediately, collapsing two unknowns into 2x = 12 ⇒ x = 6.'}
        </p>
      </figure>
    );
  }

  if (lessonId === 'secondary.functions.mappings_and_evaluation') {
    return (
      <figure className="concept-visual">
        <figcaption>Functions as Mappings: Pre-image vs. Piecewise Branch Selection</figcaption>
        <svg viewBox="0 0 480 185" role="img" aria-label={fnMapMode === 'preimage' ? 'Pre-image mapping solving f(x) = 3x - 5 = 16 to find x = 7' : 'Piecewise branch selector routing x = 3 into the x >= 1 branch'}>
          <rect x="15" y="15" width="450" height="155" rx="6" fill="#131b26" stroke="#29394b" strokeWidth="1.5" />
          {fnMapMode === 'preimage' ? (
            <>
              <rect x="40" y="45" width="120" height="80" rx="8" fill="#1e293b" stroke="#60a5fa" strokeWidth="2" />
              <text x="100" y="70" textAnchor="middle" fill="#60a5fa" fontSize="12" fontWeight="bold">Domain D (Input)</text>
              <text x="100" y="102" textAnchor="middle" fill="#7ad0b1" fontSize="18" fontWeight="bold">x = 7</text>

              <text x="240" y="68" textAnchor="middle" fill="#e6c580" fontSize="13" fontWeight="bold">f(x) = 3x − 5</text>
              <line x1="168" y1="85" x2="312" y2="85" stroke="#e6c580" strokeWidth="2.5" />
              <text x="240" y="108" textAnchor="middle" fill="#94a3b8" fontSize="12">Pre-image: 3x − 5 = 16 ⇒ 3x = 21</text>

              <rect x="320" y="45" width="120" height="80" rx="8" fill="#1e293b" stroke="#7ad0b1" strokeWidth="2" />
              <text x="380" y="70" textAnchor="middle" fill="#7ad0b1" fontSize="12" fontWeight="bold">Codomain Y (Output)</text>
              <text x="380" y="102" textAnchor="middle" fill="#edf1f5" fontSize="18" fontWeight="bold">f(7) = 16</text>

              <text x="240" y="152" textAnchor="middle" fill="#f3ddab" fontSize="12">
                Solving f(x) = 16 asks for the input x = 7, NOT f(16) = 43!
              </text>
            </>
          ) : (
            <>
              <rect x="35" y="36" width="190" height="52" rx="5" fill="#1e293b" stroke="#64748b" strokeWidth="1.5" />
              <text x="130" y="58" textAnchor="middle" fill="#94a3b8" fontSize="12">Branch 1 (if x &lt; 1): 2x + 3</text>
              <text x="130" y="76" textAnchor="middle" fill="#64748b" fontSize="11">Skipped since 3 ≥ 1</text>

              <rect x="255" y="36" width="190" height="52" rx="5" fill="#1e293b" stroke="#7ad0b1" strokeWidth="2" />
              <text x="350" y="58" textAnchor="middle" fill="#7ad0b1" fontSize="12" fontWeight="bold">Branch 2 (if x ≥ 1): 4x − 1</text>
              <text x="350" y="76" textAnchor="middle" fill="#edf1f5" fontSize="12" fontWeight="bold">Active for x = 3 ✓</text>

              <text x="240" y="122" textAnchor="middle" fill="#e6c580" fontSize="15" fontWeight="bold">
                Evaluate Active Branch Only: f(3) = 4(3) − 1 = 11
              </text>
              <text x="240" y="148" textAnchor="middle" fill="#94a3b8" fontSize="12">
                A function produces a single output: never add both piecewise branches together.
              </text>
            </>
          )}
        </svg>
        <div className="axiom-btn-group" role="group" aria-label="Select function mapping model">
          <button
            type="button"
            className={`axiom-chip ${fnMapMode === 'preimage' ? 'active' : ''}`}
            onClick={() => setFnMapMode('preimage')}
          >
            Pre-image: Solve f(x) = 16
          </button>
          <button
            type="button"
            className={`axiom-chip ${fnMapMode === 'piecewise' ? 'active' : ''}`}
            onClick={() => setFnMapMode('piecewise')}
          >
            Piecewise Guard: x = 3 in x ≥ 1
          </button>
        </div>
        <p role="status">
          {fnMapMode === 'preimage'
            ? 'Forward evaluation computes y = f(x); pre-image solving reverses the rule to find which input x maps to y = 16.'
            : 'In a piecewise definition, test the domain inequality first so you substitute x into the unique active branch.'}
        </p>
      </figure>
    );
  }

  if (lessonId === 'secondary.functions.composition_and_inverses') {
    return (
      <figure className="concept-visual">
        <figcaption>Function Pipelines: Inside-Out Composition (f ∘ g)(x) &amp; Inverse Reversal</figcaption>
        <svg viewBox="0 0 480 185" role="img" aria-label={compMode === 'chain' ? 'Inside-out pipeline applying g(2) = 5 first, then f(5) = 19' : 'Inverse function reversing f(x) = (4x - 3)/5 by multiplying by 5, adding 3, and dividing by 4'}>
          <rect x="15" y="15" width="450" height="155" rx="6" fill="#131b26" stroke="#29394b" strokeWidth="1.5" />
          {compMode === 'chain' ? (
            <>
              <rect x="30" y="52" width="85" height="60" rx="6" fill="#1e293b" stroke="#60a5fa" strokeWidth="2" />
              <text x="72" y="76" textAnchor="middle" fill="#94a3b8" fontSize="11">Input x</text>
              <text x="72" y="98" textAnchor="middle" fill="#edf1f5" fontSize="16" fontWeight="bold">x = 2</text>

              <text x="135" y="86" textAnchor="middle" fill="#e6c580" fontSize="16" fontWeight="bold">→</text>

              <rect x="155" y="45" width="130" height="74" rx="6" fill="#1e293b" stroke="#e6c580" strokeWidth="2" />
              <text x="220" y="68" textAnchor="middle" fill="#e6c580" fontSize="12" fontWeight="bold">1st (Inner): g(x) = x + 3</text>
              <text x="220" y="92" textAnchor="middle" fill="#edf1f5" fontSize="14" fontWeight="bold">u = g(2) = 5</text>

              <text x="305" y="86" textAnchor="middle" fill="#7ad0b1" fontSize="16" fontWeight="bold">→</text>

              <rect x="325" y="45" width="125" height="74" rx="6" fill="#1e293b" stroke="#7ad0b1" strokeWidth="2" />
              <text x="387" y="68" textAnchor="middle" fill="#7ad0b1" fontSize="12" fontWeight="bold">2nd (Outer): f(u) = 4u − 1</text>
              <text x="387" y="92" textAnchor="middle" fill="#7ad0b1" fontSize="14" fontWeight="bold">f(5) = 19</text>

              <text x="240" y="148" textAnchor="middle" fill="#f3ddab" fontSize="12">
                Order Matters: (f ∘ g)(2) = f(g(2)) = 19, whereas (g ∘ f)(2) = g(7) = 10.
              </text>
            </>
          ) : (
            <>
              <rect x="40" y="42" width="180" height="74" rx="6" fill="#1e293b" stroke="#60a5fa" strokeWidth="2" />
              <text x="130" y="65" textAnchor="middle" fill="#60a5fa" fontSize="12" fontWeight="bold">Forward: f(x) = (4x − 3) / 5</text>
              <text x="130" y="86" textAnchor="middle" fill="#edf1f5" fontSize="12">1. ×4   2. −3   3. ÷5</text>
              <text x="130" y="104" textAnchor="middle" fill="#94a3b8" fontSize="11">f(2) = 1</text>

              <rect x="260" y="42" width="180" height="74" rx="6" fill="#1e293b" stroke="#7ad0b1" strokeWidth="2" />
              <text x="350" y="65" textAnchor="middle" fill="#7ad0b1" fontSize="12" fontWeight="bold">Inverse: f⁻¹(y) = (5y + 3) / 4</text>
              <text x="350" y="86" textAnchor="middle" fill="#edf1f5" fontSize="12">1. ×5   2. +3   3. ÷4</text>
              <text x="350" y="104" textAnchor="middle" fill="#7ad0b1" fontSize="11" fontWeight="bold">f⁻¹(1) = 2 ✓</text>

              <text x="240" y="146" textAnchor="middle" fill="#e6c580" fontSize="12">
                f⁻¹(y) undoes every operation in reverse order; it is NOT the reciprocal 1 / f(y).
              </text>
            </>
          )}
        </svg>
        <div className="axiom-btn-group" role="group" aria-label="Select composition or inverse view">
          <button
            type="button"
            className={`axiom-chip ${compMode === 'chain' ? 'active' : ''}`}
            onClick={() => setCompMode('chain')}
          >
            Inside-Out Chain: (f ∘ g)(2) = f(g(2))
          </button>
          <button
            type="button"
            className={`axiom-chip ${compMode === 'inverse' ? 'active' : ''}`}
            onClick={() => setCompMode('inverse')}
          >
            Inverse Reversal: f⁻¹(y) = (5y + 3)/4
          </button>
        </div>
        <p role="status">
          {compMode === 'chain'
            ? 'In (f ∘ g)(x) = f(g(x)), the inner function g acts on x first because g is closest to x.'
            : 'To invert y = (4x − 3)/5, undo the last operation first: multiply by 5, add 3, and divide by 4.'}
        </p>
      </figure>
    );
  }

  if (lessonId === 'secondary.functions.transformations_and_graphs') {
    return (
      <figure className="concept-visual">
        <figcaption>Graph Transformations: Vertex Form a(x − h)² + k &amp; Point Mapping</figcaption>
        <svg viewBox="0 0 480 185" role="img" aria-label={transMode === 'vertex' ? 'Parabola y = 2(x - 4)^2 + 3 with vertex at (4, 3)' : 'Point (2, 5) on y = f(x) mapped to (5, 11) on g(x) = 3f(x - 3) - 4'}>
          <rect x="15" y="15" width="450" height="155" rx="6" fill="#131b26" stroke="#29394b" strokeWidth="1.5" />
          {transMode === 'vertex' ? (
            <>
              <rect x="35" y="38" width="195" height="90" rx="6" fill="#1e293b" stroke="#e6c580" strokeWidth="2" />
              <text x="132" y="62" textAnchor="middle" fill="#e6c580" fontSize="13" fontWeight="bold">f(x) = 2(x − 4)² + 3</text>
              <text x="132" y="86" textAnchor="middle" fill="#edf1f5" fontSize="12">Squared term (x − 4)² = 0 at x = +4</text>
              <text x="132" y="110" textAnchor="middle" fill="#7ad0b1" fontSize="15" fontWeight="bold">Vertex (h, k) = (4, 3)</text>

              <rect x="250" y="38" width="195" height="90" rx="6" fill="#1e293b" stroke="#60a5fa" strokeWidth="2" />
              <text x="347" y="62" textAnchor="middle" fill="#60a5fa" fontSize="13" fontWeight="bold">f(x) = −3(x + 5)² − 2</text>
              <text x="347" y="86" textAnchor="middle" fill="#edf1f5" fontSize="12">Rewrite x + 5 = x − (−5)</text>
              <text x="347" y="110" textAnchor="middle" fill="#7ad0b1" fontSize="15" fontWeight="bold">Vertex (h, k) = (−5, −2)</text>

              <text x="240" y="152" textAnchor="middle" fill="#f3ddab" fontSize="12">
                Inside the parentheses (x − h), the horizontal shift h has the opposite sign of −h.
              </text>
            </>
          ) : (
            <>
              <rect x="40" y="44" width="150" height="72" rx="6" fill="#1e293b" stroke="#60a5fa" strokeWidth="2" />
              <text x="115" y="68" textAnchor="middle" fill="#60a5fa" fontSize="12" fontWeight="bold">Parent Point on y = f(x)</text>
              <text x="115" y="96" textAnchor="middle" fill="#edf1f5" fontSize="16" fontWeight="bold">(x₀, y₀) = (2, 5)</text>

              <text x="240" y="72" textAnchor="middle" fill="#e6c580" fontSize="12" fontWeight="bold">g(x) = 3f(x − 3) − 4</text>
              <text x="240" y="92" textAnchor="middle" fill="#edf1f5" fontSize="16" fontWeight="bold">⟶</text>

              <rect x="290" y="44" width="150" height="72" rx="6" fill="#1e293b" stroke="#7ad0b1" strokeWidth="2" />
              <text x="365" y="68" textAnchor="middle" fill="#7ad0b1" fontSize="12" fontWeight="bold">Mapped Point on y = g(x)</text>
              <text x="365" y="96" textAnchor="middle" fill="#7ad0b1" fontSize="15" fontWeight="bold">(2 + 3, 3·5 − 4) = (5, 11)</text>

              <text x="240" y="148" textAnchor="middle" fill="#f3ddab" fontSize="12">
                Set x − 3 = 2 ⇒ x = 5, then scale and shift the output: g(5) = 3 f(2) − 4 = 15 − 4 = 11.
              </text>
            </>
          )}
        </svg>
        <div className="axiom-btn-group" role="group" aria-label="Select transformation concept">
          <button
            type="button"
            className={`axiom-chip ${transMode === 'vertex' ? 'active' : ''}`}
            onClick={() => setTransMode('vertex')}
          >
            Vertex Sign Rule: a(x − h)² + k
          </button>
          <button
            type="button"
            className={`axiom-chip ${transMode === 'point_shift' ? 'active' : ''}`}
            onClick={() => setTransMode('point_shift')}
          >
            Point Mapping: (x₀ + h, a·y₀ + k)
          </button>
        </div>
        <p role="status">
          {transMode === 'vertex'
            ? 'The vertex occurs where the squared term (x − h)² vanishes, which is at x = h and y = k.'
            : 'To track a known point (x₀, y₀) under g(x) = a f(x − h) + k, match the inner input x − h = x₀ so x = x₀ + h.'}
        </p>
      </figure>
    );
  }

  if (lessonId === 'secondary.algebra2.polynomials_and_factoring') {
    return (
      <figure className="concept-visual">
        <figcaption>Polynomial Structure: Vieta&apos;s Root Symmetries &amp; Remainder Theorem R = P(r)</figcaption>
        <svg viewBox="0 0 480 185" role="img" aria-label={polyMode === 'vieta' ? 'Factored quadratic (x - 4)(x - 5) = x^2 - 9x + 20 showing sum of roots 9 and product 20' : 'Remainder Theorem identity P(x) = (x - 3)Q(x) + R evaluated at x = 3 to get R = P(3)'}>
          <rect x="15" y="15" width="450" height="155" rx="6" fill="#131b26" stroke="#29394b" strokeWidth="1.5" />
          {polyMode === 'vieta' ? (
            <>
              <text x="240" y="44" textAnchor="middle" fill="#edf1f5" fontSize="15" fontWeight="bold">
                (x − 4)(x − 5) = x² − (4 + 5)x + (4 · 5) = x² − 9x + 20 = 0
              </text>
              <rect x="40" y="62" width="185" height="64" rx="6" fill="#1e293b" stroke="#60a5fa" strokeWidth="2" />
              <text x="132" y="84" textAnchor="middle" fill="#60a5fa" fontSize="12" fontWeight="bold">Sum of Roots (Vieta)</text>
              <text x="132" y="108" textAnchor="middle" fill="#edf1f5" fontSize="14" fontWeight="bold">r₁ + r₂ = −B/A = 9</text>

              <rect x="255" y="62" width="185" height="64" rx="6" fill="#1e293b" stroke="#7ad0b1" strokeWidth="2" />
              <text x="347" y="84" textAnchor="middle" fill="#7ad0b1" fontSize="12" fontWeight="bold">Product of Roots (Vieta)</text>
              <text x="347" y="108" textAnchor="middle" fill="#edf1f5" fontSize="14" fontWeight="bold">r₁ · r₂ = C/A = 20</text>

              <text x="240" y="152" textAnchor="middle" fill="#e6c580" fontSize="12">
                Zero-Product Roots: x − 4 = 0 ⇒ x = 4 and x − 5 = 0 ⇒ x = 5 (Larger root = 5)
              </text>
            </>
          ) : (
            <>
              <text x="240" y="46" textAnchor="middle" fill="#e6c580" fontSize="14" fontWeight="bold">
                Euclidean Division Identity: P(x) = (x − r) · Q(x) + R
              </text>
              <rect x="50" y="62" width="380" height="62" rx="6" fill="#1e293b" stroke="#7ad0b1" strokeWidth="2" />
              <text x="240" y="86" textAnchor="middle" fill="#edf1f5" fontSize="14">
                Substitute x = r:   P(r) = (r − r) · Q(r) + R = 0 · Q(r) + R
              </text>
              <text x="240" y="110" textAnchor="middle" fill="#7ad0b1" fontSize="15" fontWeight="bold">
                Remainder Theorem:  R = P(r)
              </text>
              <text x="240" y="150" textAnchor="middle" fill="#94a3b8" fontSize="12">
                Dividing by (x − 3) evaluates P(+3), and (x − r) is an exact factor iff P(r) = 0.
              </text>
            </>
          )}
        </svg>
        <div className="axiom-btn-group" role="group" aria-label="Select polynomial theorem view">
          <button
            type="button"
            className={`axiom-chip ${polyMode === 'vieta' ? 'active' : ''}`}
            onClick={() => setPolyMode('vieta')}
          >
            Factoring &amp; Vieta: r₁ + r₂ = −B/A
          </button>
          <button
            type="button"
            className={`axiom-chip ${polyMode === 'remainder' ? 'active' : ''}`}
            onClick={() => setPolyMode('remainder')}
          >
            Remainder Theorem: R = P(r)
          </button>
        </div>
        <p role="status">
          {polyMode === 'vieta'
            ? 'Expanding A(x − r₁)(x − r₂) = Ax² − A(r₁ + r₂)x + Ar₁r₂ proves that r₁ + r₂ = −B/A and r₁r₂ = C/A.'
            : 'Evaluating P(x) = (x − r)Q(x) + R at x = r annihilates the unknown quotient Q(r), leaving R = P(r) immediately.'}
        </p>
      </figure>
    );
  }

  if (lessonId === 'secondary.algebra2.quadratics_and_discriminant') {
    return (
      <figure className="concept-visual">
        <figcaption>The Quadratic Discriminant Δ = b² − 4ac &amp; Root Geometry</figcaption>
        <svg viewBox="0 0 480 185" role="img" aria-label={discMode === 'two_roots' ? 'Quadratic 2x^2 - 5x - 3 = 0 with Delta = 49 > 0 and two rational roots x = -1/2 and x = 3' : 'Repeated root condition Delta = b^2 - 4ac = 0 where vertex touches x-axis tangentially'}>
          <rect x="15" y="15" width="450" height="155" rx="6" fill="#131b26" stroke="#29394b" strokeWidth="1.5" />
          {discMode === 'two_roots' ? (
            <>
              <rect x="35" y="36" width="200" height="92" rx="6" fill="#1e293b" stroke="#e6c580" strokeWidth="2" />
              <text x="135" y="58" textAnchor="middle" fill="#e6c580" fontSize="13" fontWeight="bold">2x² − 5x − 3 = 0</text>
              <text x="135" y="80" textAnchor="middle" fill="#edf1f5" fontSize="12">Δ = (−5)² − 4(2)(−3)</text>
              <text x="135" y="102" textAnchor="middle" fill="#7ad0b1" fontSize="15" fontWeight="bold">Δ = 25 + 24 = 49 = 7²</text>

              <rect x="250" y="36" width="195" height="92" rx="6" fill="#1e293b" stroke="#7ad0b1" strokeWidth="2" />
              <text x="347" y="58" textAnchor="middle" fill="#7ad0b1" fontSize="13" fontWeight="bold">Quadratic Formula</text>
              <text x="347" y="80" textAnchor="middle" fill="#edf1f5" fontSize="12">x = (5 ± √49) / (2 · 2) = (5 ± 7) / 4</text>
              <text x="347" y="104" textAnchor="middle" fill="#7ad0b1" fontSize="14" fontWeight="bold">x = 3  or  x = −1/2</text>

              <text x="240" y="152" textAnchor="middle" fill="#f3ddab" fontSize="12">
                Always square b with parentheses: (−5)² = +25, and −4(2)(−3) = +24.
              </text>
            </>
          ) : (
            <>
              <rect x="50" y="38" width="380" height="90" rx="6" fill="#1e293b" stroke="#60a5fa" strokeWidth="2" />
              <text x="240" y="62" textAnchor="middle" fill="#60a5fa" fontSize="14" fontWeight="bold">
                Repeated Root (Tangent Vertex): Δ = b² − 4ac = 0
              </text>
              <text x="240" y="88" textAnchor="middle" fill="#edf1f5" fontSize="13">
                Example: 3x² − 10x + c = 0  ⇒  (−10)² − 4(3)c = 100 − 12c = 0
              </text>
              <text x="240" y="112" textAnchor="middle" fill="#7ad0b1" fontSize="15" fontWeight="bold">
                Exact Parameter: c = b² / (4a) = 100 / 12 = 25/3
              </text>
              <text x="240" y="152" textAnchor="middle" fill="#94a3b8" fontSize="12">
                When Δ = 0, both roots coalesce at the axis of symmetry x = −b / (2a).
              </text>
            </>
          )}
        </svg>
        <div className="axiom-btn-group" role="group" aria-label="Select discriminant case">
          <button
            type="button"
            className={`axiom-chip ${discMode === 'two_roots' ? 'active' : ''}`}
            onClick={() => setDiscMode('two_roots')}
          >
            Δ = 49 &gt; 0 (Two Rational Roots)
          </button>
          <button
            type="button"
            className={`axiom-chip ${discMode === 'tangent_zero' ? 'active' : ''}`}
            onClick={() => setDiscMode('tangent_zero')}
          >
            Δ = 0 (Repeated Root: c = b² / 4a)
          </button>
        </div>
        <p role="status">
          {discMode === 'two_roots'
            ? 'When a, b, c ∈ ℚ and Δ = b² − 4ac is a positive perfect square, the quadratic has two distinct rational roots in ℚ.'
            : 'Setting Δ = b² − 4ac = 0 forces the ±√Δ term to vanish, yielding a single repeated root at x = −b/(2a).'}
        </p>
      </figure>
    );
  }

  if (lessonId === 'secondary.algebra2.exponentials_and_logarithms') {
    return (
      <figure className="concept-visual">
        <figcaption>Inverse Exponentiation: Logarithm Laws &amp; Common-Base Equations</figcaption>
        <svg viewBox="0 0 480 185" role="img" aria-label={expLogMode === 'log_laws' ? 'Logarithm laws combining log_2(12) + log_2(4) - log_2(3) = log_2(16) = 4' : 'Common-base exponential equation 3^(4x - 1) = 9^(x + 2) rewritten as 3^(4x - 1) = 3^(2x + 4)'}>
          <rect x="15" y="15" width="450" height="155" rx="6" fill="#131b26" stroke="#29394b" strokeWidth="1.5" />
          {expLogMode === 'log_laws' ? (
            <>
              <text x="240" y="45" textAnchor="middle" fill="#e6c580" fontSize="14" fontWeight="bold">
                log₂(12) + log₂(4) − log₂(3)
              </text>
              <rect x="45" y="60" width="390" height="64" rx="6" fill="#1e293b" stroke="#7ad0b1" strokeWidth="2" />
              <text x="240" y="84" textAnchor="middle" fill="#edf1f5" fontSize="13">
                Product &amp; Quotient Laws: log_b(A) + log_b(B) − log_b(C) = log_b((A · B) / C)
              </text>
              <text x="240" y="108" textAnchor="middle" fill="#7ad0b1" fontSize="15" fontWeight="bold">
                = log₂((12 × 4) / 3) = log₂(16) = log₂(2⁴) = 4
              </text>
              <text x="240" y="150" textAnchor="middle" fill="#f3ddab" fontSize="12">
                Adding logs multiplies arguments; reciprocal powers satisfy log_b(1 / b^k) = −k.
              </text>
            </>
          ) : (
            <>
              <text x="240" y="45" textAnchor="middle" fill="#60a5fa" fontSize="14" fontWeight="bold">
                Solve over ℚ:   3^(4x − 1) = 9^(x + 2)
              </text>
              <rect x="45" y="60" width="390" height="64" rx="6" fill="#1e293b" stroke="#e6c580" strokeWidth="2" />
              <text x="240" y="84" textAnchor="middle" fill="#edf1f5" fontSize="13">
                Rewrite 9 = 3² and distribute: (3²)^(x + 2) = 3^(2(x + 2)) = 3^(2x + 4)
              </text>
              <text x="240" y="108" textAnchor="middle" fill="#7ad0b1" fontSize="15" fontWeight="bold">
                Equate Exponents: 4x − 1 = 2x + 4  ⇒  2x = 5  ⇒  x = 5/2
              </text>
              <text x="240" y="150" textAnchor="middle" fill="#94a3b8" fontSize="12">
                Power-of-a-power (b^m)^(dx + e) = b^(mdx + me) multiplies BOTH dx and e by m!
              </text>
            </>
          )}
        </svg>
        <div className="axiom-btn-group" role="group" aria-label="Select logarithm or exponential model">
          <button
            type="button"
            className={`axiom-chip ${expLogMode === 'log_laws' ? 'active' : ''}`}
            onClick={() => setExpLogMode('log_laws')}
          >
            Log Laws: log_b(AB / C)
          </button>
          <button
            type="button"
            className={`axiom-chip ${expLogMode === 'common_base' ? 'active' : ''}`}
            onClick={() => setExpLogMode('common_base')}
          >
            Common Base: b^(ax + c) = (b^m)^(dx + e)
          </button>
        </div>
        <p role="status">
          {expLogMode === 'log_laws'
            ? 'Because logarithms output exponents, adding and subtracting logs corresponds to multiplying and dividing their positive arguments.'
            : 'Rewriting both sides with a common base b lets you equate exponents directly—remember to distribute m across (dx + e).'}
        </p>
      </figure>
    );
  }

  if (lessonId === 'secondary.geometry.lines_slopes_and_perpendicularity') {
    return (
      <figure className="concept-visual">
        <figcaption>Analytic Lines: 90° Slope Triangle Rotation &amp; Internal Section Formula</figcaption>
        <svg viewBox="0 0 480 185" role="img" aria-label={secLineMode === 'perp_slope' ? 'Rotating slope triangle (4, 3) by 90 degrees to (-3, 4) proving m_1 * m_2 = -1' : 'Section formula dividing A(-2, 1) to B(8, 6) in ratio 2:3 at P(2, 3)'}>
          <rect x="15" y="15" width="450" height="155" rx="6" fill="#131b26" stroke="#29394b" strokeWidth="1.5" />
          {secLineMode === 'perp_slope' ? (
            <>
              <rect x="35" y="36" width="195" height="90" rx="6" fill="#1e293b" stroke="#60a5fa" strokeWidth="2" />
              <text x="132" y="58" textAnchor="middle" fill="#60a5fa" fontSize="13" fontWeight="bold">Line L₁: (Δx, Δy) = (4, 3)</text>
              <text x="132" y="82" textAnchor="middle" fill="#edf1f5" fontSize="13">Slope m₁ = rise / run = 3/4</text>
              <text x="132" y="106" textAnchor="middle" fill="#94a3b8" fontSize="12">90° CCW Rotation: (4, 3) ↦ (−3, 4)</text>

              <rect x="250" y="36" width="195" height="90" rx="6" fill="#1e293b" stroke="#7ad0b1" strokeWidth="2" />
              <text x="347" y="58" textAnchor="middle" fill="#7ad0b1" fontSize="13" fontWeight="bold">Perpendicular L₂ through (6, 5)</text>
              <text x="347" y="82" textAnchor="middle" fill="#7ad0b1" fontSize="14" fontWeight="bold">m_⊥ = −4/3  (m₁ · m_⊥ = −1)</text>
              <text x="347" y="106" textAnchor="middle" fill="#edf1f5" fontSize="12">b = 5 − (−4/3)(6) = 5 + 8 = 13</text>

              <text x="240" y="150" textAnchor="middle" fill="#e6c580" fontSize="12">
                Perpendicular lines have negative-reciprocal slopes: m_⊥ = −1 / m₁.
              </text>
            </>
          ) : (
            <>
              <line x1="60" y1="75" x2="420" y2="75" stroke="#60a5fa" strokeWidth="3" />
              <circle cx="60" cy="75" r="5" fill="#e6c580" />
              <text x="60" y="55" textAnchor="middle" fill="#e6c580" fontSize="12" fontWeight="bold">A(−2, 1)</text>

              <circle cx="204" cy="75" r="6" fill="#7ad0b1" />
              <text x="204" y="55" textAnchor="middle" fill="#7ad0b1" fontSize="13" fontWeight="bold">P(2, 3)</text>

              <circle cx="420" cy="75" r="5" fill="#60a5fa" />
              <text x="420" y="55" textAnchor="middle" fill="#60a5fa" fontSize="12" fontWeight="bold">B(8, 6)</text>

              <text x="132" y="96" textAnchor="middle" fill="#f3ddab" fontSize="12">m = 2 parts</text>
              <text x="312" y="96" textAnchor="middle" fill="#f3ddab" fontSize="12">n = 3 parts</text>

              <text x="240" y="130" textAnchor="middle" fill="#7ad0b1" fontSize="14" fontWeight="bold">
                x_P = (n·x₁ + m·x₂) / (m + n) = (3(−2) + 2(8)) / (2 + 3) = 10 / 5 = 2
              </text>
              <text x="240" y="152" textAnchor="middle" fill="#94a3b8" fontSize="12">
                Cross-weighting: A(x₁) is weighted by n = 3 and B(x₂) is weighted by m = 2.
              </text>
            </>
          )}
        </svg>
        <div className="axiom-btn-group" role="group" aria-label="Select analytic geometry model">
          <button
            type="button"
            className={`axiom-chip ${secLineMode === 'perp_slope' ? 'active' : ''}`}
            onClick={() => setSecLineMode('perp_slope')}
          >
            Perpendicular Slope: m_⊥ = −q/p
          </button>
          <button
            type="button"
            className={`axiom-chip ${secLineMode === 'section_ratio' ? 'active' : ''}`}
            onClick={() => setSecLineMode('section_ratio')}
          >
            Section Formula: (n·x₁ + m·x₂)/(m + n)
          </button>
        </div>
        <p role="status">
          {secLineMode === 'perp_slope'
            ? 'Rotating a slope vector ⟨q, p⟩ by 90° produces ⟨−p, q⟩, turning slope p/q into −q/p.'
            : 'In the internal section formula for ratio AP : PB = m : n, endpoint A is weighted by n and endpoint B by m.'}
        </p>
      </figure>
    );
  }

  if (lessonId === 'secondary.geometry.circles_and_theorems') {
    return (
      <figure className="concept-visual">
        <figcaption>Circle Geometry: Inscribed Angle Theorem &amp; Power of a Point</figcaption>
        <svg viewBox="0 0 480 185" role="img" aria-label={circleMode === 'inscribed_angle' ? 'Circle with central angle AOB = 116 degrees and inscribed angle ACB = 58 degrees' : 'Intersecting chords PA * PB = PC * PD inside a circle'}>
          <rect x="15" y="15" width="450" height="155" rx="6" fill="#131b26" stroke="#29394b" strokeWidth="1.5" />
          {circleMode === 'inscribed_angle' ? (
            <>
              <circle cx="115" cy="92" r="55" fill="none" stroke="#60a5fa" strokeWidth="2" />
              <circle cx="115" cy="92" r="3.5" fill="#e6c580" />
              <line x1="115" y1="92" x2="155" y2="54" stroke="#e6c580" strokeWidth="2" />
              <line x1="115" y1="92" x2="155" y2="130" stroke="#e6c580" strokeWidth="2" />
              <line x1="60" y1="92" x2="155" y2="54" stroke="#7ad0b1" strokeWidth="2" />
              <line x1="60" y1="92" x2="155" y2="130" stroke="#7ad0b1" strokeWidth="2" />
              <text x="50" y="96" textAnchor="end" fill="#7ad0b1" fontSize="12" fontWeight="bold">C</text>
              <text x="112" y="85" textAnchor="end" fill="#e6c580" fontSize="12" fontWeight="bold">O</text>
              <text x="165" y="54" fill="#edf1f5" fontSize="12" fontWeight="bold">A</text>
              <text x="165" y="136" fill="#edf1f5" fontSize="12" fontWeight="bold">B</text>

              <g transform="translate(210, 44)">
                <text x="0" y="18" fill="#e6c580" fontSize="14" fontWeight="bold">Central Angle: ∠AOB = 2θ = 116°</text>
                <text x="0" y="44" fill="#7ad0b1" fontSize="14" fontWeight="bold">Inscribed Angle: ∠ACB = θ = 58°</text>
                <text x="0" y="72" fill="#edf1f5" fontSize="12">Locus Form: x² + y² − 6x − 8y + 9 = 0</text>
                <text x="0" y="94" fill="#60a5fa" fontSize="13" fontWeight="bold">⇒ (x − 3)² + (y − 4)² = R² = 16</text>
              </g>
            </>
          ) : (
            <>
              <rect x="45" y="38" width="390" height="90" rx="6" fill="#1e293b" stroke="#7ad0b1" strokeWidth="2" />
              <text x="240" y="62" textAnchor="middle" fill="#e6c580" fontSize="14" fontWeight="bold">
                Intersecting Chords Theorem (Power of a Point P)
              </text>
              <text x="240" y="88" textAnchor="middle" fill="#edf1f5" fontSize="13">
                Since ∠ACD = ∠ABD and ∠APC = ∠DPB, △PAC ∼ △PDB:
              </text>
              <text x="240" y="112" textAnchor="middle" fill="#7ad0b1" fontSize="15" fontWeight="bold">
                PA · PB = PC · PD  ⇒  4 × 9 = 6 × PD  ⇒  PD = 6
              </text>
              <text x="240" y="150" textAnchor="middle" fill="#94a3b8" fontSize="12">
                Equate the products of chord segments (PA · PB = PC · PD), never their sums.
              </text>
            </>
          )}
        </svg>
        <div className="axiom-btn-group" role="group" aria-label="Select circle theorem view">
          <button
            type="button"
            className={`axiom-chip ${circleMode === 'inscribed_angle' ? 'active' : ''}`}
            onClick={() => setCircleMode('inscribed_angle')}
          >
            Inscribed Angle &amp; Locus R²
          </button>
          <button
            type="button"
            className={`axiom-chip ${circleMode === 'chord_power' ? 'active' : ''}`}
            onClick={() => setCircleMode('chord_power')}
          >
            Power of a Point: PA · PB = PC · PD
          </button>
        </div>
        <p role="status">
          {circleMode === 'inscribed_angle'
            ? 'An inscribed angle ∠ACB is always half the central angle ∠AOB subtending the same arc.'
            : 'Because intersecting chords form similar triangles △PAC ∼ △PDB, the product of segment lengths along each chord is equal.'}
        </p>
      </figure>
    );
  }

  if (lessonId === 'secondary.geometry.vectors_and_dot_product') {
    return (
      <figure className="concept-visual">
        <figcaption>Planar Vectors: Orthogonality u · v = 0 &amp; Scalar Projection comp_v(u)</figcaption>
        <svg viewBox="0 0 480 185" role="img" aria-label={vecMode === 'orthogonal' ? 'Orthogonal vectors u = <4, 6> and v = <3, -2> with dot product 12 - 12 = 0' : 'Scalar projection of u = <5, 2> onto v = <3, 4> giving comp_v(u) = 23/5'}>
          <rect x="15" y="15" width="450" height="155" rx="6" fill="#131b26" stroke="#29394b" strokeWidth="1.5" />
          {vecMode === 'orthogonal' ? (
            <>
              <rect x="40" y="38" width="400" height="90" rx="6" fill="#1e293b" stroke="#60a5fa" strokeWidth="2" />
              <text x="240" y="62" textAnchor="middle" fill="#60a5fa" fontSize="14" fontWeight="bold">
                Orthogonality Criterion: u ⊥ v  ⇔  u · v = u₁v₁ + u₂v₂ = 0
              </text>
              <text x="240" y="88" textAnchor="middle" fill="#edf1f5" fontSize="13">
                For u = ⟨4, 6⟩ and v = ⟨3, k⟩:   4(3) + 6k = 12 + 6k = 0
              </text>
              <text x="240" y="112" textAnchor="middle" fill="#7ad0b1" fontSize="15" fontWeight="bold">
                k = −12 / 6 = −2   (Check: ⟨4, 6⟩ · ⟨3, −2⟩ = 12 − 12 = 0 ✓)
              </text>
              <text x="240" y="150" textAnchor="middle" fill="#e6c580" fontSize="12">
                Proportional components make vectors parallel; zero dot product makes them perpendicular!
              </text>
            </>
          ) : (
            <>
              <rect x="40" y="38" width="400" height="90" rx="6" fill="#1e293b" stroke="#7ad0b1" strokeWidth="2" />
              <text x="240" y="62" textAnchor="middle" fill="#e6c580" fontSize="14" fontWeight="bold">
                Scalar Projection of u = ⟨5, 2⟩ onto v = ⟨3, 4⟩
              </text>
              <text x="240" y="88" textAnchor="middle" fill="#edf1f5" fontSize="13">
                u · v = 5(3) + 2(4) = 23,     ‖v‖ = √(3² + 4²) = 5
              </text>
              <text x="240" y="112" textAnchor="middle" fill="#7ad0b1" fontSize="15" fontWeight="bold">
                comp_v(u) = (u · v) / ‖v‖ = 23/5
              </text>
              <text x="240" y="150" textAnchor="middle" fill="#94a3b8" fontSize="12">
                Divide u · v by the magnitude ‖v‖ (not ‖v‖²) to get the signed shadow length along v.
              </text>
            </>
          )}
        </svg>
        <div className="axiom-btn-group" role="group" aria-label="Select vector operation model">
          <button
            type="button"
            className={`axiom-chip ${vecMode === 'orthogonal' ? 'active' : ''}`}
            onClick={() => setVecMode('orthogonal')}
          >
            Orthogonality: u · v = 0
          </button>
          <button
            type="button"
            className={`axiom-chip ${vecMode === 'projection' ? 'active' : ''}`}
            onClick={() => setVecMode('projection')}
          >
            Scalar Projection: (u · v) / ‖v‖
          </button>
        </div>
        <p role="status">
          {vecMode === 'orthogonal'
            ? 'Because u · v = ‖u‖ ‖v‖ cos(θ), non-zero vectors meet at 90° if and only if u₁v₁ + u₂v₂ = 0.'
            : 'Projecting u onto the unit vector v / ‖v‖ measures the exact component of u in the direction of v.'}
        </p>
      </figure>
    );
  }

  if (lessonId === 'secondary.trig.radians_and_unit_circle') {
    return (
      <figure className="concept-visual">
        <figcaption>The Unit Circle x² + y² = 1: Quadrant II Signs &amp; Sinusoidal Period</figcaption>
        <svg viewBox="0 0 480 185" role="img" aria-label={radMode === 'quadrant_ii' ? 'Unit circle point in Quadrant II with sin(theta) = 5/13 and cos(theta) = -12/13' : 'Periodic wave f(t) = 4 cos(5 pi t) + 2 with period T = 2/5'}>
          <rect x="15" y="15" width="450" height="155" rx="6" fill="#131b26" stroke="#29394b" strokeWidth="1.5" />
          {radMode === 'quadrant_ii' ? (
            <>
              <rect x="35" y="36" width="195" height="92" rx="6" fill="#1e293b" stroke="#60a5fa" strokeWidth="2" />
              <text x="132" y="58" textAnchor="middle" fill="#60a5fa" fontSize="13" fontWeight="bold">Radians &amp; Arc Length</text>
              <text x="132" y="82" textAnchor="middle" fill="#edf1f5" fontSize="12">180° = π rad  ⇒  k = D / 180</text>
              <text x="132" y="106" textAnchor="middle" fill="#7ad0b1" fontSize="14" fontWeight="bold">135° = (135/180)π = (3/4)π</text>

              <rect x="245" y="36" width="200" height="92" rx="6" fill="#1e293b" stroke="#e6c580" strokeWidth="2" />
              <text x="345" y="58" textAnchor="middle" fill="#e6c580" fontSize="13" fontWeight="bold">Quadrant II (π/2 &lt; θ &lt; π)</text>
              <text x="345" y="82" textAnchor="middle" fill="#edf1f5" fontSize="12">sin(θ) = 5/13  ⇒  cos²(θ) = 144/169</text>
              <text x="345" y="106" textAnchor="middle" fill="#7ad0b1" fontSize="14" fontWeight="bold">x &lt; 0 in QII  ⇒  cos(θ) = −12/13</text>

              <text x="240" y="152" textAnchor="middle" fill="#f3ddab" fontSize="12">
                On the unit circle, cos(θ) is the x-coordinate (negative in Quadrants II and III).
              </text>
            </>
          ) : (
            <>
              <rect x="45" y="38" width="390" height="90" rx="6" fill="#1e293b" stroke="#7ad0b1" strokeWidth="2" />
              <text x="240" y="62" textAnchor="middle" fill="#e6c580" fontSize="14" fontWeight="bold">
                Periodic Wave:   f(t) = 4 cos(5π t) + 2
              </text>
              <text x="240" y="88" textAnchor="middle" fill="#edf1f5" fontSize="13">
                Amplitude |A| = 4,   Midline y = 2,   Angular Frequency B = 5π
              </text>
              <text x="240" y="112" textAnchor="middle" fill="#7ad0b1" fontSize="15" fontWeight="bold">
                Fundamental Period: T = 2π / (5π) = 2/5
              </text>
              <text x="240" y="150" textAnchor="middle" fill="#94a3b8" fontSize="12">
                One full cycle of cosine spans 2π radians, so 5π T = 2π ⇒ T = 2/5.
              </text>
            </>
          )}
        </svg>
        <div className="axiom-btn-group" role="group" aria-label="Select unit circle or wave model">
          <button
            type="button"
            className={`axiom-chip ${radMode === 'quadrant_ii' ? 'active' : ''}`}
            onClick={() => setRadMode('quadrant_ii')}
          >
            Radians &amp; Quadrant II Cosine
          </button>
          <button
            type="button"
            className={`axiom-chip ${radMode === 'wave_period' ? 'active' : ''}`}
            onClick={() => setRadMode('wave_period')}
          >
            Wave Period: T = 2π / (ωπ) = 2/ω
          </button>
        </div>
        <p role="status">
          {radMode === 'quadrant_ii'
            ? 'In Quadrant II, y = sin(θ) > 0 and x = cos(θ) < 0, so cos(θ) = −√(1 − sin²(θ)).'
            : 'The fundamental period T depends only on the angular frequency coefficient B = ωπ: T = 2π / (ωπ) = 2/ω.'}
        </p>
      </figure>
    );
  }

  if (lessonId === 'secondary.trig.pythagorean_and_addition_identities') {
    return (
      <figure className="concept-visual">
        <figcaption>Trigonometric Identities: Double-Angle sin(2θ) &amp; Angle-Sum cos(α + β)</figcaption>
        <svg viewBox="0 0 480 185" role="img" aria-label={trigIdMode === 'double_angle' ? 'Double angle identity sin(2 theta) = 2 sin(theta) cos(theta) = 24/25 for (3,4,5) triangle' : 'Angle sum identity cos(alpha + beta) = cos(alpha)cos(beta) - sin(alpha)sin(beta) = 33/65'}>
          <rect x="15" y="15" width="450" height="155" rx="6" fill="#131b26" stroke="#29394b" strokeWidth="1.5" />
          {trigIdMode === 'double_angle' ? (
            <>
              <rect x="40" y="38" width="400" height="90" rx="6" fill="#1e293b" stroke="#e6c580" strokeWidth="2" />
              <text x="240" y="62" textAnchor="middle" fill="#e6c580" fontSize="14" fontWeight="bold">
                Double-Angle Identity:   sin(2θ) = 2 sin(θ) cos(θ)
              </text>
              <text x="240" y="88" textAnchor="middle" fill="#edf1f5" fontSize="13">
                For sin(θ) = 3/5 and cos(θ) = 4/5 (so tan(θ) = 3/4):
              </text>
              <text x="240" y="112" textAnchor="middle" fill="#7ad0b1" fontSize="15" fontWeight="bold">
                sin(2θ) = 2 × (3/5) × (4/5) = 24/25   (NOT 2 × 3/5 = 6/5 &gt; 1!)
              </text>
              <text x="240" y="150" textAnchor="middle" fill="#f3ddab" fontSize="12">
                Multiplying by cos(θ) keeps |sin(2θ)| ≤ 1 on the unit circle.
              </text>
            </>
          ) : (
            <>
              <rect x="40" y="38" width="400" height="90" rx="6" fill="#1e293b" stroke="#60a5fa" strokeWidth="2" />
              <text x="240" y="62" textAnchor="middle" fill="#60a5fa" fontSize="14" fontWeight="bold">
                Angle-Sum Cosine:   cos(α + β) = cos(α)cos(β) − sin(α)sin(β)
              </text>
              <text x="240" y="88" textAnchor="middle" fill="#edf1f5" fontSize="13">
                With (sin α, cos α) = (3/5, 4/5) and (sin β, cos β) = (5/13, 12/13):
              </text>
              <text x="240" y="112" textAnchor="middle" fill="#7ad0b1" fontSize="15" fontWeight="bold">
                cos(α + β) = (4/5)(12/13) − (3/5)(5/13) = (48 − 15)/65 = 33/65
              </text>
              <text x="240" y="150" textAnchor="middle" fill="#94a3b8" fontSize="12">
                Note the minus sign in cos(α + β): adding acute angles decreases their cosine.
              </text>
            </>
          )}
        </svg>
        <div className="axiom-btn-group" role="group" aria-label="Select trigonometric identity model">
          <button
            type="button"
            className={`axiom-chip ${trigIdMode === 'double_angle' ? 'active' : ''}`}
            onClick={() => setTrigIdMode('double_angle')}
          >
            Double-Angle: sin(2θ) = 2 sin(θ)cos(θ)
          </button>
          <button
            type="button"
            className={`axiom-chip ${trigIdMode === 'angle_sum' ? 'active' : ''}`}
            onClick={() => setTrigIdMode('angle_sum')}
          >
            Angle-Sum: cos(α + β)
          </button>
        </div>
        <p role="status">
          {trigIdMode === 'double_angle'
            ? 'Sine is non-linear: doubling θ multiplies sin(θ) by 2 cos(θ) rather than 2.'
            : 'The cosine addition formula subtracts sin(α)sin(β) from cos(α)cos(β) over the common denominator c₁c₂.'}
        </p>
      </figure>
    );
  }

  if (lessonId === 'secondary.trig.triangle_laws_and_inverse') {
    return (
      <figure className="concept-visual">
        <figcaption>Triangle Laws &amp; Inverse Branches: Law of Cosines &amp; Principal arcsin</figcaption>
        <svg viewBox="0 0 480 185" role="img" aria-label={triLawMode === 'cosine_law' ? 'Law of Cosines on triangle with sides 5, 6, 7 yielding cos(C) = 1/5' : 'Principal branch folding arcsin(sin(5 pi / 6)) = pi / 6 in [-pi/2, pi/2]'}>
          <rect x="15" y="15" width="450" height="155" rx="6" fill="#131b26" stroke="#29394b" strokeWidth="1.5" />
          {triLawMode === 'cosine_law' ? (
            <>
              <rect x="40" y="38" width="400" height="90" rx="6" fill="#1e293b" stroke="#7ad0b1" strokeWidth="2" />
              <text x="240" y="62" textAnchor="middle" fill="#e6c580" fontSize="14" fontWeight="bold">
                Law of Cosines:   c² = a² + b² − 2ab cos(C)
              </text>
              <text x="240" y="88" textAnchor="middle" fill="#edf1f5" fontSize="13">
                For triangle sides a = 5, b = 6, c = 7:
              </text>
              <text x="240" y="112" textAnchor="middle" fill="#7ad0b1" fontSize="15" fontWeight="bold">
                cos(C) = (a² + b² − c²) / (2ab) = (25 + 36 − 49) / 60 = 12/60 = 1/5
              </text>
              <text x="240" y="150" textAnchor="middle" fill="#f3ddab" fontSize="12">
                Law of Sines: a / sin(A) = b / sin(B) ⇒ b = a · sin(B) / sin(A).
              </text>
            </>
          ) : (
            <>
              <rect x="40" y="38" width="400" height="90" rx="6" fill="#1e293b" stroke="#60a5fa" strokeWidth="2" />
              <text x="240" y="62" textAnchor="middle" fill="#60a5fa" fontSize="14" fontWeight="bold">
                Principal Branch of arcsin:   Range = [−π/2, π/2]
              </text>
              <text x="240" y="88" textAnchor="middle" fill="#edf1f5" fontSize="13">
                Since 5π/6 ∈ (π/2, π) lies outside [−π/2, π/2], reflect across π/2:
              </text>
              <text x="240" y="112" textAnchor="middle" fill="#7ad0b1" fontSize="15" fontWeight="bold">
                arcsin(sin(5π/6)) = arcsin(sin(π − 5π/6)) = π/6   (k = 1/6)
              </text>
              <text x="240" y="150" textAnchor="middle" fill="#94a3b8" fontSize="12">
                arcsin(sin(θ)) equals θ only when θ belongs to the principal interval [−π/2, π/2].
              </text>
            </>
          )}
        </svg>
        <div className="axiom-btn-group" role="group" aria-label="Select triangle law or inverse branch view">
          <button
            type="button"
            className={`axiom-chip ${triLawMode === 'cosine_law' ? 'active' : ''}`}
            onClick={() => setTriLawMode('cosine_law')}
          >
            Law of Cosines: cos(C) = (a² + b² − c²)/(2ab)
          </button>
          <button
            type="button"
            className={`axiom-chip ${triLawMode === 'principal_arcsin' ? 'active' : ''}`}
            onClick={() => setTriLawMode('principal_arcsin')}
          >
            Principal Branch: arcsin(sin(5π/6)) = π/6
          </button>
        </div>
        <p role="status">
          {triLawMode === 'cosine_law'
            ? 'The Law of Cosines generalizes the Pythagorean theorem to any triangle and determines cos(C) from the three side lengths.'
            : 'Restricting sine to [−π/2, π/2] makes it bijective onto [−1, 1], so Quadrant II angles fold back to their Quadrant I reference angle π − θ.'}
        </p>
      </figure>
    );
  }

  if (lessonId === 'secondary.sequences.arithmetic_and_series') {
    return (
      <figure className="concept-visual">
        <figcaption>Arithmetic Progressions: (n − 1) Fencepost Steps &amp; Gauss Pairing</figcaption>
        <svg viewBox="0 0 480 185" role="img" aria-label={arithSeqMode === 'nth_step' ? 'Arithmetic sequence a_1 = 4, d = 3 showing 9 steps to reach a_10 = 31' : 'Gauss pairing of S_10 = 4 + 7 + ... + 31 into 10 pairs of 35 divided by 2 = 175'}>
          <rect x="15" y="15" width="450" height="155" rx="6" fill="#131b26" stroke="#29394b" strokeWidth="1.5" />
          {arithSeqMode === 'nth_step' ? (
            <>
              <rect x="40" y="38" width="400" height="90" rx="6" fill="#1e293b" stroke="#60a5fa" strokeWidth="2" />
              <text x="240" y="62" textAnchor="middle" fill="#60a5fa" fontSize="14" fontWeight="bold">
                Explicit Formula:   a_n = a₁ + (n − 1)d
              </text>
              <text x="240" y="88" textAnchor="middle" fill="#edf1f5" fontSize="13">
                For a₁ = 4, d = 3, n = 10:   9 steps of +3 from a₁ to a₁₀
              </text>
              <text x="240" y="112" textAnchor="middle" fill="#7ad0b1" fontSize="15" fontWeight="bold">
                a₁₀ = 4 + (10 − 1)(3) = 4 + 27 = 31   (And d = (a_q − a_p)/(q − p))
              </text>
              <text x="240" y="150" textAnchor="middle" fill="#e6c580" fontSize="12">
                Moving from term 1 to term n takes (n − 1) jumps of d, never n jumps.
              </text>
            </>
          ) : (
            <>
              <rect x="40" y="38" width="400" height="90" rx="6" fill="#1e293b" stroke="#7ad0b1" strokeWidth="2" />
              <text x="240" y="62" textAnchor="middle" fill="#e6c580" fontSize="14" fontWeight="bold">
                Gauss&apos;s Pairing Identity:   S_n = n(a₁ + a_n) / 2
              </text>
              <text x="240" y="88" textAnchor="middle" fill="#edf1f5" fontSize="13">
                Forward + Reverse: (4 + 31) + (7 + 28) + … + (31 + 4) = 10 × 35
              </text>
              <text x="240" y="112" textAnchor="middle" fill="#7ad0b1" fontSize="15" fontWeight="bold">
                2·S₁₀ = 350  ⇒  S₁₀ = 10 × (4 + 31) / 2 = 175
              </text>
              <text x="240" y="150" textAnchor="middle" fill="#94a3b8" fontSize="12">
                Reversing the sum pairs every k-th term with the (n − k + 1)-th term to give constant sum (a₁ + a_n).
              </text>
            </>
          )}
        </svg>
        <div className="axiom-btn-group" role="group" aria-label="Select arithmetic progression model">
          <button
            type="button"
            className={`axiom-chip ${arithSeqMode === 'nth_step' ? 'active' : ''}`}
            onClick={() => setArithSeqMode('nth_step')}
          >
            n-th Term Steps: a₁ + (n − 1)d
          </button>
          <button
            type="button"
            className={`axiom-chip ${arithSeqMode === 'gauss_sum' ? 'active' : ''}`}
            onClick={() => setArithSeqMode('gauss_sum')}
          >
            Gauss Series Sum: n(a₁ + a_n) / 2
          </button>
        </div>
        <p role="status">
          {arithSeqMode === 'nth_step'
            ? 'Between index p and index q there are exactly (q − p) steps of size d, so a_q − a_p = (q − p)d.'
            : 'Doubling the arithmetic series and reversing the second copy produces n identical columns of height (a₁ + a_n).'}
        </p>
      </figure>
    );
  }

  if (lessonId === 'secondary.sequences.geometric_and_infinite_series') {
    return (
      <figure className="concept-visual">
        <figcaption>Geometric Series: Telescoping Finite Sum &amp; Infinite Convergence (|r| &lt; 1)</figcaption>
        <svg viewBox="0 0 480 185" role="img" aria-label={geomSeqMode === 'finite_telescope' ? 'Finite geometric sum S_4 = 3(2^4 - 1)/(2 - 1) = 45' : 'Convergent infinite geometric series a = 6, r = 1/3 with sum S_inf = 6 / (1 - 1/3) = 9'}>
          <rect x="15" y="15" width="450" height="155" rx="6" fill="#131b26" stroke="#29394b" strokeWidth="1.5" />
          {geomSeqMode === 'finite_telescope' ? (
            <>
              <rect x="40" y="38" width="400" height="90" rx="6" fill="#1e293b" stroke="#e6c580" strokeWidth="2" />
              <text x="240" y="62" textAnchor="middle" fill="#e6c580" fontSize="14" fontWeight="bold">
                Finite Geometric Series:   S_n = a(r^n − 1) / (r − 1)
              </text>
              <text x="240" y="88" textAnchor="middle" fill="#edf1f5" fontSize="13">
                For a = 3, r = 2, n = 4:   g₄ = 3 · 2³ = 24,   and   r·S₄ − S₄ = 3(2⁴ − 1)
              </text>
              <text x="240" y="112" textAnchor="middle" fill="#7ad0b1" fontSize="15" fontWeight="bold">
                S₄ = 3(16 − 1) / (2 − 1) = 45   (3 + 6 + 12 + 24 = 45 ✓)
              </text>
              <text x="240" y="150" textAnchor="middle" fill="#f3ddab" fontSize="12">
                Note: the n-th term uses r^(n−1), whereas the sum of n terms uses (r^n − 1)/(r − 1).
              </text>
            </>
          ) : (
            <>
              <rect x="40" y="38" width="400" height="90" rx="6" fill="#1e293b" stroke="#60a5fa" strokeWidth="2" />
              <text x="240" y="62" textAnchor="middle" fill="#60a5fa" fontSize="14" fontWeight="bold">
                Convergent Infinite Series (|r| &lt; 1):   S_∞ = a / (1 − r)
              </text>
              <text x="240" y="88" textAnchor="middle" fill="#edf1f5" fontSize="13">
                For a = 6 and r = 1/3:   As n → ∞,  (1/3)^n → 0,  so  1 − r = 2/3
              </text>
              <text x="240" y="112" textAnchor="middle" fill="#7ad0b1" fontSize="15" fontWeight="bold">
                S_∞ = 6 ÷ (2/3) = 6 × (3/2) = 9
              </text>
              <text x="240" y="150" textAnchor="middle" fill="#94a3b8" fontSize="12">
                When |r| &lt; 1, the tail ar^n vanishes in the limit, leaving S_∞ = a / (1 − r).
              </text>
            </>
          )}
        </svg>
        <div className="axiom-btn-group" role="group" aria-label="Select geometric series model">
          <button
            type="button"
            className={`axiom-chip ${geomSeqMode === 'finite_telescope' ? 'active' : ''}`}
            onClick={() => setGeomSeqMode('finite_telescope')}
          >
            Finite Sum: a(r^n − 1)/(r − 1)
          </button>
          <button
            type="button"
            className={`axiom-chip ${geomSeqMode === 'infinite_sum' ? 'active' : ''}`}
            onClick={() => setGeomSeqMode('infinite_sum')}
          >
            Infinite Limit (|r| &lt; 1): a / (1 − r)
          </button>
        </div>
        <p role="status">
          {geomSeqMode === 'finite_telescope'
            ? 'Subtracting S_n from r·S_n cancels every middle power ar, ar², …, ar^(n−1), leaving (r − 1)S_n = a(r^n − 1).'
            : 'For |r| < 1, the geometric partial sums S_n = a(1 − r^n)/(1 − r) converge to the exact rational value a / (1 − r).'}
        </p>
      </figure>
    );
  }

  if (lessonId === 'secondary.sequences.recursion_and_binomial') {
    return (
      <figure className="concept-visual">
        <figcaption>Discrete Iteration: Linear Recurrences &amp; The Binomial Theorem</figcaption>
        <svg viewBox="0 0 480 185" role="img" aria-label={binomMode === 'recurrence' ? 'Linear recurrence a_1 = 2, a_n = 3a_(n-1) + 1 stepped to a_2 = 7, a_3 = 22, a_4 = 67' : 'Binomial expansion term [x^2] in (2x + 3)^4 = C(4,2) * 2^2 * 3^2 = 216'}>
          <rect x="15" y="15" width="450" height="155" rx="6" fill="#131b26" stroke="#29394b" strokeWidth="1.5" />
          {binomMode === 'recurrence' ? (
            <>
              <rect x="40" y="38" width="400" height="90" rx="6" fill="#1e293b" stroke="#60a5fa" strokeWidth="2" />
              <text x="240" y="62" textAnchor="middle" fill="#60a5fa" fontSize="14" fontWeight="bold">
                Iterative Recurrence:   a₁ = 2,   a_n = 3a_(n−1) + 1
              </text>
              <text x="240" y="88" textAnchor="middle" fill="#edf1f5" fontSize="13">
                a₂ = 3(2) + 1 = 7   →   a₃ = 3(7) + 1 = 22   →   a₄ = 3(22) + 1 = 67
              </text>
              <text x="240" y="112" textAnchor="middle" fill="#7ad0b1" fontSize="15" fontWeight="bold">
                Exact 4th Term: a₄ = 67
              </text>
              <text x="240" y="150" textAnchor="middle" fill="#e6c580" fontSize="12">
                Each transition scales the entire previous state by c = 3 before adding d = 1.
              </text>
            </>
          ) : (
            <>
              <rect x="40" y="38" width="400" height="90" rx="6" fill="#1e293b" stroke="#7ad0b1" strokeWidth="2" />
              <text x="240" y="62" textAnchor="middle" fill="#e6c580" fontSize="14" fontWeight="bold">
                Binomial Coefficient of x² in (2x + 3)⁴
              </text>
              <text x="240" y="88" textAnchor="middle" fill="#edf1f5" fontSize="13">
                Term k = 2:   C(4, 2) · (2x)^(4−2) · 3² = 6 · (4x²) · 9
              </text>
              <text x="240" y="112" textAnchor="middle" fill="#7ad0b1" fontSize="15" fontWeight="bold">
                [x²] = 6 × 4 × 9 = 216
              </text>
              <text x="240" y="150" textAnchor="middle" fill="#94a3b8" fontSize="12">
                Remember to raise BOTH the x-coefficient a = 2 and constant b = 3 to their powers!
              </text>
            </>
          )}
        </svg>
        <div className="axiom-btn-group" role="group" aria-label="Select recursion or binomial view">
          <button
            type="button"
            className={`axiom-chip ${binomMode === 'recurrence' ? 'active' : ''}`}
            onClick={() => setBinomMode('recurrence')}
          >
            Recurrence Chain: a_n = c·a_(n−1) + d
          </button>
          <button
            type="button"
            className={`axiom-chip ${binomMode === 'binomial_term' ? 'active' : ''}`}
            onClick={() => setBinomMode('binomial_term')}
          >
            Binomial Term: C(n, k) a^(n−k) b^k
          </button>
        </div>
        <p role="status">
          {binomMode === 'recurrence'
            ? 'Unrolling a recurrence step-by-step from a₁ tracks how multiplicative scaling and additive shifts compound.'
            : 'In (ax + b)^n, choosing b from k factors and ax from (n − k) factors in C(n, k) ways produces C(n, k) a^(n−k) b^k x^(n−k).'}
        </p>
      </figure>
    );
  }

  if (lessonId === 'secondary.calculus.limits_and_continuity') {
    return (
      <figure className="concept-visual">
        <figcaption>Limits &amp; Continuity: Healing a Removable Hole vs. Dominant Degree at ∞</figcaption>
        <svg viewBox="0 0 480 185" role="img" aria-label={limitMode === 'removable_hole' ? 'Removable discontinuity (x^2 - 9)/(x - 3) = x + 3 approaching 6 as x approaches 3' : 'Limit at infinity of (6x^2 + 5x + 4)/(7x^2 - 3x + 8) = 6/7'}>
          <rect x="15" y="15" width="450" height="155" rx="6" fill="#131b26" stroke="#29394b" strokeWidth="1.5" />
          {limitMode === 'removable_hole' ? (
            <>
              <rect x="40" y="38" width="400" height="90" rx="6" fill="#1e293b" stroke="#60a5fa" strokeWidth="2" />
              <text x="240" y="62" textAnchor="middle" fill="#60a5fa" fontSize="14" fontWeight="bold">
                Removable Discontinuity (0/0):   lim_(x→3) (x² − 9) / (x − 3)
              </text>
              <text x="240" y="88" textAnchor="middle" fill="#edf1f5" fontSize="13">
                For x ≠ 3, cancel (x − 3):   (x − 3)(x + 3) / (x − 3) = x + 3
              </text>
              <text x="240" y="112" textAnchor="middle" fill="#7ad0b1" fontSize="15" fontWeight="bold">
                lim_(x→3) (x + 3) = 6   (Continuous at x = 3 iff f(3) = 3k + 2 = 6 ⇒ k = 4/3)
              </text>
              <text x="240" y="150" textAnchor="middle" fill="#e6c580" fontSize="12">
                Because the limit inspects x near 3 (with x ≠ 3), dividing by (x − 3) is exact and valid.
              </text>
            </>
          ) : (
            <>
              <rect x="40" y="38" width="400" height="90" rx="6" fill="#1e293b" stroke="#7ad0b1" strokeWidth="2" />
              <text x="240" y="62" textAnchor="middle" fill="#e6c580" fontSize="14" fontWeight="bold">
                Limit at Infinity:   lim_(x→∞) (6x² + 5x + 4) / (7x² − 3x + 8)
              </text>
              <text x="240" y="88" textAnchor="middle" fill="#edf1f5" fontSize="13">
                Divide by x²:   (6 + 5/x + 4/x²) / (7 − 3/x + 8/x²)
              </text>
              <text x="240" y="112" textAnchor="middle" fill="#7ad0b1" fontSize="15" fontWeight="bold">
                As x → ∞,  1/x → 0  and  1/x² → 0  ⇒  Limit = 6/7
              </text>
              <text x="240" y="150" textAnchor="middle" fill="#94a3b8" fontSize="12">
                Equal-degree rational functions approach the ratio of their leading coefficients a/d.
              </text>
            </>
          )}
        </svg>
        <div className="axiom-btn-group" role="group" aria-label="Select limit concept model">
          <button
            type="button"
            className={`axiom-chip ${limitMode === 'removable_hole' ? 'active' : ''}`}
            onClick={() => setLimitMode('removable_hole')}
          >
            Removable Hole &amp; Continuity
          </button>
          <button
            type="button"
            className={`axiom-chip ${limitMode === 'infinity_lead' ? 'active' : ''}`}
            onClick={() => setLimitMode('infinity_lead')}
          >
            Horizontal Asymptote (x → ∞): a/d
          </button>
        </div>
        <p role="status">
          {limitMode === 'removable_hole'
            ? 'Factoring and canceling the zero factor (x − r) resolves the 0/0 form and reveals the exact limit as x → r.'
            : 'Dividing every term by the highest power x² proves that all lower-degree terms vanish as x grows without bound.'}
        </p>
      </figure>
    );
  }

  if (lessonId === 'secondary.calculus.derivatives_and_tangents') {
    return (
      <figure className="concept-visual">
        <figcaption>Differential Calculus: Tangent Line y = f&apos;(x₀)(x − x₀) + f(x₀) &amp; Critical Points</figcaption>
        <svg viewBox="0 0 480 185" role="img" aria-label={derivMode === 'tangent_line' ? 'Tangent line to f(x) = 2x^2 + 3x + 5 at x_0 = 2 with slope m = 11 and y-intercept B = -3' : 'Critical point of f(x) = 3x^2 - 7x + 4 where f prime(x) = 6x - 7 = 0 at x = 7/6'}>
          <rect x="15" y="15" width="450" height="155" rx="6" fill="#131b26" stroke="#29394b" strokeWidth="1.5" />
          {derivMode === 'tangent_line' ? (
            <>
              <rect x="40" y="38" width="400" height="90" rx="6" fill="#1e293b" stroke="#e6c580" strokeWidth="2" />
              <text x="240" y="62" textAnchor="middle" fill="#e6c580" fontSize="14" fontWeight="bold">
                Tangent Line to f(x) = 2x² + 3x + 5 at x₀ = 2
              </text>
              <text x="240" y="88" textAnchor="middle" fill="#edf1f5" fontSize="13">
                Point: f(2) = 19,     Slope: f&apos;(x) = 4x + 3  ⇒  m = f&apos;(2) = 11
              </text>
              <text x="240" y="112" textAnchor="middle" fill="#7ad0b1" fontSize="15" fontWeight="bold">
                y = 11(x − 2) + 19 = 11x − 3   ⇒   y-intercept B = 19 − 22 = −3
              </text>
              <text x="240" y="150" textAnchor="middle" fill="#f3ddab" fontSize="12">
                Power Rule: d/dx(ax³ + bx² + cx + d) = 3ax² + 2bx + c (constant d has derivative 0).
              </text>
            </>
          ) : (
            <>
              <rect x="40" y="38" width="400" height="90" rx="6" fill="#1e293b" stroke="#60a5fa" strokeWidth="2" />
              <text x="240" y="62" textAnchor="middle" fill="#60a5fa" fontSize="14" fontWeight="bold">
                Stationary / Critical Point (Horizontal Tangent): f&apos;(x) = 0
              </text>
              <text x="240" y="88" textAnchor="middle" fill="#edf1f5" fontSize="13">
                For f(x) = 3x² − 7x + 4:   f&apos;(x) = 2(3)x − 7 = 6x − 7 = 0
              </text>
              <text x="240" y="112" textAnchor="middle" fill="#7ad0b1" fontSize="15" fontWeight="bold">
                Exact Critical Point: x = b / (2a) = 7/6
              </text>
              <text x="240" y="150" textAnchor="middle" fill="#94a3b8" fontSize="12">
                Setting f&apos;(x) = 0 locates the exact vertex axis of symmetry of any parabola.
              </text>
            </>
          )}
        </svg>
        <div className="axiom-btn-group" role="group" aria-label="Select derivative concept model">
          <button
            type="button"
            className={`axiom-chip ${derivMode === 'tangent_line' ? 'active' : ''}`}
            onClick={() => setDerivMode('tangent_line')}
          >
            Tangent Line: B = f(x₀) − f&apos;(x₀)x₀
          </button>
          <button
            type="button"
            className={`axiom-chip ${derivMode === 'critical_point' ? 'active' : ''}`}
            onClick={() => setDerivMode('critical_point')}
          >
            Horizontal Tangent: f&apos;(x) = 2ax − b = 0
          </button>
        </div>
        <p role="status">
          {derivMode === 'tangent_line'
            ? 'The derivative f\'(x₀) gives the instantaneous slope m at (x₀, f(x₀)), anchoring the tangent line y = m(x − x₀) + f(x₀).'
            : 'At a smooth minimum or maximum, the tangent line is horizontal so the instantaneous rate of change f\'(x) equals 0.'}
        </p>
      </figure>
    );
  }

  if (lessonId === 'secondary.calculus.integrals_and_ftc') {
    return (
      <figure className="concept-visual">
        <figcaption>Integral Calculus: Fundamental Theorem F(b) − F(a) &amp; Net Change s(0) + ∫v(t)dt</figcaption>
        <svg viewBox="0 0 480 185" role="img" aria-label={intMode === 'ftc_area' ? 'Definite integral from 0 to 2 of (4x^2 + 3) dx = [4/3 x^3 + 3x]_0^2 = 50/3' : 'Net Change Theorem s(t) = s(0) + int_0^t v(tau) dtau'}>
          <rect x="15" y="15" width="450" height="155" rx="6" fill="#131b26" stroke="#29394b" strokeWidth="1.5" />
          {intMode === 'ftc_area' ? (
            <>
              <rect x="40" y="38" width="400" height="90" rx="6" fill="#1e293b" stroke="#7ad0b1" strokeWidth="2" />
              <text x="240" y="62" textAnchor="middle" fill="#e6c580" fontSize="14" fontWeight="bold">
                Fundamental Theorem of Calculus:   ∫₀² (4x² + 3) dx = F(2) − F(0)
              </text>
              <text x="240" y="88" textAnchor="middle" fill="#edf1f5" fontSize="13">
                Antiderivative: F(x) = (4/3)x³ + 3x   (since F&apos;(x) = 4x² + 3)
              </text>
              <text x="240" y="112" textAnchor="middle" fill="#7ad0b1" fontSize="15" fontWeight="bold">
                F(2) − F(0) = (4/3)(8) + 3(2) = 32/3 + 18/3 = 50/3
              </text>
              <text x="240" y="150" textAnchor="middle" fill="#f3ddab" fontSize="12">
                Reverse Power Rule: ∫ x^n dx = x^(n+1)/(n + 1), and ∫ c dx = c·x.
              </text>
            </>
          ) : (
            <>
              <rect x="40" y="38" width="400" height="90" rx="6" fill="#1e293b" stroke="#60a5fa" strokeWidth="2" />
              <text x="240" y="62" textAnchor="middle" fill="#60a5fa" fontSize="14" fontWeight="bold">
                Net Change Theorem:   s(t) = s(0) + ∫₀^t v(τ) dτ
              </text>
              <text x="240" y="88" textAnchor="middle" fill="#edf1f5" fontSize="13">
                For s(0) = 10 and v(t) = 6t² + 4t on [0, 2]:   ∫₀² v(t) dt = [2t³ + 2t²]₀² = 24
              </text>
              <text x="240" y="112" textAnchor="middle" fill="#7ad0b1" fontSize="15" fontWeight="bold">
                Final Position: s(2) = s(0) + 24 = 10 + 24 = 34
              </text>
              <text x="240" y="150" textAnchor="middle" fill="#94a3b8" fontSize="12">
                Always add the initial value s(0) to the net displacement ∫₀^t v(τ) dτ!
              </text>
            </>
          )}
        </svg>
        <div className="axiom-btn-group" role="group" aria-label="Select integral calculus model">
          <button
            type="button"
            className={`axiom-chip ${intMode === 'ftc_area' ? 'active' : ''}`}
            onClick={() => setIntMode('ftc_area')}
          >
            FTC Evaluation: F(b) − F(a)
          </button>
          <button
            type="button"
            className={`axiom-chip ${intMode === 'net_change' ? 'active' : ''}`}
            onClick={() => setIntMode('net_change')}
          >
            Net Change: s(t) = s(0) + ∫₀^t v(τ)dτ
          </button>
        </div>
        <p role="status">
          {intMode === 'ftc_area'
            ? 'Because differentiation and integration are inverse operations, accumulated area under f(x) equals the net change in its antiderivative F(x).'
            : 'Integrating velocity over [0, t] yields net displacement Δs = s(t) − s(0); adding s(0) recovers the exact position s(t).'}
        </p>
      </figure>
    );
  }

  return null;
};
