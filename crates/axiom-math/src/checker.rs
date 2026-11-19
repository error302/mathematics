//! Answer checking registry and grading disposition (Section 13.3 & 13.4).

use crate::exercise::ProblemArtifact;
use crate::number::{from_canonical, to_canonical};
use crate::parse::{evaluate, parse_scalar, MathBudget};
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
    if raw_answer.len() > budget.max_input_bytes {
        return GradeOutcome::unsupported(checker_version, "input_budget", "This answer exceeds the supported input size. Your mathematical correctness has not been decided.");
    }
    let trimmed = raw_answer.trim();

    if trimmed.is_empty() {
        return GradeOutcome::malformed(
            checker_version,
            "empty_input",
            "Please provide an answer before submitting.",
        );
    }

    match checker_version.as_str() {
        "academy-challenge-v1" => crate::challenges::check(problem, trimmed, budget),
        "rational-order-v1" => check_rational_order(problem, trimmed),
        "integer-order-v1" | "integer-order-v2" => check_integer_order(problem, trimmed),
        "fraction-form-v1" | "fraction-form-v2" => check_fraction_form(problem, trimmed, budget),
        "integer-exact-v1" | "integer-exact-v2" => check_integer_exact(problem, trimmed, budget),
        "rational-equality-v1" | "rational-equality-v2" => check_rational_equality(problem, trimmed, budget),
        "abacus-state-v1" | "abacus-state-v2" => check_abacus_state(problem, trimmed, budget),
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

/// Checker for integer ordering options: "less", "equal", "greater"
fn check_integer_order(problem: &ProblemArtifact, answer: &str) -> GradeOutcome {
    let target = match &problem.target_value {
        Some(t) => t.to_lowercase(),
        None => {
            return GradeOutcome::unsupported(
                "integer-order-v2",
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
                "integer-order-v2",
                "invalid_option",
                format!("“{answer}” is not recognized. Please choose 'less', 'equal', or 'greater'."),
            )
        }
    };

    let a_str = problem.metadata.get("a").and_then(|v| v.as_i64()).map(|v| v.to_string()).unwrap_or_else(|| "?".into());
    let b_str = problem.metadata.get("b").and_then(|v| v.as_i64()).map(|v| v.to_string()).unwrap_or_else(|| "?".into());

    if norm == target {
        let rel_word = match target.as_str() {
            "less" => "less than",
            "greater" => "greater than",
            _ => "equal to",
        };
        GradeOutcome::correct(
            "integer-order-v2",
            format!("Correct! {a_str} is {rel_word} {b_str}."),
        )
    } else {
        let rel_word = match target.as_str() {
            "less" => "less than",
            "greater" => "greater than",
            _ => "equal to",
        };
        let feedback = format!(
            "Not quite. On the signed number line, numbers further to the right are greater. {a_str} is to the {} of {b_str}, so {a_str} is {rel_word} {b_str}.",
            if target == "less" { "left" } else if target == "greater" { "right" } else { "same position as" }
        );
        GradeOutcome::incorrect("integer-order-v2", "wrong_relation", feedback)
    }
}

/// Checks that the answer is a fraction in the specific target denominator form.
fn check_fraction_form(problem: &ProblemArtifact, raw_answer: &str, budget: &MathBudget) -> GradeOutcome {
    let ast = match parse_scalar(raw_answer, budget) {
        Ok(a) => a,
        Err(e) => {
            return if e.budget || e.code == "non_integer_exponent" {
                GradeOutcome::unsupported("fraction-form-v2", &e.code, e.message)
            } else {
                GradeOutcome::malformed("fraction-form-v2", &e.code, e.message)
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
                    "fraction-form-v2",
                    format!("Excellent! {n}/{d} is the exact equivalent fraction."),
                )
            } else {
                GradeOutcome::incorrect(
                    "fraction-form-v2",
                    "numerator_mismatch",
                    format!("The denominator is correct ({d}), but {n} is not the right numerator. Check what factor scaled the denominator."),
                )
            }
        } else {
            GradeOutcome::incorrect(
                "fraction-form-v2",
                "wrong_denominator",
                format!("The problem asked for denominator {target_denom}, but you provided denominator {d}."),
            )
        }
    } else {
        GradeOutcome::malformed(
            "fraction-form-v2",
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
            return if e.budget || e.code == "non_integer_exponent" {
                GradeOutcome::unsupported("integer-exact-v2", &e.code, e.message)
            } else {
                GradeOutcome::malformed("integer-exact-v2", &e.code, e.message)
            };
        }
    };

    let val = match evaluate(&ast, budget) {
        Ok(v) => v,
        Err(e) => {
            return if e.budget || e.code == "non_integer_exponent" {
                GradeOutcome::unsupported("integer-exact-v2", &e.code, e.message)
            } else {
                GradeOutcome::malformed("integer-exact-v2", &e.code, e.message)
            };
        }
    };

    let target_str = match &problem.target_value {
        Some(t) => t,
        None => return GradeOutcome::unsupported("integer-exact-v2", "missing_target", "Missing target."),
    };

    let target_q = match from_canonical(target_str) {
        Some(q) => q,
        None => return GradeOutcome::unsupported("integer-exact-v2", "bad_target", "Corrupt target."),
    };

    if val == target_q {
        GradeOutcome::correct(
            "integer-exact-v2",
            format!("Correct! The value is {}.", to_canonical(&val)),
        )
    } else {
        GradeOutcome::incorrect(
            "integer-exact-v2",
            "value_mismatch",
            diagnose_obstacle(problem, &to_canonical(&val)).unwrap_or_else(|| format!("Your answer evaluates to {}, which is not equal to the expected result.", to_canonical(&val))),
        )
    }
}

/// Checks exact rational equality.
fn check_rational_equality(problem: &ProblemArtifact, raw_answer: &str, budget: &MathBudget) -> GradeOutcome {
    let ast = match parse_scalar(raw_answer, budget) {
        Ok(a) => a,
        Err(e) => {
            return if e.budget || e.code == "non_integer_exponent" {
                GradeOutcome::unsupported("rational-equality-v2", &e.code, e.message)
            } else {
                GradeOutcome::malformed("rational-equality-v2", &e.code, e.message)
            };
        }
    };

    let val = match evaluate(&ast, budget) {
        Ok(v) => v,
        Err(e) => {
            return if e.budget || e.code == "non_integer_exponent" {
                GradeOutcome::unsupported("rational-equality-v2", &e.code, e.message)
            } else {
                GradeOutcome::malformed("rational-equality-v2", &e.code, e.message)
            };
        }
    };

    let target_q = match problem.target_value.as_ref().and_then(|s| from_canonical(s)) {
        Some(value) => value,
        None => return GradeOutcome::unsupported("rational-equality-v2", "missing_or_invalid_target", "This exercise has no valid reference value. Your answer has not been marked incorrect."),
    };

    if val == target_q {
        GradeOutcome::correct(
            "rational-equality-v2",
            "Correct! Your answer equals the required value.",
        )
    } else {
        GradeOutcome::incorrect(
            "rational-equality-v2",
            "fraction_unequal",
            diagnose_obstacle(problem, &to_canonical(&val)).unwrap_or_else(|| format!("Your answer {} does not equal the target value {}.", to_canonical(&val), to_canonical(&target_q))),
        )
    }
}

