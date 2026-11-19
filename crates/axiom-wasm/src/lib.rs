//! Narrow wasm-bindgen facade exposing pure Rust math core to the browser (Section 18.1).
//!
//! Serialization boundary ensures JavaScript and WASM communicate via strict JSON DTOs,
//! with identical evaluation on the client and server.

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn math_core_version() -> String {
    "axiom-math-0.1.0".into()
}

/// Generates a problem artifact deterministically. Returns JSON string of ProblemArtifact or error.
#[wasm_bindgen]
pub fn generate_exercise(
    template_id: &str,
    template_version: &str,
    seed_hex: &str,
    difficulty_band: u32,
    manifest_hash: &str,
) -> Result<String, JsValue> {
    match axiom_math::generate(
        template_id,
        template_version,
        seed_hex,
        difficulty_band,
        manifest_hash,
    ) {
        Ok(artifact) => serde_json::to_string(&artifact)
            .map_err(|e| JsValue::from_str(&e.to_string())),
        Err(err) => Err(JsValue::from_str(&err.to_string())),
    }
}

/// Checks an answer string against a problem artifact JSON string.
#[wasm_bindgen]
pub fn check_answer(problem_json: &str, raw_answer: &str) -> Result<String, JsValue> {
    let problem: axiom_math::ProblemArtifact = serde_json::from_str(problem_json)
        .map_err(|e| JsValue::from_str(&format!("Invalid problem JSON: {e}")))?;

    let budget = axiom_math::MathBudget::default();
    let outcome = axiom_math::check(&problem, raw_answer, &budget);

    serde_json::to_string(&outcome)
        .map_err(|e| JsValue::from_str(&e.to_string()))
}

/// Applies an abacus action. Returns new state JSON string.
#[wasm_bindgen]
pub fn abacus_apply_action(state_json: &str, action_json: &str) -> Result<String, JsValue> {
    let state: axiom_math::AbacusState = serde_json::from_str(state_json)
        .map_err(|e| JsValue::from_str(&format!("Invalid abacus state JSON: {e}")))?;
    state.validate().map_err(|e| JsValue::from_str(&e.to_string()))?;
    let action: axiom_math::AbacusAction = serde_json::from_str(action_json)
        .map_err(|e| JsValue::from_str(&format!("Invalid abacus action JSON: {e}")))?;

    let next_state = axiom_math::apply_abacus_action(&state, &action)
        .map_err(|e| JsValue::from_str(&e.to_string()))?;

    serde_json::to_string(&next_state)
        .map_err(|e| JsValue::from_str(&e.to_string()))
}

/// Creates a new soroban state representing an exact value.
#[wasm_bindgen]
pub fn abacus_from_value(value_canonical: &str, rod_count: usize, rightmost_exponent: i32) -> Result<String, JsValue> {
    let q = axiom_math::from_canonical(value_canonical)
        .ok_or_else(|| JsValue::from_str("Invalid canonical rational string"))?;

    let state = axiom_math::AbacusState::from_value(&q, rod_count, rightmost_exponent)
        .map_err(|e| JsValue::from_str(&e.to_string()))?;

    serde_json::to_string(&state)
        .map_err(|e| JsValue::from_str(&e.to_string()))
}

/// Describes an abacus state in plain text for speech synthesis and screen readers.
#[wasm_bindgen]
pub fn abacus_describe(state_json: &str) -> Result<String, JsValue> {
    let state: axiom_math::AbacusState = serde_json::from_str(state_json)
        .map_err(|e| JsValue::from_str(&format!("Invalid abacus state JSON: {e}")))?;

    state.validate().map_err(|e| JsValue::from_str(&e.to_string()))?;
    Ok(state.describe_state())
}

