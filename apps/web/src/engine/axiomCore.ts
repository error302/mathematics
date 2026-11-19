/** Shared Rust mathematics; JavaScript only adapts UI and serialized values. */
import init, * as native from '../../../../packages/axiom-wasm-web/axiom_wasm';

let initialized = false;
let localManifestHash: string;
let worker: Worker;
let sequence = 0;
const pending = new Map<number, { resolve: (value: any) => void; reject: (error: Error) => void; timer: ReturnType<typeof setTimeout> }>();

export async function initializeMathCore(): Promise<void> {
  await init();
  initialized = true;
  const content = import.meta.glob('../../../../content/lessons/*/*/lesson.json', { eager: true, import: 'default' });
  localManifestHash = native.content_hash(JSON.stringify({lessons: Object.entries(content).sort(([left], [right]) => left < right ? -1 : left > right ? 1 : 0), abacus_tutor: JSON.parse(native.abacus_tutor_catalog())}));
  worker = new Worker(new URL('./math.worker.ts', import.meta.url), { type: 'module' });
  worker.onmessage = ({ data }) => {
    const request = pending.get(data.id);
    if (!request) return;
    clearTimeout(request.timer);
    pending.delete(data.id);
    if (data.error) request.reject(new Error(data.error));
    else request.resolve(data.result);
  };
  worker.onerror = () => {
    for (const request of pending.values()) {
      clearTimeout(request.timer);
      request.reject(new Error('The mathematics worker stopped. Your answer is still saved; reload to retry.'));
    }
    pending.clear();
  };
}

function callWorker<T>(operation: string, args: unknown[]): Promise<T> {
  if (!initialized) return Promise.reject(new Error('The mathematics engine is still loading.'));
  return new Promise((resolve, reject) => {
    const id = ++sequence;
    const timer = setTimeout(() => { pending.delete(id); reject(new Error('The checker could not finish in time. Your answer has not been marked incorrect.')); }, 10000);
    pending.set(id, { resolve, reject, timer });
    worker.postMessage({ id, operation, args });
  });
}

export interface RodState { upper: boolean; lower: number }
export interface AbacusState { schema_version: number; rod_count: number; rightmost_exponent: number; sign: number; rods: RodState[] }
export function createAbacus(rodCount = 5, rightmostExponent = 0): AbacusState {
  return JSON.parse(native.abacus_from_value('0', rodCount, rightmostExponent));
}
export function applyAbacusAction(state: AbacusState, action: Record<string, unknown>): AbacusState {
  return JSON.parse(native.abacus_apply_action(JSON.stringify(state), JSON.stringify(action)));
}
export function computeAbacusValue(state: AbacusState) {
  const canonical = native.abacus_value(JSON.stringify(state));
  const [numerator, denominator = '1'] = canonical.split('/');
  return { n: BigInt(numerator), d: BigInt(denominator), toCanonical: () => canonical };
}
export function describeAbacus(state: AbacusState): string { return native.abacus_describe(JSON.stringify(state)); }
export function abacusFromValue(value: string, rodCount: number): AbacusState { return JSON.parse(native.abacus_from_value(value, rodCount, 0)); }
export interface TutorFrame { state: AbacusState; caption: string; value: string; action: Record<string, unknown> | null; highlight_rod: number }
export interface TutorLesson { id: string; title: string; goal: string; intro: string; template_id: string; equation: string; trace: TutorFrame[] }
export function getAbacusTutorLessons(): TutorLesson[] { return JSON.parse(native.abacus_tutor_catalog()).lessons; }

