//! Deterministic exercise generation and template contract (Section 14).

use crate::jcs::canonical_hash;
use crate::number::{frac, int, to_canonical, to_latex, to_speech};
use crate::rng::{uniform_inclusive, AxiomRng, DrawSource, GENERATOR_VERSION};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReplayIdentity {
    pub template_id: String,
    pub template_version: String,
    pub generator_version: String,
    pub checker_version: String,
    pub content_manifest_hash: String,
    pub seed_hex: String,
    pub difficulty_band: u32,
    pub parameter_policy_version: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProblemArtifact {
    pub replay_identity: ReplayIdentity,
    pub prompt_text: String,
    pub prompt_latex: String,
    pub prompt_speech: String,
    pub answer_kind: String,
    pub options: Option<Vec<String>>,
    pub target_value: Option<String>,
    pub metadata: Value,
    pub problem_hash: String,
}

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error, Serialize, Deserialize)]
pub enum GenerationFailure {
    #[error("template not found: {0}")]
    TemplateNotFound(String),
    #[error("invalid seed format: {0}")]
    InvalidSeed(String),
    #[error("constraint exhaustion after {draws} candidate attempts")]
    ConstraintExhaustion { draws: u32 },
    #[error("serialization error: {0}")]
    Serialization(String),
}

/// Generates a problem artifact from a template ID, difficulty, seed, and manifest hash.
pub fn generate(
    template_id: &str,
    template_version: &str,
    seed_hex: &str,
    difficulty_band: u32,
    manifest_hash: &str,
) -> Result<ProblemArtifact, GenerationFailure> {
    let mut rng = AxiomRng::from_hex(seed_hex)
        .map_err(|e| GenerationFailure::InvalidSeed(e.to_string()))?;

    let (prompt_text, prompt_latex, prompt_speech, answer_kind, options, target_value, metadata, checker_version) =
        match template_id {
            "fractions.compare.positive" => {
                generate_fraction_compare(&mut rng, difficulty_band)?
            }
            "fractions.equivalent.find" => {
                generate_equivalent_fraction(&mut rng, difficulty_band)?
            }
            "fractions.unit.identify" => {
                generate_unit_fraction(&mut rng, difficulty_band)?
            }
            "arithmetic.whole.addition" => {
                generate_whole_addition(&mut rng, difficulty_band)?
            }
            "arithmetic.column.addition" => {
                generate_column_addition(&mut rng, difficulty_band)?
            }
            "arithmetic.column.subtraction" => {
                generate_column_subtraction(&mut rng, difficulty_band)?
            }
            "place_value.decompose" => {
                generate_place_value_decompose(&mut rng, difficulty_band)?
            }
            "abacus.read.state" => {
                generate_abacus_read(&mut rng, difficulty_band)?
            }
            "abacus.target.setting" => {
                generate_abacus_target_setting(&mut rng, difficulty_band)?
            }
            _ => return Err(GenerationFailure::TemplateNotFound(template_id.into())),
        };

    let replay = ReplayIdentity {
        template_id: template_id.into(),
        template_version: template_version.into(),
        generator_version: GENERATOR_VERSION.into(),
        checker_version,
        content_manifest_hash: manifest_hash.into(),
        seed_hex: seed_hex.into(),
        difficulty_band,
        parameter_policy_version: "1".into(),
    };

    let val = serde_json::json!({
        "replay": &replay,
        "prompt_text": &prompt_text,
        "answer_kind": &answer_kind,
        "options": &options,
        "target_value": &target_value,
        "metadata": &metadata,
    });

    let problem_hash = canonical_hash(&val)
        .map_err(|e| GenerationFailure::Serialization(e.to_string()))?;

    Ok(ProblemArtifact {
        replay_identity: replay,
        prompt_text,
        prompt_latex,
        prompt_speech,
        answer_kind,
        options,
        target_value,
        metadata,
        problem_hash,
    })
}