#[wasm_bindgen]
pub fn study_catalog() -> String {serde_json::to_string(&axiom_learning::journey::catalog()).unwrap()}
#[wasm_bindgen]
pub fn validate_exercise_artifact(problem_json:&str)->Result<(),JsValue>{if problem_json.len()>131072{return Err(JsValue::from_str("Exercise exceeds bounds"))}let p:axiom_math::ProblemArtifact=serde_json::from_str(problem_json).map_err(|e|JsValue::from_str(&e.to_string()))?;let r=&p.replay_identity;let fresh=axiom_math::generate(&r.template_id,&r.template_version,&r.seed_hex,r.difficulty_band,&r.content_manifest_hash).map_err(|e|JsValue::from_str(&e.to_string()))?;if p!=fresh{return Err(JsValue::from_str("Exercise differs from its replay identity"))}Ok(())}
fn study_state(json:&str)->Result<axiom_learning::journey::JourneyState,String>{if json.len()>262144{return Err("Study record exceeds bounds".into())}let state=if json=="null"{Default::default()}else{serde_json::from_str(json).map_err(|e|format!("Unreadable study history: {e}"))?};axiom_learning::journey::validate(&state)?;Ok(state)}
#[wasm_bindgen]
pub fn study_snapshot(state_json:&str,day:i64)->Result<String,JsValue>{let s=study_state(state_json).map_err(|e|JsValue::from_str(&e))?;let view=axiom_learning::journey::snapshot(&s,day).map_err(|e|JsValue::from_str(&e))?;Ok(serde_json::json!({"state":s,"view":view}).to_string())}
#[wasm_bindgen]
pub fn study_action(state_json:&str,action_json:&str,day:i64)->Result<String,JsValue>{if action_json.len()>2048{return Err(JsValue::from_str("Study action exceeds bounds"))}let s=study_state(state_json).map_err(|e|JsValue::from_str(&e))?;let a=serde_json::from_str(action_json).map_err(|e|JsValue::from_str(&e.to_string()))?;let s=axiom_learning::journey::action(s,a,day).map_err(|e|JsValue::from_str(&e))?;let view=axiom_learning::journey::snapshot(&s,day).map_err(|e|JsValue::from_str(&e))?;Ok(serde_json::json!({"state":s,"view":view}).to_string())}
#[wasm_bindgen]
pub fn study_check(state_json:&str,problem_json:&str,answer:&str,assisted:bool,day:i64)->Result<String,JsValue>{if problem_json.len()>131072{return Err(JsValue::from_str("Exercise exceeds bounds"))}let p:axiom_math::ProblemArtifact=serde_json::from_str(problem_json).map_err(|e|JsValue::from_str(&e.to_string()))?;let outcome=axiom_math::check(&p,answer,&Default::default());let result=study_state(state_json).and_then(|s|axiom_learning::journey::record(s,&p,&outcome,assisted,day));Ok(match result{Ok(state)=>serde_json::json!({"outcome":outcome,"state":state,"history_error":null}),Err(e)=>serde_json::json!({"outcome":outcome,"state":null,"history_error":e})}.to_string())}

#[wasm_bindgen]
pub fn abacus_tutor_catalog() -> Result<String, JsValue> {
    let catalog = axiom_math::abacus_tutor::catalog().map_err(|error|JsValue::from_str(&error.to_string()))?;
    serde_json::to_string(&catalog).map_err(|error|JsValue::from_str(&error.to_string()))
}

#[wasm_bindgen]
pub fn exercise_coaching(problem_json: &str) -> Result<String, JsValue> {
    if problem_json.len()>131072 {return Err(JsValue::from_str("Teaching request exceeds bounds"))}
    let p:axiom_math::ProblemArtifact=serde_json::from_str(problem_json).map_err(|e|JsValue::from_str(&e.to_string()))?;
    let material=axiom_math::coaching::material(&p).map_err(|e|JsValue::from_str(&e))?;
    serde_json::to_string(&material).map_err(|e|JsValue::from_str(&e.to_string()))
}

#[wasm_bindgen]
pub fn content_hash(content_json: &str) -> Result<String, JsValue> {
    let value: serde_json::Value = serde_json::from_str(content_json)
        .map_err(|error| JsValue::from_str(&error.to_string()))?;
    axiom_math::jcs::canonical_hash(&value).map_err(|error| JsValue::from_str(&error.to_string()))
}

