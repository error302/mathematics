//! Answer checking registry and grading disposition (Section 13.3 & 13.4).

use crate::exercise::ProblemArtifact;
use crate::number::{from_canonical, to_canonical, Q};
use crate::parse::{evaluate, parse_scalar, MathBudget};
use num_traits::Zero;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Disposition {
    Correct,
    Incorrect,
    Malformed,
    Unsupported,
    Inconclusive,
    PendingReview,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GradeOutcome {
    pub disposition: Disposition,
    pub checker_version: String,
    pub evidence_level: String,
    pub reason_code: String,
    pub feedback: String,
    pub score: u32,
    pub max_score: u32,
}

impl GradeOutcome {
    pub fn correct(checker_version: &str, feedback: impl Into<String>) -> Self {
        GradeOutcome {
            disposition: Disposition::Correct,
            checker_version: checker_version.into(),
            evidence_level: "answer_checked".into(),
            reason_code: "exact_match".into(),
            feedback: feedback.into(),
            score: 1,
            max_score: 1,
        }
    }

    pub fn incorrect(checker_version: &str, reason_code: &str, feedback: impl Into<String>) -> Self {
        GradeOutcome {
            disposition: Disposition::Incorrect,
            checker_version: checker_version.into(),
            evidence_level: "answer_checked".into(),
            reason_code: reason_code.into(),
            feedback: feedback.into(),
            score: 0,
            max_score: 1,
        }
    }

    pub fn malformed(checker_version: &str, reason_code: &str, feedback: impl Into<String>) -> Self {
        GradeOutcome {
            disposition: Disposition::Malformed,
            checker_version: checker_version.into(),
            evidence_level: "none".into(),
            reason_code: reason_code.into(),
            feedback: feedback.into(),
            score: 0,
            max_score: 1,
        }
    }

    pub fn unsupported(checker_version: &str, reason_code: &str, feedback: impl Into<String>) -> Self {
        GradeOutcome {
            disposition: Disposition::Unsupported,
            checker_version: checker_version.into(),
            evidence_level: "none".into(),
            reason_code: reason_code.into(),
            feedback: feedback.into(),
            score: 0,
            max_score: 1,
        }
    }
}

/// Checks an answer string against a problem artifact.
pub fn check(
    problem: &ProblemArtifact,
    raw_answer: &str,
    budget: &MathBudget,
) -> GradeOutcome {
    let checker_version = &problem.replay_identity.checker_version;
    let trimmed = raw_answer.trim();

    if trimmed.is_empty() {
        return GradeOutcome::malformed(
            checker_version,
            "empty_input",
            "Please provide an answer before submitting.",
        );
    }

    match checker_version.as_str() {
        "rational-order-v1" => check_rational_order(problem, trimmed),
        "fraction-form-v1" => check_fraction_form(problem, trimmed, budget),
        "integer-exact-v1" => check_integer_exact(problem, trimmed, budget),
        "rational-equality-v1" => check_rational_equality(problem, trimmed, budget),
        _ => GradeOutcome::unsupported(
            checker_version,
            "unsupported_checker",
            format!("The checker {checker_version} is not implemented in this version."),
        ),
    }
}

/// Checker for relational options: "less", "equal", "greater"
fn check_rational_order(problem: &ProblemArtifact, answer: &str) -> GradeOutcome {
    let target = match &problem.target_value {
        Some(t) => t.to_lowercase(),
        None => {
            return GradeOutcome::unsupported(
                "rational-order-v1",
                "missing_target",
                "Exercise missing target relation value.",
            )
        }
    };

    let norm = answer.to_lowercase();
    let norm = match norm.as_str() {
        "<" | "smaller" | "less" | "less than" => "less",
        "=" | "==" | "equal" | "equals" | "same" => "equal",
        ">" | "larger" | "greater" | "greater than" => "greater",
        _ => {
            return GradeOutcome::malformed(
                "rational-order-v1",
                "invalid_option",
                format!("“{answer}” is not recognized. Please choose 'less', 'equal', or 'greater'."),
            )
        }
    };

    if norm == target {
        let frac1 = problem.metadata.get("frac1").and_then(|v| v.as_str()).unwrap_or("");
        let frac2 = problem.metadata.get("frac2").and_then(|v| v.as_str()).unwrap_or("");
        let rel_word = if target == "less" { "less than" } else { "greater than" };
        GradeOutcome::correct(
            "rational-order-v1",
            format!("Correct! {frac1} is {rel_word} {frac2}."),
        )
    } else {
        // Provide actionable feedback identifying the common denominator
        let common_denom = problem.metadata.get("common_denom")
            .and_then(|v| v.as_str())
            .unwrap_or("a common denominator");
        let frac1_orig = problem.metadata.get("frac1_orig")
            .and_then(|v| v.as_str())
            .unwrap_or("");
        let frac2_orig = problem.metadata.get("frac2_orig")
            .and_then(|v| v.as_str())
            .unwrap_or("");

        let msg = if let (Some(_f1), Some(_f2)) = (
            problem.metadata.get("frac1").and_then(|v| v.as_str()).and_then(from_canonical),
            problem.metadata.get("frac2").and_then(|v| v.as_str()).and_then(from_canonical),
        ) {
            format!(
                "Not quite. Hint: Convert both fractions to have common denominator {common_denom}. Compare their numerators: {frac1_orig} vs {frac2_orig}."
            )
        } else {
            "Not quite. Remember to find a common denominator before comparing numerators.".into()
        };

        GradeOutcome::incorrect("rational-order-v1", "wrong_relation", msg)
    }
}

/// Checks that the answer is a fraction in the specific target denominator form.
fn check_fraction_form(problem: &ProblemArtifact, raw_answer: &str, budget: &MathBudget) -> GradeOutcome {
    let ast = match parse_scalar(raw_answer, budget) {
        Ok(a) => a,
        Err(e) => {
            return if e.budget {
                GradeOutcome::unsupported("fraction-form-v1", &e.code, e.message)
            } else {
                GradeOutcome::malformed("fraction-form-v1", &e.code, e.message)
            };
        }
    };

    let target_denom = problem.metadata.get("target_denominator")
        .and_then(|v| v.as_i64())
        .unwrap_or(0);
    let target_num = problem.metadata.get("target_numerator")
        .and_then(|v| v.as_i64())
        .unwrap_or(0);

    // Form check: must be a simple fraction with the requested denominator
    if let Some((n, d)) = ast.as_simple_fraction() {
        if d == num_bigint::BigInt::from(target_denom) {
            if n == num_bigint::BigInt::from(target_num) {
                GradeOutcome::correct(
                    "fraction-form-v1",
                    format!("Excellent! {n}/{d} is the exact equivalent fraction."),
                )
            } else {
                GradeOutcome::incorrect(
                    "fraction-form-v1",
                    "numerator_mismatch",
                    format!("The denominator is correct ({d}), but {n} is not the right numerator. Check what factor scaled the denominator."),
                )
            }
        } else {
            GradeOutcome::incorrect(
                "fraction-form-v1",
                "wrong_denominator",
                format!("The problem asked for denominator {target_denom}, but you provided denominator {d}."),
            )
        }
    } else {
        GradeOutcome::malformed(
            "fraction-form-v1",
            "not_a_fraction",
            "Please write your answer as a fraction in the form a/b.",
        )
    }
}

/// Checks that an answer evaluates to the exact integer target.
fn check_integer_exact(problem: &ProblemArtifact, raw_answer: &str, budget: &MathBudget) -> GradeOutcome {
    let ast = match parse_scalar(raw_answer, budget) {
        Ok(a) => a,
        Err(e) => {
            return if e.budget {
                GradeOutcome::unsupported("integer-exact-v1", &e.code, e.message)
            } else {
                GradeOutcome::malformed("integer-exact-v1", &e.code, e.message)
            };
        }
    };

    let val = match evaluate(&ast, budget) {
        Ok(v) => v,
        Err(e) => {
            return if e.budget {
                GradeOutcome::unsupported("integer-exact-v1", &e.code, e.message)
            } else {
                GradeOutcome::malformed("integer-exact-v1", &e.code, e.message)
            };
        }
    };

    let target_str = match &problem.target_value {
        Some(t) => t,
        None => return GradeOutcome::unsupported("integer-exact-v1", "missing_target", "Missing target."),
    };

    let target_q = match from_canonical(target_str) {
        Some(q) => q,
        None => return GradeOutcome::unsupported("integer-exact-v1", "bad_target", "Corrupt target."),
    };

    if val == target_q {
        GradeOutcome::correct(
            "integer-exact-v1",
            format!("Correct! The value is {}.", to_canonical(&val)),
        )
    } else {
        GradeOutcome::incorrect(
            "integer-exact-v1",
            "value_mismatch",
            format!("Your answer evaluates to {}, which is not equal to the expected result.", to_canonical(&val)),
        )
    }
}

/// Checks exact rational equality.
fn check_rational_equality(problem: &ProblemArtifact, raw_answer: &str, budget: &MathBudget) -> GradeOutcome {
    let ast = match parse_scalar(raw_answer, budget) {
        Ok(a) => a,
        Err(e) => {
            return if e.budget {
                GradeOutcome::unsupported("rational-equality-v1", &e.code, e.message)
            } else {
                GradeOutcome::malformed("rational-equality-v1", &e.code, e.message)
            };
        }
    };

    let val = match evaluate(&ast, budget) {
        Ok(v) => v,
        Err(e) => {
            return if e.budget {
                GradeOutcome::unsupported("rational-equality-v1", &e.code, e.message)
            } else {
                GradeOutcome::malformed("rational-equality-v1", &e.code, e.message)
            };
        }
    };

    let target_q = problem.target_value.as_ref()
        .and_then(|s| from_canonical(s))
        .unwrap_or(Q::zero());

    if val == target_q {
        GradeOutcome::correct(
            "rational-equality-v1",
            "Correct! Your answer equals the required value.",
        )
    } else {
        GradeOutcome::incorrect(
            "rational-equality-v1",
            "fraction_unequal",
            format!("Your answer {} does not equal the target value {}.", to_canonical(&val), to_canonical(&target_q)),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::exercise::generate;

    const SEED: &str = "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef";
    const MANIFEST_HASH: &str = "0000000000000000000000000000000000000000000000000000000000000000";

    #[test]
    fn check_fraction_compare() {
        let p = generate("fractions.compare.positive", "1.0.0", SEED, 1, MANIFEST_HASH).unwrap();
        let target = p.target_value.as_ref().unwrap();

        let budget = MathBudget::default();
        let outcome = check(&p, target, &budget);
        assert_eq!(outcome.disposition, Disposition::Correct);

        let wrong = if target == "less" { "greater" } else { "less" };
        let outcome_wrong = check(&p, wrong, &budget);
        assert_eq!(outcome_wrong.disposition, Disposition::Incorrect);
        assert!(outcome_wrong.feedback.contains("common denominator"));
    }

    #[test]
    fn check_malformed_input() {
        let p = generate("arithmetic.whole.addition", "1.0.0", SEED, 1, MANIFEST_HASH).unwrap();
        let budget = MathBudget::default();
        let outcome = check(&p, "1 2 3", &budget);
        assert_eq!(outcome.disposition, Disposition::Malformed);
    }
}
