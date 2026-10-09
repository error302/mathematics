//! Repository and transactional store implementing AXIOM Section 21 & 22 invariants.

use axiom_math::{check, MathBudget, ProblemArtifact};
use axiom_types::*;
use chrono::{DateTime, Duration, Utc};
use serde_json::Value;
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, RwLock};
use uuid::Uuid;

#[derive(Debug, thiserror::Error)]
pub enum StorageError {
    #[error("User not found: {0}")]
    UserNotFound(Uuid),
    #[error("Instance not found or unauthorized: {0}")]
    InstanceNotFound(Uuid),
    #[error("Conflict: duplicate submission or idempotency error: {0}")]
    Conflict(String),
    #[error("Serialization failure: {0}")]
    Serialization(String),
    #[error("Lock acquisition error: {0}")]
    Lock(String),
}

#[derive(Default)]
struct InnerStore {
    users: HashMap<Uuid, User>,
    learning_streams: HashMap<Uuid, i64>,
    instances: HashMap<(Uuid, Uuid), ExerciseInstance>, // (id, user_id)
    attempts: HashMap<(Uuid, Uuid), Attempt>,           // (id, user_id)
    attempt_by_client_event: HashMap<(Uuid, Uuid), Uuid>, // (user_id, client_event_id) -> attempt_id
    cached_responses: HashMap<Uuid, SubmissionResponse>,
    grade_results: HashMap<Uuid, GradeResult>,
    current_grades: HashMap<(Uuid, Uuid), CurrentGrade>, // (attempt_id, user_id)
    learning_events: HashMap<(Uuid, i64), LearningEvent>,
    reward_entries: HashMap<Uuid, Vec<RewardEntry>>,
    reward_keys: HashSet<(Uuid, String)>,
    mastery_projections: HashMap<(Uuid, String, String), MasteryProjection>,
    review_schedules: HashMap<(Uuid, String), ReviewSchedule>,
    outbox_jobs: Vec<Value>,
}

#[derive(Clone, Default)]
pub struct MemoryRepository {
    inner: Arc<RwLock<InnerStore>>,
}

