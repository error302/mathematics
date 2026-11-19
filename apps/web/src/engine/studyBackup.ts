import {computeAbacusValue,getAbacusTutorLessons,hashBackupRecords,STUDY_KEY,validateProblemBackup,validateStudyBackup} from './axiomCore';
const lessonFiles=import.meta.glob<{id:string}>('../../../../content/lessons/*/*/lesson.json',{eager:true,import:'default'});
const lessonIds=new Set(Object.values(lessonFiles).map(l=>l.id));
const stages=new Set(['number','add','subtract','five','ten','mental']);
const SAFETY_KEY='axiom_study_restore_safety_v1';
export interface StudyBackup {app:'axiom-academy';export_version:2;export_timestamp:string;records:Record<string,string>;checksum:string}
export function isStudyKey(key:string){return key==='axiom_reward_ledger'||key===STUDY_KEY||key==='axiom_abacus_tutor_stage_v1'||key.startsWith('axiom_lesson_reasoning:')&&lessonIds.has(key.slice(23))||/^axiom_practice_draft_v2:[a-z0-9_.]+$/.test(key)||/^axiom_challenge_draft_v1:[a-z0-9_.]+:[123]$/.test(key)||/^axiom_abacus_tutor_v1:(number|add|subtract|five|ten|mental)$/.test(key)}
const text=(s:unknown,max:number)=>typeof s==='string'&&s.length<=max;
function ledger(value:unknown){if(!Array.isArray(value)||value.length>10000||value.some(e=>!e||!text(e.award_key,512)||!text(e.reason,2000)||!text(e.timestamp_utc,64)||!Number.isSafeInteger(e.delta)||Math.abs(e.delta)>1000)||new Set(value.map(e=>e.award_key)).size!==value.length)throw Error('Rewards history is not valid');return value}
function validateRecord(key:string,raw:string){
 if(!isStudyKey(key)||!text(raw,150000))throw Error('This file contains an unsupported or oversized study record');
 if(key.startsWith('axiom_lesson_reasoning:')){if(raw.length>16000)throw Error('A lesson note exceeds the supported length');return}
 if(key==='axiom_abacus_tutor_stage_v1'){if(!stages.has(raw))throw Error('Unknown abacus stage');return}
 const v=JSON.parse(raw);
 if(key==='axiom_reward_ledger'){ledger(v);return}
 if(key===STUDY_KEY){validateStudyBackup(v);return}
 if(!v||!v.problem||!text(v.answer,8192)||typeof v.assisted!=='boolean')throw Error('An unfinished exercise is not valid');
 validateProblemBackup(v.problem);
 const id=v.problem.replay_identity.template_id;
 if(v.problem.template_id!==id)throw Error('Exercise alias and replay do not agree');
 if(key.startsWith('axiom_practice_draft_v2:')){if(key!==`axiom_practice_draft_v2:${id}`||!text(v.option,64))throw Error('Practice draft does not match its family')}
 else if(key.startsWith('axiom_challenge_draft_v1:')){if(key!==`axiom_challenge_draft_v1:${id}:${v.problem.replay_identity.difficulty_band}`||!text(v.note,8000)||!Array.isArray(v.grid)||v.grid.length!==(v.problem.answer_kind==='number_grid'?v.problem.metadata.blanks.length:0)||v.grid.some((x:unknown)=>!text(x,16)))throw Error('Puzzle draft does not match its challenge')}
 else{const stage=key.slice('axiom_abacus_tutor_v1:'.length);if(getAbacusTutorLessons().find(l=>l.id===stage)?.template_id!==id||!v.board||v.board.rod_count!==v.problem.metadata.initial_state.rod_count||!['full','outline','hidden'].includes(v.support)||!['watch','practice'].includes(v.phase)||!Number.isSafeInteger(v.demoIndex)||v.demoIndex<0||v.demoIndex>=getAbacusTutorLessons().find(l=>l.id===stage)!.trace.length)throw Error('Abacus draft is not valid');computeAbacusValue(v.board)}
}
export function createStudyBackup():StudyBackup{const records:Record<string,string>=Object.create(null);for(let i=0;i<localStorage.length;i++){const key=localStorage.key(i)!;if(isStudyKey(key)){const raw=localStorage.getItem(key)!;validateRecord(key,raw);records[key]=raw}}return {app:'axiom-academy',export_version:2,export_timestamp:new Date().toISOString(),records,checksum:hashBackupRecords(records)}}
export function parseStudyBackup(raw:string):StudyBackup{if(new TextEncoder().encode(raw).length>2*1024*1024)throw Error('Choose a study backup smaller than 2 MB');const v=JSON.parse(raw);if(v?.app!=='axiom-academy'||v.export_version!==2||!v.records||typeof v.records!=='object'||Array.isArray(v.records)||Object.keys(v.records).length>100||!text(v.export_timestamp,64)||typeof v.checksum!=='string')throw Error('Choose an AXIOM full-study backup, version 2');for(const [key,value]of Object.entries(v.records)){if(typeof value!=='string')throw Error('Study records must be saved text');validateRecord(key,value)}if(hashBackupRecords(v.records)!==v.checksum)throw Error('The backup checksum does not match; the file may be incomplete or changed');return v}
export function describeStudyBackup(b:StudyBackup){const keys=Object.keys(b.records);return {notes:keys.filter(k=>k.startsWith('axiom_lesson_reasoning:')).length,exercises:keys.filter(k=>k.includes('_draft_')||k.startsWith('axiom_abacus_tutor_v1:')).length,journey:keys.includes(STUDY_KEY),conflicts:keys.filter(k=>k!=='axiom_reward_ledger'&&localStorage.getItem(k)!==null&&localStorage.getItem(k)!==b.records[k]).length,rewards:b.records.axiom_reward_ledger?ledger(JSON.parse(b.records.axiom_reward_ledger)).length:0}}
export function restoreStudyBackup(b:StudyBackup,replace:boolean){
 // Revalidate at commit time. A file preview never grants mathematical authority.
 parseStudyBackup(JSON.stringify(b));
 const writes:Record<string,string>=Object.create(null);for(const [key,value]of Object.entries(b.records)){if(key==='axiom_reward_ledger'){const current=ledger(JSON.parse(localStorage.getItem(key)||'[]'));const incoming=ledger(JSON.parse(value));const byKey=new Map(current.map(e=>[e.award_key,e]));for(const entry of incoming)if(!byKey.has(entry.award_key))byKey.set(entry.award_key,entry);const merged=[...byKey.values()];ledger(merged);writes[key]=JSON.stringify(merged)}else if(replace||localStorage.getItem(key)===null){writes[key]=value}}
 const before:Record<string,string|null>=Object.create(null);for(const key of Object.keys(writes))before[key]=localStorage.getItem(key);
 // Persist recovery data before touching the selected records. Never clear unrelated storage.
 localStorage.setItem(SAFETY_KEY,JSON.stringify({version:1,created_at:new Date().toISOString(),records:before}));
 try{for(const [key,value]of Object.entries(writes))localStorage.setItem(key,value)}catch(e){let restored=true;for(const [key,value]of Object.entries(before)){try{if(value===null)localStorage.removeItem(key);else localStorage.setItem(key,value)}catch{restored=false}}throw Error(restored?'This device could not complete the restore. Previous study records were restored.':'The restore and its rollback could not finish. Recovery data remains on this device; keep your backup file and this page open.')}
 window.dispatchEvent(new Event('axiom-study-change'));window.dispatchEvent(new Event('axiom-backup-restored'));return Object.keys(writes).length
}
