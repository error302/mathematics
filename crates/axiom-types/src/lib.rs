//! Versioned Domain Transfer Objects and Common Types (Section 20 & 21 & 22).

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum UserStatus {
    Active,
    Disabled,
    DeletionPending,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub status: UserStatus,
    pub locale: String,
    pub timezone: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExerciseContext {
    Practice,
    Assessment,
    Review,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExerciseInstance {
    pub id: Uuid,
    pub user_id: Uuid,
    pub template_id: String,
    pub template_version: String,
    pub generator_version: String,
    pub checker_version: String,
    pub manifest_hash: String,
    pub seed_hex: String,
    pub context: ExerciseContext,
    pub problem_hash: String,
    pub artifact: Value,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AttemptOrigin {
    Online,
    Offline,
    GuestImport,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssistanceRecord {
    pub max_hint_level: u32,
    pub solution_viewed: bool,
}

impl Default for AssistanceRecord {
    fn default() -> Self {
        AssistanceRecord {
            max_hint_level: 0,
            solution_viewed: false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Attempt {
    pub id: Uuid,
    pub user_id: Uuid,
    pub instance_id: Uuid,
    pub origin: AttemptOrigin,
    pub client_event_id: Uuid,
    pub submission_hash: String,
    pub answer: Value,
    pub assistance: AssistanceRecord,
    pub received_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GradeResult {
    pub id: Uuid,
    pub attempt_id: Uuid,
    pub user_id: Uuid,
    pub ordinal: i32,
    pub disposition: String,
    pub checker_version: String,
    pub policy_version: String,
    pub evidence: Value,
    pub evaluated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CurrentGrade {
    pub attempt_id: Uuid,
    pub user_id: Uuid,
    pub grade_id: Uuid,
    pub revision: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LearningEvent {
    pub user_id: Uuid,
    pub sequence: i64,
    pub event_id: Uuid,
    pub event_type: String,
    pub schema_version: i32,
    pub payload: Value,
    pub occurred_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RewardEntry {
    pub id: Uuid,
    pub user_id: Uuid,
    pub award_key: String,
    pub source_sequence: i64,
    pub policy_version: String,
    pub delta: i32,
    pub reason: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MasteryProjection {
    pub user_id: Uuid,
    pub concept_id: String,
    pub dimension: String,
    pub alpha: f64,
    pub beta: f64,
    pub evidence_count: i64,
    pub state: String,
    pub revision: i64,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReviewSchedule {
    pub user_id: Uuid,
    pub concept_id: String,
    pub step: i32,
    pub due_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// -------------------------------------------------------------
// API Request & Response DTOs (Section 22)
// -------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateInstanceRequest {
    pub template_id: String,
    pub context: Option<ExerciseContext>,
    pub difficulty_band: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubmissionRequest {
    pub schema_version: u32,
    pub submission_id: Uuid,
    pub instance_id: Uuid,
    pub answer: Value,
    pub assistance: AssistanceRecord,
    pub client_context: Option<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GradeOutcomeDto {
    pub disposition: String,
    pub checker_version: String,
    pub evidence_level: String,
    pub reason_code: String,
    pub feedback: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RewardEntryDto {
    pub award_key: String,
    pub delta: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubmissionResponse {
    pub submission_id: Uuid,
    pub state: String,
    pub grade: GradeOutcomeDto,
    pub mastery_revision: i64,
    pub reward_entries: Vec<RewardEntryDto>,
    pub sync_cursor: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientSyncEvent {
    pub client_event_id: Uuid,
    pub event_type: String,
    pub schema_version: u32,
    pub payload: Value,
    pub occurred_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncBatchRequest {
    pub events: Vec<ClientSyncEvent>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventSyncStatus {
    pub client_event_id: Uuid,
    pub status: String, // "accepted", "duplicate", "conflict", "incompatible"
    pub server_event_id: Option<Uuid>,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncBatchResponse {
    pub processed: Vec<EventSyncStatus>,
    pub sync_cursor: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RewardLedgerResponse {
    pub user_id: Uuid,
    pub total_xp: i32,
    pub daily_practice_xp: i32,
    pub entries: Vec<RewardEntry>,
}
