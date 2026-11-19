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
use serde::Deserialize;
use serde_json::{json, Value};
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};
use uuid::Uuid;

pub const DEFAULT_LEARNER_ID: &str = "00000000-0000-0000-0000-000000000001";

#[derive(Clone)]
pub struct AppState {
    pub repo: MemoryRepository,
    pub local_demo: bool,
}

/// Production routes fail closed until identity and durable storage are implemented.
pub fn app(repo: MemoryRepository) -> Router { build_app(repo, false) }

/// Explicit, loopback-only development demonstration; never an account service.
pub fn local_demo_app(repo: MemoryRepository) -> Router { build_app(repo, true) }

fn build_app(repo: MemoryRepository, local_demo: bool) -> Router {
    let state = Arc::new(AppState { repo, local_demo });

    let cors = CorsLayer::new()
        .allow_origin(axum::http::HeaderValue::from_static("http://localhost:3000"))
        .allow_methods(Any)
        .allow_headers(Any);

    let private = Router::new()
        .route("/api/v1/exercise-instances", post(create_exercise_instance))
        .route("/api/v1/attempts", post(submit_attempt))
        .route("/api/v1/mastery", get(get_mastery))
        .route("/api/v1/reviews/due", get(get_due_reviews))
        .route("/api/v1/rewards", get(get_rewards))
        .route("/api/v1/sync/events", post(sync_events))
        .route("/api/v1/sync/changes", get(sync_changes))
        .route("/api/v1/notebooks", get(get_notebooks).post(create_notebook))
        .route("/api/v1/notebooks/{id}", get(get_notebook_by_id))
        .route("/api/v1/notebooks/{id}/revisions", get(get_notebook_revisions).post(create_notebook_revision))
        .route("/api/v1/exports", post(create_export))
        .route("/api/v1/account/deletion-requests", post(request_account_deletion))
        .route_layer(axum::middleware::from_fn_with_state(state.clone(), protect_learning_routes));
    Router::new()
        .route("/health/live", get(health_live))
        .route("/health/ready", get(health_ready))
        .route("/api/v1/catalog/courses", get(get_courses))
        .route("/api/v1/catalog/courses/{id}", get(get_course_by_id))
        .merge(private)
        .layer(cors)
        .with_state(state)
}

async fn protect_learning_routes(
    State(state): State<Arc<AppState>>,
    request: axum::extract::Request,
    next: axum::middleware::Next,
) -> Response {
    if request.headers().contains_key("x-user-id") {
        return (StatusCode::UNAUTHORIZED, Json(json!({"code":"untrusted_identity", "detail":"Client-selected identity is not accepted."}))).into_response();
    }
    if !state.local_demo {
        return (StatusCode::SERVICE_UNAVAILABLE, Json(json!({"code":"account_service_unavailable", "detail":"Identity and durable storage are not configured. Use local practice."}))).into_response();
    }
    next.run(request).await
}

fn extract_user_id(_headers: &HeaderMap) -> Uuid {
    // A single synthetic identity only in the explicitly enabled local demo.
    Uuid::parse_str(DEFAULT_LEARNER_ID).unwrap()
}

async fn health_live() -> impl IntoResponse {
    Json(json!({ "status": "live" }))
}