/// Template: `fractions.compare.positive` (Section 11.1 & Section 14.3)
/// Compares two positive proper or simple fractions. Allowed answers: "less", "equal", "greater"
fn generate_fraction_compare<R: DrawSource + ?Sized>(
    rng: &mut R,
    difficulty: u32,
) -> Result<(String, String, String, String, Option<Vec<String>>, Option<String>, Value, String), GenerationFailure> {
    let (d_min, d_max) = match difficulty {
        1 => (2, 8),
        2 => (3, 12),
        _ => (4, 16),
    };

    // Draw with maximum 128 candidate attempts
    for _ in 0..128 {
        let d1 = uniform_inclusive(rng, d_min, d_max) as i64;
        let d2 = uniform_inclusive(rng, d_min, d_max) as i64;
        let n1 = uniform_inclusive(rng, 1, (d1 - 1) as i128) as i64;
        let n2 = uniform_inclusive(rng, 1, (d2 - 1) as i128) as i64;

        let q1 = frac(n1, d1);
        let q2 = frac(n2, d2);

        // We want fractions with different values for standard comparison tests
        if q1 == q2 {
            continue;
        }

        let relation = if q1 < q2 {
            "less"
        } else {
            "greater"
        };

        let prompt_text = format!("Compare the fractions {} and {}. Is {} less than, equal to, or greater than {}?",
            to_canonical(&q1), to_canonical(&q2), to_canonical(&q1), to_canonical(&q2));
        let prompt_latex = format!("\\text{{Compare }}\\; {} \\;\\text{{ and }}\\; {}",
            to_latex(&q1), to_latex(&q2));
        let prompt_speech = format!("Compare {} and {}. Is {} less than, equal to, or greater than {}?",
            to_speech(&q1), to_speech(&q2), to_speech(&q1), to_speech(&q2));

        let metadata = serde_json::json!({
            "frac1": to_canonical(&q1),
            "frac2": to_canonical(&q2),
            "frac1_orig": format!("{}/{}", n1, d1),
            "frac2_orig": format!("{}/{}", n2, d2),
            "common_denom": (d1 * d2 / num_integer::gcd(d1, d2)).to_string(),
        });

        return Ok((
            prompt_text,
            prompt_latex,
            prompt_speech,
            "relation".into(),
            Some(vec!["less".into(), "equal".into(), "greater".into()]),
            Some(relation.into()),
            metadata,
            "rational-order-v1".into(),
        ));
    }

    Err(GenerationFailure::ConstraintExhaustion { draws: 128 })
}

/// Template: `fractions.equivalent.find`
fn generate_equivalent_fraction<R: DrawSource + ?Sized>(
    rng: &mut R,
    difficulty: u32,
) -> Result<(String, String, String, String, Option<Vec<String>>, Option<String>, Value, String), GenerationFailure> {
    let (d_min, d_max) = match difficulty {
        1 => (2, 6),
        _ => (3, 10),
    };
    let d = uniform_inclusive(rng, d_min, d_max) as i64;
    let n = uniform_inclusive(rng, 1, (d - 1) as i128) as i64;
    let scale = uniform_inclusive(rng, 2, 4) as i64;

    let target_n = n * scale;
    let target_d = d * scale;
    let base_q = frac(n, d);

    let prompt_text = format!("Find an equivalent fraction for {} with denominator {}.",
        to_canonical(&base_q), target_d);
    let prompt_latex = format!("{}\\;=\\;\\frac{{?}}{{{}}}", to_latex(&base_q), target_d);
    let prompt_speech = format!("Find an equivalent fraction for {} with denominator {}.",
        to_speech(&base_q), target_d);

    let metadata = serde_json::json!({
        "base_frac": to_canonical(&base_q),
        "target_denominator": target_d,
        "target_numerator": target_n,
    });

    Ok((
        prompt_text,
        prompt_latex,
        prompt_speech,
        "rational".into(),
        None,
        Some(format!("{}/{}", target_n, target_d)),
        metadata,
        "fraction-form-v1".into(),
    ))
}

/// Template: `arithmetic.whole.addition`
fn generate_whole_addition<R: DrawSource + ?Sized>(
    rng: &mut R,
    difficulty: u32,
) -> Result<(String, String, String, String, Option<Vec<String>>, Option<String>, Value, String), GenerationFailure> {
    let (min, max) = match difficulty {
        1 => (10, 89),
        2 => (100, 899),
        _ => (1000, 8999),
    };
    let a = uniform_inclusive(rng, min, max);
    let b = uniform_inclusive(rng, min, max);
    let sum = a + b;

    let prompt_text = format!("Calculate {} + {}.", a, b);
    let prompt_latex = format!("{} + {} = ?", a, b);
    let prompt_speech = format!("What is {} plus {}?", a, b);

    let metadata = serde_json::json!({
        "operand1": a.to_string(),
        "operand2": b.to_string(),
    });

    Ok((
        prompt_text,
        prompt_latex,
        prompt_speech,
        "integer".into(),
        None,
        Some(sum.to_string()),
        metadata,
        "integer-exact-v1".into(),
    ))
}

