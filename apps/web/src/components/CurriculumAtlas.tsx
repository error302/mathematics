import React, { useState } from 'react';
import { ArrowRight, CheckCircle2, Lock, Sparkles, BookOpen } from 'lucide-react';

interface Course {
  id: string;
  title: string;
  stage: string;
  prerequisites: string[];
  outcomes: string[];
  status: 'published' | 'planned';
}

const COURSES: Course[] = [
  {
    id: "F00",
    title: "Learning to Learn Mathematics",
    stage: "Foundation",
    prerequisites: [],
    outcomes: ["Explain an answer and identify whether a check actually supports it"],
    status: "published"
  },
  {
    id: "F01",
    title: "Number and Place Value",
    stage: "Foundation",
    prerequisites: [],
    outcomes: ["Represent and compare numbers in multiple forms"],
    status: "published"
  },
  {
    id: "F02",
    title: "Whole-Number Arithmetic",
    stage: "Foundation",
    prerequisites: ["F01"],
    outcomes: ["Solve unfamiliar multi-step arithmetic and explain regrouping"],
    status: "published"
  },
  {
    id: "F03",
    title: "Integers and Divisibility",
    stage: "Foundation",
    prerequisites: ["F02"],
    outcomes: ["Use gcd/lcm appropriately and justify elementary divisibility"],
    status: "published"
  },
  {
    id: "F04",
    title: "Fractions and Rational Numbers",
    stage: "Foundation",
    prerequisites: ["F02", "F03"],
    outcomes: ["Explain why common denominators work and transfer across representations"],
    status: "published"
  },
  {
    id: "F05",
    title: "Decimals, Ratios, and Proportions",
    stage: "Foundation",
    prerequisites: ["F04"],
    outcomes: ["Distinguish additive from multiplicative comparison and solve ratio contexts"],
    status: "published"
  },
  {
    id: "S01",
    title: "Algebra I",
    stage: "Secondary",
    prerequisites: ["F03", "F04", "F05"],
    outcomes: ["Solve and check equations, including exceptional and extraneous cases"],
    status: "planned"
  },
  {
    id: "P01",
    title: "Sets, Logic, and Mathematical Language",
    stage: "Proof Bridge",
    prerequisites: ["S01"],
    outcomes: ["Negate quantified statements and explain necessary/sufficient conditions"],
    status: "planned"
  },
  {
    id: "P02",
    title: "Proof Methods",
    stage: "Proof Bridge",
    prerequisites: ["P01"],
    outcomes: ["Produce complete proofs and diagnose invalid proof steps"],
    status: "planned"
  },
  {
    id: "U01",
    title: "Computational Linear Algebra",
    stage: "Undergraduate Core",
    prerequisites: ["S01"],
    outcomes: ["Solve a system and explain rank, nullity, and geometry"],
    status: "planned"
  },
  {
    id: "U03",
    title: "Real Analysis I",
    stage: "Undergraduate Core",
    prerequisites: ["P02"],
    outcomes: ["Construct epsilon arguments and prove a theorem using completeness"],
    status: "planned"
  },
  {
    id: "A01",
    title: "Soroban Abacus: Orientation & Place Value",
    stage: "Abacus",
    prerequisites: [],
    outcomes: ["Set and read digits and multi-digit values on the soroban abacus"],
    status: "published"
  },
  {
    id: "A02",
    title: "Direct Addition & Subtraction",
    stage: "Abacus",
    prerequisites: ["A01"],
    outcomes: ["Perform direct bead additions and subtractions without complement regrouping"],
    status: "published"
  }
];

interface AtlasProps {
  onSelectCourse: (courseId: string) => void;
}

export const CurriculumAtlas: React.FC<AtlasProps> = ({ onSelectCourse }) => {
  const [filter, setFilter] = useState<string>('all');

  const filtered = filter === 'all' ? COURSES : COURSES.filter(c => c.stage.toLowerCase().includes(filter));

  return (
    <div className="axiom-atlas">
      <div className="axiom-hero">
        <div className="axiom-hero-header">
          <div className="axiom-hero-text">
            <div className="axiom-hero-tag">
              <Sparkles size={14} />
              <span>Curriculum Atlas & Dependency Map</span>
            </div>
            <h2>A Structured Pathway from Numeracy to Proof-Based Mathematics</h2>
            <p>
              Every assessable skill in AXIOM has defined mathematical objects, domain conditions, explicit prerequisites,
              and justified assessment evidence. Fast tapping and streaks do not replace genuine understanding.
            </p>
          </div>
          <div className="axiom-hero-emblem" aria-hidden="true">
            <img src="/logo.png" alt="" className="axiom-hero-logo" />
          </div>
        </div>

        <div className="axiom-filter-chips" role="tablist">
          {['all', 'foundation', 'proof', 'undergraduate', 'abacus'].map((f) => (
            <button
              key={f}
              className={`axiom-chip ${filter === f ? 'active' : ''}`}
              onClick={() => setFilter(f)}
              role="tab"
              aria-selected={filter === f}
            >
              {f.charAt(0).toUpperCase() + f.slice(1)}
            </button>
          ))}
        </div>
      </div>

      <div className="axiom-card-grid">
        {filtered.map((course) => {
          const isPublished = course.status === 'published';
          return (
            <div
              key={course.id}
              className={`axiom-course-card ${isPublished ? 'published' : 'planned'}`}
            >
              <div className="axiom-card-header">
                <span className="axiom-course-id">{course.id}</span>
                <span className={`axiom-badge ${isPublished ? 'axiom-badge-published' : 'axiom-badge-planned'}`}>
                  {isPublished ? <CheckCircle2 size={12} /> : <Lock size={12} />}
                  <span>{isPublished ? 'Available' : 'Roadmap'}</span>
                </span>
              </div>

              <h3 className="axiom-course-title">{course.title}</h3>
              <span className="axiom-stage-label">{course.stage}</span>

              <div className="axiom-outcomes-block">
                <strong>Exit Evidence:</strong>
                <ul>
                  {course.outcomes.map((o, idx) => (
                    <li key={idx}>{o}</li>
                  ))}
                </ul>
              </div>

              {course.prerequisites.length > 0 && (
                <div className="axiom-prereqs">
                  <span>Prerequisites: </span>
                  {course.prerequisites.map((p) => (
                    <span key={p} className="axiom-prereq-tag">{p}</span>
                  ))}
                </div>
              )}

              <div className="axiom-card-footer">
                {isPublished ? (
                  <button
                    className="axiom-btn axiom-btn-primary"
                    onClick={() => onSelectCourse(course.id)}
                  >
                    <BookOpen size={16} />
                    <span>Enter Course</span>
                    <ArrowRight size={14} />
                  </button>
                ) : (
                  <span className="axiom-planned-note">Planned in future release</span>
                )}
              </div>
            </div>
          );
        })}
      </div>
    </div>
  );
};
