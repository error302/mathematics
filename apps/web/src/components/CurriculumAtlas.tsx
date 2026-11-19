import React, { useState } from 'react';
import { ArrowRight, BookOpen, Search, LayoutGrid, GitBranch } from 'lucide-react';
import { SkillDagView } from './SkillDagView';
import { CourseData } from './dagLayout';

interface Course {
  id: string; title: string; stage: string; prerequisites: string[]; outcomes: string[];
  status: 'draft' | 'planned'; prerequisite_summary: string; lesson_count: number;
  scope: string; review_status: string;
}
const catalog = import.meta.glob<Course[]>('../../../../content/catalog/courses.json', { eager: true, import: 'default' });
const COURSES = Object.values(catalog)[0];
const FILTERS = [{id:'available',label:'Available lessons'}, {id:'all',label:'Full roadmap'}, ...['foundation','secondary','proof','undergraduate','graduate','research','abacus'].map(id => ({id,label:id.charAt(0).toUpperCase()+id.slice(1)}))];

export const CurriculumAtlas: React.FC<{onSelectCourse:(id:string)=>void; onStartPractice:(template:string)=>void; onOpenAbacus:()=>void}> = ({onSelectCourse,onStartPractice,onOpenAbacus}) => {
  const [viewMode, setViewMode] = useState<'cards' | 'dag'>('cards');
  const [filter,setFilter]=useState('available');
  const [search,setSearch]=useState('');
  const filtered=COURSES.filter(course => (filter==='available' ? course.status==='draft' : filter==='all' || course.stage.includes(filter)) && `${course.id} ${course.title}`.toLowerCase().includes(search.trim().toLowerCase()));
  const drafts=filtered.filter(course=>course.status==='draft');
  const planned=filtered.filter(course=>course.status==='planned');
  return <div className="axiom-atlas">
    <section className="axiom-study-intro" aria-labelledby="academy-welcome">
      <div className="axiom-intro-copy">
        <p className="axiom-intro-context">For curious and ambitious minds</p>
        <h2 id="academy-welcome">Mathematics, the language of the universe.</h2>
        <p>Discover patterns, ask better questions, and learn to think like a mathematician. Build toward mastery through understanding, practice, and proof—one idea at a time.</p>
        <button className="axiom-btn axiom-btn-primary" onClick={()=>onSelectCourse('F01')}>Begin with place value <ArrowRight size={16} aria-hidden="true"/></button>
        <p className="axiom-draft-note">{COURSES.filter(c => c.status === 'draft').reduce((acc, c) => acc + c.lesson_count, 0)} draft lessons to explore. Independent course review is still in progress.</p>
      </div>
      <figure className="axiom-fraction-study">
        <figcaption>One idea, two ways to see it</figcaption>
        <svg viewBox="0 0 330 152" role="img" aria-label="One half is the same amount as two quarters. Each strip represents one whole.">
          <rect x="12" y="16" width="240" height="36" rx="3" fill="#29394b"/>
          <rect x="12" y="16" width="120" height="36" rx="3" fill="#e6c580"/>
          <path d="M132 16v36" stroke="#0d1521" strokeWidth="3"/>
          <text x="270" y="41" fill="#edf1f5" fontSize="20">½</text>
          <rect x="12" y="90" width="240" height="36" rx="3" fill="#29394b"/>
          <rect x="12" y="90" width="120" height="36" rx="3" fill="#e6c580"/>
          <path d="M72 90v36m60-36v36m60-36v36" stroke="#0d1521" strokeWidth="3"/>
          <text x="270" y="115" fill="#edf1f5" fontSize="20">²⁄₄</text>
        </svg>
        <p>Different parts. The same amount.</p>
        <button className="axiom-text-link" onClick={()=>onSelectCourse('F04')}>Explore fractions <ArrowRight size={14} aria-hidden="true"/></button>
      </figure>
    </section>
    <section className="axiom-brain-gym" aria-labelledby="brain-gym-heading">
      <div className="axiom-gym-intro"><h2 id="brain-gym-heading">Your mathematical brain gym</h2><p>Choose a challenge for today. Practise with numbers, explain an idea, or explore the abacus. Take your time; understanding comes first.</p></div>
      <div className="axiom-gym-activities">
        <div><h3>Work with numbers</h3><p>Try an arithmetic problem, check your answer, and learn from the feedback.</p><button className="axiom-text-link" onClick={()=>onStartPractice('arithmetic.whole.addition')}>Start a number workout <ArrowRight size={15} aria-hidden="true"/></button></div>
        <div><h3>Think it through</h3><p>Explore fractions and write an explanation in your own words.</p><button className="axiom-text-link" onClick={()=>onSelectCourse('F04')}>Explore a reasoning challenge <ArrowRight size={15} aria-hidden="true"/></button></div>
        <div><h3>See numbers differently</h3><p>Move beads on the soroban and connect their positions to place value.</p><button className="axiom-text-link" onClick={onOpenAbacus}>Open the abacus gym <ArrowRight size={15} aria-hidden="true"/></button></div>
      </div>
      <p className="axiom-draft-note">A brain gym means purposeful mathematical practice. These activities do not measure brain changes.</p>
    </section>
    <section className="axiom-curriculum-section" aria-labelledby="curriculum-heading">
      <div className="axiom-curriculum-heading">
        <div>
          <h2 id="curriculum-heading">Find your next idea</h2>
          <p>{COURSES.filter(c => c.status === 'draft').length} courses with draft lessons. An 83-course roadmap from foundations to research.</p>
        </div>
        <div style={{ display: 'flex', gap: '0.75rem', alignItems: 'center', flexWrap: 'wrap' }}>
          <div className="axiom-view-toggle" role="group" aria-label="Curriculum display mode">
            <button
              className={`axiom-chip ${viewMode==='cards'?'active':''}`}
              aria-pressed={viewMode==='cards'}
              onClick={()=>setViewMode('cards')}
            >
              <LayoutGrid size={15} aria-hidden="true"/> Course Cards
            </button>
            <button
              className={`axiom-chip ${viewMode==='dag'?'active':''}`}
              aria-pressed={viewMode==='dag'}
              onClick={()=>setViewMode('dag')}
            >
              <GitBranch size={15} aria-hidden="true"/> Skill DAG / Graph
            </button>
          </div>
          {viewMode === 'cards' && (
            <div className="axiom-curriculum-search">
              <Search size={17} aria-hidden="true"/>
              <label className="axiom-sr-only" htmlFor="curriculum-search">Search curriculum</label>
              <input id="curriculum-search" value={search} onChange={event=>setSearch(event.target.value)} placeholder="Search courses or codes" type="search"/>
            </div>
          )}
        </div>
      </div>

      {viewMode === 'dag' ? (
        <SkillDagView courses={COURSES as unknown as CourseData[]} onSelectCourse={onSelectCourse} />
      ) : (
        <>
          <div className="axiom-filter-chips" role="group" aria-label="Filter curriculum">
            {FILTERS.map(item=><button key={item.id} className={`axiom-chip ${filter===item.id?'active':''}`} aria-pressed={filter===item.id} onClick={()=>setFilter(item.id)}>{item.label}</button>)}
          </div>
          <p className="axiom-results-count" role="status">{filtered.length} {filtered.length===1?'course':'courses'}{search.trim()?` matching “${search.trim()}”`:''}</p>
          {filtered.length===0 && <div className="axiom-search-empty"><h3>No courses found</h3><p>Try a different word or view all available lessons.</p><button className="axiom-btn axiom-btn-outline" onClick={()=>{setSearch('');setFilter('available')}}>Reset search</button></div>}
          {drafts.length>0 && <div className="axiom-card-grid">{drafts.map(course=><article className="axiom-course-card" key={course.id}>
            <div className="axiom-card-header"><span className="axiom-course-id">{course.id}</span><span className="axiom-course-availability">{course.lesson_count} draft lessons</span></div>
            <h3>{course.title}</h3><p className="axiom-stage-label">{course.stage.replace(/_/g,' ')}</p>
            <p className="axiom-course-summary">{course.outcomes[0]}</p>
            <details className="axiom-course-details"><summary>What you’ll learn</summary><ul>{course.outcomes.map((outcome,index)=><li key={index}>{outcome}</li>)}</ul>{course.prerequisite_summary && <p>Start with: {course.prerequisite_summary}</p>}</details>
            <div className="axiom-card-footer"><button className="axiom-text-link" onClick={()=>onSelectCourse(course.id)}><BookOpen size={16} aria-hidden="true"/>Explore draft lessons<ArrowRight size={15} aria-hidden="true"/></button></div>
          </article>)}</div>}
          {planned.length>0 && <div className="axiom-roadmap"><div className="axiom-roadmap-heading"><h3>Further along the path</h3><p>These courses are planned. Lessons are not yet available.</p></div>{planned.map(course=><details className="axiom-roadmap-course" key={course.id}><summary><span className="axiom-course-id">{course.id}</span><span>{course.title}</span><span className="axiom-roadmap-status">Planned</span></summary><div><p className="axiom-stage-label">{course.stage.replace(/_/g,' ')}</p><ul>{course.outcomes.map((outcome,index)=><li key={index}>{outcome}</li>)}</ul><p>Start with: {course.prerequisite_summary}</p></div></details>)}</div>}
        </>
      )}
    </section>
  </div>;
};