/// Checks Soroban abacus state submissions.
fn check_abacus_state(problem: &ProblemArtifact, raw_answer: &str, budget: &MathBudget) -> GradeOutcome {
    if raw_answer.trim_start().starts_with('{') {
        if let Ok(state) = serde_json::from_str::<crate::abacus::AbacusState>(raw_answer) {
            if let Err(error) = state.validate() {
                return GradeOutcome::malformed("abacus-state-v2", "invalid_abacus_state", error.to_string());
            }
            let val = state.compute_value();
            let target_str = match &problem.target_value {
                Some(t) => t,
                None => return GradeOutcome::unsupported("abacus-state-v2", "missing_target", "Missing target."),
            };
            if let Ok(target_int) = target_str.parse::<i64>() {
                if val == crate::number::int(target_int) {
                    return GradeOutcome::correct("abacus-state-v2", format!("Correct! The Soroban rods correctly represent {target_int}."));
                } else {
                    return GradeOutcome::incorrect("abacus-state-v2", "state_value_mismatch", format!("The abacus rods currently display {}, but the target is {target_int}.", crate::number::to_canonical(&val)));
                }
            }
        }
    }
    if raw_answer.trim_start().starts_with('{') {
        return GradeOutcome::malformed("abacus-state-v2", "invalid_abacus_json", "The abacus state could not be read. Check the supported rod controls.");
    }
    // An accessible numerical equivalent demonstrates arithmetic, not bead manipulation.
    let mut outcome = check_integer_exact(problem, raw_answer, budget);
    if outcome.disposition == Disposition::Correct {
        outcome.evidence_level = "numeric_equivalent".into();
        outcome.feedback = "Correct numerical value. This checks arithmetic knowledge; it does not certify bead manipulation or physical technique.".into();
    }
    outcome
}

fn meta_matches(problem: &ProblemArtifact, key: &str, val_canon: &str) -> bool {
    match problem.metadata.get(key) {
        Some(serde_json::Value::Number(n)) => n.to_string() == val_canon,
        Some(serde_json::Value::String(s)) => s == val_canon,
        _ => false,
    }
}