export interface ProblemArtifact {
  template_id: string; seed_hex: string; prompt_text: string; prompt_latex: string;
  prompt_speech: string; answer_kind: string; options?: string[]; target_value?: string;
  metadata: Record<string, any>; problem_hash: string;
  replay_identity: { template_id: string; seed_hex: string; [key: string]: unknown };
}
export interface GradeOutcome {
  disposition: 'correct' | 'incorrect' | 'malformed' | 'unsupported' | 'inconclusive' | 'pending_review';
  feedback: string; reason_code: string; score: number; evidence_level: string;
}
export async function generateExercise(templateId: string, seedHex: string, difficulty = 1, manifestHash = localManifestHash): Promise<ProblemArtifact> {
  const raw = await callWorker<any>('generate', [templateId, '1.1.0', seedHex, difficulty, manifestHash]);
  return { ...raw, template_id: raw.replay_identity.template_id, seed_hex: raw.replay_identity.seed_hex };
}
let studyQueue:Promise<unknown>=Promise.resolve();
export function checkAnswer(problem: ProblemArtifact, rawAnswer: string, assisted=false): Promise<GradeOutcome> {
  const request=studyQueue.then(async()=>{
    const before=localStorage.getItem(STUDY_KEY)||'null';
    const result=await callWorker<{outcome:GradeOutcome;state:unknown;history_error:string|null}>('study-check',[before,JSON.stringify(problem),rawAnswer,assisted,BigInt(studyDay())]);
    if(result.state){
      // A lesson change or backup restore during a check must not be overwritten by its stale state.
      if((localStorage.getItem(STUDY_KEY)||'null')===before){try{localStorage.setItem(STUDY_KEY,JSON.stringify(result.state));studyNotice=''}catch{studyNotice='The answer was checked, but this device could not save study progress.'}}
    }else{studyNotice=result.history_error||'Study history could not be updated. Your answer was still checked.'}
    window.dispatchEvent(new Event('axiom-study-change'));
    return result.outcome;
  });studyQueue=request.catch(()=>{});return request;
}
export const STUDY_KEY='axiom_study_journey_v1';
let studyNotice='';
export interface StudyStep {id:string;title:string;template_id:string;goal:string;prerequisite:string|null;studied:boolean;reflection_recorded:boolean;independent_examples:number;practice_complete:boolean;repair_needed:boolean;due:boolean;next_due_day:number|null;recommended_ready:boolean}
export interface StudyView {policy_version:string;active_step:string;steps:StudyStep[];completed_practice_loops:number;recommendation:{step_id:string;action:string;reason:string}}
export function studyDay(){const d=new Date();return Math.floor(Date.UTC(d.getFullYear(),d.getMonth(),d.getDate())/86400000)}
export function getStudyCatalog():StudyStep[]{return JSON.parse(native.study_catalog())}
export function getStudyView():{view:StudyView|null;notice:string}{try{return {view:JSON.parse(native.study_snapshot(localStorage.getItem(STUDY_KEY)||'null',BigInt(studyDay()))).view,notice:studyNotice}}catch(e){return {view:null,notice:`Your study record could not be read; saved data has been kept. ${String(e)}`}}}
export function applyStudyAction(action:Record<string,unknown>){const result=JSON.parse(native.study_action(localStorage.getItem(STUDY_KEY)||'null',JSON.stringify(action),BigInt(studyDay())));localStorage.setItem(STUDY_KEY,JSON.stringify(result.state));studyNotice='';window.dispatchEvent(new Event('axiom-study-change'));return result.view as StudyView}
export function validateStudyBackup(value:unknown){native.study_snapshot(JSON.stringify(value),BigInt(studyDay()))}
export function validateProblemBackup(value:unknown){native.validate_exercise_artifact(JSON.stringify(value))}
export function hashBackupRecords(value:unknown){return native.content_hash(JSON.stringify(value))}
export interface CoachingMaterial {version:string;hint:string;steps:string[];conclusion:string;success_note:string;visual:any|null}
export function getExerciseCoaching(problem:ProblemArtifact):Promise<CoachingMaterial>{return callWorker('coach',[JSON.stringify(problem)])}
export interface RewardEntry { award_key: string; delta: number; reason: string; timestamp_utc: string }
export class ClientRewardLedger {
  entries: RewardEntry[];
  constructor(entries: RewardEntry[] = []) { this.entries = [...entries]; }
  getTotalXp(): number { return this.entries.reduce((sum, entry) => sum + entry.delta, 0); }
  getTodayPracticeXp(day: string): number {
    return this.entries.filter(entry => entry.timestamp_utc.startsWith(day) && entry.award_key.startsWith('instance:first-success:')).reduce((sum, entry) => sum + entry.delta, 0);
  }
  awardInstanceSuccess(instanceId: string, timestamp = new Date().toISOString()): RewardEntry | null {
    const result = JSON.parse(native.reward_instance(JSON.stringify(this.entries), instanceId, timestamp));
    this.entries = result.entries;
    return result.award;
  }
  awardProvisionalMilestone(conceptId: string, timestamp = new Date().toISOString()): RewardEntry | null {
    const result = JSON.parse(native.reward_provisional_milestone(JSON.stringify(this.entries), conceptId, timestamp));
    this.entries = result.entries;
    return result.award;
  }
}

export const INTERVAL_LADDER_DAYS = [1, 3, 7, 14, 30, 60, 120] as const;

export interface ReviewSchedule {
  concept_id: string;
  ladder_step: number;
  last_reviewed_epoch_days: number;
  next_due_epoch_days: number;
}

export function updateReviewSchedule(
  schedule: ReviewSchedule | null,
  success: boolean,
  currentEpochDays = studyDay()
): ReviewSchedule {
  return JSON.parse(native.review_schedule_update(schedule ? JSON.stringify(schedule) : 'null', success, BigInt(currentEpochDays)));
}

export function getTopicReviewSchedule(templateId: string): ReviewSchedule | null {
  try {
    const raw = localStorage.getItem(`axiom_review_schedule:${templateId}`);
    return raw ? JSON.parse(raw) : null;
  } catch {
    return null;
  }
}

export function saveTopicReviewSchedule(templateId: string, schedule: ReviewSchedule): void {
  localStorage.setItem(`axiom_review_schedule:${templateId}`, JSON.stringify(schedule));
  window.dispatchEvent(new Event('axiom-schedule-change'));
}