async fn health_ready() -> impl IntoResponse {
    (StatusCode::SERVICE_UNAVAILABLE, Json(json!({ "status": "not_ready", "durable_storage": false, "identity": false, "detail": "Local demonstration only; account service is not ready." })))
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
        if let Some(courses) = val.as_array() {
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

    if !matches!(req.context.as_ref(), None | Some(ExerciseContext::Practice)) {
        return (StatusCode::BAD_REQUEST, "The local demo supports practice only, not independent assessments.").into_response();
    }
    let difficulty = req.difficulty_band.unwrap_or(1);
    let seed_hex = format!("{:032x}{:032x}", Uuid::new_v4().as_u128(), Uuid::new_v4().as_u128());
    let manifest_hash = "0000000000000000000000000000000000000000000000000000000000000000";

    match generate(&req.template_id, "1.1.0", &seed_hex, difficulty, manifest_hash) {
        Ok(problem) => {
            let instance_id = Uuid::new_v4();
            let context = req.context.unwrap_or(ExerciseContext::Practice);

            let instance = ExerciseInstance {
                id: instance_id,
                user_id,
                template_id: req.template_id.clone(),
                template_version: "1.1.0".into(),
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
        Err(axiom_storage::StorageError::Conflict(message)) => (StatusCode::CONFLICT, message).into_response(),
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
        Ok(res) => Json(res).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

#[derive(Deserialize)]
struct SyncChangesQuery {
    after: Option<String>,
}

async fn sync_changes(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    axum::extract::Query(query): axum::extract::Query<SyncChangesQuery>,
) -> Response {
    let user_id = extract_user_id(&headers);
    if let Err(e) = state.repo.ensure_user(user_id) {
        return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response();
    }
    let after_seq = query
        .after
        .and_then(|cursor| {
            cursor.split(':').last().and_then(|s| s.parse::<i64>().ok())
        })
        .unwrap_or(0);

    match state.repo.get_sync_changes(user_id, after_seq) {
        Ok(res) => Json(res).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

async fn get_notebooks(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
) -> Response {
    let user_id = extract_user_id(&headers);
    match state.repo.get_notebooks(user_id) {
        Ok(docs) => Json(docs).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

async fn create_notebook(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Json(req): Json<CreateNotebookRequest>,
) -> Response {
    let user_id = extract_user_id(&headers);
    if let Err(e) = state.repo.ensure_user(user_id) {
        return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response();
    }
    match state.repo.create_notebook(user_id, req) {
        Ok(doc) => (StatusCode::CREATED, Json(doc)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

async fn get_notebook_by_id(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Path(id): Path<Uuid>,
) -> Response {
    let user_id = extract_user_id(&headers);
    match state.repo.get_notebook(user_id, id) {
        Ok(Some(doc)) => Json(doc).into_response(),
        Ok(None) => (StatusCode::NOT_FOUND, "Notebook not found").into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

async fn get_notebook_revisions(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Path(id): Path<Uuid>,
) -> Response {
    let user_id = extract_user_id(&headers);
    match state.repo.get_notebook_revisions(user_id, id) {
        Ok(revs) => Json(revs).into_response(),
        Err(axiom_storage::StorageError::InstanceNotFound(_)) => (StatusCode::NOT_FOUND, "Notebook not found").into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

async fn create_notebook_revision(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Path(id): Path<Uuid>,
    Json(req): Json<CreateRevisionRequest>,
) -> Response {
    let user_id = extract_user_id(&headers);
    match state.repo.create_notebook_revision(user_id, id, req) {
        Ok(rev) => (StatusCode::CREATED, Json(rev)).into_response(),
        Err(axiom_storage::StorageError::InstanceNotFound(_)) => (StatusCode::NOT_FOUND, "Notebook not found").into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

async fn create_export(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
) -> Response {
    let user_id = extract_user_id(&headers);
    if let Err(e) = state.repo.ensure_user(user_id) {
        return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response();
    }
    match state.repo.export_user_data(user_id) {
        Ok(export) => Json(export).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

async fn request_account_deletion(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
) -> Response {
    let user_id = extract_user_id(&headers);
    if let Err(e) = state.repo.ensure_user(user_id) {
        return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response();
    }
    match state.repo.request_account_deletion(user_id) {
        Ok(res) => Json(res).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

async fn _old_sync_stub() -> Response { (StatusCode::NO_CONTENT).into_response() }
async fn _ignored() -> Response {
    (StatusCode::SERVICE_UNAVAILABLE, Json(json!({"code":"sync_unavailable", "detail":"Offline event reconciliation is not implemented. Keep your local drafts and backups."}))).into_response()
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
        let app = local_demo_app(repo);

        // 1. Catalog
        let res = app
            .clone()
            .oneshot(Request::builder().uri("/api/v1/catalog/courses").body(axum::body::Body::empty()).unwrap())
            .await
            .unwrap();
        assert_eq!(res.status(), StatusCode::OK);

        // 2. Issue Exercise Instance
        let detail = app.clone().oneshot(Request::builder().uri("/api/v1/catalog/courses/F01").body(axum::body::Body::empty()).unwrap()).await.unwrap();
        assert_eq!(detail.status(), StatusCode::OK);
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
    #[tokio::test]
    async fn account_routes_fail_closed_and_reject_spoofed_identity() {
        let production = app(MemoryRepository::new());
        let response = production.oneshot(Request::builder().uri("/api/v1/rewards").body(axum::body::Body::empty()).unwrap()).await.unwrap();
        assert_eq!(response.status(), StatusCode::SERVICE_UNAVAILABLE);
        let demo = local_demo_app(MemoryRepository::new());
        let response = demo.oneshot(Request::builder().uri("/api/v1/rewards").header("x-user-id", Uuid::new_v4().to_string()).body(axum::body::Body::empty()).unwrap()).await.unwrap();
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn offline_sync_and_changes_api() {
        let repo = MemoryRepository::new();
        let app = local_demo_app(repo.clone());

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
        let problem: ProblemArtifact = serde_json::from_value(instance.artifact.clone()).unwrap();
        let target_ans = problem.target_value.unwrap();

        let sync_batch = SyncBatchRequest {
            events: vec![ClientSyncEvent {
                client_event_id: Uuid::new_v4(),
                event_type: "offline_attempt".into(),
                schema_version: 1,
                payload: json!({
                    "instance_id": instance.id,
                    "answer": target_ans,
                    "client_claimed_score": 500,
                    "assistance": {
                        "max_hint_level": 0,
                        "solution_viewed": false
                    }
                }),
                occurred_at: Utc::now(),
            }],
        };

        let sync_req = serde_json::to_vec(&sync_batch).unwrap();
        let res = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/api/v1/sync/events")
                    .header("content-type", "application/json")
                    .body(axum::body::Body::from(sync_req))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(res.status(), StatusCode::OK);
        let body = axum::body::to_bytes(res.into_body(), usize::MAX).await.unwrap();
        let sync_res: SyncBatchResponse = serde_json::from_slice(&body).unwrap();
        assert_eq!(sync_res.processed[0].status, "accepted");

        let res = app
            .clone()
            .oneshot(Request::builder().uri("/api/v1/sync/changes?after=0").body(axum::body::Body::empty()).unwrap())
            .await
            .unwrap();
        assert_eq!(res.status(), StatusCode::OK);
        let body = axum::body::to_bytes(res.into_body(), usize::MAX).await.unwrap();
        let changes: SyncChangesResponse = serde_json::from_slice(&body).unwrap();
        assert!(!changes.events.is_empty());
        assert_eq!(changes.reward_ledger.total_xp, 10);
    }

    #[tokio::test]
    async fn notebook_and_export_api_journey() {
        let repo = MemoryRepository::new();
        let app = local_demo_app(repo);

        let create_nb = serde_json::to_vec(&CreateNotebookRequest {
            id: None,
            title: "Study Log".into(),
            initial_content: Some("First note on powers".into()),
        }).unwrap();

        let res = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/api/v1/notebooks")
                    .header("content-type", "application/json")
                    .body(axum::body::Body::from(create_nb))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(res.status(), StatusCode::CREATED);
        let body = axum::body::to_bytes(res.into_body(), usize::MAX).await.unwrap();
        let doc: NotebookDocument = serde_json::from_slice(&body).unwrap();

        let create_rev = serde_json::to_vec(&CreateRevisionRequest {
            revision_id: None,
            parent_revision_id: None,
            content: "Updated note: 2^3 = 8".into(),
        }).unwrap();

        let res = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri(format!("/api/v1/notebooks/{}/revisions", doc.id))
                    .header("content-type", "application/json")
                    .body(axum::body::Body::from(create_rev))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(res.status(), StatusCode::CREATED);

        let res = app
            .clone()
            .oneshot(Request::builder().method("POST").uri("/api/v1/exports").body(axum::body::Body::empty()).unwrap())
            .await
            .unwrap();
        assert_eq!(res.status(), StatusCode::OK);
        let body = axum::body::to_bytes(res.into_body(), usize::MAX).await.unwrap();
        let export: ExportDataResponse = serde_json::from_slice(&body).unwrap();
        assert_eq!(export.notebooks.len(), 1);

        let res = app
            .clone()
            .oneshot(Request::builder().method("POST").uri("/api/v1/account/deletion-requests").body(axum::body::Body::empty()).unwrap())
            .await
            .unwrap();
        assert_eq!(res.status(), StatusCode::OK);
    }

}
