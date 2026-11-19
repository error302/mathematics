export interface CourseData {
  id: string;
  title: string;
  stage: string;
  scope: string;
  prerequisites: string[];
  prerequisite_summary: string;
  outcomes: string[];
  status: 'draft' | 'planned';
  lesson_count: number;
  review_status: string;
}

export interface DagNode {
  id: string;
  course: CourseData;
  x: number;
  y: number;
  width: number;
  height: number;
  stage: string;
  rank: number;
  prerequisites: string[];
  dependents: string[];
}

export interface DagEdge {
  id: string;
  from: string;
  to: string;
  d: string;
}

export interface DagGraph {
  nodes: DagNode[];
  edges: DagEdge[];
  nodeMap: Map<string, DagNode>;
  bounds: { minX: number; minY: number; maxX: number; maxY: number; width: number; height: number };
}

export function parseCoursePrerequisites(course: CourseData, allCourses: CourseData[]): string[] {
  const courseIds = new Set(allCourses.map(c => c.id));
  const summary = course.prerequisite_summary || '';

  // Explicit abacus ladder progression
  if (course.id.startsWith('A')) {
    const num = parseInt(course.id.slice(1), 10);
    if (num === 1) return ['F01'];
    return ['A' + String(num - 1).padStart(2, '0')];
  }

  // Research capstones link to corresponding graduate foundation tracks
  if (course.id === 'R01') return ['G19', 'G20'];
  if (course.id === 'R02') return ['G06', 'G15', 'G16'];
  if (course.id === 'R03') return ['G07', 'G08', 'G09'];
  if (course.id === 'R04') return ['G01', 'G02', 'G04'];
  if (course.id === 'R05') return ['G17', 'G18'];
  if (course.id === 'R06') return ['G05', 'G13', 'G21'];
  if (course.id === 'R07') return ['G10', 'G11', 'G12'];
  if (course.id === 'G22') return ['G01', 'G06'];

  const found = new Set<string>();

  // Parse ranges like F03–F05 or U04–U05
  const rangeRegex = /([A-Z]\d{2})\s*[–\-—]\s*([A-Z]\d{2})/g;
  let m: RegExpExecArray | null;
  while ((m = rangeRegex.exec(summary)) !== null) {
    const prefix1 = m[1][0];
    const n1 = parseInt(m[1].slice(1), 10);
    const prefix2 = m[2][0];
    const n2 = parseInt(m[2].slice(1), 10);
    if (prefix1 === prefix2 && n1 <= n2) {
      for (let i = n1; i <= n2; i++) {
        const id = prefix1 + String(i).padStart(2, '0');
        if (courseIds.has(id)) found.add(id);
      }
    }
  }

  // Parse discrete codes like F02, S01, P02, U07
  const idRegex = /\b([FSPUGR]\d{2})\b/g;
  while ((m = idRegex.exec(summary)) !== null) {
    if (courseIds.has(m[1])) found.add(m[1]);
  }

  found.delete(course.id);
  return Array.from(found);
}