impl MemoryRepository {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(RwLock::new(InnerStore::default())),
        }
    }

    pub fn ensure_user(&self, user_id: Uuid) -> Result<User, StorageError> {
        let mut store = self.inner.write().map_err(|e| StorageError::Lock(e.to_string()))?;
        if let Some(user) = store.users.get(&user_id) {
            return Ok(user.clone());
        }

        let user = User {
            id: user_id,
            status: UserStatus::Active,
            locale: "en".into(),
            timezone: "UTC".into(),
            created_at: Utc::now(),
        };

        store.users.insert(user_id, user.clone());
        store.learning_streams.insert(user_id, 1);
        Ok(user)
    }

    pub fn save_instance(&self, instance: ExerciseInstance) -> Result<(), StorageError> {
        let mut store = self.inner.write().map_err(|e| StorageError::Lock(e.to_string()))?;
        store.instances.insert((instance.id, instance.user_id), instance);
        Ok(())
    }

    pub fn get_instance(&self, id: Uuid, user_id: Uuid) -> Result<Option<ExerciseInstance>, StorageError> {
        let store = self.inner.read().map_err(|e| StorageError::Lock(e.to_string()))?;
        Ok(store.instances.get(&(id, user_id)).cloned())
    }

    pub fn record_submission_atomic(
        &self,
        user_id: Uuid,
        req: SubmissionRequest,
    ) -> Result<SubmissionResponse, StorageError> {
        let mut store = self.inner.write().map_err(|e| StorageError::Lock(e.to_string()))?;

        // 1. Idempotency check: if already processed, return cached response
        if let Some(cached) = store.cached_responses.get(&req.submission_id) {
            return Ok(cached.clone());
        }

        if let Some(&existing_attempt_id) = store.attempt_by_client_event.get(&(user_id, req.submission_id)) {
            if let Some(cached) = store.cached_responses.get(&existing_attempt_id) {
                return Ok(cached.clone());
            }
        }

        // 2. Lookup owner-bound exercise instance
        let instance = store
            .instances
            .get(&(req.instance_id, user_id))
            .cloned()
            .ok_or(StorageError::InstanceNotFound(req.instance_id))?;

        // 3. Mathematical grading via axiom-math pure checker
        let problem_artifact: ProblemArtifact = serde_json::from_value(instance.artifact.clone())
            .map_err(|e| StorageError::Serialization(e.to_string()))?;

        let raw_answer_str = match &req.answer {
            Value::String(s) => s.clone(),
            other => other.to_string(),
        };

        let budget = MathBudget::default();
        let grade_outcome = check(&problem_artifact, &raw_answer_str, &budget);

        // 4. Lock learning stream sequence
        let next_seq = store.learning_streams.entry(user_id).or_insert(1);
        let seq = *next_seq;
        *next_seq += 1;

        // 5. Calculate submission hash
        let submission_bytes = serde_json::to_vec(&req.answer)
            .map_err(|e| StorageError::Serialization(e.to_string()))?;
        let sub_hash = sha2::Sha256::digest(&submission_bytes);
        let submission_hash = hex::encode(sub_hash);

        // 6. Record Attempt
        let now = Utc::now();
        let attempt = Attempt {
            id: req.submission_id,
            user_id,
            instance_id: req.instance_id,
            origin: AttemptOrigin::Online,
            client_event_id: req.submission_id,
            submission_hash,
            answer: req.answer.clone(),
            assistance: req.assistance.clone(),
            received_at: now,
        };
        store.attempts.insert((attempt.id, user_id), attempt);
        store.attempt_by_client_event.insert((user_id, req.submission_id), req.submission_id);

        // 7. Record Grade Result & Current Grade
        let grade_id = Uuid::new_v4();
        let grade_result = GradeResult {
            id: grade_id,
            attempt_id: req.submission_id,
            user_id,
            ordinal: 1,
            disposition: format!("{:?}", grade_outcome.disposition).to_lowercase(),
            checker_version: grade_outcome.checker_version.clone(),
            policy_version: "1.0.0".into(),
            evidence: serde_json::json!({
                "score": grade_outcome.score,
                "max_score": grade_outcome.max_score,
                "reason_code": grade_outcome.reason_code,
                "feedback": grade_outcome.feedback,
            }),
            evaluated_at: now,
        };
        store.grade_results.insert(grade_id, grade_result);
        store.current_grades.insert(
            (req.submission_id, user_id),
            CurrentGrade {
                attempt_id: req.submission_id,
                user_id,
                grade_id,
                revision: 1,
            },
        );

        // 8. Record Learning Event
        let event_id = Uuid::new_v4();
        let learning_event = LearningEvent {
            user_id,
            sequence: seq,
            event_id,
            event_type: "attempt_finalized".into(),
            schema_version: 1,
            payload: serde_json::json!({
                "submission_id": req.submission_id,
                "instance_id": req.instance_id,
                "disposition": grade_outcome.disposition,
                "score": grade_outcome.score,
            }),
            occurred_at: now,
        };
        store.learning_events.insert((user_id, seq), learning_event);

        // 9. Reward Ledger Entry (Section 17: Unique keys, append-only, 100 XP/day practice cap)
        let mut awarded_entries = Vec::new();
        if grade_outcome.score > 0 {
            let award_key = format!("instance:first-success:{}", req.instance_id);
            if !store.reward_keys.contains(&(user_id, award_key.clone())) {
                // Compute today's practice XP total for user
                let day_start = now.date_naive().and_hms_opt(0, 0, 0).unwrap().and_utc();
                let user_entries = store.reward_entries.entry(user_id).or_default();
                let today_xp: i32 = user_entries
                    .iter()
                    .filter(|e| e.created_at >= day_start && e.award_key.starts_with("instance:first-success:"))
                    .map(|e| e.delta)
                    .sum();

                let delta = if today_xp + 10 <= 100 { 10 } else { (100 - today_xp).max(0) };

                if delta > 0 {
                    let reward_id = Uuid::new_v4();
                    let entry = RewardEntry {
                        id: reward_id,
                        user_id,
                        award_key: award_key.clone(),
                        source_sequence: seq,
                        policy_version: "rewards-v1".into(),
                        delta,
                        reason: "First successful solution for practice exercise".into(),
                        created_at: now,
                    };
                    user_entries.push(entry);
                    store.reward_keys.insert((user_id, award_key.clone()));
                    awarded_entries.push(RewardEntryDto {
                        award_key,
                        delta,
                    });
                }
            }
        }

        // 10. Update Mastery Projection & Review Schedule
        let concept_id = instance.template_id.clone();
        let current_proj = store
            .mastery_projections
            .entry((user_id, concept_id.clone(), "retention".into()))
            .or_insert_with(|| MasteryProjection {
                user_id,
                concept_id: concept_id.clone(),
                dimension: "retention".into(),
                alpha: 1.0,
                beta: 1.0,
                evidence_count: 0,
                state: "provisional".into(),
                revision: 1,
                updated_at: now,
            });

        // Update beta distribution
        let success = grade_outcome.score > 0;
        if success {
            current_proj.alpha += 1.0;
        } else {
            current_proj.beta += 1.0;
        }
        current_proj.evidence_count += 1;
        current_proj.revision += 1;
        current_proj.updated_at = now;
        let mastery_rev = current_proj.revision;

        // Schedule next review
        let step = if success { 1 } else { 0 };
        let due_at = now + Duration::days(if success { 2 } else { 1 });
        store.review_schedules.insert(
            (user_id, concept_id),
            ReviewSchedule {
                user_id,
                concept_id: instance.template_id,
                step,
                due_at,
                updated_at: now,
            },
        );

        // 11. Transactional Outbox Job
        store.outbox_jobs.push(serde_json::json!({
            "id": Uuid::new_v4(),
            "type": "event_sync",
            "user_id": user_id,
            "sequence": seq,
        }));

        let response = SubmissionResponse {
            submission_id: req.submission_id,
            state: "finalized".into(),
            grade: GradeOutcomeDto {
                disposition: format!("{:?}", grade_outcome.disposition).to_lowercase(),
                checker_version: grade_outcome.checker_version,
                evidence_level: grade_outcome.evidence_level,
                reason_code: grade_outcome.reason_code,
                feedback: grade_outcome.feedback,
            },
            mastery_revision: mastery_rev,
            reward_entries: awarded_entries,
            sync_cursor: format!("{}:{}", user_id, seq),
        };

        store.cached_responses.insert(req.submission_id, response.clone());
        Ok(response)
    }

    pub fn get_mastery(&self, user_id: Uuid) -> Result<Vec<MasteryProjection>, StorageError> {
        let store = self.inner.read().map_err(|e| StorageError::Lock(e.to_string()))?;
        let list: Vec<MasteryProjection> = store
            .mastery_projections
            .values()
            .filter(|m| m.user_id == user_id)
            .cloned()
            .collect();
        Ok(list)
    }

    pub fn get_due_reviews(&self, user_id: Uuid, now: DateTime<Utc>) -> Result<Vec<ReviewSchedule>, StorageError> {
        let store = self.inner.read().map_err(|e| StorageError::Lock(e.to_string()))?;
        let list: Vec<ReviewSchedule> = store
            .review_schedules
            .values()
            .filter(|r| r.user_id == user_id && r.due_at <= now)
            .cloned()
            .collect();
        Ok(list)
    }

    pub fn get_reward_ledger(&self, user_id: Uuid) -> Result<RewardLedgerResponse, StorageError> {
        let store = self.inner.read().map_err(|e| StorageError::Lock(e.to_string()))?;
        let entries = store.reward_entries.get(&user_id).cloned().unwrap_or_default();
        let total_xp: i32 = entries.iter().map(|e| e.delta).sum();
        let day_start = Utc::now().date_naive().and_hms_opt(0, 0, 0).unwrap().and_utc();
        let daily_practice_xp: i32 = entries
            .iter()
            .filter(|e| e.created_at >= day_start && e.award_key.starts_with("instance:first-success:"))
            .map(|e| e.delta)
            .sum();

        Ok(RewardLedgerResponse {
            user_id,
            total_xp,
            daily_practice_xp,
            entries,
        })
    }

    pub fn sync_client_events(&self, user_id: Uuid, batch: SyncBatchRequest) -> Result<SyncBatchResponse, StorageError> {
        let mut store = self.inner.write().map_err(|e| StorageError::Lock(e.to_string()))?;
        let mut statuses = Vec::new();

        let mut cur_seq = *store.learning_streams.get(&user_id).unwrap_or(&1);

        for client_event in batch.events {
            // Deduplicate by client_event_id
            let event_exists = store.learning_events.values().any(|e| e.user_id == user_id && e.event_id == client_event.client_event_id);
            if event_exists {
                statuses.push(EventSyncStatus {
                    client_event_id: client_event.client_event_id,
                    status: "duplicate".into(),
                    server_event_id: Some(client_event.client_event_id),
                    error: None,
                });
                continue;
            }

            let seq = cur_seq;
            cur_seq += 1;

            let server_event_id = client_event.client_event_id;
            let learning_event = LearningEvent {
                user_id,
                sequence: seq,
                event_id: server_event_id,
                event_type: client_event.event_type,
                schema_version: client_event.schema_version as i32,
                payload: client_event.payload,
                occurred_at: client_event.occurred_at,
            };

            store.learning_events.insert((user_id, seq), learning_event);
            statuses.push(EventSyncStatus {
                client_event_id: client_event.client_event_id,
                status: "accepted".into(),
                server_event_id: Some(server_event_id),
                error: None,
            });
        }

        store.learning_streams.insert(user_id, cur_seq);
        Ok(SyncBatchResponse {
            processed: statuses,
            sync_cursor: format!("{}:{}", user_id, cur_seq),
        })
    }
}