#[wasm_bindgen]
pub fn abacus_value(state_json: &str) -> Result<String, JsValue> {
    let state: axiom_math::AbacusState = serde_json::from_str(state_json)
        .map_err(|e| JsValue::from_str(&e.to_string()))?;
    state.validate().map_err(|e| JsValue::from_str(&e.to_string()))?;
    Ok(axiom_math::to_canonical(&state.compute_value()))
}

#[wasm_bindgen]
pub fn reward_instance(entries_json: &str, instance_id: &str, timestamp: &str) -> Result<String, JsValue> {
    let entries = serde_json::from_str(entries_json).map_err(|e| JsValue::from_str(&format!("Invalid reward history: {e}")))?;
    let mut ledger = axiom_learning::rewards::RewardLedger { entries };
    let mut award = ledger.award_instance_success(instance_id, timestamp);
    if let Some(entry) = award.as_mut() {
        entry.reason = "Local practice success; not independently assessed mastery".into();
        if let Some(stored) = ledger.entries.iter_mut().find(|stored| stored.award_key == entry.award_key) {
            stored.reason = entry.reason.clone();
        }
    }
    serde_json::to_string(&serde_json::json!({ "entries": ledger.entries, "award": award }))
        .map_err(|e| JsValue::from_str(&e.to_string()))
}

#[wasm_bindgen]
pub fn reward_provisional_milestone(entries_json: &str, concept_id: &str, timestamp: &str) -> Result<String, JsValue> {
    let entries = serde_json::from_str(entries_json).map_err(|e| JsValue::from_str(&format!("Invalid reward history: {e}")))?;
    let mut ledger = axiom_learning::rewards::RewardLedger { entries };
    let mut award = ledger.award_provisional_milestone(concept_id, timestamp);
    if let Some(entry) = award.as_mut() {
        entry.reason = format!("Provisional concept practice milestone: {concept_id} (local study)");
        if let Some(stored) = ledger.entries.iter_mut().find(|stored| stored.award_key == entry.award_key) {
            stored.reason = entry.reason.clone();
        }
    }
    serde_json::to_string(&serde_json::json!({ "entries": ledger.entries, "award": award }))
        .map_err(|e| JsValue::from_str(&e.to_string()))
}

/// Evaluates a scalar expression using exact arithmetic over Q and default budgets.
#[wasm_bindgen]
pub fn evaluate_scalar(expr_raw: &str) -> Result<String, JsValue> {
    let budget = axiom_math::MathBudget::default();
    let ast = match axiom_math::parse_scalar(expr_raw, &budget) {
        Ok(a) => a,
        Err(e) => return Ok(serde_json::json!({
            "status": "parse_failure",
            "code": e.code,
            "message": e.message,
            "budget": e.budget,
        }).to_string()),
    };

    match axiom_math::evaluate(&ast, &budget) {
        Ok(val) => Ok(serde_json::json!({
            "status": "success",
            "canonical": axiom_math::to_canonical(&val),
            "finite_decimal": axiom_math::to_finite_decimal(&val),
            "latex": axiom_math::to_latex(&val),
            "speech": axiom_math::to_speech(&val),
        }).to_string()),
        Err(e) => Ok(serde_json::json!({
            "status": "eval_failure",
            "code": e.code,
            "message": e.message,
            "budget": e.budget,
        }).to_string()),
    }
}

/// Updates a spaced retrieval review schedule according to Section 16.3.
#[wasm_bindgen]
pub fn review_schedule_update(schedule_json: &str, success: bool, current_epoch_days: i64) -> Result<String, JsValue> {
    let mut schedule: axiom_learning::schedule::ReviewSchedule = if schedule_json.is_empty() || schedule_json == "null" {
        axiom_learning::schedule::ReviewSchedule::new("concept", current_epoch_days)
    } else {
        serde_json::from_str(schedule_json).map_err(|e| JsValue::from_str(&format!("Invalid schedule JSON: {e}")))?
    };

    if success {
        schedule.on_success(current_epoch_days);
    } else {
        schedule.on_failure(current_epoch_days);
    }

    serde_json::to_string(&schedule).map_err(|e| JsValue::from_str(&e.to_string()))
}