/// Template: `abacus.read.state`
fn generate_abacus_read<R: DrawSource + ?Sized>(
    rng: &mut R,
    _difficulty: u32,
) -> Result<(String, String, String, String, Option<Vec<String>>, Option<String>, Value, String), GenerationFailure> {
    let val = uniform_inclusive(rng, 1, 99) as i64;
    let q = int(val);
    let abacus = crate::abacus::AbacusState::from_value(&q, 2, 0)
        .map_err(|e| GenerationFailure::Serialization(e.to_string()))?;

    let prompt_text = "Read the value displayed on the soroban abacus.".into();
    let prompt_latex = "\\text{Read the soroban abacus state}".into();
    let prompt_speech = format!("Read the soroban: {}", abacus.describe_state());

    let metadata = serde_json::json!({
        "abacus_state": abacus,
    });

    Ok((
        prompt_text,
        prompt_latex,
        prompt_speech,
        "integer".into(),
        None,
        Some(val.to_string()),
        metadata,
        "integer-exact-v1".into(),
    ))
}

/// Template: `fractions.unit.identify`
fn generate_unit_fraction<R: DrawSource + ?Sized>(
    rng: &mut R,
    difficulty: u32,
) -> Result<(String, String, String, String, Option<Vec<String>>, Option<String>, Value, String), GenerationFailure> {
    let (d_min, d_max) = match difficulty {
        1 => (2, 8),
        2 => (3, 12),
        _ => (4, 20),
    };
    let d = uniform_inclusive(rng, d_min, d_max) as i64;
    let target = frac(1, d);

    let prompt_text = format!("A unit strip is divided into {} equal parts. What unit fraction represents 1 of these parts?", d);
    let prompt_latex = format!("\\text{{Partition of 1 whole into }} {} \\text{{ equal parts}} \\implies \\frac{{1}}{{{}}}", d, d);
    let prompt_speech = format!("A unit strip is divided into {} equal parts. What unit fraction represents 1 of these parts?", d);

    let metadata = serde_json::json!({
        "denominator": d,
        "numerator": 1,
        "canonical": to_canonical(&target),
    });

    Ok((
        prompt_text,
        prompt_latex,
        prompt_speech,
        "rational".into(),
        None,
        Some(to_canonical(&target)),
        metadata,
        "rational-equality-v1".into(),
    ))
}

/// Template: `arithmetic.column.addition`
fn generate_column_addition<R: DrawSource + ?Sized>(
    rng: &mut R,
    difficulty: u32,
) -> Result<(String, String, String, String, Option<Vec<String>>, Option<String>, Value, String), GenerationFailure> {
    // Generate numbers that specifically trigger base-10 regrouping (carrying)
    let (min, max) = match difficulty {
        1 => (15, 89),
        2 => (125, 889),
        _ => (1250, 8899),
    };

    for _ in 0..128 {
        let a = uniform_inclusive(rng, min, max);
        let b = uniform_inclusive(rng, min, max);
        // Ensure units digit triggers a carry
        if (a % 10) + (b % 10) >= 10 {
            let sum = a + b;
            let prompt_text = format!("Calculate the sum: {} + {}.", a, b);
            let prompt_latex = format!("\\begin{{array}}{{r@{{\\quad}}l}} & {} \\\\[-2pt] + & {} \\\\ \\hline & ? \\end{{array}}", a, b);
            let prompt_speech = format!("Calculate {} plus {} with regrouping.", a, b);

            let metadata = serde_json::json!({
                "operand1": a.to_string(),
                "operand2": b.to_string(),
                "carry_units": true,
            });

            return Ok((
                prompt_text,
                prompt_latex,
                prompt_speech,
                "integer".into(),
                None,
                Some(sum.to_string()),
                metadata,
                "integer-exact-v1".into(),
            ));
        }
    }

    Err(GenerationFailure::ConstraintExhaustion { draws: 128 })
}

