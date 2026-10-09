//! AXIOM HTTP API Service (Section 22).

use axum::{
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
    response::{IntoResponse, Json, Response},
    routing::{get, post},
    Router,
};
use axiom_math::generate;
use axiom_storage::MemoryRepository;
use axiom_types::*;
use chrono::Utc;
use serde_json::{json, Value};
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};
use uuid::Uuid;

pub const DEFAULT_LEARNER_ID: &str = "00000000-0000-0000-0000-000000000001";

#[derive(Clone)]
pub struct AppState {
    pub repo: MemoryRepository,
}

pub fn app(repo: MemoryRepository) -> Router {
    let state = AppState { repo };

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    Router::new()
        .route("/health/live", get(health_live))
        .route("/health/ready", get(health_ready))
        .route("/api/v1/catalog/courses", get(get_courses))
        .route("/api/v1/catalog/courses/{id}", get(get_course_by_id))
        .route("/api/v1/exercise-instances", post(create_exercise_instance))
        .route("/api/v1/attempts", post(submit_attempt))
        .route("/api/v1/mastery", get(get_mastery))
        .route("/api/v1/reviews/due", get(get_due_reviews))
        .route("/api/v1/rewards", get(get_rewards))
        .route("/api/v1/sync/events", post(sync_events))
        .layer(cors)
        .with_state(Arc::new(state))
}

fn extract_user_id(headers: &HeaderMap) -> Uuid {
    headers
        .get("x-user-id")
        .and_then(|v| v.to_str().ok())
        .and_then(|s| Uuid::parse_str(s).ok())
        .unwrap_or_else(|| Uuid::parse_str(DEFAULT_LEARNER_ID).unwrap())
}

async fn health_live() -> impl IntoResponse {
    Json(json!({ "status": "live" }))
}

async fn health_ready() -> impl IntoResponse {
    Json(json!({ "status": "ready" }))
}

fn load_catalog_json() -> Option<Value> {
    let candidates = [
        std::path::PathBuf::from("content/catalog/courses.json"),
        std::path::PathBuf::from("../../content/catalog/courses.json"),
    ];
    for path in &candidates {
        if let Ok(contents) = std::fs::read_to_string(path) {
            if let Ok(val) = serde_json::from_str::<Value>(&contents) {
                return Some(val);
            }
        }
    }
    None
}

async fn get_courses() -> Response {
    if let Some(val) = load_catalog_json() {
        return Json(val).into_response();
    }
    (StatusCode::INTERNAL_SERVER_ERROR, "Failed to read course catalog").into_response()
}

async fn get_course_by_id(Path(id): Path<String>) -> Response {
    if let Some(val) = load_catalog_json() {
        if let Some(courses) = val.get("courses").and_then(|c| c.as_array()) {
            if let Some(course) = courses.iter().find(|c| c.get("id").and_then(|i| i.as_str()) == Some(&id)) {
                return Json(course.clone()).into_response();
            }
        }
    }
    (StatusCode::NOT_FOUND, "Course not found").into_response()
}

async fn create_exercise_instance(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Json(req): Json<CreateInstanceRequest>,
) -> Response {
    let user_id = extract_user_id(&headers);
    if let Err(e) = state.repo.ensure_user(user_id) {
        return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response();
    }

    let difficulty = req.difficulty_band.unwrap_or(1);
    let seed_hex = format!("{:032x}{:032x}", Uuid::new_v4().as_u128(), Uuid::new_v4().as_u128());
    let manifest_hash = "0000000000000000000000000000000000000000000000000000000000000000";

    match generate(&req.template_id, "1.0.0", &seed_hex, difficulty, manifest_hash) {
        Ok(problem) => {
            let instance_id = Uuid::new_v4();
            let context = req.context.unwrap_or(ExerciseContext::Practice);

            let instance = ExerciseInstance {
                id: instance_id,
                user_id,
                template_id: req.template_id.clone(),
                template_version: "1.0.0".into(),
                generator_version: problem.replay_identity.generator_version.clone(),
                checker_version: problem.replay_identity.checker_version.clone(),
                manifest_hash: manifest_hash.into(),
                seed_hex: seed_hex.clone(),
                context,
                problem_hash: problem.problem_hash.clone(),
                artifact: serde_json::to_value(&problem).unwrap(),
                created_at: Utc::now(),
            };

            if let Err(e) = state.repo.save_instance(instance.clone()) {
                return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response();
            }

            Json(instance).into_response()
        }
        Err(e) => (StatusCode::BAD_REQUEST, e.to_string()).into_response(),
    }
}