export function buildDag(courses: CourseData[], mode: 'stages' | 'topological' = 'stages'): DagGraph {
  const NODE_WIDTH = 220;
  const NODE_HEIGHT = 88;
  const COL_GAP = 90;
  const ROW_GAP = 30;

  // 1. Build adjacency & dependency maps
  const prereqMap = new Map<string, string[]>();
  const dependentMap = new Map<string, string[]>();
  courses.forEach(c => {
    prereqMap.set(c.id, parseCoursePrerequisites(c, courses));
    dependentMap.set(c.id, []);
  });
  courses.forEach(c => {
    const prereqs = prereqMap.get(c.id) || [];
    prereqs.forEach(pId => {
      dependentMap.get(pId)?.push(c.id);
    });
  });

  // 2. Compute topological ranks
  const rankMap = new Map<string, number>();
  function getRank(id: string, visited = new Set<string>()): number {
    if (rankMap.has(id)) return rankMap.get(id)!;
    if (visited.has(id)) return 0;
    visited.add(id);
    const p = prereqMap.get(id) || [];
    if (p.length === 0) {
      rankMap.set(id, 0);
      return 0;
    }
    let maxR = 0;
    for (const parent of p) {
      maxR = Math.max(maxR, getRank(parent, new Set(visited)) + 1);
    }
    rankMap.set(id, maxR);
    return maxR;
  }
  courses.forEach(c => getRank(c.id));

  // 3. Stage column ordering
  const STAGES_ORDER = ['foundation', 'abacus', 'secondary', 'proof_bridge', 'undergraduate', 'graduate', 'research'];

  // 4. Position nodes
  const nodes: DagNode[] = [];
  const nodeMap = new Map<string, DagNode>();

  if (mode === 'stages') {
    // Group courses by stage
    const stageGroups = new Map<string, CourseData[]>();
    STAGES_ORDER.forEach(s => stageGroups.set(s, []));
    courses.forEach(c => {
      const group = stageGroups.get(c.stage) || stageGroups.get('foundation')!;
      group.push(c);
    });

    // Place each column
    STAGES_ORDER.forEach((stage, colIdx) => {
      const list = stageGroups.get(stage) || [];
      // Sort within column by rank, then id
      list.sort((a, b) => (rankMap.get(a.id) || 0) - (rankMap.get(b.id) || 0) || a.id.localeCompare(b.id));

      const colX = 50 + colIdx * (NODE_WIDTH + COL_GAP);
      list.forEach((course, rowIdx) => {
        const rowY = 50 + rowIdx * (NODE_HEIGHT + ROW_GAP);
        const node: DagNode = {
          id: course.id,
          course,
          x: colX,
          y: rowY,
          width: NODE_WIDTH,
          height: NODE_HEIGHT,
          stage: course.stage,
          rank: rankMap.get(course.id) || 0,
          prerequisites: prereqMap.get(course.id) || [],
          dependents: dependentMap.get(course.id) || []
        };
        nodes.push(node);
        nodeMap.set(course.id, node);
      });
    });
  } else {
    // Topological ranking columns
    const maxRank = Math.max(...Array.from(rankMap.values()), 0);
    const rankGroups: CourseData[][] = Array.from({ length: maxRank + 1 }, () => []);
    courses.forEach(c => {
      const r = rankMap.get(c.id) || 0;
      rankGroups[r].push(c);
    });

    rankGroups.forEach((list, colIdx) => {
      list.sort((a, b) => a.id.localeCompare(b.id));
      const colX = 50 + colIdx * (NODE_WIDTH + COL_GAP);
      list.forEach((course, rowIdx) => {
        const rowY = 50 + rowIdx * (NODE_HEIGHT + ROW_GAP);
        const node: DagNode = {
          id: course.id,
          course,
          x: colX,
          y: rowY,
          width: NODE_WIDTH,
          height: NODE_HEIGHT,
          stage: course.stage,
          rank: colIdx,
          prerequisites: prereqMap.get(course.id) || [],
          dependents: dependentMap.get(course.id) || []
        };
        nodes.push(node);
        nodeMap.set(course.id, node);
      });
    });
  }

  // 5. Build edges with cubic Bezier curves
  const edges: DagEdge[] = [];
  courses.forEach(c => {
    const toNode = nodeMap.get(c.id);
    if (!toNode) return;
    const prereqs = prereqMap.get(c.id) || [];
    prereqs.forEach(fromId => {
      const fromNode = nodeMap.get(fromId);
      if (!fromNode) return;

      const x1 = fromNode.x + fromNode.width;
      const y1 = fromNode.y + fromNode.height / 2;
      const x2 = toNode.x;
      const y2 = toNode.y + toNode.height / 2;

      const dx = Math.abs(x2 - x1);
      const cx1 = x1 + Math.max(30, dx * 0.45);
      const cx2 = x2 - Math.max(30, dx * 0.45);

      const d = `M ${x1} ${y1} C ${cx1} ${y1}, ${cx2} ${y2}, ${x2} ${y2}`;
      edges.push({
        id: `${fromId}->${c.id}`,
        from: fromId,
        to: c.id,
        d
      });
    });
  });

  // 6. Compute canvas bounds
  let minX = Infinity, minY = Infinity, maxX = -Infinity, maxY = -Infinity;
  nodes.forEach(n => {
    if (n.x < minX) minX = n.x;
    if (n.y < minY) minY = n.y;
    if (n.x + n.width > maxX) maxX = n.x + n.width;
    if (n.y + n.height > maxY) maxY = n.y + n.height;
  });

  if (nodes.length === 0) {
    minX = 0; minY = 0; maxX = 800; maxY = 600;
  }

  return {
    nodes,
    edges,
    nodeMap,
    bounds: {
      minX: minX - 50,
      minY: minY - 50,
      maxX: maxX + 100,
      maxY: maxY + 100,
      width: maxX - minX + 150,
      height: maxY - minY + 150
    }
  };
}

export function getAncestors(startId: string, nodeMap: Map<string, DagNode>): Set<string> {
  const result = new Set<string>();
  const queue = [startId];
  while (queue.length > 0) {
    const curr = queue.shift()!;
    const node = nodeMap.get(curr);
    if (!node) continue;
    for (const p of node.prerequisites) {
      if (!result.has(p)) {
        result.add(p);
        queue.push(p);
      }
    }
  }
  return result;
}

export function getDescendants(startId: string, nodeMap: Map<string, DagNode>): Set<string> {
  const result = new Set<string>();
  const queue = [startId];
  while (queue.length > 0) {
    const curr = queue.shift()!;
    const node = nodeMap.get(curr);
    if (!node) continue;
    for (const d of node.dependents) {
      if (!result.has(d)) {
        result.add(d);
        queue.push(d);
      }
    }
  }
  return result;
}
