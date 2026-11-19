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
    cached_responses: HashMap<(Uuid, Uuid), SubmissionResponse>,
    request_hashes: HashMap<(Uuid, Uuid), String>,
    grade_results: HashMap<Uuid, GradeResult>,
    current_grades: HashMap<(Uuid, Uuid), CurrentGrade>, // (attempt_id, user_id)
    learning_events: HashMap<(Uuid, i64), LearningEvent>,
    reward_entries: HashMap<Uuid, Vec<RewardEntry>>,
    reward_keys: HashSet<(Uuid, String)>,
    mastery_projections: HashMap<(Uuid, String, String), MasteryProjection>,
    review_schedules: HashMap<(Uuid, String), ReviewSchedule>,
    notebook_documents: HashMap<(Uuid, Uuid), NotebookDocument>, // (document_id, user_id)
    notebook_revisions: HashMap<(Uuid, Uuid), NotebookRevision>, // (revision_id, user_id)
    outbox_jobs: Vec<Value>,
    evidence_verifications: HashMap<(Uuid, String), Value>,
    account_backups: HashMap<(Uuid, String), Value>,
    backup_by_client_event: HashMap<(Uuid, Uuid), String>,
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

        if req.schema_version != 1 {
            return Err(StorageError::Conflict("Unsupported submission schema.".into()));
        }
        // Principal-scoped deduplication binds the entire request, not just its ID.
        let request_value = serde_json::to_value(&req).map_err(|e| StorageError::Serialization(e.to_string()))?;
        let request_hash = axiom_math::jcs::canonical_hash(&request_value).map_err(|error| StorageError::Serialization(error.to_string()))?;
        let key = (user_id, req.submission_id);
        if let Some(cached) = store.cached_responses.get(&key) {
            if store.request_hashes.get(&key) != Some(&request_hash) {
                return Err(StorageError::Conflict("Submission ID was reused with different content.".into()));
            }
            return Ok(cached.clone());
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

        store.request_hashes.insert(key, request_hash);
        store.cached_responses.insert(key, response.clone());
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

    pub fn create_notebook(&self, user_id: Uuid, req: CreateNotebookRequest) -> Result<NotebookDocument, StorageError> {
        let mut store = self.inner.write().map_err(|e| StorageError::Lock(e.to_string()))?;
        let doc_id = req.id.unwrap_or_else(Uuid::new_v4);
        let now = Utc::now();
        let doc = NotebookDocument {
            id: doc_id,
            user_id,
            title: req.title,
            created_at: now,
            updated_at: now,
        };
        store.notebook_documents.insert((doc_id, user_id), doc.clone());

        if let Some(content) = req.initial_content {
            let rev_id = Uuid::new_v4();
            let content_hash = hex::encode(sha2::Sha256::digest(content.as_bytes()));
            let rev = NotebookRevision {
                id: rev_id,
                document_id: doc_id,
                user_id,
                parent_revision_id: None,
                content,
                content_hash,
                is_conflict: false,
                created_at: now,
            };
            store.notebook_revisions.insert((rev_id, user_id), rev);
        }
        Ok(doc)
    }

    pub fn get_notebooks(&self, user_id: Uuid) -> Result<Vec<NotebookDocument>, StorageError> {
        let store = self.inner.read().map_err(|e| StorageError::Lock(e.to_string()))?;
        let mut docs: Vec<NotebookDocument> = store
            .notebook_documents
            .values()
            .filter(|d| d.user_id == user_id)
            .cloned()
            .collect();
        docs.sort_by(|a, b| b.updated_at.cmp(&a.updated_at));
        Ok(docs)
    }

    pub fn get_notebook(&self, user_id: Uuid, doc_id: Uuid) -> Result<Option<NotebookDocument>, StorageError> {
        let store = self.inner.read().map_err(|e| StorageError::Lock(e.to_string()))?;
        Ok(store.notebook_documents.get(&(doc_id, user_id)).cloned())
    }

    pub fn create_notebook_revision(
        &self,
        user_id: Uuid,
        doc_id: Uuid,
        req: CreateRevisionRequest,
    ) -> Result<NotebookRevision, StorageError> {
        let mut store = self.inner.write().map_err(|e| StorageError::Lock(e.to_string()))?;
        let doc = store
            .notebook_documents
            .get_mut(&(doc_id, user_id))
            .ok_or(StorageError::InstanceNotFound(doc_id))?;

        let now = Utc::now();
        doc.updated_at = now;

        // Detect if parent revision already has a child for this doc -> branching conflict
        let is_conflict = if let Some(parent_id) = req.parent_revision_id {
            store.notebook_revisions.values().any(|r| {
                r.document_id == doc_id && r.user_id == user_id && r.parent_revision_id == Some(parent_id)
            })
        } else {
            store.notebook_revisions.values().any(|r| {
                r.document_id == doc_id && r.user_id == user_id && r.parent_revision_id.is_none()
            })
        };

        let rev_id = req.revision_id.unwrap_or_else(Uuid::new_v4);
        let content_hash = hex::encode(sha2::Sha256::digest(req.content.as_bytes()));
        let revision = NotebookRevision {
            id: rev_id,
            document_id: doc_id,
            user_id,
            parent_revision_id: req.parent_revision_id,
            content: req.content,
            content_hash,
            is_conflict,
            created_at: now,
        };
        store.notebook_revisions.insert((rev_id, user_id), revision.clone());
        Ok(revision)
    }

    pub fn get_notebook_revisions(&self, user_id: Uuid, doc_id: Uuid) -> Result<Vec<NotebookRevision>, StorageError> {
        let store = self.inner.read().map_err(|e| StorageError::Lock(e.to_string()))?;
        if !store.notebook_documents.contains_key(&(doc_id, user_id)) {
            return Err(StorageError::InstanceNotFound(doc_id));
        }
        let mut revs: Vec<NotebookRevision> = store
            .notebook_revisions
            .values()
            .filter(|r| r.document_id == doc_id && r.user_id == user_id)
            .cloned()
            .collect();
        revs.sort_by(|a, b| a.created_at.cmp(&b.created_at));
        Ok(revs)
    }

    pub fn get_sync_changes(&self, user_id: Uuid, after_seq: i64) -> Result<SyncChangesResponse, StorageError> {
        let store = self.inner.read().map_err(|e| StorageError::Lock(e.to_string()))?;
        let cur_seq = *store.learning_streams.get(&user_id).unwrap_or(&1);

        let mut events: Vec<LearningEvent> = store
            .learning_events
            .iter()
            .filter(|((uid, seq), _)| *uid == user_id && *seq > after_seq)
            .map(|(_, ev)| ev.clone())
            .collect();
        events.sort_by_key(|e| e.sequence);

        let mastery: Vec<MasteryProjection> = store
            .mastery_projections
            .values()
            .filter(|m| m.user_id == user_id)
            .cloned()
            .collect();

        let reviews: Vec<ReviewSchedule> = store
            .review_schedules
            .values()
            .filter(|r| r.user_id == user_id)
            .cloned()
            .collect();

        let entries = store.reward_entries.get(&user_id).cloned().unwrap_or_default();
        let total_xp: i32 = entries.iter().map(|e| e.delta).sum();
        let day_start = Utc::now().date_naive().and_hms_opt(0, 0, 0).unwrap().and_utc();
        let daily_practice_xp: i32 = entries
            .iter()
            .filter(|e| e.created_at >= day_start && e.award_key.starts_with("instance:first-success:"))
            .map(|e| e.delta)
            .sum();

        let notebooks: Vec<NotebookDocument> = store
            .notebook_documents
            .values()
            .filter(|d| d.user_id == user_id)
            .cloned()
            .collect();

        let notebook_revisions: Vec<NotebookRevision> = store
            .notebook_revisions
            .values()
            .filter(|r| r.user_id == user_id)
            .cloned()
            .collect();

        Ok(SyncChangesResponse {
            user_id,
            cursor: format!("{}:{}", user_id, cur_seq),
            events,
            mastery,
            reviews,
            reward_ledger: RewardLedgerResponse {
                user_id,
                total_xp,
                daily_practice_xp,
                entries,
            },
            notebooks,
            notebook_revisions,
        })
    }

    pub fn export_user_data(&self, user_id: Uuid) -> Result<ExportDataResponse, StorageError> {
        let store = self.inner.read().map_err(|e| StorageError::Lock(e.to_string()))?;
        let user = store.users.get(&user_id).cloned().ok_or(StorageError::UserNotFound(user_id))?;
        let instances: Vec<ExerciseInstance> = store.instances.values().filter(|i| i.user_id == user_id).cloned().collect();
        let attempts: Vec<Attempt> = store.attempts.values().filter(|a| a.user_id == user_id).cloned().collect();
        let grade_results: Vec<GradeResult> = store.grade_results.values().filter(|g| g.user_id == user_id).cloned().collect();

        let entries = store.reward_entries.get(&user_id).cloned().unwrap_or_default();
        let total_xp: i32 = entries.iter().map(|e| e.delta).sum();
        let day_start = Utc::now().date_naive().and_hms_opt(0, 0, 0).unwrap().and_utc();
        let daily_practice_xp: i32 = entries
            .iter()
            .filter(|e| e.created_at >= day_start && e.award_key.starts_with("instance:first-success:"))
            .map(|e| e.delta)
            .sum();

        let reward_ledger = RewardLedgerResponse {
            user_id,
            total_xp,
            daily_practice_xp,
            entries,
        };

        let notebooks: Vec<NotebookDocument> = store
            .notebook_documents
            .values()
            .filter(|n| n.user_id == user_id).cloned().collect();
        let notebook_revisions: Vec<NotebookRevision> = store
            .notebook_revisions
            .values()
            .filter(|r| r.user_id == user_id).cloned().collect();
        let mastery_projections: Vec<MasteryProjection> = store
            .mastery_projections
            .values()
            .filter(|m| m.user_id == user_id).cloned().collect();
        let review_schedules: Vec<ReviewSchedule> = store
            .review_schedules
            .values()
            .filter(|r| r.user_id == user_id).cloned().collect();

        Ok(ExportDataResponse {
            user_id,
            exported_at: Utc::now(),
            user,
            instances,
            attempts,
            grade_results,
            reward_ledger,
            notebooks,
            notebook_revisions,
            mastery_projections,
            review_schedules,
        })
    }

    pub fn request_account_deletion(&self, user_id: Uuid) -> Result<DeletionRequestResponse, StorageError> {
        let mut store = self.inner.write().map_err(|e| StorageError::Lock(e.to_string()))?;
        let user = store.users.get_mut(&user_id).ok_or(StorageError::UserNotFound(user_id))?;
        user.status = UserStatus::DeletionPending;
        let now = Utc::now();
        Ok(DeletionRequestResponse {
            user_id,
            status: "deletion_pending".into(),
            requested_at: now,
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

            if client_event.event_type == "offline_attempt" {
                let payload = &client_event.payload;
                let submission_id = payload
                    .get("submission_id")
                    .and_then(|v| v.as_str())
                    .and_then(|s| Uuid::parse_str(s).ok())
                    .unwrap_or(client_event.client_event_id);

                let instance_id = match payload
                    .get("instance_id")
                    .and_then(|v| v.as_str())
                    .and_then(|s| Uuid::parse_str(s).ok())
                {
                    Some(id) => id,
                    None => {
                        statuses.push(EventSyncStatus {
                            client_event_id: client_event.client_event_id,
                            status: "incompatible".into(),
                            server_event_id: None,
                            error: Some("Missing or invalid instance_id in payload".into()),
                        });
                        continue;
                    }
                };

                let instance = match store.instances.get(&(instance_id, user_id)).cloned() {
                    Some(inst) => inst,
                    None => {
                        statuses.push(EventSyncStatus {
                            client_event_id: client_event.client_event_id,
                            status: "incompatible".into(),
                            server_event_id: None,
                            error: Some(format!("Instance {} not found or unauthorized", instance_id)),
                        });
                        continue;
                    }
                };

                let answer = payload.get("answer").cloned().unwrap_or(Value::Null);
                let raw_answer_str = match &answer {
                    Value::String(s) => s.clone(),
                    other => other.to_string(),
                };

                let assistance: AssistanceRecord = payload
                    .get("assistance")
                    .and_then(|a| serde_json::from_value(a.clone()).ok())
                    .unwrap_or_default();

                let submission_bytes = serde_json::to_vec(&answer).unwrap_or_default();
                let submission_hash = hex::encode(sha2::Sha256::digest(&submission_bytes));

                let problem_artifact: Result<ProblemArtifact, _> =
                    serde_json::from_value(instance.artifact.clone());
                let problem_artifact = match problem_artifact {
                    Ok(art) => art,
                    Err(e) => {
                        statuses.push(EventSyncStatus {
                            client_event_id: client_event.client_event_id,
                            status: "incompatible".into(),
                            server_event_id: None,
                            error: Some(format!("Corrupt instance artifact: {}", e)),
                        });
                        continue;
                    }
                };

                let budget = MathBudget::default();
                let grade_outcome = check(&problem_artifact, &raw_answer_str, &budget);

                let seq = cur_seq;
                cur_seq += 1;

                let now = Utc::now();
                let attempt = Attempt {
                    id: submission_id,
                    user_id,
                    instance_id,
                    origin: AttemptOrigin::Offline,
                    client_event_id: client_event.client_event_id,
                    submission_hash,
                    answer: answer.clone(),
                    assistance,
                    received_at: now,
                };
                store.attempts.insert((attempt.id, user_id), attempt);
                store
                    .attempt_by_client_event
                    .insert((user_id, client_event.client_event_id), submission_id);

                let grade_id = Uuid::new_v4();
                let grade_result = GradeResult {
                    id: grade_id,
                    attempt_id: submission_id,
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
                        "origin": "offline_sync_rechecked",
                    }),
                    evaluated_at: now,
                };
                store.grade_results.insert(grade_id, grade_result);
                store.current_grades.insert(
                    (submission_id, user_id),
                    CurrentGrade {
                        attempt_id: submission_id,
                        user_id,
                        grade_id,
                        revision: 1,
                    },
                );

                let learning_event = LearningEvent {
                    user_id,
                    sequence: seq,
                    event_id: client_event.client_event_id,
                    event_type: "offline_attempt_rechecked".into(),
                    schema_version: 1,
                    payload: serde_json::json!({
                        "submission_id": submission_id,
                        "instance_id": instance_id,
                        "disposition": grade_outcome.disposition,
                        "score": grade_outcome.score,
                    }),
                    occurred_at: now,
                };
                store.learning_events.insert((user_id, seq), learning_event);

                if grade_outcome.score > 0 {
                    let award_key = format!("instance:first-success:{}", instance_id);
                    if !store.reward_keys.contains(&(user_id, award_key.clone())) {
                        let day_start = now.date_naive().and_hms_opt(0, 0, 0).unwrap().and_utc();
                        let user_entries = store.reward_entries.entry(user_id).or_default();
                        let today_xp: i32 = user_entries
                            .iter()
                            .filter(|e| {
                                e.created_at >= day_start
                                    && e.award_key.starts_with("instance:first-success:")
                            })
                            .map(|e| e.delta)
                            .sum();

                        let delta = if today_xp + 10 <= 100 {
                            10
                        } else {
                            (100 - today_xp).max(0)
                        };
                        if delta > 0 {
                            let reward_id = Uuid::new_v4();
                            let entry = RewardEntry {
                                id: reward_id,
                                user_id,
                                award_key: award_key.clone(),
                                source_sequence: seq,
                                policy_version: "rewards-v1".into(),
                                delta,
                                reason: "Rechecked offline practice first success".into(),
                                created_at: now,
                            };
                            user_entries.push(entry);
                            store.reward_keys.insert((user_id, award_key));
                        }
                    }
                }

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

                let success = grade_outcome.score > 0;
                if success {
                    current_proj.alpha += 1.0;
                } else {
                    current_proj.beta += 1.0;
                }
                current_proj.evidence_count += 1;
                current_proj.revision += 1;
                current_proj.updated_at = now;

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

                statuses.push(EventSyncStatus {
                    client_event_id: client_event.client_event_id,
                    status: "accepted".into(),
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

    pub fn record_evidence_verification(
        &self,
        _user_id: Uuid,
        record: Value,
    ) -> Result<(), StorageError> {
        let mut store = self.inner.write().map_err(|e| StorageError::Lock(e.to_string()))?;
        let attempt_id_str = record["attempt_id"]
            .as_str()
            .ok_or_else(|| StorageError::Serialization("missing attempt_id".into()))?;
        let attempt_id = Uuid::parse_str(attempt_id_str)
            .map_err(|e| StorageError::Serialization(e.to_string()))?;
        let checker_version = record["checker_version"]
            .as_str()
            .ok_or_else(|| StorageError::Serialization("missing checker_version".into()))?
            .to_string();

        if let Some(existing) = store.evidence_verifications.get(&(attempt_id, checker_version.clone())) {
            let existing_hash = existing["evidence_hash"].as_str().unwrap_or_default();
            let new_hash = record["evidence_hash"].as_str().unwrap_or_default();
            if existing_hash != new_hash {
                return Err(StorageError::Conflict("Conflicting evidence verification for attempt".into()));
            }
            return Ok(());
        }

        store.evidence_verifications.insert((attempt_id, checker_version), record);
        Ok(())
    }

    pub fn get_evidence_verification(
        &self,
        attempt_id: Uuid,
        checker_version: &str,
    ) -> Result<Option<Value>, StorageError> {
        let store = self.inner.read().map_err(|e| StorageError::Lock(e.to_string()))?;
        Ok(store.evidence_verifications.get(&(attempt_id, checker_version.to_string())).cloned())
    }

    pub fn save_account_backup(
        &self,
        user_id: Uuid,
        backup: Value,
    ) -> Result<(), StorageError> {
        let mut store = self.inner.write().map_err(|e| StorageError::Lock(e.to_string()))?;
        let snapshot_hash = backup["snapshot_hash"]
            .as_str()
            .ok_or_else(|| StorageError::Serialization("missing snapshot_hash".into()))?
            .to_string();
        let client_event_id_str = backup["client_event_id"]
            .as_str()
            .ok_or_else(|| StorageError::Serialization("missing client_event_id".into()))?;
        let client_event_id = Uuid::parse_str(client_event_id_str)
            .map_err(|e| StorageError::Serialization(e.to_string()))?;

        if let Some(existing_hash) = store.backup_by_client_event.get(&(user_id, client_event_id)) {
            if existing_hash != &snapshot_hash {
                return Err(StorageError::Conflict("Reused client_event_id with different backup hash".into()));
            }
            return Ok(());
        }

        store.account_backups.insert((user_id, snapshot_hash.clone()), backup);
        store.backup_by_client_event.insert((user_id, client_event_id), snapshot_hash);
        Ok(())
    }

    pub fn get_account_backups(&self, user_id: Uuid) -> Result<Vec<Value>, StorageError> {
        let store = self.inner.read().map_err(|e| StorageError::Lock(e.to_string()))?;
        let backups = store
            .account_backups
            .iter()
            .filter(|((uid, _), _)| *uid == user_id)
            .map(|(_, b)| b.clone())
            .collect();
        Ok(backups)
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

        // A changed payload cannot reuse an accepted request ID.
        let changed = SubmissionRequest {
            schema_version: 1,
            submission_id: sub_id,
            instance_id,
            answer: Value::String("999999".into()),
            assistance: AssistanceRecord::default(),
            client_context: None,
        };
        assert!(matches!(repo.record_submission_atomic(user_id, changed.clone()), Err(StorageError::Conflict(_))));
        // The same UUID from another principal must never reveal the cached grade.
        let other_user = Uuid::new_v4();
        repo.ensure_user(other_user).unwrap();
        assert!(matches!(repo.record_submission_atomic(other_user, changed), Err(StorageError::InstanceNotFound(_))));

        // Check ledger
        let ledger = repo.get_reward_ledger(user_id).unwrap();
        assert_eq!(ledger.total_xp, 10);
        assert_eq!(ledger.daily_practice_xp, 10);
    }

    #[test]
    fn offline_sync_rechecking_ignores_client_claims_and_awards_xp() {
        let repo = MemoryRepository::new();
        let user_id = Uuid::new_v4();
        repo.ensure_user(user_id).unwrap();

        let seed = "abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789";
        let manifest_hash = "0000000000000000000000000000000000000000000000000000000000000000";
        let problem = generate("arithmetic.whole.addition", "1.0.0", seed, 1, manifest_hash).unwrap();
        let target = problem.target_value.clone().unwrap();

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

        let client_event_id = Uuid::new_v4();
        let batch = SyncBatchRequest {
            events: vec![ClientSyncEvent {
                client_event_id,
                event_type: "offline_attempt".into(),
                schema_version: 1,
                payload: serde_json::json!({
                    "instance_id": instance_id,
                    "answer": target,
                    "client_claimed_score": 1000,
                    "client_claimed_mastery": 1.0,
                    "assistance": {
                        "max_hint_level": 0,
                        "solution_viewed": false
                    }
                }),
                occurred_at: Utc::now() - Duration::hours(2),
            }],
        };

        let sync_res = repo.sync_client_events(user_id, batch.clone()).unwrap();
        assert_eq!(sync_res.processed.len(), 1);
        assert_eq!(sync_res.processed[0].status, "accepted");

        // Duplicate sync returns "duplicate"
        let dup_res = repo.sync_client_events(user_id, batch).unwrap();
        assert_eq!(dup_res.processed[0].status, "duplicate");

        // Verify rewards awarded under daily cap (10 XP, not client's 1000)
        let ledger = repo.get_reward_ledger(user_id).unwrap();
        assert_eq!(ledger.total_xp, 10);
        assert_eq!(ledger.daily_practice_xp, 10);

        // Verify mastery was updated by server
        let mastery = repo.get_mastery(user_id).unwrap();
        assert_eq!(mastery.len(), 1);
        assert_eq!(mastery[0].evidence_count, 1);
        assert_eq!(mastery[0].alpha, 2.0); // 1.0 prior + 1.0 success
    }

    #[test]
    fn notebook_branching_preserves_concurrent_edits_without_data_loss() {
        let repo = MemoryRepository::new();
        let user_id = Uuid::new_v4();
        repo.ensure_user(user_id).unwrap();

        // 1. Create a notebook
        let doc = repo
            .create_notebook(
                user_id,
                CreateNotebookRequest {
                    id: None,
                    title: "Fractions & Roots Notes".into(),
                    initial_content: Some("Initial definition of fractions".into()),
                },
            )
            .unwrap();

        let initial_revs = repo.get_notebook_revisions(user_id, doc.id).unwrap();
        assert_eq!(initial_revs.len(), 1);
        let rev1_id = initial_revs[0].id;

        // 2. Device A creates Revision 2 based on Rev 1
        let rev_a = repo
            .create_notebook_revision(
                user_id,
                doc.id,
                CreateRevisionRequest {
                    revision_id: None,
                    parent_revision_id: Some(rev1_id),
                    content: "Device A edits: 1/2 = 2/4".into(),
                },
            )
            .unwrap();
        assert!(!rev_a.is_conflict);

        // 3. Device B concurrently creates Revision 2' based on the SAME Rev 1
        let rev_b = repo
            .create_notebook_revision(
                user_id,
                doc.id,
                CreateRevisionRequest {
                    revision_id: None,
                    parent_revision_id: Some(rev1_id),
                    content: "Device B edits: 1/2 is also 0.5".into(),
                },
            )
            .unwrap();
        // Conflict detected: both branches preserved!
        assert!(rev_b.is_conflict);

        let all_revs = repo.get_notebook_revisions(user_id, doc.id).unwrap();
        assert_eq!(all_revs.len(), 3); // initial + rev_a + rev_b

        // 4. Two-device sync changes reconciliation check
        let sync_changes = repo.get_sync_changes(user_id, 0).unwrap();
        assert_eq!(sync_changes.notebooks.len(), 1);
        assert_eq!(sync_changes.notebook_revisions.len(), 3);
    }

    #[test]
    fn cross_user_isolation_blocks_unauthorized_access() {
        let repo = MemoryRepository::new();
        let user_a = Uuid::new_v4();
        let user_b = Uuid::new_v4();
        repo.ensure_user(user_a).unwrap();
        repo.ensure_user(user_b).unwrap();

        let doc = repo
            .create_notebook(
                user_a,
                CreateNotebookRequest {
                    id: None,
                    title: "User A Private Notebook".into(),
                    initial_content: Some("Secret derivation".into()),
                },
            )
            .unwrap();

        // User B cannot see User A's notebook
        assert!(repo.get_notebook(user_b, doc.id).unwrap().is_none());
        assert!(repo.get_notebook_revisions(user_b, doc.id).is_err());

        // User B cannot add revisions to User A's notebook
        assert!(repo
            .create_notebook_revision(
                user_b,
                doc.id,
                CreateRevisionRequest {
                    revision_id: None,
                    parent_revision_id: None,
                    content: "Malicious insertion".into(),
                }
            )
            .is_err());
    }

    #[test]
    fn export_and_account_deletion_work() {
        let repo = MemoryRepository::new();
        let user_id = Uuid::new_v4();
        repo.ensure_user(user_id).unwrap();

        let export = repo.export_user_data(user_id).unwrap();
        assert_eq!(export.user_id, user_id);
        assert_eq!(export.user.status, UserStatus::Active);

        let del = repo.request_account_deletion(user_id).unwrap();
        assert_eq!(del.status, "deletion_pending");

        let export_after = repo.export_user_data(user_id).unwrap();
        assert_eq!(export_after.user.status, UserStatus::DeletionPending);
    }

    #[test]
    fn phase2_evidence_and_backup_durability() {
        let repo = MemoryRepository::new();
        let user_id = Uuid::new_v4();
        repo.ensure_user(user_id).unwrap();

        let attempt_id = Uuid::new_v4();
        let ev_record = serde_json::json!({
            "id": Uuid::new_v4().to_string(),
            "attempt_id": attempt_id.to_string(),
            "problem_hash": "0".repeat(64),
            "submission_hash": "1".repeat(64),
            "verified_disposition": "correct",
            "verified_score": 100,
            "checker_version": "integer-exact-v2",
            "policy_version": "1.1.0",
            "evidence_hash": "a".repeat(64),
            "raw_evidence": {"steps": ["exact-check"]}
        });

        // Record verification
        repo.record_evidence_verification(user_id, ev_record.clone()).unwrap();
        let retrieved = repo.get_evidence_verification(attempt_id, "integer-exact-v2").unwrap();
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap()["verified_disposition"], "correct");

        // Idempotent duplicate verification passes
        repo.record_evidence_verification(user_id, ev_record).unwrap();

        // Conflicting verification fails
        let conflict_record = serde_json::json!({
            "id": Uuid::new_v4().to_string(),
            "attempt_id": attempt_id.to_string(),
            "problem_hash": "0".repeat(64),
            "submission_hash": "1".repeat(64),
            "verified_disposition": "correct",
            "verified_score": 100,
            "checker_version": "integer-exact-v2",
            "policy_version": "1.1.0",
            "evidence_hash": "b".repeat(64),
            "raw_evidence": {"steps": ["conflict"]}
        });
        assert!(repo.record_evidence_verification(user_id, conflict_record).is_err());

        // Account backup durability
        let client_event_id = Uuid::new_v4();
        let backup = serde_json::json!({
            "id": Uuid::new_v4().to_string(),
            "user_id": user_id.to_string(),
            "snapshot_hash": "c".repeat(64),
            "client_event_id": client_event_id.to_string(),
            "entry_count": 12,
            "payload": {"history": []}
        });
        repo.save_account_backup(user_id, backup.clone()).unwrap();
        let backups = repo.get_account_backups(user_id).unwrap();
        assert_eq!(backups.len(), 1);

        // Idempotent duplicate backup
        repo.save_account_backup(user_id, backup).unwrap();

        // Conflicting backup with reused client_event_id fails
        let conflict_backup = serde_json::json!({
            "id": Uuid::new_v4().to_string(),
            "user_id": user_id.to_string(),
            "snapshot_hash": "d".repeat(64),
            "client_event_id": client_event_id.to_string(),
            "entry_count": 15,
            "payload": {"history": ["tampered"]}
        });
        assert!(repo.save_account_backup(user_id, conflict_backup).is_err());
    }
}