async fn submit_attempt(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Json(req): Json<SubmissionRequest>,
) -> Response {
    let user_id = extract_user_id(&headers);
    if let Err(e) = state.repo.ensure_user(user_id) {
        return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response();
    }

    match state.repo.record_submission_atomic(user_id, req) {
        Ok(res) => Json(res).into_response(),
        Err(axiom_storage::StorageError::InstanceNotFound(id)) => {
            (StatusCode::NOT_FOUND, format!("Instance not found: {id}")).into_response()
        }
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

async fn get_mastery(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
) -> Response {
    let user_id = extract_user_id(&headers);
    match state.repo.get_mastery(user_id) {
        Ok(mastery) => Json(mastery).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

async fn get_due_reviews(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
) -> Response {
    let user_id = extract_user_id(&headers);
    match state.repo.get_due_reviews(user_id, Utc::now()) {
        Ok(reviews) => Json(reviews).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

async fn get_rewards(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
) -> Response {
    let user_id = extract_user_id(&headers);
    match state.repo.get_reward_ledger(user_id) {
        Ok(ledger) => Json(ledger).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

async fn sync_events(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Json(batch): Json<SyncBatchRequest>,
) -> Response {
    let user_id = extract_user_id(&headers);
    if let Err(e) = state.repo.ensure_user(user_id) {
        return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response();
    }

    match state.repo.sync_client_events(user_id, batch) {
        Ok(resp) => Json(resp).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axiom_math::ProblemArtifact;
    use axum::http::Request;
    use tower::ServiceExt;

    #[tokio::test]
    async fn health_endpoints_pass() {
        let repo = MemoryRepository::new();
        let app = app(repo);

        let res = app
            .clone()
            .oneshot(Request::builder().uri("/health/live").body(axum::body::Body::empty()).unwrap())
            .await
            .unwrap();
        assert_eq!(res.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn full_learning_api_journey() {
        let repo = MemoryRepository::new();
        let app = app(repo);

        // 1. Catalog
        let res = app
            .clone()
            .oneshot(Request::builder().uri("/api/v1/catalog/courses").body(axum::body::Body::empty()).unwrap())
            .await
            .unwrap();
        assert_eq!(res.status(), StatusCode::OK);

        // 2. Issue Exercise Instance
        let create_req = serde_json::to_vec(&CreateInstanceRequest {
            template_id: "arithmetic.whole.addition".into(),
            context: Some(ExerciseContext::Practice),
            difficulty_band: Some(1),
        }).unwrap();

        let res = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/api/v1/exercise-instances")
                    .header("content-type", "application/json")
                    .body(axum::body::Body::from(create_req))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(res.status(), StatusCode::OK);
        let body = axum::body::to_bytes(res.into_body(), usize::MAX).await.unwrap();
        let instance: ExerciseInstance = serde_json::from_slice(&body).unwrap();
        assert_eq!(instance.template_id, "arithmetic.whole.addition");

        // 3. Submit Attempt with Target Answer
        let problem: ProblemArtifact = serde_json::from_value(instance.artifact.clone()).unwrap();
        let target_ans = problem.target_value.unwrap();

        let sub_id = Uuid::new_v4();
        let sub_req = serde_json::to_vec(&SubmissionRequest {
            schema_version: 1,
            submission_id: sub_id,
            instance_id: instance.id,
            answer: Value::String(target_ans),
            assistance: AssistanceRecord::default(),
            client_context: None,
        }).unwrap();

        let res = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/api/v1/attempts")
                    .header("content-type", "application/json")
                    .body(axum::body::Body::from(sub_req))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(res.status(), StatusCode::OK);
        let body = axum::body::to_bytes(res.into_body(), usize::MAX).await.unwrap();
        let sub_res: SubmissionResponse = serde_json::from_slice(&body).unwrap();
        assert_eq!(sub_res.grade.disposition, "correct");
        assert_eq!(sub_res.reward_entries.len(), 1);

        // 4. Rewards endpoint
        let res = app
            .clone()
            .oneshot(Request::builder().uri("/api/v1/rewards").body(axum::body::Body::empty()).unwrap())
            .await
            .unwrap();
        assert_eq!(res.status(), StatusCode::OK);
        let body = axum::body::to_bytes(res.into_body(), usize::MAX).await.unwrap();
        let ledger: RewardLedgerResponse = serde_json::from_slice(&body).unwrap();
        assert_eq!(ledger.total_xp, 10);
    }
}
