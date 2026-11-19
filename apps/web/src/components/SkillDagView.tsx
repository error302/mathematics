import React, { useMemo, useState, useRef, useCallback } from 'react';
import { Search, ZoomIn, ZoomOut, Maximize2, RotateCcw, X, ArrowRight, BookOpen, Layers, GitBranch } from 'lucide-react';
import { CourseData, DagGraph, buildDag, getAncestors, getDescendants } from './dagLayout';
import './skillDag.css';

interface SkillDagViewProps {
  courses: CourseData[];
  onSelectCourse: (id: string) => void;
  initialSelectedId?: string;
}

const STAGE_LABELS: Record<string, string> = {
  foundation: 'Foundation',
  abacus: 'Abacus',
  secondary: 'Secondary',
  proof_bridge: 'Proof Bridge',
  undergraduate: 'Undergraduate',
  graduate: 'Graduate',
  research: 'Research',
};

export const SkillDagView: React.FC<SkillDagViewProps> = ({ courses, onSelectCourse, initialSelectedId }) => {
  const [layoutMode, setLayoutMode] = useState<'stages' | 'topological'>('stages');
  const [search, setSearch] = useState('');
  const [stageFilter, setStageFilter] = useState('all');
  const [selectedId, setSelectedId] = useState<string | null>(initialSelectedId || 'F01');

  // Pan & Zoom state
  const [pan, setPan] = useState({ x: 40, y: 30 });
  const [zoom, setZoom] = useState(0.85);
  const [isDragging, setIsDragging] = useState(false);
  const dragStart = useRef({ x: 0, y: 0 });
  const svgRef = useRef<SVGSVGElement | null>(null);

  // Filter courses
  const filteredCourses = useMemo(() => {
    return courses.filter(c => {
      const matchesStage = stageFilter === 'all' || c.stage === stageFilter;
      const matchesSearch = !search.trim() || `${c.id} ${c.title} ${c.stage} ${c.scope}`.toLowerCase().includes(search.trim().toLowerCase());
      return matchesStage && matchesSearch;
    });
  }, [courses, stageFilter, search]);

  // Build DAG Graph
  const graph: DagGraph = useMemo(() => {
    return buildDag(filteredCourses, layoutMode);
  }, [filteredCourses, layoutMode]);

  // Ancestor & Descendant sets for selected course
  const { ancestors, descendants } = useMemo(() => {
    if (!selectedId || !graph.nodeMap.has(selectedId)) {
      return { ancestors: new Set<string>(), descendants: new Set<string>() };
    }
    return {
      ancestors: getAncestors(selectedId, graph.nodeMap),
      descendants: getDescendants(selectedId, graph.nodeMap),
    };
  }, [selectedId, graph.nodeMap]);

  // Fit View
  const fitView = useCallback(() => {
    if (!svgRef.current || graph.nodes.length === 0) return;
    const rect = svgRef.current.getBoundingClientRect();
    const scaleX = (rect.width - 60) / graph.bounds.width;
    const scaleY = (rect.height - 60) / graph.bounds.height;
    const newZoom = Math.max(0.2, Math.min(1.2, Math.min(scaleX, scaleY)));
    setZoom(newZoom);
    setPan({
      x: (rect.width - graph.bounds.width * newZoom) / 2 - graph.bounds.minX * newZoom,
      y: 40 - graph.bounds.minY * newZoom,
    });
  }, [graph]);

  // Reset View
  const resetView = () => {
    setZoom(0.85);
    setPan({ x: 40, y: 30 });
  };

  // Center on node
  const centerOnNode = useCallback((nodeId: string) => {
    const node = graph.nodeMap.get(nodeId);
    if (!node || !svgRef.current) return;
    const rect = svgRef.current.getBoundingClientRect();
    setPan({
      x: rect.width / 2 - (node.x + node.width / 2) * zoom,
      y: rect.height / 2 - (node.y + node.height / 2) * zoom,
    });
  }, [graph.nodeMap, zoom]);

  // Drag handlers
  const handleMouseDown = (e: React.MouseEvent) => {
    if (e.button !== 0) return;
    setIsDragging(true);
    dragStart.current = { x: e.clientX - pan.x, y: e.clientY - pan.y };
  };

  const handleMouseMove = (e: React.MouseEvent) => {
    if (!isDragging) return;
    setPan({
      x: e.clientX - dragStart.current.x,
      y: e.clientY - dragStart.current.y,
    });
  };

  const handleMouseUp = () => setIsDragging(false);

  const handleWheel = (e: React.WheelEvent) => {
    e.preventDefault();
    const zoomFactor = e.deltaY < 0 ? 1.1 : 0.9;
    setZoom(z => Math.max(0.2, Math.min(2.5, z * zoomFactor)));
  };

  const selectedNode = selectedId ? graph.nodeMap.get(selectedId) : null;

  return (
    <div className="axiom-dag-wrapper" aria-label="Curriculum Skill DAG and Prerequisite Pathways">
      {/* Toolbar */}
      <div className="axiom-dag-toolbar">
        <div className="axiom-dag-toolbar-left">
          <div className="axiom-dag-search-box">
            <Search size={15} aria-hidden="true" />
            <input
              type="search"
              placeholder="Search 83 courses..."
              value={search}
              onChange={e => setSearch(e.target.value)}
              aria-label="Search courses in DAG"
            />
            {search && <button className="axiom-text-link" onClick={() => setSearch('')}>Clear</button>}
          </div>

          <div className="axiom-dag-controls" role="group" aria-label="Stage Filters">
            <button
              className={`axiom-dag-btn ${stageFilter === 'all' ? 'active' : ''}`}
              onClick={() => setStageFilter('all')}
            >
              All Stages ({courses.length})
            </button>
            {Object.entries(STAGE_LABELS).map(([stageKey, label]) => {
              const count = courses.filter(c => c.stage === stageKey).length;
              return (
                <button
                  key={stageKey}
                  className={`axiom-dag-btn ${stageFilter === stageKey ? 'active' : ''}`}
                  onClick={() => setStageFilter(stageKey)}
                >
                  {label} ({count})
                </button>
              );
            })}
          </div>
        </div>

        <div className="axiom-dag-controls" role="group" aria-label="Graph Layout and Zoom Controls">
          <button
            className={`axiom-dag-btn ${layoutMode === 'stages' ? 'active' : ''}`}
            onClick={() => setLayoutMode('stages')}
            title="Arrange in Stage Columns"
          >
            <Layers size={14} aria-hidden="true" /> Stages
          </button>
          <button
            className={`axiom-dag-btn ${layoutMode === 'topological' ? 'active' : ''}`}
            onClick={() => setLayoutMode('topological')}
            title="Arrange by Topological Dependency Depth"
          >
            <GitBranch size={14} aria-hidden="true" /> Flow
          </button>
          <div style={{ width: 1, height: 20, background: '#22354c', margin: '0 4px' }} />
          <button className="axiom-dag-btn" onClick={() => setZoom(z => Math.min(2.5, z * 1.15))} title="Zoom In">
            <ZoomIn size={14} aria-hidden="true" />
          </button>
          <button className="axiom-dag-btn" onClick={() => setZoom(z => Math.max(0.2, z * 0.85))} title="Zoom Out">
            <ZoomOut size={14} aria-hidden="true" />
          </button>
          <button className="axiom-dag-btn" onClick={fitView} title="Fit to Screen">
            <Maximize2 size={14} aria-hidden="true" /> Fit
          </button>
          <button className="axiom-dag-btn" onClick={resetView} title="Reset View">
            <RotateCcw size={14} aria-hidden="true" />
          </button>
        </div>
      </div>

      {/* SVG Canvas Viewport */}
      <div
        className="axiom-dag-viewport"
        onMouseDown={handleMouseDown}
        onMouseMove={handleMouseMove}
        onMouseUp={handleMouseUp}
        onMouseLeave={handleMouseUp}
        onWheel={handleWheel}
      >
        <svg
          ref={svgRef}
          className={`axiom-dag-svg ${isDragging ? 'dragging' : ''}`}
          role="img"
          aria-label="Skill DAG showing courses and directed prerequisite edges"
        >
          <g transform={`translate(${pan.x}, ${pan.y}) scale(${zoom})`}>
            {/* 1. Edges */}
            <g className="axiom-dag-edges-layer">
              {graph.edges.map(edge => {
                const isUpstreamEdge = (ancestors.has(edge.from) || edge.from === selectedId) && (ancestors.has(edge.to) || edge.to === selectedId);
                const isDownstreamEdge = (descendants.has(edge.from) || edge.from === selectedId) && (descendants.has(edge.to) || edge.to === selectedId);
                const isDimmed = Boolean(selectedId && !isUpstreamEdge && !isDownstreamEdge);

                let edgeClass = 'axiom-dag-edge';
                if (isUpstreamEdge) edgeClass += ' upstream';
                else if (isDownstreamEdge) edgeClass += ' downstream';
                else if (isDimmed) edgeClass += ' dimmed';

                return (
                  <path
                    key={edge.id}
                    d={edge.d}
                    className={edgeClass}
                  />
                );
              })}
            </g>

            {/* 2. Nodes */}
            <g className="axiom-dag-nodes-layer">
              {graph.nodes.map(node => {
                const isSelected = selectedId === node.id;
                const isAncestor = ancestors.has(node.id);
                const isDescendant = descendants.has(node.id);
                const isDimmed = Boolean(selectedId && !isSelected && !isAncestor && !isDescendant);

                let nodeClass = 'axiom-dag-node-group';
                if (isSelected) nodeClass += ' selected';
                else if (isAncestor) nodeClass += ' ancestor';
                else if (isDescendant) nodeClass += ' descendant';
                else if (isDimmed) nodeClass += ' dimmed';

                const stageName = STAGE_LABELS[node.stage] || node.stage;
                const truncatedTitle = node.course.title.length > 25
                  ? node.course.title.slice(0, 24) + '…'
                  : node.course.title;

                return (
                  <g
                    key={node.id}
                    id={`dag-node-${node.id}`}
                    className={nodeClass}
                    transform={`translate(${node.x}, ${node.y})`}
                    onClick={e => {
                      e.stopPropagation();
                      setSelectedId(node.id);
                    }}
                    onKeyDown={e => {
                      if (e.key === 'Enter' || e.key === ' ') {
                        e.preventDefault();
                        setSelectedId(node.id);
                      }
                    }}
                    tabIndex={0}
                    role="button"
                    aria-label={`Course ${node.id}: ${node.course.title}, Stage ${stageName}, Status ${node.course.status}`}
                  >
                    {/* Background Card */}
                    <rect
                      className="axiom-node-bg"
                      width={node.width}
                      height={node.height}
                    />

                    {/* Stage color bar */}
                    <rect
                      className={`axiom-node-stage-bar stage-${node.stage}`}
                      x={8}
                      y={8}
                      width={4}
                      height={node.height - 16}
                    />

                    {/* Course ID */}
                    <text
                      className="axiom-node-id"
                      x={20}
                      y={26}
                    >
                      {node.id}
                    </text>

                    {/* Stage Subtitle */}
                    <text
                      className="axiom-node-stage-text"
                      x={20}
                      y={42}
                    >
                      {stageName}
                    </text>

                    {/* Course Title */}
                    <text
                      className="axiom-node-title"
                      x={20}
                      y={62}
                    >
                      {truncatedTitle}
                    </text>

                    {/* Status Indicator Dot */}
                    <circle
                      className={`axiom-node-status-dot ${node.course.status}`}
                      cx={node.width - 16}
                      cy={22}
                    />
                    <text
                      x={node.width - 26}
                      y={25}
                      textAnchor="end"
                      fill="#7b8eaf"
                      fontSize="9.5"
                    >
                      {node.course.status === 'draft' ? `${node.course.lesson_count} lessons` : 'planned'}
                    </text>
                  </g>
                );
              })}
            </g>
          </g>
        </svg>

        {/* Selected Course Inspector Drawer */}
        {selectedNode && (
          <aside className="axiom-dag-drawer" aria-labelledby="dag-drawer-title">
            <div className="axiom-dag-drawer-header">
              <div>
                <span className={`axiom-tag stage-${selectedNode.stage}`} style={{ marginBottom: 4, display: 'inline-block' }}>
                  {STAGE_LABELS[selectedNode.stage] || selectedNode.stage}
                </span>
                <h4 id="dag-drawer-title">{selectedNode.id} — {selectedNode.course.title}</h4>
              </div>
              <button
                className="axiom-text-link"
                onClick={() => setSelectedId(null)}
                aria-label="Close Course Drawer"
              >
                <X size={18} aria-hidden="true" />
              </button>
            </div>

            <div className="axiom-dag-drawer-body">
              {/* Status and Lessons */}
              <div className="axiom-dag-drawer-section">
                <h5>Status</h5>
                <p style={{ margin: 0, fontSize: '0.88rem', color: selectedNode.course.status === 'draft' ? '#34d399' : '#94a3b8' }}>
                  {selectedNode.course.status === 'draft'
                    ? `✓ ${selectedNode.course.lesson_count} draft interactive lessons available`
                    : '○ Planned in 83-course roadmap'}
                </p>
              </div>

              {/* Scope */}
              <div className="axiom-dag-drawer-section">
                <h5>Scope</h5>
                <p style={{ margin: 0, fontSize: '0.86rem', color: '#c4d1e0', lineHeight: 1.45 }}>
                  {selectedNode.course.scope}
                </p>
              </div>

              {/* Direct Prerequisites */}
              <div className="axiom-dag-drawer-section">
                <h5>Direct Prerequisites ({selectedNode.prerequisites.length})</h5>
                {selectedNode.prerequisites.length === 0 ? (
                  <p style={{ margin: 0, fontSize: '0.84rem', color: '#7b8eaf' }}>No prerequisite courses (foundational entry)</p>
                ) : (
                  <div className="axiom-dag-pill-group">
                    {selectedNode.prerequisites.map(pId => {
                      const pNode = graph.nodeMap.get(pId);
                      return (
                        <button
                          key={pId}
                          className="axiom-dag-pill upstream"
                          onClick={() => {
                            setSelectedId(pId);
                            centerOnNode(pId);
                          }}
                          title={pNode?.course.title}
                        >
                          ← {pId} {pNode ? `(${pNode.course.title})` : ''}
                        </button>
                      );
                    })}
                  </div>
                )}
              </div>

              {/* Unlocked Downstream Courses */}
              <div className="axiom-dag-drawer-section">
                <h5>Unlocks Subsequent Pathways ({selectedNode.dependents.length})</h5>
                {selectedNode.dependents.length === 0 ? (
                  <p style={{ margin: 0, fontSize: '0.84rem', color: '#7b8eaf' }}>Capstone / Terminal branch</p>
                ) : (
                  <div className="axiom-dag-pill-group">
                    {selectedNode.dependents.map(dId => {
                      const dNode = graph.nodeMap.get(dId);
                      return (
                        <button
                          key={dId}
                          className="axiom-dag-pill downstream"
                          onClick={() => {
                            setSelectedId(dId);
                            centerOnNode(dId);
                          }}
                          title={dNode?.course.title}
                        >
                          → {dId} {dNode ? `(${dNode.course.title})` : ''}
                        </button>
                      );
                    })}
                  </div>
                )}
              </div>

              {/* Key Outcomes */}
              <div className="axiom-dag-drawer-section">
                <h5>Key Learning Outcomes</h5>
                <ul style={{ margin: 0, paddingLeft: '1.2rem', fontSize: '0.84rem', color: '#cbd7e6', lineHeight: 1.45 }}>
                  {selectedNode.course.outcomes.map((outcome, idx) => (
                    <li key={idx}>{outcome}</li>
                  ))}
                </ul>
              </div>
            </div>

            <div className="axiom-dag-drawer-footer">
              {selectedNode.course.status === 'draft' ? (
                <button
                  className="axiom-btn axiom-btn-primary"
                  style={{ width: '100%', justifyContent: 'center' }}
                  onClick={() => onSelectCourse(selectedNode.id)}
                >
                  <BookOpen size={16} aria-hidden="true" /> Explore Draft Lessons <ArrowRight size={15} aria-hidden="true" />
                </button>
              ) : (
                <p style={{ margin: 0, fontSize: '0.82rem', color: '#7b8eaf', textAlign: 'center' }}>
                  Roadmap course. Draft lessons in active planning.
                </p>
              )}
            </div>
          </aside>
        )}
      </div>

      {/* Legend */}
      <div className="axiom-dag-legend">
        <div className="axiom-dag-legend-items">
          <div className="axiom-dag-legend-item">
            <span className="axiom-dag-legend-line gold" />
            <span>Prerequisites (Ancestors)</span>
          </div>
          <div className="axiom-dag-legend-item">
            <span className="axiom-dag-legend-line emerald" />
            <span>Unlocked (Descendants)</span>
          </div>
          <div className="axiom-dag-legend-item">
            <span className="axiom-dag-legend-line navy" />
            <span>Independent Pathways</span>
          </div>
        </div>
        <div>
          <span>Drag to pan · Scroll to zoom · Click any node to trace prerequisite pathways</span>
        </div>
      </div>
    </div>
  );
};