/// Template: `arithmetic.column.subtraction`
fn generate_column_subtraction<R: DrawSource + ?Sized>(
    rng: &mut R,
    difficulty: u32,
) -> Result<(String, String, String, String, Option<Vec<String>>, Option<String>, Value, String), GenerationFailure> {
    // Generate numbers that specifically trigger base-10 decomposition (borrowing)
    let (min, max) = match difficulty {
        1 => (25, 95),
        2 => (225, 985),
        _ => (2250, 9850),
    };

    for _ in 0..128 {
        let mut a = uniform_inclusive(rng, min, max);
        let mut b = uniform_inclusive(rng, min / 2, max - 10);
        if a < b {
            std::mem::swap(&mut a, &mut b);
        }
        // Ensure units digit triggers a borrow
        if (a % 10) < (b % 10) {
            let diff = a - b;
            let prompt_text = format!("Calculate the difference: {} - {}.", a, b);
            let prompt_latex = format!("\\begin{{array}}{{r@{{\\quad}}l}} & {} \\\\[-2pt] - & {} \\\\ \\hline & ? \\end{{array}}", a, b);
            let prompt_speech = format!("Calculate {} minus {} with borrowing.", a, b);

            let metadata = serde_json::json!({
                "minuend": a.to_string(),
                "subtrahend": b.to_string(),
                "borrow_units": true,
            });

            return Ok((
                prompt_text,
                prompt_latex,
                prompt_speech,
                "integer".into(),
                None,
                Some(diff.to_string()),
                metadata,
                "integer-exact-v1".into(),
            ));
        }
    }

    Err(GenerationFailure::ConstraintExhaustion { draws: 128 })
}

/// Template: `place_value.decompose`
fn generate_place_value_decompose<R: DrawSource + ?Sized>(
    rng: &mut R,
    difficulty: u32,
) -> Result<(String, String, String, String, Option<Vec<String>>, Option<String>, Value, String), GenerationFailure> {
    let num: i64 = match difficulty {
        1 => uniform_inclusive(rng, 100, 999) as i64,
        2 => uniform_inclusive(rng, 1000, 9999) as i64,
        _ => uniform_inclusive(rng, 10000, 99999) as i64,
    };

    let places: Vec<(&str, i64)> = match difficulty {
        1 => vec![("units", 1), ("tens", 10), ("hundreds", 100)],
        _ => vec![("units", 1), ("tens", 10), ("hundreds", 100), ("thousands", 1000)],
    };

    let idx = uniform_inclusive(rng, 0, (places.len() - 1) as i128) as usize;
    let (place_name, weight) = places[idx];
    let digit = ((num / weight) % 10) as i64;
    let place_value = digit * weight;

    let prompt_text = format!("In the number {}, what is the value of the digit in the {} place?", num, place_name);
    let prompt_latex = format!("\\text{{In the number }} {} \\text{{, what is the value of the {} place?}}", num, place_name);
    let prompt_speech = format!("In the number {}, what is the value of the {} place?", num, place_name);

    let metadata = serde_json::json!({
        "number": num.to_string(),
        "place_name": place_name,
        "digit": digit,
        "place_value": place_value,
    });

    Ok((
        prompt_text,
        prompt_latex,
        prompt_speech,
        "integer".into(),
        None,
        Some(place_value.to_string()),
        metadata,
        "integer-exact-v1".into(),
    ))
}

/// Template: `abacus.target.setting`
fn generate_abacus_target_setting<R: DrawSource + ?Sized>(
    rng: &mut R,
    difficulty: u32,
) -> Result<(String, String, String, String, Option<Vec<String>>, Option<String>, Value, String), GenerationFailure> {
    let (min, max) = match difficulty {
        1 => (1, 9),
        2 => (10, 99),
        _ => (100, 999),
    };
    let val = uniform_inclusive(rng, min, max) as i64;

    let prompt_text = format!("Set the soroban abacus to represent {}.", val);
    let prompt_latex = format!("\\text{{Set Soroban to }}\\; {}", val);
    let prompt_speech = format!("Set the soroban abacus to represent {}.", val);

    let metadata = serde_json::json!({
        "target": val,
    });

    Ok((
        prompt_text,
        prompt_latex,
        prompt_speech,
        "soroban_state".into(),
        None,
        Some(val.to_string()),
        metadata,
        "abacus-state-v1".into(),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_SEED: &str = "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef";
    const MANIFEST_HASH: &str = "0000000000000000000000000000000000000000000000000000000000000000";

    #[test]
    fn deterministic_generation() {
        let p1 = generate("fractions.compare.positive", "1.0.0", TEST_SEED, 1, MANIFEST_HASH).unwrap();
        let p2 = generate("fractions.compare.positive", "1.0.0", TEST_SEED, 1, MANIFEST_HASH).unwrap();
        assert_eq!(p1.problem_hash, p2.problem_hash);
        assert_eq!(p1.prompt_text, p2.prompt_text);
        assert_eq!(p1.target_value, p2.target_value);
    }

    #[test]
    fn abacus_exercise_generation() {
        let p = generate("abacus.read.state", "1.0.0", TEST_SEED, 1, MANIFEST_HASH).unwrap();
        assert_eq!(p.answer_kind, "integer");
        assert!(p.metadata.get("abacus_state").is_some());
    }
}