fn diagnose_obstacle(problem: &ProblemArtifact, val_canon: &str) -> Option<String> {
    let tmpl = problem.replay_identity.template_id.as_str();
    match tmpl {
        "data.mean_median_calc" => {
            if meta_matches(problem, "unsorted_middle", val_canon) {
                return Some(format!("Your answer evaluates to {val_canon}. Obstacle identified: You picked the middle value of the unsorted list. Sort all values from least to greatest before selecting the median."));
            }
            if meta_matches(problem, "sum", val_canon) {
                return Some(format!("Your answer evaluates to {val_canon}. Obstacle identified: You computed the total sum, but still need to divide by the number of data points (5) to find the mean."));
            }
        }
        "counting.permutations_combinations" => {
            if meta_matches(problem, "additive_trap", val_canon) {
                return Some(format!("Your answer evaluates to {val_canon}. Obstacle identified: You added the choices instead of multiplying them. Independent multi-stage choices combine by multiplication."));
            }
            if meta_matches(problem, "with_replacement_trap", val_canon) {
                return Some(format!("Your answer evaluates to {val_canon}. Obstacle identified: A single competitor cannot hold two podium spots at once. Subtract 1 from the available choices at each step: n × (n − 1) × (n − 2)."));
            }
            if meta_matches(problem, "permutation_trap", val_canon) {
                return Some(format!("Your answer evaluates to {val_canon}. Obstacle identified: You counted ordered permutations, but committee membership is unordered. Divide by k! to eliminate duplicate orderings."));
            }
        }
        "probability.simple_exact" => {
            if meta_matches(problem, "odds_trap", val_canon) {
                return Some(format!("Your answer evaluates to {val_canon}. Obstacle identified: The denominator of a probability is the total sample space (all tokens), not just the non-favorable tokens."));
            }
            if meta_matches(problem, "direct_trap", val_canon) {
                return Some(format!("Your answer evaluates to {val_canon}. Obstacle identified: That is P(E), the probability that E occurs. For P(not E), subtract from 1: P(not E) = 1 − P(E)."));
            }
            if meta_matches(problem, "with_replacement_trap", val_canon) {
                return Some(format!("Your answer evaluates to {val_canon}. Obstacle identified: Because the draw is WITHOUT replacement, both the remaining gold coins and the total remaining coins decrease by 1 on the second draw."));
            }
        }
        "algebra.expression_eval" => {
            if meta_matches(problem, "sign_trap", val_canon) {
                return Some(format!("Your answer evaluates to {val_canon}. Obstacle identified: Check negative sign rules—squaring a negative number is positive ((−x)² > 0), and subtracting a negative term adds its value."));
            }
        }
        "algebra.linear_equation_solve" => {
            if meta_matches(problem, "undist_trap", val_canon) {
                return Some(format!("Your answer evaluates to {val_canon}. Obstacle identified: Distribute the outer factor across BOTH terms inside the parentheses: k(ax + b) = kax + kb."));
            }
        }
        "algebra.linear_system_solve" => {
            if meta_matches(problem, "y_trap", val_canon) {
                return Some(format!("Your answer evaluates to {val_canon}. Obstacle identified: That value is y, whereas the problem asks for x. Substitute y back into one of the equations to find x."));
            }
        }
        "powers.integer_eval" => {
            let kind = problem.metadata.get("kind").and_then(|v| v.as_str()).unwrap_or("");
            if kind == "zero_power" && val_canon == "0" {
                return Some("Your answer evaluates to 0. Obstacle identified: Any non-zero base raised to the 0 power equals 1 (the multiplicative identity), not 0.".into());
            }
            if kind == "negative_power" && val_canon.starts_with('-') {
                return Some(format!("Your answer evaluates to {val_canon}. Obstacle identified: A negative exponent denotes the multiplicative reciprocal (1 / b^n), which is positive, not a negative number."));
            }
        }
        "functions.evaluate_and_range" => {
            if meta_matches(problem, "direct_eval_trap", val_canon) {
                return Some(format!("Your answer evaluates to {val_canon}. Obstacle identified: You computed f(y₀) by substituting y₀ for x. Instead, set the formula f(x) equal to y₀ and solve for the input x."));
            }
            if meta_matches(problem, "wrong_branch_trap", val_canon) {
                return Some(format!("Your answer evaluates to {val_canon}. Obstacle identified: You evaluated the wrong piecewise branch. Check whether the input x satisfies x < c or x ≥ c first."));
            }
            if meta_matches(problem, "unscaled_diff_trap", val_canon) {
                return Some(format!("Your answer evaluates to {val_canon}. Obstacle identified: That is the total change in output f(x₂) − f(x₁). Divide by the interval width (x₂ − x₁) to find the average rate of change."));
            }
        }
        "functions.composition_inverse" => {
            if meta_matches(problem, "reverse_order_trap", val_canon) {
                return Some(format!("Your answer evaluates to {val_canon}. Obstacle identified: You evaluated g(f(x)) in reverse order. For (f ∘ g)(x) = f(g(x)), evaluate the inner function g(x) first."));
            }
            if meta_matches(problem, "product_trap", val_canon) {
                return Some(format!("Your answer evaluates to {val_canon}. Obstacle identified: Function composition (f ∘ g)(x) means substituting g(x) into f, not multiplying f(x) × g(x)."));
            }
            if meta_matches(problem, "forward_eval_trap", val_canon) {
                return Some(format!("Your answer evaluates to {val_canon}. Obstacle identified: That is the forward value f(y₀). For the inverse f⁻¹(y₀), solve ax + b = y₀ for x."));
            }
            if meta_matches(problem, "reciprocal_trap", val_canon) {
                return Some(format!("Your answer evaluates to {val_canon}. Obstacle identified: The notation f⁻¹(y₀) denotes the inverse function (solving f(x) = y₀), not the multiplicative reciprocal 1 / f(y₀)."));
            }
        }
        "functions.transformations_vertex" => {
            if meta_matches(problem, "horizontal_sign_trap", val_canon) {
                return Some(format!("Your answer evaluates to {val_canon}. Obstacle identified: In vertex form a(x − h)² + k, subtracting h inside the parentheses shifts the vertex to x = +h (where x − h = 0), not −h."));
            }
            if meta_matches(problem, "k_trap", val_canon) {
                return Some(format!("Your answer evaluates to {val_canon}. Obstacle identified: That is k, the y-coordinate of the vertex, whereas the problem asks for the x-coordinate h."));
            }
            if meta_matches(problem, "unscaled_y_trap", val_canon) {
                return Some(format!("Your answer evaluates to {val_canon}. Obstacle identified: Remember to multiply the original output f(x₀) by the vertical stretch factor a before adding the vertical shift k."));
            }
        }
        "algebra2.polynomial_factor_roots" => {
            if meta_matches(problem, "smaller_root_trap", val_canon) {
                return Some(format!("Your answer evaluates to {val_canon}. Obstacle identified: That is the smaller root of the quadratic, whereas the problem asks for the LARGER root."));
            }
            if meta_matches(problem, "sign_flip_trap", val_canon) {
                return Some(format!("Your answer evaluates to {val_canon}. Obstacle identified: When a factor is (x − r) = 0, setting it to zero gives the root x = +r, not −r."));
            }
            if meta_matches(problem, "neg_root_trap", val_canon) {
                return Some(format!("Your answer evaluates to {val_canon}. Obstacle identified: By the Remainder Theorem, dividing P(x) by (x − r) gives remainder P(+r), not P(−r)."));
            }
            if meta_matches(problem, "unscaled_vieta_trap", val_canon) {
                return Some(format!("Your answer evaluates to {val_canon}. Obstacle identified: In Vieta's formula r₁ + r₂ = −B/A, remember to divide by the leading coefficient A."));
            }
            if meta_matches(problem, "sign_vieta_trap", val_canon) {
                return Some(format!("Your answer evaluates to {val_canon}. Obstacle identified: Watch the minus sign in Vieta's sum formula: r₁ + r₂ = −B / A."));
            }
        }
        "algebra2.quadratic_discriminant" => {
            if meta_matches(problem, "sign_4ac_trap", val_canon) {
                return Some(format!("Your answer evaluates to {val_canon}. Obstacle identified: Check the sign of −4ac in Δ = b² − 4ac (subtract 4ac when a and c have the same sign; add when c is negative)."));
            }
            if meta_matches(problem, "forgot_2a_trap", val_canon) {
                return Some(format!("Your answer evaluates to {val_canon}. Obstacle identified: In the quadratic formula x = (−b ± √Δ) / (2a), divide by the entire denominator 2a."));
            }
            if meta_matches(problem, "smaller_root_trap", val_canon) {
                return Some(format!("Your answer evaluates to {val_canon}. Obstacle identified: That is the negative root, whereas the problem asks for the POSITIVE root."));
            }
            if meta_matches(problem, "forgot_4_trap", val_canon) {
                return Some(format!("Your answer evaluates to {val_canon}. Obstacle identified: Setting Δ = b² − 4ac = 0 requires dividing b² by 4a (including the factor of 4)."));
            }
        }
        "algebra2.exp_log_eval" => {
            if meta_matches(problem, "division_trap", val_canon) {
                return Some(format!("Your answer evaluates to {val_canon}. Obstacle identified: A logarithm log_b(N) is the exponent k such that b^k = N, not the quotient N ÷ b."));
            }
            if meta_matches(problem, "sign_log_trap", val_canon) {
                return Some(format!("Your answer evaluates to {val_canon}. Obstacle identified: Check the sign of the exponent—a reciprocal 1 / b^k equals b^(−k), so its base-b logarithm is negative."));
            }
            if meta_matches(problem, "arg_value_trap", val_canon) {
                return Some(format!("Your answer evaluates to {val_canon}. Obstacle identified: That is the simplified argument (A·B)/C inside the logarithm. Now take log_b of that power to find the exponent."));
            }
            if meta_matches(problem, "unscaled_exp_trap", val_canon) {
                return Some(format!("Your answer evaluates to {val_canon}. Obstacle identified: Rewrite the right-hand base as b^m and multiply its entire exponent (dx + e) by m before equating exponents."));
            }
        }
        "sec_geom.slope_and_lines" => {
            if meta_matches(problem, "run_over_rise_trap", val_canon) {
                return Some(format!("Your answer evaluates to {val_canon}. Obstacle identified: Slope is vertical change over horizontal change (rise / run = (y₂ − y₁)/(x₂ − x₁)), not run over rise."));
            }
            if meta_matches(problem, "sign_order_trap", val_canon) {
                return Some(format!("Your answer evaluates to {val_canon}. Obstacle identified: Subtract the coordinates in the same order in both numerator and denominator: (y₂ − y₁)/(x₂ − x₁)."));
            }
            if meta_matches(problem, "parallel_slope_trap", val_canon) {
                return Some(format!("Your answer evaluates to {val_canon}. Obstacle identified: That uses the parallel slope m₁ instead of the perpendicular negative-reciprocal slope m_⊥ = −1/m₁."));
            }
            if meta_matches(problem, "positive_reciprocal_trap", val_canon) {
                return Some(format!("Your answer evaluates to {val_canon}. Obstacle identified: Perpendicular lines have negative-reciprocal slopes (m_⊥ = −q/p, not +q/p)."));
            }
            if meta_matches(problem, "unswapped_weights_trap", val_canon) {
                return Some(format!("Your answer evaluates to {val_canon}. Obstacle identified: In the section formula x_P = (n x₁ + m x₂)/(m + n), multiply A's coordinate x₁ by n and B's coordinate x₂ by m."));
            }
            if meta_matches(problem, "midpoint_trap", val_canon) {
                return Some(format!("Your answer evaluates to {val_canon}. Obstacle identified: That is the midpoint (1:1 ratio); weight the endpoints by the given ratio m : n."));
            }
        }
        "sec_geom.circle_equations_theorems" => {
            if meta_matches(problem, "central_equal_trap", val_canon) {
                return Some(format!("Your answer evaluates to {val_canon}. Obstacle identified: An inscribed angle is half of the central angle subtending the same arc, not equal to it."));
            }
            if meta_matches(problem, "doubled_central_trap", val_canon) {
                return Some(format!("Your answer evaluates to {val_canon}. Obstacle identified: Divide the central angle by 2 to get the inscribed angle (do not double it)."));
            }
            if meta_matches(problem, "forgot_complete_square_trap", val_canon) {
                return Some(format!("Your answer evaluates to {val_canon}. Obstacle identified: When completing the square (x − h)² + (y − k)² = R², remember to add h² + k² to the right-hand side."));
            }
            if meta_matches(problem, "unhalved_coeff_trap", val_canon) {
                return Some(format!("Your answer evaluates to {val_canon}. Obstacle identified: Halve the linear coefficients before squaring them: add (D/2)² + (E/2)², not D² + E²."));
            }
            if meta_matches(problem, "additive_chord_trap", val_canon) {
                return Some(format!("Your answer evaluates to {val_canon}. Obstacle identified: The Intersecting Chords Theorem equates the products of segments (PA · PB = PC · PD), not their sums."));
            }
            if meta_matches(problem, "inverted_ratio_trap", val_canon) {
                return Some(format!("Your answer evaluates to {val_canon}. Obstacle identified: From PA · PB = PC · PD, solve for PD by dividing PA · PB by PC."));
            }
        }
        "sec_geom.planar_vectors_dot" => {
            if meta_matches(problem, "cross_product_trap", val_canon) {
                return Some(format!("Your answer evaluates to {val_canon}. Obstacle identified: The dot product multiplies matching components (u₁v₁ + u₂v₂), not cross-components."));
            }
            if meta_matches(problem, "subtract_components_trap", val_canon) {
                return Some(format!("Your answer evaluates to {val_canon}. Obstacle identified: Add the component products in the dot product: u · v = u₁v₁ + u₂v₂."));
            }
            if meta_matches(problem, "sign_orthogonal_trap", val_canon) {
                return Some(format!("Your answer evaluates to {val_canon}. Obstacle identified: Solving a·c + b·k = 0 for k requires subtracting a·c, giving k = −(a·c)/b."));
            }
            if meta_matches(problem, "parallel_proportion_trap", val_canon) {
                return Some(format!("Your answer evaluates to {val_canon}. Obstacle identified: Setting components proportional makes vectors parallel; perpendicularity requires u · v = a·c + b·k = 0."));
            }
            if meta_matches(problem, "norm_squared_trap", val_canon) {
                return Some(format!("Your answer evaluates to {val_canon}. Obstacle identified: Scalar projection comp_v(u) divides u · v by ‖v‖, not by ‖v‖²."));
            }
            if meta_matches(problem, "dot_only_trap", val_canon) {
                return Some(format!("Your answer evaluates to {val_canon}. Obstacle identified: That is the dot product u · v; divide by the magnitude ‖v‖ to obtain the scalar projection."));
            }
        }
        "trig.radians_unit_circle" => {
            if meta_matches(problem, "div_360_trap", val_canon) {
                return Some(format!("Your answer evaluates to {val_canon}. Obstacle identified: Since π radians = 180° (and 2π = 360°), divide the degree measure by 180, not 360."));
            }
            if meta_matches(problem, "inverted_rad_trap", val_canon) {
                return Some(format!("Your answer evaluates to {val_canon}. Obstacle identified: Multiply degrees by π/180 (so k = deg / 180, not 180 / deg)."));
            }
            if meta_matches(problem, "positive_quadrant_trap", val_canon) {
                return Some(format!("Your answer evaluates to {val_canon}. Obstacle identified: In Quadrant II (π/2 < θ < π), the x-coordinate cos(θ) is negative."));
            }
            if meta_matches(problem, "linear_complement_trap", val_canon) {
                return Some(format!("Your answer evaluates to {val_canon}. Obstacle identified: Use the Pythagorean identity sin²(θ) + cos²(θ) = 1 rather than 1 − sin(θ)."));
            }
            if meta_matches(problem, "half_period_trap", val_canon) {
                return Some(format!("Your answer evaluates to {val_canon}. Obstacle identified: One full period of cosine is 2π, so T = 2π / (ωπ) = 2/ω, not 1/ω."));
            }
            if meta_matches(problem, "freq_instead_of_period_trap", val_canon) {
                return Some(format!("Your answer evaluates to {val_canon}. Obstacle identified: That is the frequency factor ω; the period is T = 2π / (ωπ) = 2/ω."));
            }
        }
        "trig.identities_double_angle" => {
            if meta_matches(problem, "sine_instead_of_tan_trap", val_canon) {
                return Some(format!("Your answer evaluates to {val_canon}. Obstacle identified: That is sin(θ) = a/c; divide by cos(θ) = b/c to get tan(θ) = a/b."));
            }
            if meta_matches(problem, "cotangent_trap", val_canon) {
                return Some(format!("Your answer evaluates to {val_canon}. Obstacle identified: tan(θ) is sin(θ)/cos(θ), whereas cos(θ)/sin(θ) is cot(θ)."));
            }
            if meta_matches(problem, "linear_double_sine_trap", val_canon) {
                return Some(format!("Your answer evaluates to {val_canon}. Obstacle identified: Sine is not a linear function—use sin(2θ) = 2 sin(θ) cos(θ), not 2 sin(θ)."));
            }
            if meta_matches(problem, "product_without_two_trap", val_canon) {
                return Some(format!("Your answer evaluates to {val_canon}. Obstacle identified: Include the factor of 2 in the double-angle identity sin(2θ) = 2 sin(θ) cos(θ)."));
            }
            if meta_matches(problem, "angle_diff_sign_trap", val_canon) {
                return Some(format!("Your answer evaluates to {val_canon}. Obstacle identified: For a sum of angles, cosine subtracts the sine product: cos(α + β) = cos(α)cos(β) − sin(α)sin(β)."));
            }
            if meta_matches(problem, "add_cosines_trap", val_canon) {
                return Some(format!("Your answer evaluates to {val_canon}. Obstacle identified: cos(α + β) does not equal cos(α) + cos(β); apply cos(α)cos(β) − sin(α)sin(β)."));
            }
        }
        "trig.law_of_sines_cosines" => {
            if meta_matches(problem, "unfolded_branch_trap", val_canon) {
                return Some(format!("Your answer evaluates to {val_canon}. Obstacle identified: That angle lies outside the principal range [−π/2, π/2] of arcsin; use the Quadrant I reference angle π − θ."));
            }
            if meta_matches(problem, "negative_branch_trap", val_canon) {
                return Some(format!("Your answer evaluates to {val_canon}. Obstacle identified: Since sin(mπ/n) > 0 in Quadrant II, its arcsin lies in (0, π/2] and is positive."));
            }
            if meta_matches(problem, "inverted_sine_law_trap", val_canon) {
                return Some(format!("Your answer evaluates to {val_canon}. Obstacle identified: From a / sin(A) = b / sin(B), multiply a by sin(B) and divide by sin(A)."));
            }
            if meta_matches(problem, "unscaled_sine_trap", val_canon) {
                return Some(format!("Your answer evaluates to {val_canon}. Obstacle identified: Multiply the ratio sin(B)/sin(A) by side length a."));
            }
            if meta_matches(problem, "sign_cosine_law_trap", val_canon) {
                return Some(format!("Your answer evaluates to {val_canon}. Obstacle identified: Rearranging c² = a² + b² − 2ab cos(C) gives cos(C) = (a² + b² − c²) / (2ab)."));
            }
            if meta_matches(problem, "forgot_two_denom_trap", val_canon) {
                return Some(format!("Your answer evaluates to {val_canon}. Obstacle identified: Divide by the full coefficient 2ab in the denominator of the Law of Cosines."));
            }
        }
        "sequences.arithmetic_progression" => {
            if meta_matches(problem, "n_steps_trap", val_canon) {
                return Some(format!("Your answer evaluates to {val_canon}. Obstacle identified: Moving from a₁ to a_n takes (n − 1) steps of d, not n steps: use a_n = a₁ + (n − 1)d."));
            }
            if meta_matches(problem, "mult_trap", val_canon) {
                return Some(format!("Your answer evaluates to {val_canon}. Obstacle identified: Multiply the step count (n − 1) by the common difference d, not by the first term a₁."));
            }
            if meta_matches(problem, "unhalved_sum_trap", val_canon) {
                return Some(format!("Your answer evaluates to {val_canon}. Obstacle identified: Remember to divide by 2 in Gauss's arithmetic series formula S_n = n(a₁ + a_n) / 2."));
            }
            if meta_matches(problem, "nth_times_n_trap", val_canon) {
                return Some(format!("Your answer evaluates to {val_canon}. Obstacle identified: Average the first and last terms (a₁ + a_n)/2 before multiplying by n."));
            }
            if meta_matches(problem, "div_by_q_trap", val_canon) {
                return Some(format!("Your answer evaluates to {val_canon}. Obstacle identified: Divide the difference (a_q − a_p) by the index gap (q − p), not by q."));
            }
            if meta_matches(problem, "inclusive_count_trap", val_canon) {
                return Some(format!("Your answer evaluates to {val_canon}. Obstacle identified: Between term p and term q there are (q − p) steps, not (q − p + 1)."));
            }
        }
        "sequences.geometric_and_infinite" => {
            if meta_matches(problem, "exp_n_trap", val_canon) {
                return Some(format!("Your answer evaluates to {val_canon}. Obstacle identified: From g₁ to g_n there are (n − 1) multiplications by r, so g_n = g₁ · r^(n−1), not g₁ · r^n."));
            }
            if meta_matches(problem, "base_mult_trap", val_canon) {
                return Some(format!("Your answer evaluates to {val_canon}. Obstacle identified: By order of operations, raise r to the power (n − 1) before multiplying by g₁."));
            }
            if meta_matches(problem, "forgot_denom_trap", val_canon) {
                return Some(format!("Your answer evaluates to {val_canon}. Obstacle identified: Remember to divide by (r − 1) in S_n = a(r^n − 1) / (r − 1)."));
            }
            if meta_matches(problem, "n_minus_1_trap", val_canon) {
                return Some(format!("Your answer evaluates to {val_canon}. Obstacle identified: For the sum of the first n terms, the exponent in S_n = a(r^n − 1)/(r − 1) is n, not n − 1."));
            }
            if meta_matches(problem, "plus_r_trap", val_canon) {
                return Some(format!("Your answer evaluates to {val_canon}. Obstacle identified: For a positive ratio r, the infinite sum denominator is 1 − r, not 1 + r."));
            }
            if meta_matches(problem, "mult_instead_div_trap", val_canon) {
                return Some(format!("Your answer evaluates to {val_canon}. Obstacle identified: Divide the first term a by (1 − r) rather than multiplying by (1 − r)."));
            }
        }
        "sequences.recursion_binomial" => {
            if meta_matches(problem, "stop_at_a3_trap", val_canon) {
                return Some(format!("Your answer evaluates to {val_canon}. Obstacle identified: That is the 3rd term a₃; apply the recurrence one more time to compute a₄."));
            }
            if meta_matches(problem, "arithmetic_trap", val_canon) {
                return Some(format!("Your answer evaluates to {val_canon}. Obstacle identified: Each step multiplies the previous term by c before adding d."));
            }
            if meta_matches(problem, "binom_only_trap", val_canon) {
                return Some(format!("Your answer evaluates to {val_canon}. Obstacle identified: That is the binomial number C(n, k); multiply it by c^k to get the full coefficient."));
            }
            if meta_matches(problem, "power_only_trap", val_canon) {
                return Some(format!("Your answer evaluates to {val_canon}. Obstacle identified: Multiply c^k by the binomial coefficient C(n, k)."));
            }
            if meta_matches(problem, "forgot_a_pow_trap", val_canon) {
                return Some(format!("Your answer evaluates to {val_canon}. Obstacle identified: In (ax + b)⁴, the x-coefficient a is also raised to the power 2: compute C(4, 2) · a² · b²."));
            }
            if meta_matches(problem, "unexponentiated_ab_trap", val_canon) {
                return Some(format!("Your answer evaluates to {val_canon}. Obstacle identified: Remember to square both a and b in C(4, 2)(ax)²b²."));
            }
        }
        "calculus.limits_rational" => {
            if meta_matches(problem, "zero_num_trap", val_canon) {
                return Some(format!("Your answer evaluates to {val_canon}. Obstacle identified: Direct substitution gives the indeterminate form 0/0; factor and cancel (x − r) first."));
            }
            if meta_matches(problem, "sign_factor_trap", val_canon) {
                return Some(format!("Your answer evaluates to {val_canon}. Obstacle identified: After canceling (x − r), evaluate the remaining factor (x − s) at x = r with the proper sign."));
            }
            if meta_matches(problem, "const_ratio_trap", val_canon) {
                return Some(format!("Your answer evaluates to {val_canon}. Obstacle identified: That is the ratio of constant terms (x → 0); as x → ∞, use the ratio of leading x² coefficients."));
            }
            if meta_matches(problem, "inverted_lead_trap", val_canon) {
                return Some(format!("Your answer evaluates to {val_canon}. Obstacle identified: Divide the numerator's leading coefficient by the denominator's leading coefficient, not the reverse."));
            }
            if meta_matches(problem, "zero_limit_trap", val_canon) {
                return Some(format!("Your answer evaluates to {val_canon}. Obstacle identified: The limit of (x² − a²)/(x − a) as x → a is 2a, not 0."));
            }
            if meta_matches(problem, "unscaled_k_trap", val_canon) {
                return Some(format!("Your answer evaluates to {val_canon}. Obstacle identified: After finding a·k = 2a − b, divide by a to isolate k."));
            }
        }
        "calculus.derivatives_tangent" => {
            if meta_matches(problem, "kept_const_trap", val_canon) {
                return Some(format!("Your answer evaluates to {val_canon}. Obstacle identified: The derivative of a constant term d is 0; do not include d in f'(x)."));
            }
            if meta_matches(problem, "func_eval_trap", val_canon) {
                return Some(format!("Your answer evaluates to {val_canon}. Obstacle identified: That is the function value f(x₀); differentiate using the power rule first to find f'(x₀)."));
            }
            if meta_matches(problem, "slope_only_trap", val_canon) {
                return Some(format!("Your answer evaluates to {val_canon}. Obstacle identified: That is the tangent slope m = f'(x₀); compute the y-intercept B = f(x₀) − m·x₀."));
            }
            if meta_matches(problem, "curve_intercept_trap", val_canon) {
                return Some(format!("Your answer evaluates to {val_canon}. Obstacle identified: That is the parabola's y-intercept f(0); use B = f(x₀) − f'(x₀)x₀ for the tangent line at x₀."));
            }
            if meta_matches(problem, "forgot_two_deriv_trap", val_canon) {
                return Some(format!("Your answer evaluates to {val_canon}. Obstacle identified: By the power rule, d/dx(ax²) = 2ax, so solving 2ax − b = 0 gives x = b / (2a)."));
            }
            if meta_matches(problem, "sign_crit_trap", val_canon) {
                return Some(format!("Your answer evaluates to {val_canon}. Obstacle identified: Adding b to both sides of 2ax − b = 0 gives a positive numerator x = b / (2a)."));
            }
        }
        "calculus.definite_integral_ftc" => {
            if meta_matches(problem, "unhalved_antideriv_trap", val_canon) {
                return Some(format!("Your answer evaluates to {val_canon}. Obstacle identified: Antidifferentiating 2ax divides by the new exponent 2, giving ax², not 2ax²."));
            }
            if meta_matches(problem, "integrand_eval_trap", val_canon) {
                return Some(format!("Your answer evaluates to {val_canon}. Obstacle identified: Find the antiderivative F(x) = ax² + bx before evaluating at x = t."));
            }
            if meta_matches(problem, "forgot_div_3_trap", val_canon) {
                return Some(format!("Your answer evaluates to {val_canon}. Obstacle identified: By the reverse power rule, ∫ ax² dx = (a/3)x³; remember to divide the cubic term by 3."));
            }
            if meta_matches(problem, "forgot_const_antideriv_trap", val_canon) {
                return Some(format!("Your answer evaluates to {val_canon}. Obstacle identified: Antidifferentiating the constant c gives c·x, which evaluates to c·b at the upper limit x = b."));
            }
            if meta_matches(problem, "disp_only_trap", val_canon) {
                return Some(format!("Your answer evaluates to {val_canon}. Obstacle identified: That is the net displacement ∫₀^t v(t) dt; add the initial position s(0) to find s(t)."));
            }
        }
        "proof.sets_and_cardinality" => {
            if meta_matches(problem, "double_count_trap", val_canon) {
                return Some(format!("Your answer evaluates to {val_canon}. Obstacle identified: Adding |A| + |B| double-counts the intersection; subtract |A ∩ B| once."));
            }
            if meta_matches(problem, "add_inter_trap", val_canon) {
                return Some(format!("Your answer evaluates to {val_canon}. Obstacle identified: By Inclusion-Exclusion, subtract |A ∩ B| from |A| + |B| rather than adding it."));
            }
            if meta_matches(problem, "all_subsets_trap", val_canon) {
                return Some(format!("Your answer evaluates to {val_canon}. Obstacle identified: That is the total size 2^n of P(A); subtract 2 to exclude both ∅ and A itself."));
            }
            if meta_matches(problem, "proper_only_trap", val_canon) {
                return Some(format!("Your answer evaluates to {val_canon}. Obstacle identified: 2^n − 1 excludes A, but you must also exclude the empty set ∅ (so 2^n − 2)."));
            }
            if meta_matches(problem, "subtract_triple_trap", val_canon) {
                return Some(format!("Your answer evaluates to {val_canon}. Obstacle identified: In 3-set Inclusion-Exclusion, ADD back the triple intersection |A ∩ B ∩ C|."));
            }
            if meta_matches(problem, "omit_triple_trap", val_canon) {
                return Some(format!("Your answer evaluates to {val_canon}. Obstacle identified: After subtracting the 3 pairwise intersections, add back |A ∩ B ∩ C|."));
            }
        }
        "proof.logic_truth_quantifiers" => {
            if meta_matches(problem, "non_vacuous_only_trap", val_canon) {
                return Some(format!("Your answer evaluates to {val_canon}. Obstacle identified: That counts only elements where P(x) and Q(x) are both true; remember that P(x) ⇒ Q(x) is also vacuously true whenever P(x) is false."));
            }
            if meta_matches(problem, "falsifying_count_trap", val_canon) {
                return Some(format!("Your answer evaluates to {val_canon}. Obstacle identified: That is the number of counterexamples where P(x) ∧ ¬Q(x); subtract that from |D| to count where P(x) ⇒ Q(x) is true."));
            }
            if meta_matches(problem, "strict_inequality_trap", val_canon) {
                return Some(format!("Your answer evaluates to {val_canon}. Obstacle identified: The logical negation of '<' is '≥' (not '>'), so the boundary integer where ax + b = c is already a valid witness."));
            }
            if meta_matches(problem, "unshifted_trap", val_canon) {
                return Some(format!("Your answer evaluates to {val_canon}. Obstacle identified: Subtract b from c before dividing by a when solving ax + b ≥ c."));
            }
            if meta_matches(problem, "minus_one_trap", val_canon) {
                return Some(format!("Your answer evaluates to {val_canon}. Obstacle identified: Account for the free variables not appearing in the implication: there are 2^(n − k − 1) falsifying rows."));
            }
            if meta_matches(problem, "false_rows_trap", val_canon) {
                return Some(format!("Your answer evaluates to {val_canon}. Obstacle identified: That is the number of rows where the implication is FALSE; subtract it from 2^n."));
            }
        }
        "proof.relations_and_bijections" => {
            if meta_matches(problem, "neg_rem_trap", val_canon) {
                return Some(format!("Your answer evaluates to {val_canon}. Obstacle identified: The canonical representative r of [a]_m must be non-negative (0 ≤ r < m); add m to a negative remainder."));
            }
            if meta_matches(problem, "complement_trap", val_canon) {
                return Some(format!("Your answer evaluates to {val_canon}. Obstacle identified: Check the sign when writing a = q·m + r with 0 ≤ r < m."));
            }
            if meta_matches(problem, "all_funcs_trap", val_canon) {
                return Some(format!("Your answer evaluates to {val_canon}. Obstacle identified: That is n^k (all functions A → B); for injective functions, already-used images cannot be reused."));
            }
            if meta_matches(problem, "subsets_trap", val_canon) {
                return Some(format!("Your answer evaluates to {val_canon}. Obstacle identified: That is C(n, k); for functions, the order in which images are assigned to domain elements matters, so multiply by k!."));
            }
            if meta_matches(problem, "unordered_no_refl_trap", val_canon) {
                return Some(format!("Your answer evaluates to {val_canon}. Obstacle identified: Include reflexive pairs (x, x) and both orders (x, y) and (y, x): each class of size s contributes s² ordered pairs."));
            }
            if meta_matches(problem, "full_cartesian_trap", val_canon) {
                return Some(format!("Your answer evaluates to {val_canon}. Obstacle identified: Only pairs within the same equivalence class belong to R, so sum s₁² + s₂² + s₃² rather than (s₁ + s₂ + s₃)²."));
            }
        }
        "proof.direct_and_contrapositive" => {
            if meta_matches(problem, "unfactored_trap", val_canon) {
                return Some(format!("Your answer evaluates to {val_canon}. Obstacle identified: That is n² − 1; divide by 4 to find the integer witness M in n² − 1 = 4M."));
            }
            if meta_matches(problem, "forgot_linear_k_trap", val_canon) {
                return Some(format!("Your answer evaluates to {val_canon}. Obstacle identified: Expanding (2k + 1)² − 1 gives 4k² + 4k = 4(k² + k); do not drop the +k term."));
            }
            if meta_matches(problem, "boundary_k_trap", val_canon) {
                return Some(format!("Your answer evaluates to {val_canon}. Obstacle identified: For integers, the negation of n ≥ k is n ≤ k − 1; evaluate at n = k − 1 rather than n = k."));
            }
            if meta_matches(problem, "sign_b_trap", val_canon) {
                return Some(format!("Your answer evaluates to {val_canon}. Obstacle identified: Add +b after multiplying a(k − 1)."));
            }
            if meta_matches(problem, "max_avoid_trap", val_canon) {
                return Some(format!("Your answer evaluates to {val_canon}. Obstacle identified: m(k − 1) is the maximum number of items that can avoid having k in one box; add 1 to guarantee at least k."));
            }
            if meta_matches(problem, "naive_product_trap", val_canon) {
                return Some(format!("Your answer evaluates to {val_canon}. Obstacle identified: By the Pigeonhole Principle, the threshold is m(k − 1) + 1, not m·k."));
            }
        }
        "proof.mathematical_induction" => {
            if meta_matches(problem, "kth_term_trap", val_canon) {
                return Some(format!("Your answer evaluates to {val_canon}. Obstacle identified: That is the k-th term T_k; the inductive step from k to k + 1 adds T_(k+1) = a(2k + 1)."));
            }
            if meta_matches(problem, "full_sum_trap", val_canon) {
                return Some(format!("Your answer evaluates to {val_canon}. Obstacle identified: That is the total sum S_(k+1); subtract S_k to get the incremental term T_(k+1)."));
            }
            if meta_matches(problem, "linear_sum_trap", val_canon) {
                return Some(format!("Your answer evaluates to {val_canon}. Obstacle identified: That is ∑ i = n(n + 1)/2; for squares ∑ i², use n(n + 1)(2n + 1)/6."));
            }
            if meta_matches(problem, "wrong_factor_trap", val_canon) {
                return Some(format!("Your answer evaluates to {val_canon}. Obstacle identified: The third factor in the sum of squares formula is (2n + 1), not (n + 2)."));
            }
            if meta_matches(problem, "forgot_n_num_trap", val_canon) {
                return Some(format!("Your answer evaluates to {val_canon}. Obstacle identified: The telescoping sum 1 − 1/(n + 1) simplifies to n/(n + 1); include the factor n in the numerator."));
            }
            if meta_matches(problem, "inverted_trap", val_canon) {
                return Some(format!("Your answer evaluates to {val_canon}. Obstacle identified: The closed form is (a·n)/(n + 1), with n in the numerator and (n + 1) in the denominator."));
            }
        }
        "proof.counterexamples_and_extremal" => {
            if meta_matches(problem, "obvious_p_trap", val_canon) {
                return Some(format!("Your answer evaluates to {val_canon}. Obstacle identified: While n = p gives a composite value, n = p − 1 is a SMALLER positive counterexample where P(p − 1) = p²."));
            }
            if meta_matches(problem, "composite_val_trap", val_canon) {
                return Some(format!("Your answer evaluates to {val_canon}. Obstacle identified: That is the composite output P(n) = p²; report the input counterexample n = p − 1."));
            }
            if meta_matches(problem, "trunc_zero_trap", val_canon) {
                return Some(format!("Your answer evaluates to {val_canon}. Obstacle identified: Truncating toward zero leaves a negative remainder; step the quotient down by 1 so 0 ≤ r < b."));
            }
            if meta_matches(problem, "pos_q_trap", val_canon) {
                return Some(format!("Your answer evaluates to {val_canon}. Obstacle identified: Because the dividend a is negative and divisor b > 0, the quotient q is negative."));
            }
            if meta_matches(problem, "sign_x_trap", val_canon) {
                return Some(format!("Your answer evaluates to {val_canon}. Obstacle identified: Check the sign of x when solving ax + by = rhs."));
            }
            if meta_matches(problem, "unscaled_trap", val_canon) {
                return Some(format!("Your answer evaluates to {val_canon}. Obstacle identified: Divide (rhs − by) by the coefficient a to isolate x."));
            }
        }
        "proof.definitions_and_biconditionals" => {
            if meta_matches(problem, "unscaled_sum_trap", val_canon) {
                return Some(format!("Your answer evaluates to {val_canon}. Obstacle identified: Multiply p by 2^(n − m) to convert p/2^m to denominator 2^n before adding q."));
            }
            if meta_matches(problem, "full_pow_trap", val_canon) {
                return Some(format!("Your answer evaluates to {val_canon}. Obstacle identified: Multiply p by 2^(n − m), not by 2^n."));
            }
            if meta_matches(problem, "suff_only_trap", val_canon) {
                return Some(format!("Your answer evaluates to {val_canon}. Obstacle identified: That counts elements satisfying the sufficient condition S(x); subtract that from the elements satisfying N(x)."));
            }
            if meta_matches(problem, "nec_only_trap", val_canon) {
                return Some(format!("Your answer evaluates to {val_canon}. Obstacle identified: Subtract the elements that also satisfy S(x) to isolate the counterexamples where N(x) holds but S(x) fails."));
            }
            if meta_matches(problem, "additive_shift_trap", val_canon) {
                return Some(format!("Your answer evaluates to {val_canon}. Obstacle identified: Scale multiplicatively so both equations have the same root x = b/a: d = c(b/a)."));
            }
            if meta_matches(problem, "inverted_ratio_trap", val_canon) {
                return Some(format!("Your answer evaluates to {val_canon}. Obstacle identified: From ax = b we have x = b/a, so d = cx = (b·c)/a."));
            }
        }
        "proof.hypotheses_and_lemmas" => {
            if meta_matches(problem, "divided_out_root_trap", val_canon) {
                return Some(format!("Your answer evaluates to {val_canon}. Obstacle identified: That is only the root for x ≠ r; include the root x = r where the common factor (x − r) is zero."));
            }
            if meta_matches(problem, "only_cancelled_root_trap", val_canon) {
                return Some(format!("Your answer evaluates to {val_canon}. Obstacle identified: Add the second root from ax − b = cx + d to x = r."));
            }
            if meta_matches(problem, "exp_rem_trap", val_canon) {
                return Some(format!("Your answer evaluates to {val_canon}. Obstacle identified: That is the reduced exponent r; evaluate b^r mod m."));
            }
            if meta_matches(problem, "mult_trap", val_canon) {
                return Some(format!("Your answer evaluates to {val_canon}. Obstacle identified: Compute the power b^r mod m, not the product b·r mod m."));
            }
            if meta_matches(problem, "unscaled_lemma_trap", val_canon) {
                return Some(format!("Your answer evaluates to {val_canon}. Obstacle identified: Multiply the bounds from Lemma A and Lemma B by the coefficients a and b."));
            }
            if meta_matches(problem, "add_denom_trap", val_canon) {
                return Some(format!("Your answer evaluates to {val_canon}. Obstacle identified: Add 1/2 + 1/c using the common denominator 2c: (c + 2)/(2c)."));
            }
        }
        "proof.proof_audit_and_revision" => {
            if meta_matches(problem, "hypothesis_r_trap", val_canon) {
                return Some(format!("Your answer evaluates to {val_canon}. Obstacle identified: That is the original hypothesis x = r; find the OTHER root x = s of (x − r)(x − s) = 0."));
            }
            if meta_matches(problem, "sum_roots_trap", val_canon) {
                return Some(format!("Your answer evaluates to {val_canon}. Obstacle identified: That is r + s; subtract r to get the second root x = s."));
            }
            if meta_matches(problem, "min_x1_trap", val_canon) {
                return Some(format!("Your answer evaluates to {val_canon}. Obstacle identified: Evaluating at x = 1 only bounds the smallest element; for ∀x ∈ X, Y must bound the maximum at x = m."));
            }
            if meta_matches(problem, "forgot_b_trap", val_canon) {
                return Some(format!("Your answer evaluates to {val_canon}. Obstacle identified: Remember to add the constant +b in Y = a·m + b."));
            }
            if meta_matches(problem, "true_sum_trap", val_canon) {
                return Some(format!("Your answer evaluates to {val_canon}. Obstacle identified: That is the true sum k(k + 1); subtract it from the student's claimed value to find the constant error C."));
            }
            if meta_matches(problem, "claimed_val_trap", val_canon) {
                return Some(format!("Your answer evaluates to {val_canon}. Obstacle identified: Subtract the true sum k(k + 1) from the claimed value to isolate C."));
            }
        }
        _ => {}
    }
    None
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

    #[test]
    fn check_abacus_target_setting() {
        let p = generate("abacus.target.setting", "1.0.0", SEED, 1, MANIFEST_HASH).unwrap();
        let target = p.target_value.as_ref().unwrap();
        let budget = MathBudget::default();
        let outcome = check(&p, target, &budget);
        assert_eq!(outcome.disposition, Disposition::Correct);
    }

    #[test]
    fn check_new_exercise_families() {
        let budget = MathBudget::default();

        // 1. fractions.unit.identify
        let p = generate("fractions.unit.identify", "1.0.0", SEED, 1, MANIFEST_HASH).unwrap();
        let target = p.target_value.as_ref().unwrap();
        assert_eq!(check(&p, target, &budget).disposition, Disposition::Correct);

        // 2. arithmetic.column.addition
        let p = generate("arithmetic.column.addition", "1.0.0", SEED, 1, MANIFEST_HASH).unwrap();
        let target = p.target_value.as_ref().unwrap();
        assert_eq!(check(&p, target, &budget).disposition, Disposition::Correct);

        // 3. arithmetic.column.subtraction
        let p = generate("arithmetic.column.subtraction", "1.0.0", SEED, 1, MANIFEST_HASH).unwrap();
        let target = p.target_value.as_ref().unwrap();
        assert_eq!(check(&p, target, &budget).disposition, Disposition::Correct);

        // 4. place_value.decompose
        let p = generate("place_value.decompose", "1.0.0", SEED, 1, MANIFEST_HASH).unwrap();
        let target = p.target_value.as_ref().unwrap();
        assert_eq!(check(&p, target, &budget).disposition, Disposition::Correct);

        // 5. Misconception obstacle diagnostics on F08 and S01
        let p_med = generate("data.mean_median_calc", "1.0.0", SEED, 2, MANIFEST_HASH).unwrap();
        let unsorted_mid = p_med.metadata["unsorted_middle"].as_i64().unwrap().to_string();
        let res_med = check(&p_med, &unsorted_mid, &budget);
        assert_eq!(res_med.disposition, Disposition::Incorrect);
        assert!(res_med.feedback.contains("Sort all values from least to greatest"));

        let p_prob = generate("probability.simple_exact", "1.0.0", SEED, 2, MANIFEST_HASH).unwrap();
        let direct_trap = p_prob.metadata["direct_trap"].as_str().unwrap();
        let res_prob = check(&p_prob, direct_trap, &budget);
        assert_eq!(res_prob.disposition, Disposition::Incorrect);
        assert!(res_prob.feedback.contains("subtract from 1"));

        let p_alg = generate("algebra.expression_eval", "1.0.0", SEED, 1, MANIFEST_HASH).unwrap();
        let sign_err = p_alg.metadata["sign_trap"].as_i64().unwrap().to_string();
        let res_alg = check(&p_alg, &sign_err, &budget);
        assert_eq!(res_alg.disposition, Disposition::Incorrect);
        assert!(res_alg.feedback.contains("squaring a negative number"));

        // 6. Misconception obstacle diagnostics on S02 and S03
        let p_comp = generate("functions.composition_inverse", "1.1.0", SEED, 1, MANIFEST_HASH).unwrap();
        let rev_err = p_comp.metadata["reverse_order_trap"].as_i64().unwrap().to_string();
        let res_comp = check(&p_comp, &rev_err, &budget);
        assert_eq!(res_comp.disposition, Disposition::Incorrect);
        assert!(res_comp.feedback.contains("reverse order"));

        let p_disc = generate("algebra2.quadratic_discriminant", "1.1.0", SEED, 1, MANIFEST_HASH).unwrap();
        let sign_4ac = p_disc.metadata["sign_4ac_trap"].as_i64().unwrap().to_string();
        let res_disc = check(&p_disc, &sign_4ac, &budget);
        assert_eq!(res_disc.disposition, Disposition::Incorrect);
        assert!(res_disc.feedback.contains("Check the sign of −4ac"));

        // 7. Misconception obstacle diagnostics on S04 and S05
        let p_slope = generate("sec_geom.slope_and_lines", "1.1.0", SEED, 1, MANIFEST_HASH).unwrap();
        let ror_err = p_slope.metadata["run_over_rise_trap"].as_str().unwrap();
        let res_slope = check(&p_slope, ror_err, &budget);
        assert_eq!(res_slope.disposition, Disposition::Incorrect);
        assert!(res_slope.feedback.contains("rise / run"));

        let p_trig = generate("trig.radians_unit_circle", "1.1.0", SEED, 2, MANIFEST_HASH).unwrap();
        let pos_q2 = p_trig.metadata["positive_quadrant_trap"].as_str().unwrap();
        let res_trig = check(&p_trig, pos_q2, &budget);
        assert_eq!(res_trig.disposition, Disposition::Incorrect);
        assert!(res_trig.feedback.contains("Quadrant II"));

        // 8. Misconception obstacle diagnostics on S06 and S07
        let p_seq = generate("sequences.arithmetic_progression", "1.1.0", SEED, 1, MANIFEST_HASH).unwrap();
        let n_steps_err = p_seq.metadata["n_steps_trap"].as_i64().unwrap().to_string();
        let res_seq = check(&p_seq, &n_steps_err, &budget);
        assert_eq!(res_seq.disposition, Disposition::Incorrect);
        assert!(res_seq.feedback.contains("(n − 1) steps"));

        let p_calc = generate("calculus.definite_integral_ftc", "1.1.0", SEED, 3, MANIFEST_HASH).unwrap();
        let disp_err = p_calc.metadata["disp_only_trap"].as_i64().unwrap().to_string();
        let res_calc = check(&p_calc, &disp_err, &budget);
        assert_eq!(res_calc.disposition, Disposition::Incorrect);
        assert!(res_calc.feedback.contains("initial position s(0)"));

        // 9. Misconception obstacle diagnostics on P01, P02, and P03
        let p_logic = generate("proof.logic_truth_quantifiers", "1.1.0", SEED, 2, MANIFEST_HASH).unwrap();
        let strict_err = p_logic.metadata["strict_inequality_trap"].as_i64().unwrap().to_string();
        let res_logic = check(&p_logic, &strict_err, &budget);
        assert_eq!(res_logic.disposition, Disposition::Incorrect);
        assert!(res_logic.feedback.contains("logical negation of '<' is '≥'"));

        let p_euler = generate("proof.counterexamples_and_extremal", "1.1.0", SEED, 1, MANIFEST_HASH).unwrap();
        let p_trap = p_euler.metadata["obvious_p_trap"].as_i64().unwrap().to_string();
        let res_euler = check(&p_euler, &p_trap, &budget);
        assert_eq!(res_euler.disposition, Disposition::Incorrect);
        assert!(res_euler.feedback.contains("SMALLER positive counterexample"));
    }
}
