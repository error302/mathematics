import React, { useState } from 'react';
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
}

export const LESSONS: LessonDef[] = [
  {
    id: "foundation.fractions.compare",
    courseId: "F04",
    title: "Comparing Fractions: The Common Denominator",
    subtitle: "Why comparing fractions requires equal-sized units, and how to find them using rational equivalence.",
    templateId: "fractions.compare.positive",
    challenge: "Two cylinders of equal capacity are filled to 3/4 and 5/8 of their volumes. Which cylinder contains more liquid?",
    predictionPrompt: "Make a prediction: Is 3/4 smaller than, equal to, or greater than 5/8?",
    predictionOptions: [
      { label: "3/4 < 5/8", value: "less" },
      { label: "3/4 = 5/8", value: "equal" },
      { label: "3/4 > 5/8", value: "greater", isCorrect: true },
    ],
    formalDefinition: "A positive rational fraction a/b denotes a parts of a whole partitioned into b equal pieces. The denominator b determines part size; the numerator a counts the parts. Comparison requires identical part sizes.",
    workedExample: {
      steps: [
        "Find the least common multiple of 4 and 8: lcm(4, 8) = 8.",
        "Convert 3/4 by multiplying numerator and denominator by 2: (3×2)/(4×2) = 6/8.",
        "Compare numerators: 6/8 > 5/8.",
      ],
      result: "Therefore, 3/4 is strictly greater than 5/8.",
    },
    misconception: {
      mistake: "Since 5 > 3 and 8 > 4, 5/8 must be greater than 3/4.",
      correction: "Fractions cannot be compared by numerators alone when part sizes differ. Each eighth is half the size of a fourth.",
    },
    reflection: "To compare two rational numbers a/b and c/d, scale both to a common denominator D = lcm(b, d) and compare the integer numerators.",
  },
  {
    id: "foundation.fractions.unit_fractions",
    courseId: "F04",
    title: "Unit Fractions: Partitioning the Whole",
    subtitle: "Reciprocal relationship between partition size and piece magnitude.",
    templateId: "fractions.unit.identify",
    challenge: "A ribbon of length 1 meter is cut into 5 identical pieces. What fraction of the ribbon is each piece?",
    predictionPrompt: "If the ribbon was cut into 10 pieces instead of 5, would each piece be larger or smaller?",
    predictionOptions: [
      { label: "1/10 is larger than 1/5", value: "larger" },
      { label: "1/10 is equal to 1/5", value: "equal" },
      { label: "1/10 is smaller than 1/5", value: "smaller", isCorrect: true },
    ],
    formalDefinition: "A unit fraction 1/n (n in Z+) represents exactly one part when a whole unit is divided into n equal parts. As n grows, 1/n decreases monotonically toward zero.",
    workedExample: {
      steps: [
        "Partition the unit interval [0, 1] into 5 equal sub-intervals.",
        "Each segment has length 1/5.",
        "Summing all 5 parts restores the whole: 1/5 + 1/5 + 1/5 + 1/5 + 1/5 = 5/5 = 1.",
      ],
      result: "Every proper fraction m/n is simply m copies of the unit fraction 1/n.",
    },
    misconception: {
      mistake: "1/8 is larger than 1/4 because 8 is larger than 4.",
      correction: "The denominator indicates division: cutting a pie into 8 slices yields smaller slices than cutting into 4.",
    },
    reflection: "Unit fractions are the elemental basis of all rational numbers. Understanding 1/n grounds all fraction arithmetic.",
  },
  {
    id: "foundation.place_value.decimal_expansion",
    courseId: "F01",
    title: "Positional Notation & Decimal Expansion",
    subtitle: "Representing quantities as weighted polynomial sums of powers of ten.",
    templateId: "place_value.decompose",
    challenge: "Why does the numeral 74 represent a vastly different quantity than 47, using the exact same two symbols?",
    predictionPrompt: "In the number 843, what does the digit 4 represent?",
    predictionOptions: [
      { label: "4 units (4)", value: "4" },
      { label: "4 tens (40)", value: "40", isCorrect: true },
      { label: "4 hundreds (400)", value: "400" },
    ],
    formalDefinition: "In base 10, an integer with digits d_k...d_0 equals sum_{i=0}^k d_i * 10^i. The place value equals the face value multiplied by 10^i.",
    workedExample: {
      steps: [
        "Take 843: digit 3 is in units (3 * 10^0 = 3).",
        "Digit 4 is in tens (4 * 10^1 = 40).",
        "Digit 8 is in hundreds (8 * 10^2 = 800).",
      ],
      result: "800 + 40 + 3 = 843.",
    },
    misconception: {
      mistake: "Confusing face value with place value.",
      correction: "The symbol 4 always has face value 4, but its contribution to the number depends entirely on its column position.",
    },
    reflection: "Positional notation is one of humanity's greatest mathematical inventions, allowing 10 symbols to represent the infinite.",
  },
  {
    id: "foundation.place_value.regrouping",
    courseId: "F01",
    title: "Base-10 Regrouping & Trading",
    subtitle: "Bundling 10 units into 1 ten, and unbundling 1 ten into 10 units.",
    templateId: "place_value.decompose",
    challenge: "You have 14 single unit blocks. How do you express this using standard base-10 rods and units?",
    predictionPrompt: "Does regrouping 10 units into 1 ten change the total quantity?",
    predictionOptions: [
      { label: "Yes, 1 ten is greater than 10 units", value: "greater" },
      { label: "No, 1 ten is exactly equal to 10 units", value: "equal", isCorrect: true },
      { label: "Yes, it becomes smaller", value: "smaller" },
    ],
    formalDefinition: "The base-10 invariant: 10 * 10^k = 1 * 10^(k+1). Regrouping preserves exact numerical equality while maintaining the canonical rule that no column contains more than 9 units.",
    workedExample: {
      steps: [
        "14 units = 10 units + 4 units.",
        "Trade 10 units for 1 ten rod: 1 ten + 4 units.",
        "Write in positional columns: tens = 1, units = 4.",
      ],
      result: "Total = 14.",
    },
    misconception: {
      mistake: "Writing 14 in a single column.",
      correction: "Base-10 positional columns can hold only digits 0 through 9. Ten units must roll into the next column.",
    },
    reflection: "Regrouping is reversible: bundling 10 units into 1 ten is carrying; unbundling 1 ten into 10 units is borrowing.",
  },
  {
    id: "foundation.arithmetic.addition_disjoint",
    courseId: "F02",
    title: "Addition: Union of Disjoint Sets",
    subtitle: "Cardinals of disjoint sets and the algebraic laws of addition.",
    templateId: "arithmetic.whole.addition",
    challenge: "Box A has 5 red beads, Box B has 3 green beads. How many beads are in the combined collection?",
    predictionPrompt: "Does changing the order of addition change the final sum? (5 + 3 vs 3 + 5)",
    predictionOptions: [
      { label: "Order matters", value: "diff" },
      { label: "Order does not matter (commutative)", value: "same", isCorrect: true },
    ],
    formalDefinition: "For disjoint finite sets A and B (A cap B = emptyset), |A cup B| = |A| + |B|. Addition is commutative (a+b = b+a) and associative ((a+b)+c = a+(b+c)).",
    workedExample: {
      steps: [
        "To calculate 7 + 8, decompose 8 into 3 + 5 to make a ten.",
        "7 + (3 + 5) = (7 + 3) + 5 by associativity.",
        "10 + 5 = 15.",
      ],
      result: "7 + 8 = 15.",
    },
    misconception: {
      mistake: "Adding non-disjoint sets without subtracting the overlap.",
      correction: "If collections share common elements, naive addition double-counts the intersection.",
    },
    reflection: "Making a ten using associativity simplifies all mental and written addition.",
  },
  {
    id: "foundation.arithmetic.subtraction_inverse",
    courseId: "F02",
    title: "Subtraction: Additive Inverse & Difference",
    subtitle: "Modeling subtraction as the search for a missing addend.",
    templateId: "arithmetic.whole.addition",
    challenge: "A road is 12 km long. You have walked 7 km. How many kilometers remain?",
    predictionPrompt: "Which equation defines 12 - 7 = c?",
    predictionOptions: [
      { label: "c + 7 = 12", value: "add", isCorrect: true },
      { label: "c - 7 = 12", value: "sub" },
      { label: "c * 7 = 12", value: "mul" },
    ],
    formalDefinition: "Subtraction is the inverse of addition: a - b = c <=> c + b = a. On the number line, a - b represents directed distance.",
    workedExample: {
      steps: [
        "To compute 43 - 28, find what to add to 28 to reach 43.",
        "28 + 2 = 30 (nearest ten).",
        "30 + 13 = 43.",
        "Total added: 2 + 13 = 15.",
      ],
      result: "43 - 28 = 15.",
    },
    misconception: {
      mistake: "Assuming subtraction is commutative (thinking 12 - 7 equals 7 - 12).",
      correction: "Subtraction is non-commutative: 12 - 7 = 5, while 7 - 12 = -5.",
    },
    reflection: "Thinking of subtraction as a missing addend transforms subtraction into familiar addition.",
  },
  {
    id: "foundation.arithmetic.column_addition_carry",
    courseId: "F02",
    title: "Column Addition with Carry Regrouping",
    subtitle: "Executing columnar addition and transferring bundles of 10.",
    templateId: "arithmetic.column.addition",
    challenge: "Add 47 + 38. The units sum to 15. How is this recorded in columnar notation?",
    predictionPrompt: "What is carried to the tens column in 47 + 38?",
    predictionOptions: [
      { label: "Carry 5", value: "5" },
      { label: "Carry 1", value: "1", isCorrect: true },
      { label: "Nothing carried", value: "0" },
    ],
    formalDefinition: "Column addition works right-to-left. When the sum in column k exceeds 9, the quotient floor(sum/10) is added to column k+1 as a carry.",
    workedExample: {
      steps: [
        "Units column: 7 + 8 = 15. Write 5, carry 1 ten.",
        "Tens column: 1 (carried) + 4 + 3 = 8.",
      ],
      result: "Total = 85.",
    },
    misconception: {
      mistake: "Writing 15 in the units column (resulting in 715).",
      correction: "Each column holds exactly one digit. The 1 in 15 represents 1 ten and belongs in the tens column.",
    },
    reflection: "Carrying is simply the algebraic regrouping of 10 smaller units into 1 unit of ten.",
  },
  {
    id: "foundation.arithmetic.column_subtraction_borrow",
    courseId: "F02",
    title: "Column Subtraction with Decomposition",
    subtitle: "Unbundling tens to execute multi-digit differences.",
    templateId: "arithmetic.column.subtraction",
    challenge: "Compute 52 - 27. The units column asks 2 - 7. How do we proceed?",
    predictionPrompt: "When we borrow 1 ten from 52, what does the units digit 2 become?",
    predictionOptions: [
      { label: "It becomes 3", value: "3" },
      { label: "It becomes 12", value: "12", isCorrect: true },
      { label: "It becomes 7", value: "7" },
    ],
    formalDefinition: "When the minuend digit is smaller than the subtrahend digit, decompose 1 unit of the adjacent higher column (10^1) into 10 units of the current column (10^0).",
    workedExample: {
      steps: [
        "Borrow 1 ten from 5 (leaves 4 tens).",
        "Add 10 to 2 units to make 12 units.",
        "Units: 12 - 7 = 5.",
        "Tens: 4 - 2 = 2.",
      ],
      result: "52 - 27 = 25.",
    },
    misconception: {
      mistake: "Subtracting the smaller from the larger digit blindly: 7 - 2 = 5, writing 35.",
      correction: "Subtraction is directional: we must remove 27 from 52, not recombine arbitrary differences.",
    },
    reflection: "Decomposition and borrowing are exact mirror images of bundling and carrying.",
  },
  {
    id: "abacus.orientation.place_value",
    courseId: "A01",
    title: "Soroban 1:4 Orientation & Reckoning Beam",
    subtitle: "Physical instrument structure, active vs inactive beads, and place value rods.",
    templateId: "abacus.read.state",
    challenge: "How does a Soroban represent numbers using a single upper bead and four lower beads?",
    predictionPrompt: "When are beads counted toward the active value on a Soroban?",
    predictionOptions: [
      { label: "When pushed toward the outer frame", value: "frame" },
      { label: "When moved toward the reckoning beam", value: "beam", isCorrect: true },
    ],
    formalDefinition: "A Soroban rod has 1 upper bead (value 5) and 4 lower beads (value 1 each). Beads are active strictly when moved toward the central reckoning beam.",
    workedExample: {
      steps: [
        "Neutral state: all upper beads pushed UP; all lower beads pulled DOWN (reads 0).",
        "To set 7: move upper bead DOWN (5) and two lower beads UP (2).",
        "5 + 2 = 7.",
      ],
      result: "Rods represent powers of ten from right to left.",
    },
    misconception: {
      mistake: "Counting beads pushed against the outer wooden frame.",
      correction: "Only beads touching or shifted toward the central reckoning beam are active.",
    },
    reflection: "The 1:4 Soroban bi-quinary system minimizes finger travel and provides instant tactile feedback.",
  },
  {
    id: "abacus.orientation.bead_setting",
    courseId: "A01",
    title: "Single-Rod Bead Manipulation (0–9)",
    subtitle: "Tactile fingering rules: thumb for lower beads up, index finger for down and upper beads.",
    templateId: "abacus.target.setting",
    challenge: "Which fingers should you use to set the number 6 on a single rod in a single swift motion?",
    predictionPrompt: "How do you set 6 in standard Japanese Soroban fingering?",
    predictionOptions: [
      { label: "Push lower bead then upper bead with index finger", value: "index" },
      { label: "Pinch thumb and index finger together toward the beam", value: "pinch", isCorrect: true },
    ],
    formalDefinition: "Standard fingering: thumb pushes lower beads up (+1 to +4); index finger pulls lower beads down (-1 to -4); index finger moves upper bead down (+5) and up (-5). Digits 6-9 use a simultaneous pinch.",
    workedExample: {
      steps: [
        "Start from cleared neutral rod.",
        "Simultaneously bring 1 upper bead down and 1 lower bead up with a pinch.",
        "Read: 5 + 1 = 6.",
      ],
      result: "Single efficient tactile movement.",
    },
    misconception: {
      mistake: "Using the index finger to push lower beads up.",
      correction: "The thumb has greater tactile strength and ergonomic speed for upward pushes.",
    },
    reflection: "Consistent fingering creates muscle memory, which later enables rapid mental abacus (Anzan) calculation.",
  },
  {
    id: "abacus.direct.addition",
    courseId: "A02",
    title: "Direct Addition on the Soroban",
    subtitle: "Adding beads directly when sufficient inactive beads are available on the rod.",
    templateId: "abacus.target.setting",
    challenge: "You have 2 lower beads active. You want to add 2. Can you do this directly?",
    predictionPrompt: "Are there enough lower beads to compute 2 + 2 directly?",
    predictionOptions: [
      { label: "Yes, 2 inactive lower beads remain", value: "yes", isCorrect: true },
      { label: "No, we must use the upper bead", value: "no" },
    ],
    formalDefinition: "Direct addition applies when L lower beads are active and k are to be added, with L + k <= 4. Push k beads up directly with the thumb.",
    workedExample: {
      steps: [
        "Set 2 lower beads up.",
        "Push 2 more lower beads up.",
        "All 4 lower beads now touch the beam.",
      ],
      result: "2 + 2 = 4.",
    },
    misconception: {
      mistake: "Trying to add 2 directly when 3 lower beads are already active (3 + 2).",
      correction: "Only 1 lower bead is available. Adding 2 requires the 5-complement formula (+2 = +5 - 3).",
    },
    reflection: "Direct addition is the foundational reflex of all abacus arithmetic.",
  },
  {
    id: "abacus.direct.subtraction",
    courseId: "A02",
    title: "Direct Subtraction on the Soroban",
    subtitle: "Removing beads directly when sufficient active beads touch the beam.",
    templateId: "abacus.target.setting",
    challenge: "You have 4 lower beads active. You want to subtract 3. How is this done?",
    predictionPrompt: "Which finger pulls the 3 lower beads down away from the beam?",
    predictionOptions: [
      { label: "Thumb", value: "thumb" },
      { label: "Index finger", value: "index", isCorrect: true },
    ],
    formalDefinition: "Direct subtraction removes active beads touching the beam directly away using the index finger, requiring no borrowing from higher rods or 5-complements.",
    workedExample: {
      steps: [
        "Set 4 lower beads up.",
        "Pull 3 lower beads down away from the beam with the index finger.",
        "1 lower bead remains active.",
      ],
      result: "4 - 3 = 1.",
    },
    misconception: {
      mistake: "Attempting direct subtraction on 5 - 2 when no lower beads are active.",
      correction: "Direct subtraction requires active beads of the right type. 5 - 2 requires the 5-complement formula (-2 = -5 + 3).",
    },
    reflection: "Direct operations establish the baseline tactile vocabulary of the Soroban.",
  },
];

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

  const lesson = LESSONS.find((l) => l.id === selectedId) || LESSONS[0];

  const handleSelectLesson = (id: string) => {
    setSelectedId(id);
    setPrediction(null);
    setShowExplanation(false);
  };

  const handlePredict = (value: string) => {
    setPrediction(value);
    setShowExplanation(true);
  };

  return (
    <div className="axiom-lesson-container">
      {/* Lesson Selector Bar */}
      <div className="axiom-lesson-nav-bar">
        <div className="axiom-lesson-nav-header">
          <BookOpen size={18} className="axiom-accent" />
          <span>Curriculum Lessons (12 Pilot Lessons Available):</span>
        </div>
        <select
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
        <span className="axiom-tag">Course {lesson.courseId} • Pilot Lesson</span>
        <h2>{lesson.title}</h2>
        <p className="axiom-lead">{lesson.subtitle}</p>
      </div>

      {/* 1. The Challenge */}
      <section className="axiom-section axiom-card">
        <div className="axiom-section-title">
          <HelpCircle size={18} className="axiom-accent" />
          <h3>1. The Purpose Challenge</h3>
        </div>
        <p className="axiom-challenge-text">{lesson.challenge}</p>

        {/* 2. Predict before calculating */}
        <div className="axiom-interactive-box">
          <h4>2. Test Your Intuition (Make a Prediction):</h4>
          <p className="axiom-subtext">{lesson.predictionPrompt}</p>
          <div className="axiom-btn-group">
            {lesson.predictionOptions.map((opt) => (
              <button
                key={opt.value}
                className={`axiom-btn ${prediction === opt.value ? 'axiom-btn-selected' : 'axiom-btn-outline'}`}
                onClick={() => handlePredict(opt.value)}
              >
                <span>{opt.label}</span>
              </button>
            ))}
          </div>

          {showExplanation && (
            <div className="axiom-feedback-reveal">
              <CheckCircle2 size={16} className="axiom-success" />
              <span>
                Hypothesis recorded! Mathematical understanding advances when we test our intuition against rigorous proof.
              </span>
            </div>
          )}
        </div>
      </section>

      {/* 3. Formal Definition */}
      <section className="axiom-section axiom-card">
        <div className="axiom-section-title">
          <BookOpen size={18} className="axiom-accent" />
          <h3>3. Mathematical Grounding</h3>
        </div>
        <p>{lesson.formalDefinition}</p>
      </section>

      {/* 4. Worked Example */}
      <section className="axiom-section axiom-card">
        <div className="axiom-section-title">
          <Lightbulb size={18} className="axiom-accent" />
          <h3>4. Step-by-Step Worked Example</h3>
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
          <h3>5. Common Misconception Trap</h3>
        </div>
        <div className="axiom-misconception-content">
          <p className="axiom-error-quote">❌ <em>"{lesson.misconception.mistake}"</em></p>
          <p className="axiom-correct-explanation">
            <strong>The Mathematical Truth:</strong> {lesson.misconception.correction}
          </p>
        </div>
      </section>

      {/* 6. Reflection & Action */}
      <section className="axiom-section axiom-card">
        <div className="axiom-section-title">
          <Layers size={18} className="axiom-accent" />
          <h3>6. Synthesis & Practice Transition</h3>
        </div>
        <p>{lesson.reflection}</p>
        <div className="axiom-lesson-actions">
          <button
            className="axiom-btn axiom-btn-primary axiom-btn-large"
            onClick={() => onStartPractice(lesson.templateId)}
          >
            <span>Practice this Mathematical Concept</span>
            <ArrowRight size={18} />
          </button>
        </div>
      </section>
    </div>
  );
};