use sha2::Digest;

#[cfg(test)]
mod tests {
    use super::*;
    use axiom_math::generate;

    #[test]
    fn record_submission_idempotency_and_rewards() {
        let repo = MemoryRepository::new();
        let user_id = Uuid::new_v4();
        repo.ensure_user(user_id).unwrap();

        let seed = "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef";
        let manifest_hash = "0000000000000000000000000000000000000000000000000000000000000000";
        let problem = generate("arithmetic.whole.addition", "1.0.0", seed, 1, manifest_hash).unwrap();

        let instance_id = Uuid::new_v4();
        let instance = ExerciseInstance {
            id: instance_id,
            user_id,
            template_id: "arithmetic.whole.addition".into(),
            template_version: "1.0.0".into(),
            generator_version: "1.0.0".into(),
            checker_version: "integer-exact-v1".into(),
            manifest_hash: manifest_hash.into(),
            seed_hex: seed.into(),
            context: ExerciseContext::Practice,
            problem_hash: problem.problem_hash.clone(),
            artifact: serde_json::to_value(&problem).unwrap(),
            created_at: Utc::now(),
        };
        repo.save_instance(instance).unwrap();

        let sub_id = Uuid::new_v4();
        let target = problem.target_value.clone().unwrap();
        let req = SubmissionRequest {
            schema_version: 1,
            submission_id: sub_id,
            instance_id,
            answer: Value::String(target),
            assistance: AssistanceRecord::default(),
            client_context: None,
        };

        // First submission
        let res1 = repo.record_submission_atomic(user_id, req.clone()).unwrap();
        assert_eq!(res1.grade.disposition, "correct");
        assert_eq!(res1.reward_entries.len(), 1);
        assert_eq!(res1.reward_entries[0].delta, 10);

        // Second submission with identical submission_id (Idempotency retry)
        let res2 = repo.record_submission_atomic(user_id, req).unwrap();
        assert_eq!(res2.grade.disposition, "correct");
        assert_eq!(res2.submission_id, sub_id);

        // Check ledger
        let ledger = repo.get_reward_ledger(user_id).unwrap();
        assert_eq!(ledger.total_xp, 10);
        assert_eq!(ledger.daily_practice_xp, 10);
    }
}
