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

    Ok(state.describe_state())
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
