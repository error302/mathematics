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
    if template_version != "1.0.0" && template_version != "1.1.0" {
        return Err(GenerationFailure::TemplateNotFound(format!("{template_id}@{template_version}")));
    }
    let mut rng = AxiomRng::from_hex(seed_hex)
        .map_err(|e| GenerationFailure::InvalidSeed(e.to_string()))?;

    let (prompt_text, mut prompt_latex, mut prompt_speech, answer_kind, options, target_value, metadata, mut checker_version) =
        match template_id {
            "puzzles.arithmetic.grid" | "reasoning.pattern" | "reasoning.logic" | "reasoning.comprehension" | "reasoning.analysis" | "visual.rectangle" | "visual.linear" | "visual.quadratic" => {
                if template_version != "1.1.0" { return Err(GenerationFailure::TemplateNotFound(format!("{template_id}@{template_version}"))); }
                crate::challenges::generate(&mut rng, template_id, difficulty_band)?
            }
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
            "arithmetic.whole.multiplication" => {
                generate_whole_multiplication(&mut rng, difficulty_band)?
            }
            "arithmetic.whole.division" => {
                generate_whole_division(&mut rng, difficulty_band)?
            }
            "integers.order" => {
                generate_integer_order(&mut rng, difficulty_band)?
            }
            "divisibility.factors.find" => {
                generate_factors_greatest_proper(&mut rng, difficulty_band)?
            }
            "divisibility.gcd_lcm" => {
                generate_gcd_lcm(&mut rng, difficulty_band)?
            }
            "learning.problem_reading" => {
                generate_problem_reading(&mut rng, difficulty_band)?
            }
            "learning.equality_balance" => {
                generate_equality_balance(&mut rng, difficulty_band)?
            }
            "learning.checking_bounds" => {
                generate_checking_bounds(&mut rng, difficulty_band)?
            }
            "decimals.fraction_convert" => {
                generate_decimal_fraction_convert(&mut rng, difficulty_band)?
            }
            "percentages.benchmark_calc" => {
                generate_percentage_benchmark_calc(&mut rng, difficulty_band)?
            }
            "ratios.scaling_solve" => {
                generate_ratio_scaling_solve(&mut rng, difficulty_band)?
            }
            "powers.integer_eval" => {
                generate_powers_integer_eval(&mut rng, difficulty_band)?
            }
            "powers.scientific_notation" => {
                generate_powers_scientific_notation(&mut rng, difficulty_band)?
            }
            "roots.radical_eval" => {
                generate_roots_radical_eval(&mut rng, difficulty_band)?
            }
            "geometry.perimeter_and_area" => {
                generate_geometry_perimeter_and_area(&mut rng, difficulty_band)?
            }
            "geometry.angles_triangles" => {
                generate_geometry_angles_triangles(&mut rng, difficulty_band)?
            }
            "geometry.coordinate_distance" => {
                generate_geometry_coordinate_distance(&mut rng, difficulty_band)?
            }
            "data.mean_median_calc" => {
                generate_data_mean_median_calc(&mut rng, difficulty_band)?
            }
            "counting.permutations_combinations" => {
                generate_counting_permutations_combinations(&mut rng, difficulty_band)?
            }
            "probability.simple_exact" => {
                generate_probability_simple_exact(&mut rng, difficulty_band)?
            }
            "algebra.expression_eval" => {
                generate_algebra_expression_eval(&mut rng, difficulty_band)?
            }
            "algebra.linear_equation_solve" => {
                generate_algebra_linear_equation_solve(&mut rng, difficulty_band)?
            }
            "algebra.linear_system_solve" => {
                generate_algebra_linear_system_solve(&mut rng, difficulty_band)?
            }
            "functions.evaluate_and_range" => {
                generate_functions_evaluate_and_range(&mut rng, difficulty_band)?
            }
            "functions.composition_inverse" => {
                generate_functions_composition_inverse(&mut rng, difficulty_band)?
            }
            "functions.transformations_vertex" => {
                generate_functions_transformations_vertex(&mut rng, difficulty_band)?
            }
            "algebra2.polynomial_factor_roots" => {
                generate_algebra2_polynomial_factor_roots(&mut rng, difficulty_band)?
            }
            "algebra2.quadratic_discriminant" => {
                generate_algebra2_quadratic_discriminant(&mut rng, difficulty_band)?
            }
            "algebra2.exp_log_eval" => {
                generate_algebra2_exp_log_eval(&mut rng, difficulty_band)?
            }
            "sec_geom.slope_and_lines" => {
                generate_sec_geom_slope_and_lines(&mut rng, difficulty_band)?
            }
            "sec_geom.circle_equations_theorems" => {
                generate_sec_geom_circle_equations_theorems(&mut rng, difficulty_band)?
            }
            "sec_geom.planar_vectors_dot" => {
                generate_sec_geom_planar_vectors_dot(&mut rng, difficulty_band)?
            }
            "trig.radians_unit_circle" => {
                generate_trig_radians_unit_circle(&mut rng, difficulty_band)?
            }
            "trig.identities_double_angle" => {
                generate_trig_identities_double_angle(&mut rng, difficulty_band)?
            }
            "trig.law_of_sines_cosines" => {
                generate_trig_law_of_sines_cosines(&mut rng, difficulty_band)?
            }
            "sequences.arithmetic_progression" => {
                generate_sequences_arithmetic_progression(&mut rng, difficulty_band)?
            }
            "sequences.geometric_and_infinite" => {
                generate_sequences_geometric_and_infinite(&mut rng, difficulty_band)?
            }
            "sequences.recursion_binomial" => {
                generate_sequences_recursion_binomial(&mut rng, difficulty_band)?
            }
            "calculus.limits_rational" => {
                generate_calculus_limits_rational(&mut rng, difficulty_band)?
            }
            "calculus.derivatives_tangent" => {
                generate_calculus_derivatives_tangent(&mut rng, difficulty_band)?
            }
            "calculus.definite_integral_ftc" => {
                generate_calculus_definite_integral_ftc(&mut rng, difficulty_band)?
            }
            "proof.sets_and_cardinality" => {
                generate_proof_sets_and_cardinality(&mut rng, difficulty_band)?
            }
            "proof.logic_truth_quantifiers" => {
                generate_proof_logic_truth_quantifiers(&mut rng, difficulty_band)?
            }
            "proof.relations_and_bijections" => {
                generate_proof_relations_and_bijections(&mut rng, difficulty_band)?
            }
            "proof.direct_and_contrapositive" => {
                generate_proof_direct_and_contrapositive(&mut rng, difficulty_band)?
            }
            "proof.mathematical_induction" => {
                generate_proof_mathematical_induction(&mut rng, difficulty_band)?
            }
            "proof.counterexamples_and_extremal" => {
                generate_proof_counterexamples_and_extremal(&mut rng, difficulty_band)?
            }
            "proof.definitions_and_biconditionals" => {
                generate_proof_definitions_and_biconditionals(&mut rng, difficulty_band)?
            }
            "proof.hypotheses_and_lemmas" => {
                generate_proof_hypotheses_and_lemmas(&mut rng, difficulty_band)?
            }
            "proof.proof_audit_and_revision" => {
                generate_proof_proof_audit_and_revision(&mut rng, difficulty_band)?
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
            "abacus.trainer.number" | "abacus.trainer.add" | "abacus.trainer.subtract" | "abacus.trainer.five" | "abacus.trainer.five_sub" | "abacus.trainer.ten" | "abacus.trainer.ten_sub" | "abacus.trainer.compound" | "abacus.trainer.mental" => {
                if template_version != "1.1.0" || difficulty_band != 1 { return Err(GenerationFailure::TemplateNotFound(format!("{template_id}@{template_version}: beginner band 1 only"))); }
                crate::abacus_tutor::generate_training(&mut rng, template_id)?
            }
            _ => return Err(GenerationFailure::TemplateNotFound(template_id.into())),
        };

    // Keep previously generated v1 artifacts replayable; corrected presentation ships as v1.1.
    if template_version == "1.0.0" {
        if template_id == "abacus.read.state" {
            let state: crate::abacus::AbacusState = serde_json::from_value(metadata["abacus_state"].clone())
                .map_err(|error| GenerationFailure::Serialization(error.to_string()))?;
            prompt_speech = format!("Read the soroban: {}", state.describe_state());
        }
        if template_id == "fractions.unit.identify" {
            let denominator = metadata["denominator"].as_u64().ok_or_else(|| GenerationFailure::Serialization("missing denominator".into()))?;
            prompt_latex = format!("\\text{{Partition of 1 whole into }} {} \\text{{ equal parts}} \\implies \\frac{{1}}{{{}}}", denominator, denominator);
        }
    }

    if template_version == "1.1.0" {
        checker_version = match checker_version.as_str() {
            "integer-exact-v1" => "integer-exact-v2".into(),
            "rational-equality-v1" => "rational-equality-v2".into(),
            "fraction-form-v1" => "fraction-form-v2".into(),
            "abacus-state-v1" => "abacus-state-v2".into(),
            "integer-order-v1" => "integer-order-v2".into(),
            _ => checker_version,
        };
    }

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

/// Template: `arithmetic.whole.multiplication`
fn generate_whole_multiplication<R: DrawSource + ?Sized>(
    rng: &mut R,
    difficulty: u32,
) -> Result<(String, String, String, String, Option<Vec<String>>, Option<String>, Value, String), GenerationFailure> {
    let (a, b) = match difficulty {
        1 => {
            let x = uniform_inclusive(rng, 2, 9);
            let y = uniform_inclusive(rng, 2, 9);
            (x, y)
        }
        2 => {
            let x = uniform_inclusive(rng, 11, 49);
            let y = uniform_inclusive(rng, 2, 9);
            (x, y)
        }
        _ => {
            let x = uniform_inclusive(rng, 12, 99);
            let y = uniform_inclusive(rng, 11, 50);
            (x, y)
        }
    };
    let prod = a * b;
    let prompt_text = format!("Calculate the product: {} × {}.", a, b);
    let prompt_latex = format!("{} \\times {} = ?", a, b);
    let prompt_speech = format!("Calculate {} times {}.", a, b);
    let metadata = serde_json::json!({
        "factor1": a.to_string(),
        "factor2": b.to_string(),
        "product": prod.to_string(),
    });
    Ok((
        prompt_text,
        prompt_latex,
        prompt_speech,
        "integer".into(),
        None,
        Some(prod.to_string()),
        metadata,
        "integer-exact-v1".into(),
    ))
}

/// Template: `arithmetic.whole.division`
fn generate_whole_division<R: DrawSource + ?Sized>(
    rng: &mut R,
    difficulty: u32,
) -> Result<(String, String, String, String, Option<Vec<String>>, Option<String>, Value, String), GenerationFailure> {
    let (divisor, quotient) = match difficulty {
        1 => {
            let b = uniform_inclusive(rng, 2, 9);
            let q = uniform_inclusive(rng, 2, 9);
            (b, q)
        }
        2 => {
            let b = uniform_inclusive(rng, 2, 9);
            let q = uniform_inclusive(rng, 10, 49);
            (b, q)
        }
        _ => {
            let b = uniform_inclusive(rng, 11, 25);
            let q = uniform_inclusive(rng, 10, 50);
            (b, q)
        }
    };
    let dividend = divisor * quotient;
    let prompt_text = format!("Calculate the quotient: {} ÷ {}.", dividend, divisor);
    let prompt_latex = format!("{} \\div {} = ?", dividend, divisor);
    let prompt_speech = format!("Calculate {} divided by {}.", dividend, divisor);
    let metadata = serde_json::json!({
        "dividend": dividend.to_string(),
        "divisor": divisor.to_string(),
        "quotient": quotient.to_string(),
    });
    Ok((
        prompt_text,
        prompt_latex,
        prompt_speech,
        "integer".into(),
        None,
        Some(quotient.to_string()),
        metadata,
        "integer-exact-v1".into(),
    ))
}

/// Template: `integers.order`
fn generate_integer_order<R: DrawSource + ?Sized>(
    rng: &mut R,
    difficulty: u32,
) -> Result<(String, String, String, String, Option<Vec<String>>, Option<String>, Value, String), GenerationFailure> {
    let (min, max) = match difficulty {
        1 => (-12, 12),
        2 => (-99, 99),
        _ => (-999, 999),
    };
    let mut a = uniform_inclusive(rng, min, max) as i64;
    let b = uniform_inclusive(rng, min, max) as i64;
    if difficulty == 1 && a > 0 && b > 0 {
        a = -a;
    }
    let target = if a < b {
        "less"
    } else if a > b {
        "greater"
    } else {
        "equal"
    };

    let prompt_text = format!("Compare the integers {a} and {b} on the number line. Is {a} less, equal, or greater than {b}?");
    let prompt_latex = format!("{a} \\;\\text{{?}}\\; {b}");
    let prompt_speech = format!("Compare the integers {a} and {b} on the number line. Is {a} less, equal, or greater than {b}?");
    let options = Some(vec!["less".into(), "equal".into(), "greater".into()]);

    let metadata = serde_json::json!({
        "a": a,
        "b": b,
        "target": target,
    });

    Ok((
        prompt_text,
        prompt_latex,
        prompt_speech,
        "enum".into(),
        options,
        Some(target.into()),
        metadata,
        "integer-order-v1".into(),
    ))
}

/// Template: `divisibility.factors.find`
fn generate_factors_greatest_proper<R: DrawSource + ?Sized>(
    rng: &mut R,
    difficulty: u32,
) -> Result<(String, String, String, String, Option<Vec<String>>, Option<String>, Value, String), GenerationFailure> {
    let (min, max) = match difficulty {
        1 => (12, 50),
        2 => (52, 160),
        _ => (162, 600),
    };

    for _ in 0..128 {
        let n = uniform_inclusive(rng, min, max) as i64;
        let mut greatest = 1;
        for d in (2..=(n / 2)).rev() {
            if n % d == 0 {
                greatest = d;
                break;
            }
        }
        if greatest > 1 {
            let prompt_text = format!("Find the greatest proper divisor of {n} (the largest positive integer strictly less than {n} that divides {n}).");
            let prompt_latex = format!("\\text{{Greatest proper divisor of }}\\; {n}");
            let prompt_speech = format!("Find the greatest proper divisor of {n}.");

            let metadata = serde_json::json!({
                "number": n,
                "greatest_proper_divisor": greatest,
            });

            return Ok((
                prompt_text,
                prompt_latex,
                prompt_speech,
                "integer".into(),
                None,
                Some(greatest.to_string()),
                metadata,
                "integer-exact-v1".into(),
            ));
        }
    }

    Err(GenerationFailure::ConstraintExhaustion { draws: 128 })
}

fn gcd(mut x: i64, mut y: i64) -> i64 {
    while y != 0 {
        let t = y;
        y = x % y;
        x = t;
    }
    x.abs()
}

/// Template: `divisibility.gcd_lcm`
fn generate_gcd_lcm<R: DrawSource + ?Sized>(
    rng: &mut R,
    difficulty: u32,
) -> Result<(String, String, String, String, Option<Vec<String>>, Option<String>, Value, String), GenerationFailure> {

    if difficulty <= 2 {
        let (min, max) = if difficulty == 1 { (8, 48) } else { (36, 144) };
        for _ in 0..128 {
            let a = uniform_inclusive(rng, min, max) as i64;
            let b = uniform_inclusive(rng, min, max) as i64;
            let g = gcd(a, b);
            if g > 1 && a != b {
                let prompt_text = format!("Find the greatest common divisor: gcd({a}, {b}).");
                let prompt_latex = format!("\\gcd({a}, {b}) = ?");
                let prompt_speech = format!("Find the greatest common divisor of {a} and {b}.");

                let metadata = serde_json::json!({
                    "kind": "gcd",
                    "operand1": a,
                    "operand2": b,
                    "result": g,
                });

                return Ok((
                    prompt_text,
                    prompt_latex,
                    prompt_speech,
                    "integer".into(),
                    None,
                    Some(g.to_string()),
                    metadata,
                    "integer-exact-v1".into(),
                ));
            }
        }
    } else {
        for _ in 0..128 {
            let a = uniform_inclusive(rng, 6, 36) as i64;
            let b = uniform_inclusive(rng, 6, 36) as i64;
            let g = gcd(a, b);
            if g > 1 && a != b {
                let lcm = (a * b) / g;
                let prompt_text = format!("Find the least common multiple: lcm({a}, {b}).");
                let prompt_latex = format!("\\operatorname{{lcm}}({a}, {b}) = ?");
                let prompt_speech = format!("Find the least common multiple of {a} and {b}.");

                let metadata = serde_json::json!({
                    "kind": "lcm",
                    "operand1": a,
                    "operand2": b,
                    "gcd": g,
                    "result": lcm,
                });

                return Ok((
                    prompt_text,
                    prompt_latex,
                    prompt_speech,
                    "integer".into(),
                    None,
                    Some(lcm.to_string()),
                    metadata,
                    "integer-exact-v1".into(),
                ));
            }
        }
    }

    Err(GenerationFailure::ConstraintExhaustion { draws: 128 })
}

/// Template: `learning.problem_reading`
fn generate_problem_reading<R: DrawSource + ?Sized>(
    rng: &mut R,
    difficulty: u32,
) -> Result<(String, String, String, String, Option<Vec<String>>, Option<String>, Value, String), GenerationFailure> {
    match difficulty {
        1 => {
            let c = uniform_inclusive(rng, 30, 95) as i64;
            let a = uniform_inclusive(rng, 10, (c - 5) as i128) as i64;
            let x = c - a;
            let prompt_text = format!("A water storage container has a capacity of {c} litres. It currently contains {a} litres. How many litres x must be added to fill the container to capacity?");
            let prompt_latex = format!("{a} + x = {c}");
            let prompt_speech = format!("A container has a capacity of {c} litres and holds {a} litres. How many litres x must be added to fill it?");
            let metadata = serde_json::json!({
                "context": "container_capacity",
                "capacity": c,
                "current": a,
                "target": x,
            });
            Ok((prompt_text, prompt_latex, prompt_speech, "integer".into(), None, Some(x.to_string()), metadata, "integer-exact-v1".into()))
        }
        2 => {
            let b = uniform_inclusive(rng, 60, 180) as i64;
            let s = uniform_inclusive(rng, 4, 12) as i64;
            let r = uniform_inclusive(rng, 15, 50) as i64;
            let x = b - 4 * s + r;
            let prompt_text = format!("A shop inventory started with {b} books. The shop sold {s} sets of 4 books each, and then received a restocking shipment of {r} books. How many books x are in stock now?");
            let prompt_latex = format!("x = {b} - ({s} \\times 4) + {r}");
            let prompt_speech = format!("A shop had {b} books, sold {s} sets of 4, and received {r} books. How many books x are now in stock?");
            let metadata = serde_json::json!({
                "context": "inventory_multistep",
                "initial": b,
                "sets_sold": s,
                "set_size": 4,
                "received": r,
                "target": x,
            });
            Ok((prompt_text, prompt_latex, prompt_speech, "integer".into(), None, Some(x.to_string()), metadata, "integer-exact-v1".into()))
        }
        _ => {
            let b_pts = uniform_inclusive(rng, 35, 90) as i64;
            let a_pts = uniform_inclusive(rng, 20, (2 * b_pts - 10) as i128) as i64;
            let x = 2 * b_pts - a_pts;
            let prompt_text = format!("Team Alpha has {a_pts} points and Team Beta has {b_pts} points. How many additional points x must Team Alpha score so that Team Alpha has exactly double Team Beta's score?");
            let prompt_latex = format!("{a_pts} + x = 2 \\times {b_pts}");
            let prompt_speech = format!("Team Alpha has {a_pts} points and Team Beta has {b_pts} points. How many points must Team Alpha score to have double Team Beta's score?");
            let metadata = serde_json::json!({
                "context": "score_doubling",
                "team_a": a_pts,
                "team_b": b_pts,
                "target": x,
            });
            Ok((prompt_text, prompt_latex, prompt_speech, "integer".into(), None, Some(x.to_string()), metadata, "integer-exact-v1".into()))
        }
    }
}

/// Template: `learning.equality_balance`
fn generate_equality_balance<R: DrawSource + ?Sized>(
    rng: &mut R,
    difficulty: u32,
) -> Result<(String, String, String, String, Option<Vec<String>>, Option<String>, Value, String), GenerationFailure> {
    match difficulty {
        1 => {
            let b = uniform_inclusive(rng, 15, 60) as i64;
            let c = uniform_inclusive(rng, 10, 50) as i64;
            let sum_right = b + c;
            let a = uniform_inclusive(rng, 5, (sum_right - 3) as i128) as i64;
            let x = sum_right - a;
            let prompt_text = format!("Find the value of x that balances the equation: {a} + x = {b} + {c}.");
            let prompt_latex = format!("{a} + x = {b} + {c}");
            let prompt_speech = format!("Find x such that {a} plus x equals {b} plus {c}.");
            let metadata = serde_json::json!({
                "type": "additive_balance",
                "left_const": a,
                "right_b": b,
                "right_c": c,
                "right_sum": sum_right,
                "target": x,
            });
            Ok((prompt_text, prompt_latex, prompt_speech, "integer".into(), None, Some(x.to_string()), metadata, "integer-exact-v1".into()))
        }
        2 => {
            let a = uniform_inclusive(rng, 3, 9) as i64;
            let x = uniform_inclusive(rng, 4, 18) as i64;
            let total = a * x;
            let b = uniform_inclusive(rng, 3, (total - 3) as i128) as i64;
            let c = total - b;
            let prompt_text = format!("Find the value of x that balances the equation: {a} × x = {b} + {c}.");
            let prompt_latex = format!("{a} \\times x = {b} + {c}");
            let prompt_speech = format!("Find x such that {a} times x equals {b} plus {c}.");
            let metadata = serde_json::json!({
                "type": "multiplicative_balance",
                "coeff": a,
                "right_b": b,
                "right_c": c,
                "right_sum": total,
                "target": x,
            });
            Ok((prompt_text, prompt_latex, prompt_speech, "integer".into(), None, Some(x.to_string()), metadata, "integer-exact-v1".into()))
        }
        _ => {
            let c_coeff = uniform_inclusive(rng, 1, 4) as i64;
            let diff = uniform_inclusive(rng, 2, 5) as i64;
            let a_coeff = c_coeff + diff;
            let x = uniform_inclusive(rng, 3, 15) as i64;
            let b_const = uniform_inclusive(rng, 2, 30) as i64;
            let d_const = diff * x + b_const;
            let prompt_text = format!("Find the value of x that balances the balance scale: {a_coeff}x + {b_const} = {c_coeff}x + {d_const}.");
            let prompt_latex = format!("{a_coeff}x + {b_const} = {c_coeff}x + {d_const}");
            let prompt_speech = format!("Find x such that {a_coeff} x plus {b_const} equals {c_coeff} x plus {d_const}.");
            let metadata = serde_json::json!({
                "type": "two_sided_balance",
                "left_coeff": a_coeff,
                "left_const": b_const,
                "right_coeff": c_coeff,
                "right_const": d_const,
                "target": x,
            });
            Ok((prompt_text, prompt_latex, prompt_speech, "integer".into(), None, Some(x.to_string()), metadata, "integer-exact-v1".into()))
        }
    }
}

/// Template: `learning.checking_bounds`
fn generate_checking_bounds<R: DrawSource + ?Sized>(
    rng: &mut R,
    difficulty: u32,
) -> Result<(String, String, String, String, Option<Vec<String>>, Option<String>, Value, String), GenerationFailure> {
    match difficulty {
        1 => {
            let is_odd_mult = uniform_inclusive(rng, 0, 1) == 1;
            let a = if is_odd_mult {
                (uniform_inclusive(rng, 5, 25) * 2 + 1) as i64
            } else {
                (uniform_inclusive(rng, 5, 25) * 2) as i64
            };
            let b = (uniform_inclusive(rng, 5, 25) * 2 + 1) as i64;
            let c = (uniform_inclusive(rng, 4, 30) * 2) as i64;
            let actual = a * b + c;
            let parity = if actual % 2 != 0 { 1 } else { 0 };
            let parity_word = if parity == 1 { "ODD (1)" } else { "EVEN (0)" };
            let prompt_text = format!("Consider the expression: {a} × {b} + {c}. Using parity rules (without full computation), determine whether the result is odd (enter 1) or even (enter 0).");
            let prompt_latex = format!("\\text{{Parity of }}\\; ({a} \\times {b}) + {c} \\implies 1\\;(\\text{{odd}})\\;\\text{{or}}\\;0\\;(\\text{{even}})");
            let prompt_speech = format!("Determine if the expression {a} times {b} plus {c} is odd (enter 1) or even (enter 0).");
            let metadata = serde_json::json!({
                "kind": "parity_check",
                "operand_a": a,
                "operand_b": b,
                "operand_c": c,
                "parity": parity,
                "parity_word": parity_word,
                "target": parity,
            });
            Ok((prompt_text, prompt_latex, prompt_speech, "integer".into(), None, Some(parity.to_string()), metadata, "integer-exact-v1".into()))
        }
        2 => {
            let a = uniform_inclusive(rng, 23, 89) as i64;
            let b = uniform_inclusive(rng, 32, 98) as i64;
            let a_down = (a / 10) * 10;
            let b_down = (b / 10) * 10;
            let lower_bound = a_down * b_down;
            let prompt_text = format!("Estimate a conservative lower bound for the product {a} × {b} by rounding both factors down to the nearest ten ({a_down} and {b_down}) and computing their product.");
            let prompt_latex = format!("{a_down} \\times {b_down} = ?");
            let prompt_speech = format!("Find the lower bound for {a} times {b} by computing {a_down} times {b_down}.");
            let metadata = serde_json::json!({
                "kind": "bounds_estimation",
                "a": a,
                "b": b,
                "a_down": a_down,
                "b_down": b_down,
                "target": lower_bound,
            });
            Ok((prompt_text, prompt_latex, prompt_speech, "integer".into(), None, Some(lower_bound.to_string()), metadata, "integer-exact-v1".into()))
        }
        _ => {
            let use_17 = uniform_inclusive(rng, 0, 1) == 1;
            let p_val: i64 = if use_17 { 17 } else { 11 };
            let prompt_text = format!("A mathematical conjecture asserts: 'For all positive integers n, n² - n + {p_val} is always prime.' What is the smallest positive integer n that serves as a counterexample (making n² - n + {p_val} composite)?");
            let prompt_latex = format!("n^2 - n + {p_val} \\;\\text{{is composite for smallest positive integer}}\\; n = ?");
            let prompt_speech = format!("Find the smallest positive integer n where n squared minus n plus {p_val} is composite.");
            let metadata = serde_json::json!({
                "kind": "counterexample",
                "p": p_val,
                "target": p_val,
            });
            Ok((prompt_text, prompt_latex, prompt_speech, "integer".into(), None, Some(p_val.to_string()), metadata, "integer-exact-v1".into()))
        }
    }
}

/// Template: `decimals.fraction_convert`
fn generate_decimal_fraction_convert<R: DrawSource + ?Sized>(
    rng: &mut R,
    difficulty: u32,
) -> Result<(String, String, String, String, Option<Vec<String>>, Option<String>, Value, String), GenerationFailure> {
    match difficulty {
        1 => {
            let is_tenths = uniform_inclusive(rng, 0, 1) == 1;
            if is_tenths {
                let n = uniform_inclusive(rng, 1, 9) as i64;
                let target = frac(n, 10);
                let prompt_text = format!("Convert the fraction {n}/10 to an exact finite decimal (or simplified fraction).");
                let prompt_latex = format!("\\frac{{{n}}}{{10}} = ?");
                let prompt_speech = format!("Convert the fraction {n} over 10 to a decimal.");
                let metadata = serde_json::json!({
                    "kind": "tenths",
                    "numerator": n,
                    "denominator": 10,
                    "canonical": to_canonical(&target),
                    "decimal_str": format!("0.{n}"),
                });
                Ok((prompt_text, prompt_latex, prompt_speech, "rational".into(), None, Some(to_canonical(&target)), metadata, "rational-equality-v1".into()))
            } else {
                let n = uniform_inclusive(rng, 1, 99) as i64;
                let target = frac(n, 100);
                let dec_str = format!("0.{:02}", n);
                let prompt_text = format!("Convert the decimal {dec_str} to an exact canonical fraction in lowest terms.");
                let prompt_latex = format!("{dec_str} = ?");
                let prompt_speech = format!("Convert the decimal {dec_str} to an exact fraction in lowest terms.");
                let metadata = serde_json::json!({
                    "kind": "hundredths",
                    "numerator": n,
                    "denominator": 100,
                    "canonical": to_canonical(&target),
                    "decimal_str": dec_str,
                });
                Ok((prompt_text, prompt_latex, prompt_speech, "rational".into(), None, Some(to_canonical(&target)), metadata, "rational-equality-v1".into()))
            }
        }
        2 => {
            let denom_choices = [4, 5, 20, 25, 50];
            let d_idx = uniform_inclusive(rng, 0, (denom_choices.len() - 1) as i128) as usize;
            let d = denom_choices[d_idx] as i64;
            let n = uniform_inclusive(rng, 1, (d - 1) as i128) as i64;
            let target = frac(n, d);
            let prompt_text = format!("Convert the fraction {n}/{d} to an exact finite decimal.");
            let prompt_latex = format!("\\frac{{{n}}}{{{d}}} = ?");
            let prompt_speech = format!("Convert the fraction {n} over {d} to a decimal.");
            let metadata = serde_json::json!({
                "kind": "powers_of_ten_factors",
                "numerator": n,
                "denominator": d,
                "canonical": to_canonical(&target),
            });
            Ok((prompt_text, prompt_latex, prompt_speech, "rational".into(), None, Some(to_canonical(&target)), metadata, "rational-equality-v1".into()))
        }
        _ => {
            let whole = uniform_inclusive(rng, 1, 5) as i64;
            let denom_choices = [8, 20, 25, 40];
            let d_idx = uniform_inclusive(rng, 0, (denom_choices.len() - 1) as i128) as usize;
            let d = denom_choices[d_idx] as i64;
            let n = uniform_inclusive(rng, 1, (d - 1) as i128) as i64;
            let target = int(whole) + frac(n, d);
            let prompt_text = format!("Convert the mixed quantity {whole} and {n}/{d} to an exact decimal.");
            let prompt_latex = format!("{whole} + \\frac{{{n}}}{{{d}}} = ?");
            let prompt_speech = format!("Convert {whole} and {n} over {d} to an exact decimal.");
            let metadata = serde_json::json!({
                "kind": "mixed_decimal",
                "whole": whole,
                "numerator": n,
                "denominator": d,
                "canonical": to_canonical(&target),
            });
            Ok((prompt_text, prompt_latex, prompt_speech, "rational".into(), None, Some(to_canonical(&target)), metadata, "rational-equality-v1".into()))
        }
    }
}

/// Template: `percentages.benchmark_calc`
fn generate_percentage_benchmark_calc<R: DrawSource + ?Sized>(
    rng: &mut R,
    difficulty: u32,
) -> Result<(String, String, String, String, Option<Vec<String>>, Option<String>, Value, String), GenerationFailure> {
    match difficulty {
        1 => {
            let benchmarks = [10, 25, 50, 75];
            let p_idx = uniform_inclusive(rng, 0, (benchmarks.len() - 1) as i128) as usize;
            let p = benchmarks[p_idx];
            let mult = uniform_inclusive(rng, 1, 10) as i64;
            let total = mult * 40;
            let target = (total * p) / 100;

            let prompt_text = format!("Calculate {p}% of {total}.");
            let prompt_latex = format!("{p}\\% \\text{{ of }} {total} = ?");
            let prompt_speech = format!("What is {p} percent of {total}?");
            let metadata = serde_json::json!({
                "kind": "benchmark_simple",
                "percent": p,
                "total": total,
                "target": target,
            });
            Ok((prompt_text, prompt_latex, prompt_speech, "integer".into(), None, Some(target.to_string()), metadata, "integer-exact-v1".into()))
        }
        2 => {
            let composites = [15, 35, 45, 65];
            let c_idx = uniform_inclusive(rng, 0, (composites.len() - 1) as i128) as usize;
            let p = composites[c_idx];
            let mult = uniform_inclusive(rng, 1, 8) as i64;
            let total = mult * 20;
            let target = (total * p) / 100;

            let prompt_text = format!("Calculate {p}% of {total} using benchmark parts (e.g. 10% and 5% decomposition).");
            let prompt_latex = format!("{p}\\% \\text{{ of }} {total} = ?");
            let prompt_speech = format!("Calculate {p} percent of {total}.");
            let metadata = serde_json::json!({
                "kind": "benchmark_composite",
                "percent": p,
                "total": total,
                "target": target,
            });
            Ok((prompt_text, prompt_latex, prompt_speech, "integer".into(), None, Some(target.to_string()), metadata, "integer-exact-v1".into()))
        }
        _ => {
            let benchmarks = [20, 25, 50, 75];
            let p_idx = uniform_inclusive(rng, 0, (benchmarks.len() - 1) as i128) as usize;
            let p = benchmarks[p_idx];
            let whole_mult = uniform_inclusive(rng, 2, 12) as i64;
            let whole = whole_mult * 20;
            let part = (whole * p) / 100;

            let prompt_text = format!("If {p}% of an unknown whole number is {part}, what is the whole number?");
            let prompt_latex = format!("{p}\\% \\times x = {part} \\implies x = ?");
            let prompt_speech = format!("If {p} percent of a number is {part}, what is the number?");
            let metadata = serde_json::json!({
                "kind": "reverse_percentage",
                "percent": p,
                "part": part,
                "whole": whole,
                "target": whole,
            });
            Ok((prompt_text, prompt_latex, prompt_speech, "integer".into(), None, Some(whole.to_string()), metadata, "integer-exact-v1".into()))
        }
    }
}

/// Template: `ratios.scaling_solve`
fn generate_ratio_scaling_solve<R: DrawSource + ?Sized>(
    rng: &mut R,
    difficulty: u32,
) -> Result<(String, String, String, String, Option<Vec<String>>, Option<String>, Value, String), GenerationFailure> {
    match difficulty {
        1 => {
            let a = uniform_inclusive(rng, 2, 7) as i64;
            let b = uniform_inclusive(rng, 3, 9) as i64;
            let k = uniform_inclusive(rng, 2, 6) as i64;
            let c = a * k;
            let x = b * k;

            let prompt_text = format!("Two quantities scale in the ratio {a}:{b}. If the first quantity is {c}, what is the scaled value of the second quantity x?");
            let prompt_latex = format!("\\frac{{{a}}}{{{b}}} = \\frac{{{c}}}{{x}} \\implies x = ?");
            let prompt_speech = format!("If the ratio is {a} to {b} and the first quantity is {c}, what is the second quantity x?");
            let metadata = serde_json::json!({
                "kind": "direct_scaling",
                "term_a": a,
                "term_b": b,
                "scale_factor": k,
                "term_c": c,
                "target": x,
            });
            Ok((prompt_text, prompt_latex, prompt_speech, "integer".into(), None, Some(x.to_string()), metadata, "integer-exact-v1".into()))
        }
        2 => {
            let a = uniform_inclusive(rng, 3, 8) as i64;
            let b = uniform_inclusive(rng, 2, 7) as i64;
            let k = uniform_inclusive(rng, 3, 9) as i64;
            let c = a * k;
            let x = b * k;

            let prompt_text = format!("A workshop mixture requires {a} parts copper for every {b} parts tin. To maintain the exact same proportion using {c} kg of copper, how many kg of tin x are needed?");
            let prompt_latex = format!("\\frac{{{a}\\text{{ kg copper}}}}{{{b}\\text{{ kg tin}}}} = \\frac{{{c}\\text{{ kg copper}}}}{{x\\text{{ kg tin}}}} \\implies x = ?");
            let prompt_speech = format!("A mixture requires {a} parts copper to {b} parts tin. For {c} kilograms of copper, how many kilograms of tin are needed?");
            let metadata = serde_json::json!({
                "kind": "contextual_mixture",
                "term_a": a,
                "term_b": b,
                "scale_factor": k,
                "term_c": c,
                "target": x,
            });
            Ok((prompt_text, prompt_latex, prompt_speech, "integer".into(), None, Some(x.to_string()), metadata, "integer-exact-v1".into()))
        }
        _ => {
            let g = uniform_inclusive(rng, 2, 4) as i64;
            let u = uniform_inclusive(rng, 2, 5) as i64;
            let v = uniform_inclusive(rng, 3, 7) as i64;
            if gcd(u, v) == 1 {
                let a = u * g;
                let b = v * g;
                let k = uniform_inclusive(rng, 3, 8) as i64;
                let c = u * k;
                let x = v * k;

                let prompt_text = format!("Solve the proportion for x by simplifying the ratio first: {a}/{b} = {c}/x.");
                let prompt_latex = format!("\\frac{{{a}}}{{{b}}} = \\frac{{{c}}}{{x}} \\implies x = ?");
                let prompt_speech = format!("Solve the proportion {a} over {b} equals {c} over x.");
                let metadata = serde_json::json!({
                    "kind": "reduced_proportion",
                    "term_a": a,
                    "term_b": b,
                    "simplified_a": u,
                    "simplified_b": v,
                    "scale_factor": k,
                    "term_c": c,
                    "target": x,
                });
                return Ok((prompt_text, prompt_latex, prompt_speech, "integer".into(), None, Some(x.to_string()), metadata, "integer-exact-v1".into()));
            }
            let a = 6;
            let b = 9;
            let c = 10;
            let x = 15;
            let prompt_text = format!("Solve the proportion for x by simplifying the ratio first: {a}/{b} = {c}/x.");
            let prompt_latex = format!("\\frac{{{a}}}{{{b}}} = \\frac{{{c}}}{{x}} \\implies x = ?");
            let prompt_speech = format!("Solve the proportion {a} over {b} equals {c} over x.");
            let metadata = serde_json::json!({
                "kind": "reduced_proportion",
                "term_a": a,
                "term_b": b,
                "simplified_a": 2,
                "simplified_b": 3,
                "scale_factor": 5,
                "term_c": c,
                "target": x,
            });
            Ok((prompt_text, prompt_latex, prompt_speech, "integer".into(), None, Some(x.to_string()), metadata, "integer-exact-v1".into()))
        }
    }
}

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
    let prompt_speech = format!("Read the soroban: {}", abacus.describe_state().split(". Value:").next().unwrap_or("Read the bead positions"));

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
    let prompt_latex = format!("\\text{{Partition of 1 whole into }} {} \\text{{ equal parts}} \\text{{; one part = ?}}", d);
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

/// Template: `powers.integer_eval` (Course F06)
fn generate_powers_integer_eval<R: DrawSource + ?Sized>(
    rng: &mut R,
    difficulty: u32,
) -> Result<(String, String, String, String, Option<Vec<String>>, Option<String>, Value, String), GenerationFailure> {
    match difficulty {
        1 => {
            let b = uniform_inclusive(rng, 2, 6) as i64;
            let e = uniform_inclusive(rng, 2, 4) as u32;
            let ans = b.pow(e);

            let prompt_text = format!("Evaluate the power {}^{}.", b, e);
            let prompt_latex = format!("{}^{{{}}} = ?", b, e);
            let prompt_speech = format!("Evaluate {} to the power {}.", b, e);

            let metadata = serde_json::json!({
                "kind": "positive_power",
                "base": b,
                "exponent": e,
                "target": ans,
            });

            Ok((prompt_text, prompt_latex, prompt_speech, "integer".into(), None, Some(ans.to_string()), metadata, "integer-exact-v1".into()))
        }
        2 => {
            let b = uniform_inclusive(rng, 2, 6) as i64;
            let e = uniform_inclusive(rng, 0, 3) as i64;
            let exp = -e;
            if exp == 0 {
                let prompt_text = format!("Evaluate {}^{}.", b, exp);
                let prompt_latex = format!("{}^{{0}} = ?", b);
                let prompt_speech = format!("Evaluate {} to the power 0.", b);
                let metadata = serde_json::json!({
                    "kind": "zero_power",
                    "base": b,
                    "exponent": 0,
                    "target": 1,
                });
                Ok((prompt_text, prompt_latex, prompt_speech, "integer".into(), None, Some("1".into()), metadata, "integer-exact-v1".into()))
            } else {
                let pos_e = exp.abs() as u32;
                let den = b.pow(pos_e);
                let target = format!("1/{}", den);
                let prompt_text = format!("Evaluate {}^{} as an exact simplified fraction.", b, exp);
                let prompt_latex = format!("{}^{{{}}} = ?", b, exp);
                let prompt_speech = format!("Evaluate {} to the power negative {}.", b, pos_e);
                let metadata = serde_json::json!({
                    "kind": "negative_power",
                    "base": b,
                    "exponent": exp,
                    "denominator": den,
                    "target": target,
                });
                Ok((prompt_text, prompt_latex, prompt_speech, "rational".into(), None, Some(target), metadata, "rational-equality-v1".into()))
            }
        }
        _ => {
            let b = match uniform_inclusive(rng, 1, 3) {
                1 => 2i64,
                2 => 3i64,
                _ => 5i64,
            };
            let p = uniform_inclusive(rng, 2, 4) as u32;
            let q = uniform_inclusive(rng, 1, 3) as u32;
            let r = uniform_inclusive(rng, 1, (p + q - 1).min(4) as i128) as u32;
            let net = p + q - r;
            let ans = b.pow(net);

            let prompt_text = format!("Simplify using exponent laws: ({}^{} * {}^{}) / {}^{}.", b, p, b, q, b, r);
            let prompt_latex = format!("\\frac{{{}^{{{}}} \\cdot {}^{{{}}}}}{{{}^{{{}}}}} = ?", b, p, b, q, b, r);
            let prompt_speech = format!("Simplify using exponent laws: {} to the {} times {} to the {} divided by {} to the {}.", b, p, b, q, b, r);

            let metadata = serde_json::json!({
                "kind": "exponent_laws",
                "base": b,
                "p": p,
                "q": q,
                "r": r,
                "net_exponent": net,
                "target": ans,
            });

            Ok((prompt_text, prompt_latex, prompt_speech, "integer".into(), None, Some(ans.to_string()), metadata, "integer-exact-v1".into()))
        }
    }
}

/// Template: `powers.scientific_notation` (Course F06)
fn generate_powers_scientific_notation<R: DrawSource + ?Sized>(
    rng: &mut R,
    difficulty: u32,
) -> Result<(String, String, String, String, Option<Vec<String>>, Option<String>, Value, String), GenerationFailure> {
    match difficulty {
        1 => {
            let m = uniform_inclusive(rng, 2, 9) as i64;
            let k = uniform_inclusive(rng, 2, 5) as u32;
            let ans = m * 10i64.pow(k);

            let prompt_text = format!("Evaluate {} × 10^{} as an exact integer.", m, k);
            let prompt_latex = format!("{} \\times 10^{{{}}} = ?", m, k);
            let prompt_speech = format!("Evaluate {} times 10 to the power {}.", m, k);

            let metadata = serde_json::json!({
                "kind": "integer_scientific",
                "significand": m,
                "exponent": k,
                "target": ans,
            });

            Ok((prompt_text, prompt_latex, prompt_speech, "integer".into(), None, Some(ans.to_string()), metadata, "integer-exact-v1".into()))
        }
        2 => {
            let w = uniform_inclusive(rng, 1, 9) as i64;
            let t = uniform_inclusive(rng, 1, 9) as i64;
            let k = uniform_inclusive(rng, 2, 4) as u32;
            let ans = (w * 10 + t) * 10i64.pow(k - 1);

            let prompt_text = format!("Evaluate {}.{} × 10^{} as an exact integer.", w, t, k);
            let prompt_latex = format!("{}.{} \\times 10^{{{}}} = ?", w, t, k);
            let prompt_speech = format!("Evaluate {}.{} times 10 to the power {}.", w, t, k);

            let metadata = serde_json::json!({
                "kind": "decimal_scientific",
                "whole": w,
                "tenth": t,
                "exponent": k,
                "target": ans,
            });

            Ok((prompt_text, prompt_latex, prompt_speech, "integer".into(), None, Some(ans.to_string()), metadata, "integer-exact-v1".into()))
        }
        _ => {
            let a = uniform_inclusive(rng, 2, 4) as i64;
            let b = uniform_inclusive(rng, 2, 3) as i64;
            let p = uniform_inclusive(rng, 2, 3) as u32;
            let q = uniform_inclusive(rng, 2, 3) as u32;
            let coeff = a * b;
            let total_exp = p + q;
            let ans = coeff * 10i64.pow(total_exp);

            let prompt_text = format!("Calculate: ({} × 10^{}) × ({} × 10^{}). Express as an exact integer.", a, p, b, q);
            let prompt_latex = format!("({} \\times 10^{{{}}}) \\times ({} \\times 10^{{{}}}) = ?", a, p, b, q);
            let prompt_speech = format!("Calculate {} times 10 to the {} multiplied by {} times 10 to the {}.", a, p, b, q);

            let metadata = serde_json::json!({
                "kind": "multiply_scientific",
                "a": a,
                "b": b,
                "p": p,
                "q": q,
                "target": ans,
            });

            Ok((prompt_text, prompt_latex, prompt_speech, "integer".into(), None, Some(ans.to_string()), metadata, "integer-exact-v1".into()))
        }
    }
}

/// Template: `roots.radical_eval` (Course F06)
fn generate_roots_radical_eval<R: DrawSource + ?Sized>(
    rng: &mut R,
    difficulty: u32,
) -> Result<(String, String, String, String, Option<Vec<String>>, Option<String>, Value, String), GenerationFailure> {
    match difficulty {
        1 => {
            let n = uniform_inclusive(rng, 2, 15) as i64;
            let s = n * n;

            let prompt_text = format!("Evaluate the principal square root √{}.", s);
            let prompt_latex = format!("\\sqrt{{{}}} = ?", s);
            let prompt_speech = format!("Evaluate the principal square root of {}.", s);

            let metadata = serde_json::json!({
                "kind": "perfect_square",
                "radicand": s,
                "root": n,
            });

            Ok((prompt_text, prompt_latex, prompt_speech, "integer".into(), None, Some(n.to_string()), metadata, "integer-exact-v1".into()))
        }
        2 => {
            let a_raw = uniform_inclusive(rng, 1, 9) as i64;
            let b_raw = uniform_inclusive(rng, 2, 10) as i64;
            let g = gcd(a_raw, b_raw);
            let (a, b) = if a_raw == b_raw { (1, 2) } else { (a_raw / g, b_raw / g) };
            let sq_a = a * a;
            let sq_b = b * b;
            let target = format!("{}/{}", a, b);

            let prompt_text = format!("Evaluate √({}/{}) as an exact simplified fraction.", sq_a, sq_b);
            let prompt_latex = format!("\\sqrt{{\\frac{{{}}}{{{}}}}} = ?", sq_a, sq_b);
            let prompt_speech = format!("Evaluate the square root of {} over {}.", sq_a, sq_b);

            let metadata = serde_json::json!({
                "kind": "rational_root",
                "num_sq": sq_a,
                "den_sq": sq_b,
                "root_num": a,
                "root_den": b,
                "target": target,
            });

            Ok((prompt_text, prompt_latex, prompt_speech, "rational".into(), None, Some(target), metadata, "rational-equality-v1".into()))
        }
        _ => {
            let a = uniform_inclusive(rng, 2, 8) as i64;
            let mut b = uniform_inclusive(rng, 2, 8) as i64;
            if a == b { b += 1; }
            let sq_a = a * a;
            let sq_b = b * b;
            let ans = a + b;

            let prompt_text = format!("Evaluate the sum of radicals: √{} + √{}.", sq_a, sq_b);
            let prompt_latex = format!("\\sqrt{{{}}} + \\sqrt{{{}}} = ?", sq_a, sq_b);
            let prompt_speech = format!("Evaluate the square root of {} plus the square root of {}.", sq_a, sq_b);

            let metadata = serde_json::json!({
                "kind": "radical_sum",
                "sq_a": sq_a,
                "sq_b": sq_b,
                "root_a": a,
                "root_b": b,
                "target": ans,
            });

            Ok((prompt_text, prompt_latex, prompt_speech, "integer".into(), None, Some(ans.to_string()), metadata, "integer-exact-v1".into()))
        }
    }
}

/// Template: `geometry.perimeter_and_area` (Course F07)
fn generate_geometry_perimeter_and_area<R: DrawSource + ?Sized>(
    rng: &mut R,
    difficulty: u32,
) -> Result<(String, String, String, String, Option<Vec<String>>, Option<String>, Value, String), GenerationFailure> {
    match difficulty {
        1 => {
            let w = uniform_inclusive(rng, 2, 12) as i64;
            let h = uniform_inclusive(rng, 2, 12) as i64;
            let is_perimeter = uniform_inclusive(rng, 0, 1) == 0;
            if is_perimeter {
                let p = 2 * (w + h);
                let prompt_text = format!("A rectangle has width {} and height {}. Calculate its perimeter.", w, h);
                let prompt_latex = format!("\\text{{Perimeter of rectangle with width }} {} \\text{{ and height }} {} = ?", w, h);
                let prompt_speech = format!("Find the perimeter of a rectangle with width {} and height {}.", w, h);
                let metadata = serde_json::json!({
                    "kind": "rectangle_perimeter",
                    "width": w,
                    "height": h,
                    "target": p,
                });
                Ok((prompt_text, prompt_latex, prompt_speech, "integer".into(), None, Some(p.to_string()), metadata, "integer-exact-v1".into()))
            } else {
                let a = w * h;
                let prompt_text = format!("A rectangle has width {} and height {}. Calculate its area.", w, h);
                let prompt_latex = format!("\\text{{Area of rectangle with width }} {} \\text{{ and height }} {} = ?", w, h);
                let prompt_speech = format!("Find the area of a rectangle with width {} and height {}.", w, h);
                let metadata = serde_json::json!({
                    "kind": "rectangle_area",
                    "width": w,
                    "height": h,
                    "target": a,
                });
                Ok((prompt_text, prompt_latex, prompt_speech, "integer".into(), None, Some(a.to_string()), metadata, "integer-exact-v1".into()))
            }
        }
        2 => {
            let b_half = uniform_inclusive(rng, 2, 8) as i64;
            let b = b_half * 2;
            let h = uniform_inclusive(rng, 3, 14) as i64;
            let area = b_half * h;
            let prompt_text = format!("A triangle has a base of {} units and a perpendicular height of {} units. Calculate its area.", b, h);
            let prompt_latex = format!("\\text{{Area of triangle with base }} {} \\text{{ and height }} {} = \\frac{{1}}{{2}} \\cdot {} \\cdot {} = ?", b, h, b, h);
            let prompt_speech = format!("Find the area of a triangle with base {} and perpendicular height {}.", b, h);
            let metadata = serde_json::json!({
                "kind": "triangle_area",
                "base": b,
                "height": h,
                "target": area,
            });
            Ok((prompt_text, prompt_latex, prompt_speech, "integer".into(), None, Some(area.to_string()), metadata, "integer-exact-v1".into()))
        }
        _ => {
            let w = uniform_inclusive(rng, 3, 12) as i64;
            let h = uniform_inclusive(rng, 3, 12) as i64;
            let area = w * h;
            let p = 2 * (w + h);
            let prompt_text = format!("A rectangle has an area of {} square units and a width of {} units. Calculate its perimeter.", area, w);
            let prompt_latex = format!("\\text{{Perimeter of rectangle with Area }} {} \\text{{ and width }} {} = ?", area, w);
            let prompt_speech = format!("A rectangle has area {} square units and width {} units. Calculate its perimeter.", area, w);
            let metadata = serde_json::json!({
                "kind": "rectangle_missing_dim",
                "width": w,
                "height": h,
                "area": area,
                "target": p,
            });
            Ok((prompt_text, prompt_latex, prompt_speech, "integer".into(), None, Some(p.to_string()), metadata, "integer-exact-v1".into()))
        }
    }
}

/// Template: `geometry.angles_triangles` (Course F07)
fn generate_geometry_angles_triangles<R: DrawSource + ?Sized>(
    rng: &mut R,
    difficulty: u32,
) -> Result<(String, String, String, String, Option<Vec<String>>, Option<String>, Value, String), GenerationFailure> {
    match difficulty {
        1 => {
            let a = uniform_inclusive(rng, 30, 80) as i64;
            let b = uniform_inclusive(rng, 30, (150 - a).max(31) as i128) as i64;
            let c = 180 - a - b;
            let prompt_text = format!("Two interior angles of a triangle measure {}° and {}°. What is the measure of the third angle in degrees?", a, b);
            let prompt_latex = format!("{}° + {}° + \\theta = 180° \\implies \\theta = ?", a, b);
            let prompt_speech = format!("Two interior angles of a triangle measure {} degrees and {} degrees. What is the measure of the third angle?", a, b);
            let metadata = serde_json::json!({
                "kind": "triangle_angle_sum",
                "angle_a": a,
                "angle_b": b,
                "target": c,
            });
            Ok((prompt_text, prompt_latex, prompt_speech, "integer".into(), None, Some(c.to_string()), metadata, "integer-exact-v1".into()))
        }
        2 => {
            let is_supp = uniform_inclusive(rng, 0, 1) == 0;
            if is_supp {
                let theta = uniform_inclusive(rng, 25, 155) as i64;
                let target = 180 - theta;
                let prompt_text = format!("Two angles form a linear pair and sum to 180°. If one angle is {}°, what is the measure of the supplementary angle in degrees?", theta);
                let prompt_latex = format!("{}° + \\theta = 180° \\implies \\theta = ?", theta);
                let prompt_speech = format!("If one angle in a linear pair is {} degrees, what is the supplementary angle?", theta);
                let metadata = serde_json::json!({
                    "kind": "supplementary_angle",
                    "theta": theta,
                    "target": target,
                });
                Ok((prompt_text, prompt_latex, prompt_speech, "integer".into(), None, Some(target.to_string()), metadata, "integer-exact-v1".into()))
            } else {
                let theta = uniform_inclusive(rng, 15, 75) as i64;
                let target = 90 - theta;
                let prompt_text = format!("Two angles are complementary and sum to 90°. If one angle is {}°, what is the measure of the other angle in degrees?", theta);
                let prompt_latex = format!("{}° + \\theta = 90° \\implies \\theta = ?", theta);
                let prompt_speech = format!("If one angle is {} degrees and they are complementary, what is the other angle?", theta);
                let metadata = serde_json::json!({
                    "kind": "complementary_angle",
                    "theta": theta,
                    "target": target,
                });
                Ok((prompt_text, prompt_latex, prompt_speech, "integer".into(), None, Some(target.to_string()), metadata, "integer-exact-v1".into()))
            }
        }
        _ => {
            let v_half = uniform_inclusive(rng, 10, 70) as i64;
            let v = v_half * 2;
            let base_angle = (180 - v) / 2;
            let prompt_text = format!("An isosceles triangle has a vertex angle measuring {}°. What is the measure of each of the two equal base angles in degrees?", v);
            let prompt_latex = format!("{}° + 2\\beta = 180° \\implies \\beta = ?", v);
            let prompt_speech = format!("An isosceles triangle has a vertex angle of {} degrees. What is the measure of each base angle?", v);
            let metadata = serde_json::json!({
                "kind": "isosceles_base_angle",
                "vertex_angle": v,
                "target": base_angle,
            });
            Ok((prompt_text, prompt_latex, prompt_speech, "integer".into(), None, Some(base_angle.to_string()), metadata, "integer-exact-v1".into()))
        }
    }
}

/// Template: `geometry.coordinate_distance` (Course F07)
fn generate_geometry_coordinate_distance<R: DrawSource + ?Sized>(
    rng: &mut R,
    difficulty: u32,
) -> Result<(String, String, String, String, Option<Vec<String>>, Option<String>, Value, String), GenerationFailure> {
    match difficulty {
        1 => {
            let y = uniform_inclusive(rng, -10, 10) as i64;
            let x1 = uniform_inclusive(rng, -9, 0) as i64;
            let diff = uniform_inclusive(rng, 2, 12) as i64;
            let x2 = x1 + diff;
            let prompt_text = format!("Find the distance between the points ({}, {}) and ({}, {}) on the Cartesian coordinate plane.", x1, y, x2, y);
            let prompt_latex = format!("d(({}, {}), ({}, {})) = ?", x1, y, x2, y);
            let prompt_speech = format!("Find the distance between the points {} comma {} and {} comma {}.", x1, y, x2, y);
            let metadata = serde_json::json!({
                "kind": "axis_aligned_distance",
                "x1": x1,
                "y1": y,
                "x2": x2,
                "y2": y,
                "target": diff,
            });
            Ok((prompt_text, prompt_latex, prompt_speech, "integer".into(), None, Some(diff.to_string()), metadata, "integer-exact-v1".into()))
        }
        2 => {
            let triples: &[(i64, i64, i64)] = &[
                (3, 4, 5),
                (5, 12, 13),
                (6, 8, 10),
                (8, 15, 17),
                (9, 12, 15),
            ];
            let idx = uniform_inclusive(rng, 0, (triples.len() - 1) as i128) as usize;
            let (dx, dy, dist) = triples[idx];
            let x1 = uniform_inclusive(rng, -5, 5) as i64;
            let y1 = uniform_inclusive(rng, -5, 5) as i64;
            let sx = if uniform_inclusive(rng, 0, 1) == 0 { 1 } else { -1 };
            let sy = if uniform_inclusive(rng, 0, 1) == 0 { 1 } else { -1 };
            let x2 = x1 + sx * dx;
            let y2 = y1 + sy * dy;
            let prompt_text = format!("Find the Euclidean distance between the points ({}, {}) and ({}, {}) on the Cartesian plane.", x1, y1, x2, y2);
            let prompt_latex = format!("d(({}, {}), ({}, {})) = \\sqrt{{(\\Delta x)^2 + (\\Delta y)^2}} = ?", x1, y1, x2, y2);
            let prompt_speech = format!("Find the Euclidean distance between the points {} comma {} and {} comma {}.", x1, y1, x2, y2);
            let metadata = serde_json::json!({
                "kind": "pythagorean_distance",
                "x1": x1,
                "y1": y1,
                "x2": x2,
                "y2": y2,
                "dx": dx,
                "dy": dy,
                "target": dist,
            });
            Ok((prompt_text, prompt_latex, prompt_speech, "integer".into(), None, Some(dist.to_string()), metadata, "integer-exact-v1".into()))
        }
        _ => {
            let x1 = uniform_inclusive(rng, -8, 2) as i64;
            let y1 = uniform_inclusive(rng, -8, 2) as i64;
            let w = uniform_inclusive(rng, 3, 10) as i64;
            let h = uniform_inclusive(rng, 3, 10) as i64;
            let x2 = x1 + w;
            let y2 = y1 + h;
            let area = w * h;
            let prompt_text = format!("A rectangle with sides parallel to the coordinate axes has opposite diagonal vertices at ({}, {}) and ({}, {}). Find its area.", x1, y1, x2, y2);
            let prompt_latex = format!("\\text{{Area of coordinate rectangle with vertices }} ({}, {}) \\text{{ and }} ({}, {}) = ?", x1, y1, x2, y2);
            let prompt_speech = format!("Find the area of the rectangle with opposite vertices at {} comma {} and {} comma {}.", x1, y1, x2, y2);
            let metadata = serde_json::json!({
                "kind": "coordinate_rectangle_area",
                "x1": x1,
                "y1": y1,
                "x2": x2,
                "y2": y2,
                "width": w,
                "height": h,
                "target": area,
            });
            Ok((prompt_text, prompt_latex, prompt_speech, "integer".into(), None, Some(area.to_string()), metadata, "integer-exact-v1".into()))
        }
    }
}

/// Template: `data.mean_median_calc` (Course F08)
fn generate_data_mean_median_calc<R: DrawSource + ?Sized>(
    rng: &mut R,
    difficulty: u32,
) -> Result<(String, String, String, String, Option<Vec<String>>, Option<String>, Value, String), GenerationFailure> {
    match difficulty {
        1 => {
            let m = uniform_inclusive(rng, 12, 32) as i64;
            let d1 = uniform_inclusive(rng, 1, 4) as i64;
            let d2 = uniform_inclusive(rng, 5, 9) as i64;
            let v1 = m - d2;
            let v2 = m + d1;
            let v3 = m;
            let v4 = m - d1;
            let v5 = m + d2;
            let sum = 5 * m;
            let prompt_text = format!("Find the arithmetic mean (average) of the dataset: {}, {}, {}, {}, {}.", v1, v2, v3, v4, v5);
            let prompt_latex = format!("\\text{{Mean}}(\\{{{}, {}, {}, {}, {}\\}}) = \\frac{{{} + {} + {} + {} + {}}}{{5}} = ?", v1, v2, v3, v4, v5, v1, v2, v3, v4, v5);
            let prompt_speech = format!("Find the arithmetic mean of the five values {}, {}, {}, {}, and {}.", v1, v2, v3, v4, v5);
            let metadata = serde_json::json!({
                "kind": "mean_balance",
                "v1": v1,
                "v2": v2,
                "v3": v3,
                "v4": v4,
                "v5": v5,
                "sum": sum,
                "count": 5,
                "target": m,
            });
            Ok((prompt_text, prompt_latex, prompt_speech, "integer".into(), None, Some(m.to_string()), metadata, "integer-exact-v1".into()))
        }
        2 => {
            let base = uniform_inclusive(rng, 8, 25) as i64;
            let g1 = uniform_inclusive(rng, 2, 5) as i64;
            let g2 = uniform_inclusive(rng, 2, 5) as i64;
            let g3 = uniform_inclusive(rng, 2, 5) as i64;
            let g4 = uniform_inclusive(rng, 3, 7) as i64;
            let s1 = base;
            let s2 = s1 + g1;
            let s3 = s2 + g2;
            let s4 = s3 + g3;
            let s5 = s4 + g4;
            // Present in unsorted order [s4, s1, s5, s2, s3] so middle element before sorting is s5
            let prompt_text = format!("Find the median of the following unsorted dataset: {}, {}, {}, {}, {}.", s4, s1, s5, s2, s3);
            let prompt_latex = format!("\\text{{Median}}(\\{{{}, {}, {}, {}, {}\\}}) = ?", s4, s1, s5, s2, s3);
            let prompt_speech = format!("Find the median of the unsorted dataset {}, {}, {}, {}, and {}.", s4, s1, s5, s2, s3);
            let metadata = serde_json::json!({
                "kind": "median_unsorted",
                "u1": s4,
                "u2": s1,
                "u3": s5,
                "u4": s2,
                "u5": s3,
                "s1": s1,
                "s2": s2,
                "s3": s3,
                "s4": s4,
                "s5": s5,
                "unsorted_middle": s5,
                "target": s3,
            });
            Ok((prompt_text, prompt_latex, prompt_speech, "integer".into(), None, Some(s3.to_string()), metadata, "integer-exact-v1".into()))
        }
        _ => {
            let m = uniform_inclusive(rng, 70, 90) as i64;
            let d1 = uniform_inclusive(rng, -6, 6) as i64;
            let d2 = uniform_inclusive(rng, -8, 5) as i64;
            let d3 = uniform_inclusive(rng, -4, 7) as i64;
            let d4 = uniform_inclusive(rng, -5, 4) as i64;
            let x1 = m + d1;
            let x2 = m + d2;
            let x3 = m + d3;
            let x4 = m + d4;
            let known_sum = x1 + x2 + x3 + x4;
            let total_needed = 5 * m;
            let x5 = total_needed - known_sum;
            let prompt_text = format!("A student's first four scores are {}, {}, {}, and {}. What score is required on the 5th assessment so that the arithmetic mean of all 5 scores is exactly {}?", x1, x2, x3, x4, m);
            let prompt_latex = format!("\\frac{{{} + {} + {} + {} + x_5}}{{5}} = {} \\implies x_5 = ?", x1, x2, x3, x4, m);
            let prompt_speech = format!("Four scores are {}, {}, {}, and {}. What fifth score gives an exact mean of {}?", x1, x2, x3, x4, m);
            let metadata = serde_json::json!({
                "kind": "missing_value_for_mean",
                "x1": x1,
                "x2": x2,
                "x3": x3,
                "x4": x4,
                "known_sum": known_sum,
                "target_mean": m,
                "total_needed": total_needed,
                "target": x5,
            });
            Ok((prompt_text, prompt_latex, prompt_speech, "integer".into(), None, Some(x5.to_string()), metadata, "integer-exact-v1".into()))
        }
    }
}

/// Template: `counting.permutations_combinations` (Course F08)
fn generate_counting_permutations_combinations<R: DrawSource + ?Sized>(
    rng: &mut R,
    difficulty: u32,
) -> Result<(String, String, String, String, Option<Vec<String>>, Option<String>, Value, String), GenerationFailure> {
    match difficulty {
        1 => {
            let a = uniform_inclusive(rng, 3, 6) as i64;
            let b = uniform_inclusive(rng, 2, 5) as i64;
            let c = uniform_inclusive(rng, 2, 4) as i64;
            let total = a * b * c;
            let additive_trap = a + b + c;
            let prompt_text = format!("A security badge configuration chooses 1 of {} zone prefixes, 1 of {} department codes, and 1 of {} access levels. How many distinct badge configurations are possible?", a, b, c);
            let prompt_latex = format!("{} \\times {} \\times {} = ?", a, b, c);
            let prompt_speech = format!("How many configurations can be formed from {} prefixes, {} department codes, and {} access levels?", a, b, c);
            let metadata = serde_json::json!({
                "kind": "multiplication_principle",
                "a": a,
                "b": b,
                "c": c,
                "additive_trap": additive_trap,
                "target": total,
            });
            Ok((prompt_text, prompt_latex, prompt_speech, "integer".into(), None, Some(total.to_string()), metadata, "integer-exact-v1".into()))
        }
        2 => {
            let n = uniform_inclusive(rng, 5, 9) as i64;
            let perm = n * (n - 1) * (n - 2);
            let with_replacement_trap = n * n * n;
            let prompt_text = format!("In a finals heat with {} distinct competitors and no ties, in how many different ways can 1st, 2nd, and 3rd place be awarded?", n);
            let prompt_latex = format!("P({}, 3) = {} \\times {} \\times {} = ?", n, n, n - 1, n - 2);
            let prompt_speech = format!("From {} competitors, in how many ordered ways can first, second, and third place be awarded?", n);
            let metadata = serde_json::json!({
                "kind": "ordered_permutation",
                "n": n,
                "k": 3,
                "with_replacement_trap": with_replacement_trap,
                "target": perm,
            });
            Ok((prompt_text, prompt_latex, prompt_speech, "integer".into(), None, Some(perm.to_string()), metadata, "integer-exact-v1".into()))
        }
        _ => {
            let n = uniform_inclusive(rng, 6, 10) as i64;
            let k = if uniform_inclusive(rng, 0, 1) == 0 { 2i64 } else { 3i64 };
            let (ordered_count, fact_k, comb) = if k == 2 {
                let ord = n * (n - 1);
                (ord, 2i64, ord / 2)
            } else {
                let ord = n * (n - 1) * (n - 2);
                (ord, 6i64, ord / 6)
            };
            let prompt_text = format!("A research team must select an unordered committee of {} members from {} eligible mathematicians. How many distinct committees of {} can be chosen?", k, n, k);
            let prompt_latex = format!("\\binom{{{}}}{{{}}} = \\frac{{P({}, {})}}{{{}!}} = ?", n, k, n, k, k);
            let prompt_speech = format!("How many unordered committees of {} can be chosen from {} eligible people?", k, n);
            let metadata = serde_json::json!({
                "kind": "unordered_combination",
                "n": n,
                "k": k,
                "ordered_count": ordered_count,
                "factorial_k": fact_k,
                "permutation_trap": ordered_count,
                "target": comb,
            });
            Ok((prompt_text, prompt_latex, prompt_speech, "integer".into(), None, Some(comb.to_string()), metadata, "integer-exact-v1".into()))
        }
    }
}

/// Template: `probability.simple_exact` (Course F08)
fn generate_probability_simple_exact<R: DrawSource + ?Sized>(
    rng: &mut R,
    difficulty: u32,
) -> Result<(String, String, String, String, Option<Vec<String>>, Option<String>, Value, String), GenerationFailure> {
    match difficulty {
        1 => {
            let r = uniform_inclusive(rng, 2, 7) as i64;
            let b = uniform_inclusive(rng, 3, 8) as i64;
            let g = uniform_inclusive(rng, 1, 6) as i64;
            let total = r + b + g;
            let q = frac(r, total);
            let target_str = to_canonical(&q);
            let odds_trap = to_canonical(&frac(r, b + g));
            let prompt_text = format!("A bag contains {} red tokens, {} blue tokens, and {} green tokens. One token is drawn at random. What is the exact probability of drawing a red token? Express your answer as a fraction.", r, b, g);
            let prompt_latex = format!("P(\\text{{red}}) = \\frac{{{}}}{{{} + {} + {}}} = ?", r, r, b, g);
            let prompt_speech = format!("A bag has {} red, {} blue, and {} green tokens. What is the exact probability of drawing a red token?", r, b, g);
            let metadata = serde_json::json!({
                "kind": "single_event_probability",
                "red": r,
                "blue": b,
                "green": g,
                "favorable": r,
                "total": total,
                "odds_trap": odds_trap,
                "target": target_str,
            });
            Ok((prompt_text, prompt_latex, prompt_speech, "rational".into(), None, Some(target_str), metadata, "rational-equality-v1".into()))
        }
        2 => {
            let total = uniform_inclusive(rng, 10, 24) as i64;
            let fav = uniform_inclusive(rng, 2, (total - 3) as i128) as i64;
            let comp = total - fav;
            let q = frac(comp, total);
            let target_str = to_canonical(&q);
            let direct_trap = to_canonical(&frac(fav, total));
            let prompt_text = format!("An experiment has {} equally likely outcomes, and event E occurs in exactly {} of those outcomes. What is the exact probability that event E does NOT occur?", total, fav);
            let prompt_latex = format!("P(\\text{{not }} E) = 1 - \\frac{{{}}}{{{}}} = ?", fav, total);
            let prompt_speech = format!("An experiment has {} equally likely outcomes and event E occurs in {} of them. What is the probability that E does not occur?", total, fav);
            let metadata = serde_json::json!({
                "kind": "complementary_probability",
                "event_outcomes": fav,
                "complement_outcomes": comp,
                "total": total,
                "direct_trap": direct_trap,
                "target": target_str,
            });
            Ok((prompt_text, prompt_latex, prompt_speech, "rational".into(), None, Some(target_str), metadata, "rational-equality-v1".into()))
        }
        _ => {
            let gold = uniform_inclusive(rng, 3, 6) as i64;
            let silver = uniform_inclusive(rng, 3, 6) as i64;
            let total = gold + silver;
            let num = gold * (gold - 1);
            let den = total * (total - 1);
            let q = frac(num, den);
            let target_str = to_canonical(&q);
            let with_replacement_trap = to_canonical(&frac(gold * gold, total * total));
            let prompt_text = format!("A pouch contains {} gold coins and {} silver coins. Two coins are drawn one after another WITHOUT replacement. What is the exact probability that both coins drawn are gold?", gold, silver);
            let prompt_latex = format!("P(\\text{{both gold}}) = \\frac{{{}}}{{{}}} \\times \\frac{{{}}}{{{}}} = ?", gold, total, gold - 1, total - 1);
            let prompt_speech = format!("A pouch has {} gold and {} silver coins. Two are drawn without replacement. What is the probability both are gold?", gold, silver);
            let metadata = serde_json::json!({
                "kind": "two_step_without_replacement",
                "gold": gold,
                "silver": silver,
                "total": total,
                "num": num,
                "den": den,
                "with_replacement_trap": with_replacement_trap,
                "target": target_str,
            });
            Ok((prompt_text, prompt_latex, prompt_speech, "rational".into(), None, Some(target_str), metadata, "rational-equality-v1".into()))
        }
    }
}

/// Template: `algebra.expression_eval` (Course S01)
fn generate_algebra_expression_eval<R: DrawSource + ?Sized>(
    rng: &mut R,
    difficulty: u32,
) -> Result<(String, String, String, String, Option<Vec<String>>, Option<String>, Value, String), GenerationFailure> {
    match difficulty {
        1 => {
            let a = uniform_inclusive(rng, 2, 4) as i64;
            let b = uniform_inclusive(rng, 2, 6) as i64;
            let c = uniform_inclusive(rng, -7, 7) as i64;
            let x = -(uniform_inclusive(rng, 2, 5) as i64);
            let val = a * x * x + b * x + c;
            let sign_trap = -a * x * x + b * x + c;
            let c_str = if c >= 0 { format!("+ {}", c) } else { format!("- {}", -c) };
            let prompt_text = format!("Evaluate the quadratic expression {}x² + {}x {} when x = {}.", a, b, c_str, x);
            let prompt_latex = format!("{}x^2 + {}x {} \\Big|_{{x = {}}} = ?", a, b, c_str, x);
            let prompt_speech = format!("Evaluate {} x squared plus {} x {} when x equals {}.", a, b, c_str, x);
            let metadata = serde_json::json!({
                "kind": "quadratic_substitution",
                "a": a,
                "b": b,
                "c": c,
                "x": x,
                "sign_trap": sign_trap,
                "target": val,
            });
            Ok((prompt_text, prompt_latex, prompt_speech, "integer".into(), None, Some(val.to_string()), metadata, "integer-exact-v1".into()))
        }
        2 => {
            let p = uniform_inclusive(rng, 2, 5) as i64;
            let q = uniform_inclusive(rng, 2, 4) as i64;
            let x = uniform_inclusive(rng, 2, 6) as i64;
            let y = -(uniform_inclusive(rng, 2, 5) as i64);
            let val = p * (x - y) + q * x * y;
            let sign_trap = p * (x + y) + q * x * y;
            let prompt_text = format!("Evaluate the two-variable expression {}(x - y) + {}xy when x = {} and y = {}.", p, q, x, y);
            let prompt_latex = format!("{}(x - y) + {}xy \\Big|_{{x={},\\, y={}}} = ?", p, q, x, y);
            let prompt_speech = format!("Evaluate {} times open parenthesis x minus y close parenthesis plus {} x y when x equals {} and y equals {}.", p, q, x, y);
            let metadata = serde_json::json!({
                "kind": "two_variable_eval",
                "p": p,
                "q": q,
                "x": x,
                "y": y,
                "sign_trap": sign_trap,
                "target": val,
            });
            Ok((prompt_text, prompt_latex, prompt_speech, "integer".into(), None, Some(val.to_string()), metadata, "integer-exact-v1".into()))
        }
        _ => {
            let a = uniform_inclusive(rng, 3, 5) as i64;
            let b = uniform_inclusive(rng, 2, 4) as i64;
            let c = uniform_inclusive(rng, 2, 5) as i64;
            let d = uniform_inclusive(rng, 2, 4) as i64;
            let e = uniform_inclusive(rng, 1, 3) as i64;
            let f = uniform_inclusive(rng, 2, 6) as i64;
            let x0 = uniform_inclusive(rng, 2, 4) as i64;
            let net_x_coeff = a * b - d * e;
            let net_const = a * c + d * f;
            let val = net_x_coeff * x0 + net_const;
            let sign_trap = net_x_coeff * x0 + (a * c - d * f);
            let prompt_text = format!("Expand and simplify {}({}x + {}) - {}({}x - {}), then evaluate the expression at x = {}.", a, b, c, d, e, f, x0);
            let prompt_latex = format!("{}({}x + {}) - {}({}x - {}) \\Big|_{{x = {}}} = ?", a, b, c, d, e, f, x0);
            let prompt_speech = format!("Expand {} times {} x plus {}, minus {} times {} x minus {}, and evaluate at x equals {}.", a, b, c, d, e, f, x0);
            let metadata = serde_json::json!({
                "kind": "distributive_coeff_simplify",
                "a": a,
                "b": b,
                "c": c,
                "d": d,
                "e": e,
                "f": f,
                "x0": x0,
                "net_x_coeff": net_x_coeff,
                "net_const": net_const,
                "sign_trap": sign_trap,
                "target": val,
            });
            Ok((prompt_text, prompt_latex, prompt_speech, "integer".into(), None, Some(val.to_string()), metadata, "integer-exact-v1".into()))
        }
    }
}

/// Template: `algebra.linear_equation_solve` (Course S01)
fn generate_algebra_linear_equation_solve<R: DrawSource + ?Sized>(
    rng: &mut R,
    difficulty: u32,
) -> Result<(String, String, String, String, Option<Vec<String>>, Option<String>, Value, String), GenerationFailure> {
    match difficulty {
        1 => {
            let a = uniform_inclusive(rng, 2, 7) as i64;
            let mut x = uniform_inclusive(rng, -5, 9) as i64;
            if x == 0 { x = 4; }
            let b = uniform_inclusive(rng, 2, 14) as i64;
            let c = a * x + b;
            let prompt_text = format!("Solve the linear equation for x: {}x + {} = {}.", a, b, c);
            let prompt_latex = format!("{}x + {} = {} \\implies x = ?", a, b, c);
            let prompt_speech = format!("Solve for x: {} x plus {} equals {}.", a, b, c);
            let metadata = serde_json::json!({
                "kind": "two_step_linear",
                "a": a,
                "b": b,
                "c": c,
                "rhs_after_sub": c - b,
                "target": x,
            });
            Ok((prompt_text, prompt_latex, prompt_speech, "integer".into(), None, Some(x.to_string()), metadata, "integer-exact-v1".into()))
        }
        2 => {
            let a = uniform_inclusive(rng, 5, 10) as i64;
            let c = uniform_inclusive(rng, 2, (a - 2) as i128) as i64;
            let mut x = uniform_inclusive(rng, -5, 8) as i64;
            if x == 0 { x = 3; }
            let b = uniform_inclusive(rng, 1, 12) as i64;
            let d = (a - c) * x + b;
            let d_str = if d >= 0 { format!("+ {}", d) } else { format!("- {}", -d) };
            let prompt_text = format!("Solve the equation with variables on both sides for x: {}x + {} = {}x {}.", a, b, c, d_str);
            let prompt_latex = format!("{}x + {} = {}x {} \\implies x = ?", a, b, c, d_str);
            let prompt_speech = format!("Solve for x: {} x plus {} equals {} x {}.", a, b, c, d_str);
            let metadata = serde_json::json!({
                "kind": "variables_both_sides",
                "a": a,
                "b": b,
                "c": c,
                "d": d,
                "net_coeff": a - c,
                "net_rhs": d - b,
                "target": x,
            });
            Ok((prompt_text, prompt_latex, prompt_speech, "integer".into(), None, Some(x.to_string()), metadata, "integer-exact-v1".into()))
        }
        _ => {
            let k = uniform_inclusive(rng, 2, 4) as i64;
            let a = uniform_inclusive(rng, 2, 4) as i64;
            let lhs_coeff = k * a;
            let c = uniform_inclusive(rng, 1, (lhs_coeff - 2) as i128) as i64;
            let net_coeff = lhs_coeff - c;
            let num = uniform_inclusive(rng, 3, 17) as i64;
            let b = uniform_inclusive(rng, 1, 5) as i64;
            let d = num + k * b;
            let q = frac(num, net_coeff);
            let target_str = to_canonical(&q);
            // Undistributed trap: if learner writes k*a*x + b = c*x + d => (ka - c)x = d - b
            let undist_trap = to_canonical(&frac(d - b, net_coeff));
            let prompt_text = format!("Solve the distributive linear equation for x (express your answer as an exact integer or fraction): {}({}x + {}) = {}x + {}.", k, a, b, c, d);
            let prompt_latex = format!("{}({}x + {}) = {}x + {} \\implies x = ?", k, a, b, c, d);
            let prompt_speech = format!("Solve for x: {} times open parenthesis {} x plus {} close parenthesis equals {} x plus {}.", k, a, b, c, d);
            let metadata = serde_json::json!({
                "kind": "distributive_rational_linear",
                "k": k,
                "a": a,
                "b": b,
                "c": c,
                "d": d,
                "lhs_x_coeff": lhs_coeff,
                "lhs_const": k * b,
                "net_coeff": net_coeff,
                "net_rhs": num,
                "undist_trap": undist_trap,
                "target": target_str,
            });
            Ok((prompt_text, prompt_latex, prompt_speech, "rational".into(), None, Some(target_str), metadata, "rational-equality-v1".into()))
        }
    }
}

/// Template: `algebra.linear_system_solve` (Course S01)
fn generate_algebra_linear_system_solve<R: DrawSource + ?Sized>(
    rng: &mut R,
    difficulty: u32,
) -> Result<(String, String, String, String, Option<Vec<String>>, Option<String>, Value, String), GenerationFailure> {
    match difficulty {
        1 => {
            let x = uniform_inclusive(rng, 4, 12) as i64;
            let y = uniform_inclusive(rng, 1, (x - 1) as i128) as i64;
            let s = x + y;
            let d = x - y;
            let prompt_text = format!("Solve the system of linear equations for x: x + y = {} and x - y = {}.", s, d);
            let prompt_latex = format!("\\begin{{cases}} x + y = {} \\\\ x - y = {} \\end{{cases}} \\implies x = ?", s, d);
            let prompt_speech = format!("Solve the system x plus y equals {} and x minus y equals {} for x.", s, d);
            let metadata = serde_json::json!({
                "kind": "sum_difference_system",
                "x": x,
                "y": y,
                "s": s,
                "d": d,
                "y_trap": y,
                "target": x,
            });
            Ok((prompt_text, prompt_latex, prompt_speech, "integer".into(), None, Some(x.to_string()), metadata, "integer-exact-v1".into()))
        }
        2 => {
            let x = uniform_inclusive(rng, 2, 7) as i64;
            let m = uniform_inclusive(rng, 2, 4) as i64;
            let b = uniform_inclusive(rng, 1, 5) as i64;
            let y = m * x + b;
            let c = uniform_inclusive(rng, 2, 4) as i64;
            let d = uniform_inclusive(rng, 2, 3) as i64;
            let e = c * x + d * y;
            let combined_x_coeff = c + d * m;
            let const_term = d * b;
            let prompt_text = format!("Solve the system of equations for x using substitution: y = {}x + {} and {}x + {}y = {}.", m, b, c, d, e);
            let prompt_latex = format!("\\begin{{cases}} y = {}x + {} \\\\ {}x + {}y = {} \\end{{cases}} \\implies x = ?", m, b, c, d, e);
            let prompt_speech = format!("Solve the system y equals {} x plus {} and {} x plus {} y equals {} for x.", m, b, c, d, e);
            let metadata = serde_json::json!({
                "kind": "substitution_system",
                "m": m,
                "b": b,
                "c": c,
                "d": d,
                "e": e,
                "x": x,
                "y": y,
                "combined_x_coeff": combined_x_coeff,
                "const_term": const_term,
                "y_trap": y,
                "target": x,
            });
            Ok((prompt_text, prompt_latex, prompt_speech, "integer".into(), None, Some(x.to_string()), metadata, "integer-exact-v1".into()))
        }
        _ => {
            let x = uniform_inclusive(rng, 2, 7) as i64;
            let y = uniform_inclusive(rng, 2, 6) as i64;
            let a1 = uniform_inclusive(rng, 2, 5) as i64;
            let b1 = uniform_inclusive(rng, 2, 4) as i64;
            let a2 = uniform_inclusive(rng, 2, 5) as i64;
            let b2 = -2 * b1;
            let c1 = a1 * x + b1 * y;
            let c2 = a2 * x + b2 * y;
            let elim_coeff = 2 * a1 + a2;
            let elim_rhs = 2 * c1 + c2;
            let prompt_text = format!("Solve the linear system for x by elimination: {}x + {}y = {} and {}x - {}y = {}.", a1, b1, c1, a2, -b2, c2);
            let prompt_latex = format!("\\begin{{cases}} {}x + {}y = {} \\\\ {}x - {}y = {} \\end{{cases}} \\implies x = ?", a1, b1, c1, a2, -b2, c2);
            let prompt_speech = format!("Solve the linear system {} x plus {} y equals {} and {} x minus {} y equals {} for x.", a1, b1, c1, a2, -b2, c2);
            let metadata = serde_json::json!({
                "kind": "elimination_system",
                "a1": a1,
                "b1": b1,
                "c1": c1,
                "a2": a2,
                "b2": b2,
                "c2": c2,
                "x": x,
                "y": y,
                "elim_coeff": elim_coeff,
                "elim_rhs": elim_rhs,
                "y_trap": y,
                "target": x,
            });
            Ok((prompt_text, prompt_latex, prompt_speech, "integer".into(), None, Some(x.to_string()), metadata, "integer-exact-v1".into()))
        }
    }
}

/// Template: `functions.evaluate_and_range` (Course S02)
fn generate_functions_evaluate_and_range<R: DrawSource + ?Sized>(
    rng: &mut R,
    difficulty: u32,
) -> Result<(String, String, String, String, Option<Vec<String>>, Option<String>, Value, String), GenerationFailure> {
    match difficulty {
        1 => {
            let m = uniform_inclusive(rng, 2, 6) as i64;
            let b = uniform_inclusive(rng, -9, 9) as i64;
            let mut x = uniform_inclusive(rng, -5, 8) as i64;
            if x == 0 { x = 3; }
            let y0 = m * x + b;
            let direct_eval_trap = m * y0 + b;
            let b_str = if b >= 0 { format!("+ {}", b) } else { format!("- {}", -b) };
            let prompt_text = format!("For the linear function f(x) = {}x {}, find the exact domain input x such that f(x) = {}.", m, b_str, y0);
            let prompt_latex = format!("f(x) = {}x {}, \\quad f(x) = {} \\implies x = ?", m, b_str, y0);
            let prompt_speech = format!("For f of x equals {} x {}, find the input x where f of x equals {}.", m, b_str, y0);
            let metadata = serde_json::json!({
                "kind": "preimage_linear",
                "m": m,
                "b": b,
                "y0": y0,
                "direct_eval_trap": direct_eval_trap,
                "target": x,
            });
            Ok((prompt_text, prompt_latex, prompt_speech, "integer".into(), None, Some(x.to_string()), metadata, "integer-exact-v1".into()))
        }
        2 => {
            let a = uniform_inclusive(rng, 2, 5) as i64;
            let b = uniform_inclusive(rng, 1, 8) as i64;
            let c = uniform_inclusive(rng, 2, 5) as i64;
            let d = uniform_inclusive(rng, 1, 3) as i64;
            let e = uniform_inclusive(rng, -6, -1) as i64;
            let at_boundary = uniform_inclusive(rng, 0, 1) == 0;
            let x0 = if at_boundary { c } else { c - 2 };
            let (val, wrong_branch_trap) = if x0 >= c {
                (d * x0 * x0 + e, a * x0 + b)
            } else {
                (a * x0 + b, d * x0 * x0 + e)
            };
            let prompt_text = format!(
                "A piecewise function is defined by f(x) = {}x + {} for x < {}, and f(x) = {}x² - {} for x ≥ {}. Evaluate f({}).",
                a, b, c, d, -e, c, x0
            );
            let prompt_latex = format!(
                "f(x) = \\begin{{cases}} {}x + {} & x < {} \\\\ {}x^2 - {} & x \\ge {} \\end{{cases}}, \\quad f({}) = ?",
                a, b, c, d, -e, c, x0
            );
            let prompt_speech = format!(
                "A piecewise function equals {} x plus {} when x is less than {}, and {} x squared minus {} when x is at least {}. Evaluate f of {}.",
                a, b, c, d, -e, c, x0
            );
            let metadata = serde_json::json!({
                "kind": "piecewise_evaluation",
                "a": a,
                "b": b,
                "c": c,
                "d": d,
                "e": e,
                "x0": x0,
                "wrong_branch_trap": wrong_branch_trap,
                "target": val,
            });
            Ok((prompt_text, prompt_latex, prompt_speech, "integer".into(), None, Some(val.to_string()), metadata, "integer-exact-v1".into()))
        }
        _ => {
            let a = uniform_inclusive(rng, 1, 3) as i64;
            let b = uniform_inclusive(rng, 1, 5) as i64;
            let c = uniform_inclusive(rng, 1, 6) as i64;
            let x1 = uniform_inclusive(rng, 1, 3) as i64;
            let dx = uniform_inclusive(rng, 2, 4) as i64;
            let x2 = x1 + dx;
            let fx1 = a * x1 * x1 + b * x1 + c;
            let fx2 = a * x2 * x2 + b * x2 + c;
            let dy = fx2 - fx1;
            let rate = dy / dx;
            let prompt_text = format!(
                "For the quadratic function f(x) = {}x² + {}x + {}, calculate the average rate of change (difference quotient) [f({}) - f({})] / ({} - {}) over the interval [{}, {}].",
                a, b, c, x2, x1, x2, x1, x1, x2
            );
            let prompt_latex = format!(
                "f(x) = {}x^2 + {}x + {}, \\quad \\frac{{f({}) - f({})}}{{{} - {}}} = ?",
                a, b, c, x2, x1, x2, x1
            );
            let prompt_speech = format!(
                "For f of x equals {} x squared plus {} x plus {}, find the average rate of change from x equals {} to x equals {}.",
                a, b, c, x1, x2
            );
            let metadata = serde_json::json!({
                "kind": "average_rate_of_change",
                "a": a,
                "b": b,
                "c": c,
                "x1": x1,
                "x2": x2,
                "fx1": fx1,
                "fx2": fx2,
                "unscaled_diff_trap": dy,
                "target": rate,
            });
            Ok((prompt_text, prompt_latex, prompt_speech, "integer".into(), None, Some(rate.to_string()), metadata, "integer-exact-v1".into()))
        }
    }
}

/// Template: `functions.composition_inverse` (Course S02)
fn generate_functions_composition_inverse<R: DrawSource + ?Sized>(
    rng: &mut R,
    difficulty: u32,
) -> Result<(String, String, String, String, Option<Vec<String>>, Option<String>, Value, String), GenerationFailure> {
    match difficulty {
        1 => {
            let a = uniform_inclusive(rng, 2, 4) as i64;
            let b = uniform_inclusive(rng, -5, 6) as i64;
            let c = uniform_inclusive(rng, 1, 5) as i64;
            let x0 = uniform_inclusive(rng, 2, 5) as i64;
            let gx0 = x0 * x0 + c;
            let fgx0 = a * gx0 + b;
            let fx0 = a * x0 + b;
            let reverse_order_trap = fx0 * fx0 + c;
            let product_trap = fx0 * gx0;
            let b_str = if b >= 0 { format!("+ {}", b) } else { format!("- {}", -b) };
            let prompt_text = format!(
                "Let f(x) = {}x {} and g(x) = x² + {}. Evaluate the composite function (f ∘ g)({}) = f(g({})).",
                a, b_str, c, x0, x0
            );
            let prompt_latex = format!(
                "f(x) = {}x {}, \\; g(x) = x^2 + {} \\implies f(g({})) = ?",
                a, b_str, c, x0
            );
            let prompt_speech = format!(
                "Let f of x equal {} x {} and g of x equal x squared plus {}. Evaluate f of g of {}.",
                a, b_str, c, x0
            );
            let metadata = serde_json::json!({
                "kind": "compose_linear_quadratic",
                "a": a,
                "b": b,
                "c": c,
                "x0": x0,
                "gx0": gx0,
                "reverse_order_trap": reverse_order_trap,
                "product_trap": product_trap,
                "target": fgx0,
            });
            Ok((prompt_text, prompt_latex, prompt_speech, "integer".into(), None, Some(fgx0.to_string()), metadata, "integer-exact-v1".into()))
        }
        2 => {
            let a = uniform_inclusive(rng, 2, 6) as i64;
            let b = uniform_inclusive(rng, 1, 9) as i64;
            let y0 = uniform_inclusive(rng, 11, 29) as i64;
            let q = frac(y0 - b, a);
            let target_str = to_canonical(&q);
            let forward_eval_trap = to_canonical(&int(a * y0 + b));
            let reciprocal_trap = to_canonical(&frac(1, a * y0 + b));
            let prompt_text = format!(
                "The invertible linear function f is given by f(x) = {}x + {}. Find the exact value of the inverse function f⁻¹({}) as an integer or fraction.",
                a, b, y0
            );
            let prompt_latex = format!(
                "f(x) = {}x + {} \\implies f^{{-1}}({}) = ?",
                a, b, y0
            );
            let prompt_speech = format!(
                "For f of x equals {} x plus {}, find the exact value of f inverse of {}.",
                a, b, y0
            );
            let metadata = serde_json::json!({
                "kind": "inverse_affine_q",
                "a": a,
                "b": b,
                "y0": y0,
                "forward_eval_trap": forward_eval_trap,
                "reciprocal_trap": reciprocal_trap,
                "target": target_str,
            });
            Ok((prompt_text, prompt_latex, prompt_speech, "rational".into(), None, Some(target_str), metadata, "rational-equality-v1".into()))
        }
        _ => {
            let a = uniform_inclusive(rng, 2, 4) as i64;
            let b = uniform_inclusive(rng, 1, 7) as i64;
            let c = uniform_inclusive(rng, 2, 4) as i64;
            let d = uniform_inclusive(rng, 2, 6) as i64;
            let x = uniform_inclusive(rng, 2, 7) as i64;
            let gx = c * x - d;
            let k = a * gx + b;
            let prompt_text = format!(
                "Let f(u) = {}u + {} and g(x) = {}x - {}. Solve the composite equation f(g(x)) = {} for x.",
                a, b, c, d, k
            );
            let prompt_latex = format!(
                "f(u) = {}u + {}, \\; g(x) = {}x - {}, \\; f(g(x)) = {} \\implies x = ?",
                a, b, c, d, k
            );
            let prompt_speech = format!(
                "Let f of u equal {} u plus {} and g of x equal {} x minus {}. Solve f of g of x equals {} for x.",
                a, b, c, d, k
            );
            let metadata = serde_json::json!({
                "kind": "solve_composite_equation",
                "a": a,
                "b": b,
                "c": c,
                "d": d,
                "k": k,
                "gx": gx,
                "target": x,
            });
            Ok((prompt_text, prompt_latex, prompt_speech, "integer".into(), None, Some(x.to_string()), metadata, "integer-exact-v1".into()))
        }
    }
}

/// Template: `functions.transformations_vertex` (Course S02)
fn generate_functions_transformations_vertex<R: DrawSource + ?Sized>(
    rng: &mut R,
    difficulty: u32,
) -> Result<(String, String, String, String, Option<Vec<String>>, Option<String>, Value, String), GenerationFailure> {
    match difficulty {
        1 => {
            let a = uniform_inclusive(rng, 2, 5) as i64;
            let h = uniform_inclusive(rng, 3, 9) as i64;
            let mut k = uniform_inclusive(rng, 1, 11) as i64;
            if k == h { k += 2; }
            let prompt_text = format!(
                "A quadratic function is written in vertex form as g(x) = {}(x - {})² + {}. What is the x-coordinate of the vertex of this parabola?",
                a, h, k
            );
            let prompt_latex = format!(
                "g(x) = {}(x - {})^2 + {} \\implies x_{{\\text{{vertex}}}} = ?",
                a, h, k
            );
            let prompt_speech = format!(
                "For g of x equals {} times open parenthesis x minus {} close parenthesis squared plus {}, what is the x coordinate of the vertex?",
                a, h, k
            );
            let metadata = serde_json::json!({
                "kind": "vertex_form_coords",
                "a": a,
                "h": h,
                "k": k,
                "horizontal_sign_trap": -h,
                "k_trap": k,
                "target": h,
            });
            Ok((prompt_text, prompt_latex, prompt_speech, "integer".into(), None, Some(h.to_string()), metadata, "integer-exact-v1".into()))
        }
        2 => {
            let x0 = uniform_inclusive(rng, 1, 5) as i64;
            let y0 = uniform_inclusive(rng, 3, 8) as i64;
            let a = uniform_inclusive(rng, 2, 4) as i64;
            let h = uniform_inclusive(rng, 2, 6) as i64;
            let k = uniform_inclusive(rng, -5, 7) as i64;
            let new_x = x0 + h;
            let new_y = a * y0 + k;
            let unscaled_y_trap = y0 + k;
            let k_str = if k >= 0 { format!("+ {}", k) } else { format!("- {}", -k) };
            let prompt_text = format!(
                "The graph of y = f(x) passes through the point ({}, {}), so f({}) = {}. If g(x) = {}f(x - {}) {}, what is the value of g({})?",
                x0, y0, x0, y0, a, h, k_str, new_x
            );
            let prompt_latex = format!(
                "f({}) = {}, \\; g(x) = {}f(x - {}) {} \\implies g({}) = ?",
                x0, y0, a, h, k_str, new_x
            );
            let prompt_speech = format!(
                "Given f of {} equals {}, and g of x equals {} f of x minus {} {}, find g of {}.",
                x0, y0, a, h, k_str, new_x
            );
            let metadata = serde_json::json!({
                "kind": "point_transformation",
                "x0": x0,
                "y0": y0,
                "a": a,
                "h": h,
                "k": k,
                "new_x": new_x,
                "unscaled_y_trap": unscaled_y_trap,
                "target": new_y,
            });
            Ok((prompt_text, prompt_latex, prompt_speech, "integer".into(), None, Some(new_y.to_string()), metadata, "integer-exact-v1".into()))
        }
        _ => {
            let a = uniform_inclusive(rng, 2, 4) as i64;
            let h = uniform_inclusive(rng, 2, 6) as i64;
            let k = uniform_inclusive(rng, 1, 9) as i64;
            let d = uniform_inclusive(rng, 2, 3) as i64;
            let x0 = h + d;
            let val = a * d * d + k;
            let horizontal_sign_trap = a * (x0 + h) * (x0 + h) + k;
            let prompt_text = format!(
                "The parent parabola y = x² is shifted {} units to the right, stretched vertically by a factor of {}, and shifted {} units up to form g(x). Evaluate g({}).",
                h, a, k, x0
            );
            let prompt_latex = format!(
                "g(x) = {}(x - {})^2 + {} \\implies g({}) = ?",
                a, h, k, x0
            );
            let prompt_speech = format!(
                "y equals x squared is shifted {} units right, stretched vertically by {}, and shifted {} units up to form g of x. Evaluate g of {}.",
                h, a, k, x0
            );
            let metadata = serde_json::json!({
                "kind": "constructed_transform_eval",
                "a": a,
                "h": h,
                "k": k,
                "x0": x0,
                "d": d,
                "horizontal_sign_trap": horizontal_sign_trap,
                "target": val,
            });
            Ok((prompt_text, prompt_latex, prompt_speech, "integer".into(), None, Some(val.to_string()), metadata, "integer-exact-v1".into()))
        }
    }
}

/// Template: `algebra2.polynomial_factor_roots` (Course S03)
fn generate_algebra2_polynomial_factor_roots<R: DrawSource + ?Sized>(
    rng: &mut R,
    difficulty: u32,
) -> Result<(String, String, String, String, Option<Vec<String>>, Option<String>, Value, String), GenerationFailure> {
    match difficulty {
        1 => {
            let r1 = uniform_inclusive(rng, 1, 5) as i64;
            let gap = uniform_inclusive(rng, 2, 5) as i64;
            let r2 = r1 + gap;
            let sum_r = r1 + r2;
            let prod_r = r1 * r2;
            let prompt_text = format!(
                "Factor the quadratic equation x² - {}x + {} = 0 and find its LARGER root.",
                sum_r, prod_r
            );
            let prompt_latex = format!(
                "x^2 - {}x + {} = 0 \\implies x_{{\\max}} = ?",
                sum_r, prod_r
            );
            let prompt_speech = format!(
                "Factor x squared minus {} x plus {} equals 0 and find its larger root.",
                sum_r, prod_r
            );
            let metadata = serde_json::json!({
                "kind": "monic_quadratic_larger_root",
                "r1": r1,
                "r2": r2,
                "sum_r": sum_r,
                "prod_r": prod_r,
                "smaller_root_trap": r1,
                "sign_flip_trap": -r2,
                "target": r2,
            });
            Ok((prompt_text, prompt_latex, prompt_speech, "integer".into(), None, Some(r2.to_string()), metadata, "integer-exact-v1".into()))
        }
        2 => {
            let a = uniform_inclusive(rng, 1, 4) as i64;
            let b = uniform_inclusive(rng, 2, 6) as i64;
            let c = uniform_inclusive(rng, 1, 9) as i64;
            let r = uniform_inclusive(rng, 2, 4) as i64;
            let rem = r * r * r + a * r * r - b * r + c;
            let neg_root_trap = -r * r * r + a * r * r + b * r + c;
            let prompt_text = format!(
                "By the Polynomial Remainder Theorem, what is the exact remainder when P(x) = x³ + {}x² - {}x + {} is divided by (x - {})?",
                a, b, c, r
            );
            let prompt_latex = format!(
                "P(x) = x^3 + {}x^2 - {}x + {} \\;\\text{{divided by}}\\; (x - {}) \\implies R = P({}) = ?",
                a, b, c, r, r
            );
            let prompt_speech = format!(
                "Find the remainder when P of x equals x cubed plus {} x squared minus {} x plus {} is divided by x minus {}.",
                a, b, c, r
            );
            let metadata = serde_json::json!({
                "kind": "remainder_theorem_eval",
                "a": a,
                "b": b,
                "c": c,
                "r": r,
                "neg_root_trap": neg_root_trap,
                "target": rem,
            });
            Ok((prompt_text, prompt_latex, prompt_speech, "integer".into(), None, Some(rem.to_string()), metadata, "integer-exact-v1".into()))
        }
        _ => {
            let a = uniform_inclusive(rng, 2, 5) as i64;
            let b = uniform_inclusive(rng, 7, 17) as i64;
            let c = uniform_inclusive(rng, 1, 4) as i64;
            let q = frac(b, a);
            let target_str = to_canonical(&q);
            let unscaled_vieta_trap = to_canonical(&int(b));
            let sign_vieta_trap = to_canonical(&frac(-b, a));
            let prompt_text = format!(
                "Using Vieta's formulas, find the exact sum of the roots (r₁ + r₂) of the quadratic equation {}x² - {}x + {} = 0. Express your answer as an integer or fraction.",
                a, b, c
            );
            let prompt_latex = format!(
                "{}x^2 - {}x + {} = 0 \\implies r_1 + r_2 = ?",
                a, b, c
            );
            let prompt_speech = format!(
                "Find the exact sum of the roots of {} x squared minus {} x plus {} equals 0.",
                a, b, c
            );
            let metadata = serde_json::json!({
                "kind": "vieta_sum_of_roots",
                "a": a,
                "b": b,
                "c": c,
                "unscaled_vieta_trap": unscaled_vieta_trap,
                "sign_vieta_trap": sign_vieta_trap,
                "target": target_str,
            });
            Ok((prompt_text, prompt_latex, prompt_speech, "rational".into(), None, Some(target_str), metadata, "rational-equality-v1".into()))
        }
    }
}

/// Template: `algebra2.quadratic_discriminant` (Course S03)
fn generate_algebra2_quadratic_discriminant<R: DrawSource + ?Sized>(
    rng: &mut R,
    difficulty: u32,
) -> Result<(String, String, String, String, Option<Vec<String>>, Option<String>, Value, String), GenerationFailure> {
    match difficulty {
        1 => {
            let a = uniform_inclusive(rng, 2, 5) as i64;
            let b = uniform_inclusive(rng, 3, 9) as i64;
            let mut c = uniform_inclusive(rng, -5, 5) as i64;
            if c == 0 { c = -3; }
            let disc = b * b - 4 * a * c;
            let sign_4ac_trap = b * b + 4 * a * c;
            let c_str = if c >= 0 { format!("+ {}", c) } else { format!("- {}", -c) };
            let prompt_text = format!(
                "Calculate the exact discriminant Δ = b² - 4ac of the quadratic equation {}x² + {}x {} = 0.",
                a, b, c_str
            );
            let prompt_latex = format!(
                "{}x^2 + {}x {} = 0 \\implies \\Delta = b^2 - 4ac = ?",
                a, b, c_str
            );
            let prompt_speech = format!(
                "Calculate the discriminant b squared minus 4 a c for {} x squared plus {} x {} equals 0.",
                a, b, c_str
            );
            let metadata = serde_json::json!({
                "kind": "discriminant_calc",
                "a": a,
                "b": b,
                "c": c,
                "sign_4ac_trap": sign_4ac_trap,
                "target": disc,
            });
            Ok((prompt_text, prompt_latex, prompt_speech, "integer".into(), None, Some(disc.to_string()), metadata, "integer-exact-v1".into()))
        }
        2 => {
            // Construct (a*x - p)(x + m) = a*x^2 + (a*m - p)x - p*m = 0 with roots x = p/a > 0 and x = -m < 0
            let a = uniform_inclusive(rng, 2, 4) as i64;
            let p = uniform_inclusive(rng, 3, 9) as i64;
            let m = uniform_inclusive(rng, 1, 4) as i64;
            let b_coeff = a * m - p;
            let c_coeff = -p * m;
            let sqrt_disc = a * m + p;
            let disc = sqrt_disc * sqrt_disc;
            let q = frac(p, a);
            let target_str = to_canonical(&q);
            let forgot_2a_trap = to_canonical(&int(2 * p));
            let smaller_root_trap = to_canonical(&int(-m));
            let b_str = if b_coeff >= 0 { format!("+ {}x", b_coeff) } else { format!("- {}x", -b_coeff) };
            let prompt_text = format!(
                "Use the quadratic formula to find the POSITIVE root of {}x² {} - {} = 0. Express your answer as an exact integer or fraction.",
                a, b_str, -c_coeff
            );
            let prompt_latex = format!(
                "{}x^2 {} - {} = 0 \\implies x_{{>0}} = \\frac{{-b + \\sqrt{{b^2 - 4ac}}}}{{2a}} = ?",
                a, b_str, -c_coeff
            );
            let prompt_speech = format!(
                "Use the quadratic formula to find the positive root of {} x squared {} minus {} equals 0.",
                a, b_str, -c_coeff
            );
            let metadata = serde_json::json!({
                "kind": "quadratic_formula_positive_root",
                "a": a,
                "b": b_coeff,
                "c": c_coeff,
                "disc": disc,
                "sqrt_disc": sqrt_disc,
                "p": p,
                "m": m,
                "forgot_2a_trap": forgot_2a_trap,
                "smaller_root_trap": smaller_root_trap,
                "target": target_str,
            });
            Ok((prompt_text, prompt_latex, prompt_speech, "rational".into(), None, Some(target_str), metadata, "rational-equality-v1".into()))
        }
        _ => {
            let a = uniform_inclusive(rng, 2, 5) as i64;
            let b = uniform_inclusive(rng, 3, 9) as i64;
            let q = frac(b * b, 4 * a);
            let target_str = to_canonical(&q);
            let forgot_4_trap = to_canonical(&frac(b * b, a));
            let prompt_text = format!(
                "For what exact rational value of c does the quadratic equation {}x² + {}x + c = 0 have a single repeated (double) root (Δ = 0)?",
                a, b
            );
            let prompt_latex = format!(
                "{}x^2 + {}x + c = 0, \\; \\Delta = 0 \\implies c = \\frac{{b^2}}{{4a}} = ?",
                a, b
            );
            let prompt_speech = format!(
                "For what exact value of c does {} x squared plus {} x plus c equals 0 have a repeated root?",
                a, b
            );
            let metadata = serde_json::json!({
                "kind": "tangent_double_root_param",
                "a": a,
                "b": b,
                "forgot_4_trap": forgot_4_trap,
                "target": target_str,
            });
            Ok((prompt_text, prompt_latex, prompt_speech, "rational".into(), None, Some(target_str), metadata, "rational-equality-v1".into()))
        }
    }
}

/// Template: `algebra2.exp_log_eval` (Course S03)
fn generate_algebra2_exp_log_eval<R: DrawSource + ?Sized>(
    rng: &mut R,
    difficulty: u32,
) -> Result<(String, String, String, String, Option<Vec<String>>, Option<String>, Value, String), GenerationFailure> {
    let bases = [2i64, 3, 5];
    match difficulty {
        1 => {
            let b = bases[uniform_inclusive(rng, 0, 2) as usize];
            let k = uniform_inclusive(rng, 2, 4) as u32;
            let is_reciprocal = uniform_inclusive(rng, 0, 1) == 1;
            let n = b.pow(k);
            let target_exp = if is_reciprocal { -(k as i64) } else { k as i64 };
            let division_trap = n / b;
            let sign_log_trap = -target_exp;
            let (prompt_text, prompt_latex, prompt_speech) = if is_reciprocal {
                (
                    format!("Evaluate the exact logarithm: log_{}(1/{}).", b, n),
                    format!("\\log_{{{}}}\\!\\left(\\frac{{1}}{{{}}}\\right) = ?", b, n),
                    format!("Evaluate log base {} of 1 over {}.", b, n),
                )
            } else {
                (
                    format!("Evaluate the exact logarithm: log_{}({}).", b, n),
                    format!("\\log_{{{}}}({}) = ?", b, n),
                    format!("Evaluate log base {} of {}.", b, n),
                )
            };
            let metadata = serde_json::json!({
                "kind": "log_exact_power",
                "base": b,
                "n": n,
                "is_reciprocal": is_reciprocal,
                "division_trap": division_trap,
                "sign_log_trap": sign_log_trap,
                "target": target_exp,
            });
            Ok((prompt_text, prompt_latex, prompt_speech, "integer".into(), None, Some(target_exp.to_string()), metadata, "integer-exact-v1".into()))
        }
        2 => {
            let b = bases[uniform_inclusive(rng, 0, 2) as usize];
            let k = uniform_inclusive(rng, 2, 4) as u32;
            let m = if b == 3 { 5i64 } else { 3i64 };
            let cap_c = m;
            let cap_a = m * b;
            let cap_b = b.pow(k - 1);
            let combined_arg = b.pow(k);
            let target_k = k as i64;
            let prompt_text = format!(
                "Use logarithm laws to combine and evaluate the exact expression: log_{}({}) + log_{}({}) - log_{}({}).",
                b, cap_a, b, cap_b, b, cap_c
            );
            let prompt_latex = format!(
                "\\log_{{{}}}({}) + \\log_{{{}}}({}) - \\log_{{{}}}({}) = ?",
                b, cap_a, b, cap_b, b, cap_c
            );
            let prompt_speech = format!(
                "Evaluate log base {} of {} plus log base {} of {} minus log base {} of {}.",
                b, cap_a, b, cap_b, b, cap_c
            );
            let metadata = serde_json::json!({
                "kind": "log_laws_combine",
                "base": b,
                "A": cap_a,
                "B": cap_b,
                "C": cap_c,
                "combined_arg": combined_arg,
                "arg_value_trap": combined_arg,
                "target": target_k,
            });
            Ok((prompt_text, prompt_latex, prompt_speech, "integer".into(), None, Some(target_k.to_string()), metadata, "integer-exact-v1".into()))
        }
        _ => {
            let b = bases[uniform_inclusive(rng, 0, 2) as usize];
            let m = uniform_inclusive(rng, 2, 3) as u32;
            let big_b = b.pow(m);
            let m_i64 = m as i64;
            let d = uniform_inclusive(rng, 1, 3) as i64;
            let a = m_i64 * d + (uniform_inclusive(rng, 2, 4) as i64);
            let e = uniform_inclusive(rng, 2, 5) as i64;
            let c = uniform_inclusive(rng, 1, 6) as i64;
            let net_coeff = a - m_i64 * d;
            let net_rhs = m_i64 * e - c;
            let q = frac(net_rhs, net_coeff);
            let target_str = to_canonical(&q);
            let unscaled_exp_trap = to_canonical(&frac(e - c, a - d));
            let prompt_text = format!(
                "Solve the exponential equation for x by rewriting both sides with common base {}: {}^({}x + {}) = {}^({}x + {}). Express your answer as an exact integer or fraction.",
                b, b, a, c, big_b, d, e
            );
            let prompt_latex = format!(
                "{}^{{{}x + {}}} = {}^{{{}x + {}}} \\implies x = ?",
                b, a, c, big_b, d, e
            );
            let prompt_speech = format!(
                "Solve {} to the power {} x plus {} equals {} to the power {} x plus {} for x.",
                b, a, c, big_b, d, e
            );
            let metadata = serde_json::json!({
                "kind": "exp_common_base_solve",
                "base": b,
                "m": m_i64,
                "big_b": big_b,
                "a": a,
                "c": c,
                "d": d,
                "e": e,
                "net_coeff": net_coeff,
                "net_rhs": net_rhs,
                "unscaled_exp_trap": unscaled_exp_trap,
                "target": target_str,
            });
            Ok((prompt_text, prompt_latex, prompt_speech, "rational".into(), None, Some(target_str), metadata, "rational-equality-v1".into()))
        }
    }
}

/// Template: `sec_geom.slope_and_lines` (Course S04)
fn generate_sec_geom_slope_and_lines<R: DrawSource + ?Sized>(
    rng: &mut R,
    difficulty: u32,
) -> Result<(String, String, String, String, Option<Vec<String>>, Option<String>, Value, String), GenerationFailure> {
    match difficulty {
        1 => {
            let x1 = uniform_inclusive(rng, -5, 2) as i64;
            let dx = uniform_inclusive(rng, 2, 6) as i64;
            let x2 = x1 + dx;
            let y1 = uniform_inclusive(rng, -6, 6) as i64;
            let mut dy = uniform_inclusive(rng, -9, 9) as i64;
            if dy == 0 || dy == dx || dy == -dx {
                dy = 5;
            }
            let y2 = y1 + dy;
            let q = frac(dy, dx);
            let target_str = to_canonical(&q);
            let run_over_rise_trap = to_canonical(&frac(dx, dy));
            let sign_order_trap = to_canonical(&frac(-dy, dx));
            let prompt_text = format!(
                "Find the exact slope m of the line passing through the points A({}, {}) and B({}, {}). Express your answer as an exact integer or fraction.",
                x1, y1, x2, y2
            );
            let prompt_latex = format!(
                "A({}, {}), \\; B({}, {}) \\implies m = \\frac{{y_2 - y_1}}{{x_2 - x_1}} = ?",
                x1, y1, x2, y2
            );
            let prompt_speech = format!(
                "Find the exact slope m of the line passing through A {} comma {} and B {} comma {}.",
                x1, y1, x2, y2
            );
            let metadata = serde_json::json!({
                "kind": "slope_exact_rational",
                "x1": x1,
                "y1": y1,
                "x2": x2,
                "y2": y2,
                "dx": dx,
                "dy": dy,
                "run_over_rise_trap": run_over_rise_trap,
                "sign_order_trap": sign_order_trap,
                "target": target_str,
            });
            Ok((prompt_text, prompt_latex, prompt_speech, "rational".into(), None, Some(target_str), metadata, "rational-equality-v1".into()))
        }
        2 => {
            let p = uniform_inclusive(rng, 2, 5) as i64;
            let mut q_den = uniform_inclusive(rng, 2, 5) as i64;
            if p == q_den {
                q_den += 1;
            }
            let x0 = uniform_inclusive(rng, 2, 6) as i64;
            let y0 = uniform_inclusive(rng, -4, 7) as i64;
            let c = uniform_inclusive(rng, 1, 8) as i64;
            // Original slope m1 = p / q_den; perpendicular slope m_perp = -q_den / p
            // y - y0 = m_perp (x - x0) => y-intercept b = y0 - m_perp * x0 = y0 + (q_den * x0) / p
            let m_perp = frac(-q_den, p);
            let b_val = frac(y0 * p + q_den * x0, p);
            let target_str = to_canonical(&b_val);
            let parallel_slope_trap = to_canonical(&frac(y0 * q_den - p * x0, q_den));
            let positive_reciprocal_trap = to_canonical(&frac(y0 * p - q_den * x0, p));
            let m1_str = to_canonical(&frac(p, q_den));
            let m_perp_str = to_canonical(&m_perp);
            let prompt_text = format!(
                "Line L is perpendicular to y = ({})x + {} and passes through the point ({}, {}). Find the exact y-intercept b of line L (where L is written as y = mx + b).",
                m1_str, c, x0, y0
            );
            let prompt_latex = format!(
                "L \\perp \\left(y = \\frac{{{}}}{{{}}}x + {}\\right), \\; ({}, {}) \\in L \\implies b = y_0 - m_{{\\perp}} x_0 = ?",
                p, q_den, c, x0, y0
            );
            let prompt_speech = format!(
                "Line L is perpendicular to y equals {} over {} x plus {} and passes through {} comma {}. Find the y intercept b of line L.",
                p, q_den, c, x0, y0
            );
            let metadata = serde_json::json!({
                "kind": "perpendicular_line_intercept",
                "p": p,
                "q": q_den,
                "m1": m1_str,
                "m_perp": m_perp_str,
                "x0": x0,
                "y0": y0,
                "c": c,
                "parallel_slope_trap": parallel_slope_trap,
                "positive_reciprocal_trap": positive_reciprocal_trap,
                "target": target_str,
            });
            Ok((prompt_text, prompt_latex, prompt_speech, "rational".into(), None, Some(target_str), metadata, "rational-equality-v1".into()))
        }
        _ => {
            let x1 = uniform_inclusive(rng, -4, 3) as i64;
            let y1 = uniform_inclusive(rng, -3, 5) as i64;
            let x2 = x1 + (uniform_inclusive(rng, 5, 11) as i64);
            let y2 = y1 + (uniform_inclusive(rng, 3, 9) as i64);
            let m = uniform_inclusive(rng, 1, 3) as i64;
            let n = m + (uniform_inclusive(rng, 1, 3) as i64);
            let xp = frac(n * x1 + m * x2, m + n);
            let target_str = to_canonical(&xp);
            let unswapped_weights_trap = to_canonical(&frac(m * x1 + n * x2, m + n));
            let midpoint_trap = to_canonical(&frac(x1 + x2, 2));
            let prompt_text = format!(
                "Point P divides the directed line segment from A({}, {}) to B({}, {}) internally in the ratio AP : PB = {} : {}. Find the exact x-coordinate of point P.",
                x1, y1, x2, y2, m, n
            );
            let prompt_latex = format!(
                "A({}, {}), \\; B({}, {}), \\; AP : PB = {} : {} \\implies x_P = \\frac{{n x_1 + m x_2}}{{m + n}} = ?",
                x1, y1, x2, y2, m, n
            );
            let prompt_speech = format!(
                "Point P divides the segment from A {} comma {} to B {} comma {} in the ratio {} to {}. Find the exact x coordinate of P.",
                x1, y1, x2, y2, m, n
            );
            let metadata = serde_json::json!({
                "kind": "section_formula_internal",
                "x1": x1,
                "y1": y1,
                "x2": x2,
                "y2": y2,
                "m": m,
                "n": n,
                "unswapped_weights_trap": unswapped_weights_trap,
                "midpoint_trap": midpoint_trap,
                "target": target_str,
            });
            Ok((prompt_text, prompt_latex, prompt_speech, "rational".into(), None, Some(target_str), metadata, "rational-equality-v1".into()))
        }
    }
}

/// Template: `sec_geom.circle_equations_theorems` (Course S04)
fn generate_sec_geom_circle_equations_theorems<R: DrawSource + ?Sized>(
    rng: &mut R,
    difficulty: u32,
) -> Result<(String, String, String, String, Option<Vec<String>>, Option<String>, Value, String), GenerationFailure> {
    match difficulty {
        1 => {
            let inscribed = uniform_inclusive(rng, 22, 68) as i64;
            let central = 2 * inscribed;
            let central_equal_trap = central;
            let doubled_central_trap = 2 * central;
            let prompt_text = format!(
                "Points A, B, and C lie on a circle with center O. If the central angle ∠AOB subtending minor arc AB measures {}°, what is the exact measure (in degrees) of the inscribed angle ∠ACB subtending the same arc?",
                central
            );
            let prompt_latex = format!(
                "\\angle AOB = {}^\\circ \\implies \\angle ACB = \\frac{{1}}{{2}}\\angle AOB = ?^\\circ",
                central
            );
            let prompt_speech = format!(
                "If central angle A O B measures {} degrees, what is the inscribed angle A C B subtending the same arc?",
                central
            );
            let metadata = serde_json::json!({
                "kind": "inscribed_angle_theorem",
                "central": central,
                "inscribed": inscribed,
                "central_equal_trap": central_equal_trap,
                "doubled_central_trap": doubled_central_trap,
                "target": inscribed,
            });
            Ok((prompt_text, prompt_latex, prompt_speech, "integer".into(), None, Some(inscribed.to_string()), metadata, "integer-exact-v1".into()))
        }
        2 => {
            let h = uniform_inclusive(rng, 2, 6) as i64;
            let k = uniform_inclusive(rng, 2, 6) as i64;
            let r = uniform_inclusive(rng, 3, 8) as i64;
            let r_sq = r * r;
            let f_const = h * h + k * k - r_sq;
            let d_coeff = -2 * h;
            let e_coeff = -2 * k;
            let forgot_complete_square_trap = -f_const;
            let unhalved_coeff_trap = d_coeff * d_coeff + e_coeff * e_coeff - f_const;
            let f_str = if f_const >= 0 {
                format!("+ {}", f_const)
            } else {
                format!("- {}", -f_const)
            };
            let prompt_text = format!(
                "Complete the square on the circle equation x² + y² - {}x - {}y {} = 0 to rewrite it in standard form (x - h)² + (y - k)² = R². What is the exact value of the squared radius R²?",
                2 * h, 2 * k, f_str
            );
            let prompt_latex = format!(
                "x^2 + y^2 - {}x - {}y {} = 0 \\implies (x - h)^2 + (y - k)^2 = R^2, \\quad R^2 = ?",
                2 * h, 2 * k, f_str
            );
            let prompt_speech = format!(
                "Complete the square on x squared plus y squared minus {} x minus {} y {} equals 0 to find the squared radius R squared.",
                2 * h, 2 * k, f_str
            );
            let metadata = serde_json::json!({
                "kind": "circle_complete_square_radius",
                "h": h,
                "k": k,
                "r": r,
                "r_sq": r_sq,
                "f_const": f_const,
                "forgot_complete_square_trap": forgot_complete_square_trap,
                "unhalved_coeff_trap": unhalved_coeff_trap,
                "target": r_sq,
            });
            Ok((prompt_text, prompt_latex, prompt_speech, "integer".into(), None, Some(r_sq.to_string()), metadata, "integer-exact-v1".into()))
        }
        _ => {
            let a = uniform_inclusive(rng, 3, 8) as i64;
            let b = uniform_inclusive(rng, 4, 9) as i64;
            let mut c = uniform_inclusive(rng, 3, 7) as i64;
            if a + b == c {
                c += 1;
            }
            let pd = frac(a * b, c);
            let target_str = to_canonical(&pd);
            let additive_chord_trap = to_canonical(&int(a + b - c));
            let inverted_ratio_trap = to_canonical(&frac(a * c, b));
            let prompt_text = format!(
                "Two chords AB and CD intersect at point P inside a circle. If PA = {}, PB = {}, and PC = {}, use the Intersecting Chords Theorem (Power of a Point) to find the exact length of segment PD.",
                a, b, c
            );
            let prompt_latex = format!(
                "PA \\cdot PB = PC \\cdot PD, \\quad PA = {}, \\; PB = {}, \\; PC = {} \\implies PD = ?",
                a, b, c
            );
            let prompt_speech = format!(
                "Two chords A B and C D intersect at P inside a circle with P A equals {}, P B equals {}, and P C equals {}. Find the exact length of P D.",
                a, b, c
            );
            let metadata = serde_json::json!({
                "kind": "power_of_a_point_chords",
                "a": a,
                "b": b,
                "c": c,
                "ab": a * b,
                "additive_chord_trap": additive_chord_trap,
                "inverted_ratio_trap": inverted_ratio_trap,
                "target": target_str,
            });
            Ok((prompt_text, prompt_latex, prompt_speech, "rational".into(), None, Some(target_str), metadata, "rational-equality-v1".into()))
        }
    }
}

/// Template: `sec_geom.planar_vectors_dot` (Course S04)
fn generate_sec_geom_planar_vectors_dot<R: DrawSource + ?Sized>(
    rng: &mut R,
    difficulty: u32,
) -> Result<(String, String, String, String, Option<Vec<String>>, Option<String>, Value, String), GenerationFailure> {
    match difficulty {
        1 => {
            let u1 = uniform_inclusive(rng, 2, 7) as i64;
            let u2 = uniform_inclusive(rng, -6, -2) as i64;
            let v1 = uniform_inclusive(rng, 3, 8) as i64;
            let v2 = uniform_inclusive(rng, 2, 6) as i64;
            let dot = u1 * v1 + u2 * v2;
            let cross_product_trap = u1 * v2 - u2 * v1;
            let subtract_components_trap = u1 * v1 - u2 * v2;
            let prompt_text = format!(
                "Compute the exact scalar dot product u · v of the planar vectors u = ⟨{}, {}⟩ and v = ⟨{}, {}⟩.",
                u1, u2, v1, v2
            );
            let prompt_latex = format!(
                "\\vec{{u}} = \\langle {}, {} \\rangle, \\; \\vec{{v}} = \\langle {}, {} \\rangle \\implies \\vec{{u}} \\cdot \\vec{{v}} = ?",
                u1, u2, v1, v2
            );
            let prompt_speech = format!(
                "Compute the dot product of vector u equals {} comma {} and vector v equals {} comma {}.",
                u1, u2, v1, v2
            );
            let metadata = serde_json::json!({
                "kind": "dot_product_eval",
                "u1": u1,
                "u2": u2,
                "v1": v1,
                "v2": v2,
                "u1v1": u1 * v1,
                "u2v2": u2 * v2,
                "cross_product_trap": cross_product_trap,
                "subtract_components_trap": subtract_components_trap,
                "target": dot,
            });
            Ok((prompt_text, prompt_latex, prompt_speech, "integer".into(), None, Some(dot.to_string()), metadata, "integer-exact-v1".into()))
        }
        2 => {
            let a = uniform_inclusive(rng, 2, 6) as i64;
            let b = uniform_inclusive(rng, 3, 7) as i64;
            let c = uniform_inclusive(rng, 2, 8) as i64;
            // a*c + b*k = 0 => k = -a*c / b
            let k_val = frac(-a * c, b);
            let target_str = to_canonical(&k_val);
            let sign_orthogonal_trap = to_canonical(&frac(a * c, b));
            let parallel_proportion_trap = to_canonical(&frac(b * c, a));
            let prompt_text = format!(
                "Find the exact rational value of k so that the vectors u = ⟨{}, {}⟩ and v = ⟨{}, k⟩ are orthogonal (perpendicular, u · v = 0).",
                a, b, c
            );
            let prompt_latex = format!(
                "\\vec{{u}} = \\langle {}, {} \\rangle \\perp \\vec{{v}} = \\langle {}, k \\rangle \\implies \\vec{{u}} \\cdot \\vec{{v}} = 0 \\implies k = ?",
                a, b, c
            );
            let prompt_speech = format!(
                "Find the exact value of k so that vector {} comma {} and vector {} comma k are perpendicular.",
                a, b, c
            );
            let metadata = serde_json::json!({
                "kind": "orthogonal_vector_param",
                "a": a,
                "b": b,
                "c": c,
                "ac": a * c,
                "sign_orthogonal_trap": sign_orthogonal_trap,
                "parallel_proportion_trap": parallel_proportion_trap,
                "target": target_str,
            });
            Ok((prompt_text, prompt_latex, prompt_speech, "rational".into(), None, Some(target_str), metadata, "rational-equality-v1".into()))
        }
        _ => {
            let triples = [(3i64, 4i64, 5i64), (5, 12, 13), (8, 15, 17)];
            let (v1, v2, norm_v) = triples[uniform_inclusive(rng, 0, 2) as usize];
            let u1 = uniform_inclusive(rng, 2, 6) as i64;
            let u2 = uniform_inclusive(rng, 1, 5) as i64;
            let dot = u1 * v1 + u2 * v2;
            let comp = frac(dot, norm_v);
            let target_str = to_canonical(&comp);
            let norm_squared_trap = to_canonical(&frac(dot, norm_v * norm_v));
            let dot_only_trap = to_canonical(&int(dot));
            let prompt_text = format!(
                "Given u = ⟨{}, {}⟩ and v = ⟨{}, {}⟩ (where ‖v‖ = {}), compute the exact scalar projection of u onto v: comp_v(u) = (u · v) / ‖v‖.",
                u1, u2, v1, v2, norm_v
            );
            let prompt_latex = format!(
                "\\vec{{u}} = \\langle {}, {} \\rangle, \\; \\vec{{v}} = \\langle {}, {} \\rangle \\implies \\operatorname{{comp}}_{{\\vec{{v}}}}(\\vec{{u}}) = \\frac{{\\vec{{u}} \\cdot \\vec{{v}}}}{{\\|\\vec{{v}}\\|}} = ?",
                u1, u2, v1, v2
            );
            let prompt_speech = format!(
                "Given u equals {} comma {} and v equals {} comma {}, compute the scalar projection u dot v divided by the magnitude of v.",
                u1, u2, v1, v2
            );
            let metadata = serde_json::json!({
                "kind": "scalar_projection_pythagorean",
                "u1": u1,
                "u2": u2,
                "v1": v1,
                "v2": v2,
                "norm_v": norm_v,
                "dot": dot,
                "norm_squared_trap": norm_squared_trap,
                "dot_only_trap": dot_only_trap,
                "target": target_str,
            });
            Ok((prompt_text, prompt_latex, prompt_speech, "rational".into(), None, Some(target_str), metadata, "rational-equality-v1".into()))
        }
    }
}

/// Template: `trig.radians_unit_circle` (Course S05)
fn generate_trig_radians_unit_circle<R: DrawSource + ?Sized>(
    rng: &mut R,
    difficulty: u32,
) -> Result<(String, String, String, String, Option<Vec<String>>, Option<String>, Value, String), GenerationFailure> {
    match difficulty {
        1 => {
            let degs = [30i64, 45, 60, 75, 120, 135, 150, 210, 225, 240, 300, 315, 330];
            let deg = degs[uniform_inclusive(rng, 0, (degs.len() - 1) as i128) as usize];
            let k_val = frac(deg, 180);
            let target_str = to_canonical(&k_val);
            let div_360_trap = to_canonical(&frac(deg, 360));
            let inverted_rad_trap = to_canonical(&frac(180, deg));
            let prompt_text = format!(
                "Convert the angle {}° into radians in the form k·π. What is the exact rational multiplier k?",
                deg
            );
            let prompt_latex = format!(
                "{}^\\circ = k\\pi \\text{{ radians}} \\implies k = \\frac{{{}}}{{180}} = ?",
                deg, deg
            );
            let prompt_speech = format!(
                "Convert {} degrees into radians as k pi. What is the exact rational multiplier k?",
                deg
            );
            let metadata = serde_json::json!({
                "kind": "deg_to_rad_multiplier",
                "deg": deg,
                "div_360_trap": div_360_trap,
                "inverted_rad_trap": inverted_rad_trap,
                "target": target_str,
            });
            Ok((prompt_text, prompt_latex, prompt_speech, "rational".into(), None, Some(target_str), metadata, "rational-equality-v1".into()))
        }
        2 => {
            let triples = [(3i64, 4i64, 5i64), (4, 3, 5), (5, 12, 13), (12, 5, 13), (8, 15, 17)];
            let (a, b, c) = triples[uniform_inclusive(rng, 0, (triples.len() - 1) as i128) as usize];
            let cos_val = frac(-b, c);
            let target_str = to_canonical(&cos_val);
            let positive_quadrant_trap = to_canonical(&frac(b, c));
            let linear_complement_trap = to_canonical(&frac(-(c - a), c));
            let prompt_text = format!(
                "An angle θ lies in Quadrant II (π/2 < θ < π) on the unit circle with sin(θ) = {}/{}. Using sin²(θ) + cos²(θ) = 1 and the sign of cosine in Quadrant II, find the exact value of cos(θ).",
                a, c
            );
            let prompt_latex = format!(
                "\\theta \\in \\left(\\frac{{\\pi}}{{2}}, \\pi\\right), \\; \\sin(\\theta) = \\frac{{{}}}{{{}}} \\implies \\cos(\\theta) = -\\sqrt{{1 - \\sin^2(\\theta)}} = ?",
                a, c
            );
            let prompt_speech = format!(
                "Angle theta is in Quadrant 2 with sine theta equal to {} over {}. Find the exact value of cosine theta.",
                a, c
            );
            let metadata = serde_json::json!({
                "kind": "quadrant_two_cosine",
                "a": a,
                "b": b,
                "c": c,
                "positive_quadrant_trap": positive_quadrant_trap,
                "linear_complement_trap": linear_complement_trap,
                "target": target_str,
            });
            Ok((prompt_text, prompt_latex, prompt_speech, "rational".into(), None, Some(target_str), metadata, "rational-equality-v1".into()))
        }
        _ => {
            let amp = uniform_inclusive(rng, 2, 7) as i64;
            let omega = uniform_inclusive(rng, 3, 9) as i64;
            let midline = uniform_inclusive(rng, 1, 5) as i64;
            let period = frac(2, omega);
            let target_str = to_canonical(&period);
            let half_period_trap = to_canonical(&frac(1, omega));
            let freq_instead_of_period_trap = to_canonical(&int(omega));
            let prompt_text = format!(
                "Find the exact fundamental period T of the periodic wave f(t) = {} cos({}π t) + {}.",
                amp, omega, midline
            );
            let prompt_latex = format!(
                "f(t) = {}\\cos({}\\pi t) + {} \\implies T = \\frac{{2\\pi}}{{{}\\pi}} = ?",
                amp, omega, midline, omega
            );
            let prompt_speech = format!(
                "Find the exact fundamental period T of {} cosine of {} pi t plus {}.",
                amp, omega, midline
            );
            let metadata = serde_json::json!({
                "kind": "sinusoid_fundamental_period",
                "amp": amp,
                "omega": omega,
                "midline": midline,
                "half_period_trap": half_period_trap,
                "freq_instead_of_period_trap": freq_instead_of_period_trap,
                "target": target_str,
            });
            Ok((prompt_text, prompt_latex, prompt_speech, "rational".into(), None, Some(target_str), metadata, "rational-equality-v1".into()))
        }
    }
}

/// Template: `trig.identities_double_angle` (Course S05)
fn generate_trig_identities_double_angle<R: DrawSource + ?Sized>(
    rng: &mut R,
    difficulty: u32,
) -> Result<(String, String, String, String, Option<Vec<String>>, Option<String>, Value, String), GenerationFailure> {
    let triples = [(3i64, 4i64, 5i64), (4, 3, 5), (5, 12, 13), (12, 5, 13), (8, 15, 17)];
    match difficulty {
        1 => {
            let (a, b, c) = triples[uniform_inclusive(rng, 0, (triples.len() - 1) as i128) as usize];
            let tan_val = frac(a, b);
            let target_str = to_canonical(&tan_val);
            let sine_instead_of_tan_trap = to_canonical(&frac(a, c));
            let cotangent_trap = to_canonical(&frac(b, a));
            let prompt_text = format!(
                "For an acute angle θ in (0, π/2) with cos(θ) = {}/{}, use the Pythagorean identity to find sin(θ) and compute the exact value of tan(θ) = sin(θ)/cos(θ).",
                b, c
            );
            let prompt_latex = format!(
                "\\theta \\in \\left(0, \\frac{{\\pi}}{{2}}\\right), \\; \\cos(\\theta) = \\frac{{{}}}{{{}}} \\implies \\tan(\\theta) = \\frac{{\\sin(\\theta)}}{{\\cos(\\theta)}} = ?",
                b, c
            );
            let prompt_speech = format!(
                "For acute angle theta with cosine theta equal to {} over {}, find the exact value of tangent theta.",
                b, c
            );
            let metadata = serde_json::json!({
                "kind": "pythagorean_tangent_acute",
                "a": a,
                "b": b,
                "c": c,
                "sine_instead_of_tan_trap": sine_instead_of_tan_trap,
                "cotangent_trap": cotangent_trap,
                "target": target_str,
            });
            Ok((prompt_text, prompt_latex, prompt_speech, "rational".into(), None, Some(target_str), metadata, "rational-equality-v1".into()))
        }
        2 => {
            let (a, b, c) = triples[uniform_inclusive(rng, 0, (triples.len() - 1) as i128) as usize];
            let sin_2theta = frac(2 * a * b, c * c);
            let target_str = to_canonical(&sin_2theta);
            let linear_double_sine_trap = to_canonical(&frac(2 * a, c));
            let product_without_two_trap = to_canonical(&frac(a * b, c * c));
            let prompt_text = format!(
                "Given an acute angle θ in (0, π/2) with sin(θ) = {}/{} and cos(θ) = {}/{}, use the double-angle identity sin(2θ) = 2 sin(θ) cos(θ) to find the exact value of sin(2θ).",
                a, c, b, c
            );
            let prompt_latex = format!(
                "\\sin(\\theta) = \\frac{{{}}}{{{}}}, \\; \\cos(\\theta) = \\frac{{{}}}{{{}}} \\implies \\sin(2\\theta) = 2\\sin(\\theta)\\cos(\\theta) = ?",
                a, c, b, c
            );
            let prompt_speech = format!(
                "Given sine theta equals {} over {} and cosine theta equals {} over {}, find the exact value of sine of 2 theta.",
                a, c, b, c
            );
            let metadata = serde_json::json!({
                "kind": "double_angle_sine",
                "a": a,
                "b": b,
                "c": c,
                "num": 2 * a * b,
                "den": c * c,
                "linear_double_sine_trap": linear_double_sine_trap,
                "product_without_two_trap": product_without_two_trap,
                "target": target_str,
            });
            Ok((prompt_text, prompt_latex, prompt_speech, "rational".into(), None, Some(target_str), metadata, "rational-equality-v1".into()))
        }
        _ => {
            let (a1, b1, c1) = (3i64, 4i64, 5i64);
            let second_choices = [(5i64, 12i64, 13i64), (12, 5, 13), (8, 15, 17)];
            let (a2, b2, c2) = second_choices[uniform_inclusive(rng, 0, 2) as usize];
            // cos(alpha + beta) = cos(alpha)cos(beta) - sin(alpha)sin(beta) = (b1*b2 - a1*a2)/(c1*c2)
            let num = b1 * b2 - a1 * a2;
            let den = c1 * c2;
            let cos_sum = frac(num, den);
            let target_str = to_canonical(&cos_sum);
            let angle_diff_sign_trap = to_canonical(&frac(b1 * b2 + a1 * a2, den));
            let add_cosines_trap = to_canonical(&(frac(b1, c1) + frac(b2, c2)));
            let prompt_text = format!(
                "Acute angles α and β satisfy sin(α) = {}/{}, cos(α) = {}/{} and sin(β) = {}/{}, cos(β) = {}/{}. Use the angle-sum identity cos(α + β) = cos(α)cos(β) - sin(α)sin(β) to compute the exact value of cos(α + β).",
                a1, c1, b1, c1, a2, c2, b2, c2
            );
            let prompt_latex = format!(
                "\\cos(\\alpha + \\beta) = \\cos\\alpha\\cos\\beta - \\sin\\alpha\\sin\\beta = \\left(\\frac{{{}}}{{{}}}\\right)\\left(\\frac{{{}}}{{{}}}\\right) - \\left(\\frac{{{}}}{{{}}}\\right)\\left(\\frac{{{}}}{{{}}}\\right) = ?",
                b1, c1, b2, c2, a1, c1, a2, c2
            );
            let prompt_speech = format!(
                "Use the cosine angle sum formula to find cosine of alpha plus beta when sine alpha is {} over {}, cosine alpha is {} over {}, sine beta is {} over {}, and cosine beta is {} over {}.",
                a1, c1, b1, c1, a2, c2, b2, c2
            );
            let metadata = serde_json::json!({
                "kind": "angle_sum_cosine",
                "a1": a1,
                "b1": b1,
                "c1": c1,
                "a2": a2,
                "b2": b2,
                "c2": c2,
                "b1b2": b1 * b2,
                "a1a2": a1 * a2,
                "num": num,
                "den": den,
                "angle_diff_sign_trap": angle_diff_sign_trap,
                "add_cosines_trap": add_cosines_trap,
                "target": target_str,
            });
            Ok((prompt_text, prompt_latex, prompt_speech, "rational".into(), None, Some(target_str), metadata, "rational-equality-v1".into()))
        }
    }
}

/// Template: `trig.law_of_sines_cosines` (Course S05)
fn generate_trig_law_of_sines_cosines<R: DrawSource + ?Sized>(
    rng: &mut R,
    difficulty: u32,
) -> Result<(String, String, String, String, Option<Vec<String>>, Option<String>, Value, String), GenerationFailure> {
    match difficulty {
        1 => {
            let denoms = [3i64, 4, 6];
            let n = denoms[uniform_inclusive(rng, 0, 2) as usize];
            let m = n - 1;
            // arcsin(sin(m*pi/n)) where m/n in (1/2, 1): reference angle in [-pi/2, pi/2] is pi - m*pi/n = (1/n)*pi
            let k_val = frac(1, n);
            let target_str = to_canonical(&k_val);
            let unfolded_branch_trap = to_canonical(&frac(m, n));
            let negative_branch_trap = to_canonical(&frac(-1, n));
            let prompt_text = format!(
                "The principal branch of arcsin(y) has range [-π/2, π/2]. Evaluate arcsin(sin({}π/{})) in the form k·π and enter the exact rational coefficient k.",
                m, n
            );
            let prompt_latex = format!(
                "\\arcsin\\!\\left(\\sin\\!\\left(\\frac{{{}\\pi}}{{{}}}\\right)\\right) = k\\pi, \\quad k \\in \\left[-\\frac{{1}}{{2}}, \\frac{{1}}{{2}}\\right] \\implies k = ?",
                m, n
            );
            let prompt_speech = format!(
                "Evaluate arcsine of sine of {} pi over {} in the form k pi, and enter the exact rational coefficient k.",
                m, n
            );
            let metadata = serde_json::json!({
                "kind": "inverse_trig_principal_branch",
                "m": m,
                "n": n,
                "unfolded_branch_trap": unfolded_branch_trap,
                "negative_branch_trap": negative_branch_trap,
                "target": target_str,
            });
            Ok((prompt_text, prompt_latex, prompt_speech, "rational".into(), None, Some(target_str), metadata, "rational-equality-v1".into()))
        }
        2 => {
            let side_a = uniform_inclusive(rng, 6, 15) as i64;
            let p1 = uniform_inclusive(rng, 2, 4) as i64;
            let q1 = 5i64;
            let p2 = uniform_inclusive(rng, 3, 7) as i64;
            let q2 = 8i64;
            // b = a * sin(B) / sin(A) = side_a * (p2/q2) / (p1/q1)
            let side_b = frac(side_a * p2 * q1, q2 * p1);
            let target_str = to_canonical(&side_b);
            let inverted_sine_law_trap = to_canonical(&frac(side_a * p1 * q2, q1 * p2));
            let unscaled_sine_trap = to_canonical(&frac(p2 * q1, q2 * p1));
            let prompt_text = format!(
                "In triangle ABC, side a = {}, sin(A) = {}/{}, and sin(B) = {}/{}. Use the Law of Sines (a / sin A = b / sin B) to find the exact length of side b.",
                side_a, p1, q1, p2, q2
            );
            let prompt_latex = format!(
                "\\frac{{a}}{{\\sin A}} = \\frac{{b}}{{\\sin B}}, \\quad a = {}, \\; \\sin A = \\frac{{{}}}{{{}}}, \\; \\sin B = \\frac{{{}}}{{{}}} \\implies b = ?",
                side_a, p1, q1, p2, q2
            );
            let prompt_speech = format!(
                "In triangle A B C, side a equals {}, sine A equals {} over {}, and sine B equals {} over {}. Use the Law of Sines to find side b.",
                side_a, p1, q1, p2, q2
            );
            let metadata = serde_json::json!({
                "kind": "law_of_sines_side",
                "side_a": side_a,
                "p1": p1,
                "q1": q1,
                "p2": p2,
                "q2": q2,
                "inverted_sine_law_trap": inverted_sine_law_trap,
                "unscaled_sine_trap": unscaled_sine_trap,
                "target": target_str,
            });
            Ok((prompt_text, prompt_latex, prompt_speech, "rational".into(), None, Some(target_str), metadata, "rational-equality-v1".into()))
        }
        _ => {
            let triangles = [(5i64, 6i64, 7i64), (4, 5, 6), (5, 7, 8), (6, 7, 8), (3, 5, 7)];
            let (a, b, c) = triangles[uniform_inclusive(rng, 0, (triangles.len() - 1) as i128) as usize];
            let num = a * a + b * b - c * c;
            let den = 2 * a * b;
            let cos_c = frac(num, den);
            let target_str = to_canonical(&cos_c);
            let sign_cosine_law_trap = to_canonical(&frac(-num, den));
            let forgot_two_denom_trap = to_canonical(&frac(num, a * b));
            let prompt_text = format!(
                "In triangle ABC with side lengths a = {}, b = {}, and c = {}, use the Law of Cosines (c² = a² + b² - 2ab cos C) to find the exact rational value of cos(C).",
                a, b, c
            );
            let prompt_latex = format!(
                "c^2 = a^2 + b^2 - 2ab\\cos C, \\quad a = {}, \\; b = {}, \\; c = {} \\implies \\cos C = \\frac{{a^2 + b^2 - c^2}}{{2ab}} = ?",
                a, b, c
            );
            let prompt_speech = format!(
                "In triangle A B C with sides a equals {}, b equals {}, and c equals {}, use the Law of Cosines to find the exact value of cosine C.",
                a, b, c
            );
            let metadata = serde_json::json!({
                "kind": "law_of_cosines_angle_cos",
                "a": a,
                "b": b,
                "c": c,
                "num": num,
                "den": den,
                "sign_cosine_law_trap": sign_cosine_law_trap,
                "forgot_two_denom_trap": forgot_two_denom_trap,
                "target": target_str,
            });
            Ok((prompt_text, prompt_latex, prompt_speech, "rational".into(), None, Some(target_str), metadata, "rational-equality-v1".into()))
        }
    }
}

/// Template: `sequences.arithmetic_progression` (Course S06)
fn generate_sequences_arithmetic_progression<R: DrawSource + ?Sized>(
    rng: &mut R,
    difficulty: u32,
) -> Result<(String, String, String, String, Option<Vec<String>>, Option<String>, Value, String), GenerationFailure> {
    match difficulty {
        1 => {
            let a1 = uniform_inclusive(rng, -5, 12) as i64;
            let d = uniform_inclusive(rng, 3, 9) as i64;
            let n = uniform_inclusive(rng, 10, 22) as i64;
            let an = a1 + (n - 1) * d;
            let n_steps_trap = a1 + n * d;
            let mult_trap = a1 * n + d;
            let prompt_text = format!(
                "An arithmetic sequence has first term a₁ = {} and common difference d = {}. Find the exact value of the {}-th term a_{}.",
                a1, d, n, n
            );
            let prompt_latex = format!(
                "a_1 = {}, \\; d = {} \\implies a_{{{}}} = a_1 + ({} - 1)d = ?",
                a1, d, n, n
            );
            let prompt_speech = format!(
                "An arithmetic sequence has first term {} and common difference {}. Find the {}-th term.",
                a1, d, n
            );
            let metadata = serde_json::json!({
                "kind": "arithmetic_nth_term",
                "a1": a1,
                "d": d,
                "n": n,
                "n_steps_trap": n_steps_trap,
                "mult_trap": mult_trap,
                "target": an,
            });
            Ok((prompt_text, prompt_latex, prompt_speech, "integer".into(), None, Some(an.to_string()), metadata, "integer-exact-v1".into()))
        }
        2 => {
            let a1 = uniform_inclusive(rng, 2, 10) as i64;
            let d = uniform_inclusive(rng, 2, 6) as i64;
            let n = (uniform_inclusive(rng, 4, 9) as i64) * 2;
            let an = a1 + (n - 1) * d;
            let sum_n = n * (a1 + an) / 2;
            let unhalved_sum_trap = n * (a1 + an);
            let nth_times_n_trap = n * an;
            let prompt_text = format!(
                "Find the exact sum S_{} of the first {} terms of the arithmetic sequence with first term a₁ = {} and common difference d = {}.",
                n, n, a1, d
            );
            let prompt_latex = format!(
                "a_1 = {}, \\; d = {} \\implies S_{{{}}} = \\frac{{{}(a_1 + a_{{{}}})}}{{2}} = ?",
                a1, d, n, n, n
            );
            let prompt_speech = format!(
                "Find the sum of the first {} terms of the arithmetic sequence with first term {} and common difference {}.",
                n, a1, d
            );
            let metadata = serde_json::json!({
                "kind": "arithmetic_series_sum",
                "a1": a1,
                "d": d,
                "n": n,
                "an": an,
                "unhalved_sum_trap": unhalved_sum_trap,
                "nth_times_n_trap": nth_times_n_trap,
                "target": sum_n,
            });
            Ok((prompt_text, prompt_latex, prompt_speech, "integer".into(), None, Some(sum_n.to_string()), metadata, "integer-exact-v1".into()))
        }
        _ => {
            let p = uniform_inclusive(rng, 3, 6) as i64;
            let step_gap = uniform_inclusive(rng, 4, 7) as i64;
            let q = p + step_gap;
            let ap = uniform_inclusive(rng, 4, 15) as i64;
            let delta_val = uniform_inclusive(rng, 9, 25) as i64;
            let aq = ap + delta_val;
            let d_val = frac(delta_val, step_gap);
            let target_str = to_canonical(&d_val);
            let div_by_q_trap = to_canonical(&frac(delta_val, q));
            let inclusive_count_trap = to_canonical(&frac(delta_val, step_gap + 1));
            let prompt_text = format!(
                "In an arithmetic sequence, the {}-th term is a_{} = {} and the {}-th term is a_{} = {}. Find the exact rational common difference d.",
                p, p, ap, q, q, aq
            );
            let prompt_latex = format!(
                "a_{{{}}} = {}, \\; a_{{{}}} = {} \\implies d = \\frac{{a_{{{}}} - a_{{{}}}}}{{{} - {}}} = ?",
                p, ap, q, aq, q, p, q, p
            );
            let prompt_speech = format!(
                "In an arithmetic sequence, term {} is {} and term {} is {}. Find the exact common difference d.",
                p, ap, q, aq
            );
            let metadata = serde_json::json!({
                "kind": "arithmetic_distant_terms_d",
                "p": p,
                "q": q,
                "ap": ap,
                "aq": aq,
                "step_gap": step_gap,
                "delta_val": delta_val,
                "div_by_q_trap": div_by_q_trap,
                "inclusive_count_trap": inclusive_count_trap,
                "target": target_str,
            });
            Ok((prompt_text, prompt_latex, prompt_speech, "rational".into(), None, Some(target_str), metadata, "rational-equality-v1".into()))
        }
    }
}

/// Template: `sequences.geometric_and_infinite` (Course S06)
fn generate_sequences_geometric_and_infinite<R: DrawSource + ?Sized>(
    rng: &mut R,
    difficulty: u32,
) -> Result<(String, String, String, String, Option<Vec<String>>, Option<String>, Value, String), GenerationFailure> {
    match difficulty {
        1 => {
            let a = uniform_inclusive(rng, 2, 5) as i64;
            let r = uniform_inclusive(rng, 2, 3) as i64;
            let n = uniform_inclusive(rng, 4, 6) as u32;
            let gn = a * r.pow(n - 1);
            let exp_n_trap = a * r.pow(n);
            let base_mult_trap = (a * r).pow(n - 1);
            let prompt_text = format!(
                "A geometric sequence has first term g₁ = {} and common ratio r = {}. Find the exact value of the {}-th term g_{}.",
                a, r, n, n
            );
            let prompt_latex = format!(
                "g_1 = {}, \\; r = {} \\implies g_{{{}}} = g_1 \\cdot r^{{{} - 1}} = ?",
                a, r, n, n
            );
            let prompt_speech = format!(
                "A geometric sequence has first term {} and common ratio {}. Find the {}-th term.",
                a, r, n
            );
            let metadata = serde_json::json!({
                "kind": "geometric_nth_term",
                "a": a,
                "r": r,
                "n": n as i64,
                "exp_n_trap": exp_n_trap,
                "base_mult_trap": base_mult_trap,
                "target": gn,
            });
            Ok((prompt_text, prompt_latex, prompt_speech, "integer".into(), None, Some(gn.to_string()), metadata, "integer-exact-v1".into()))
        }
        2 => {
            let a = uniform_inclusive(rng, 2, 5) as i64;
            let r = uniform_inclusive(rng, 2, 3) as i64;
            let n = uniform_inclusive(rng, 4, 5) as u32;
            let r_pow_n = r.pow(n);
            let sum_n = a * (r_pow_n - 1) / (r - 1);
            let forgot_denom_trap = a * (r_pow_n - 1);
            let n_minus_1_trap = a * (r.pow(n - 1) - 1) / (r - 1);
            let prompt_text = format!(
                "Find the exact sum S_{} of the first {} terms of the geometric series with first term a = {} and common ratio r = {}.",
                n, n, a, r
            );
            let prompt_latex = format!(
                "a = {}, \\; r = {} \\implies S_{{{}}} = \\frac{{a(r^{{{}}} - 1)}}{{r - 1}} = ?",
                a, r, n, n
            );
            let prompt_speech = format!(
                "Find the sum of the first {} terms of the geometric series with first term {} and common ratio {}.",
                n, a, r
            );
            let metadata = serde_json::json!({
                "kind": "geometric_finite_sum",
                "a": a,
                "r": r,
                "n": n as i64,
                "r_pow_n": r_pow_n,
                "forgot_denom_trap": forgot_denom_trap,
                "n_minus_1_trap": n_minus_1_trap,
                "target": sum_n,
            });
            Ok((prompt_text, prompt_latex, prompt_speech, "integer".into(), None, Some(sum_n.to_string()), metadata, "integer-exact-v1".into()))
        }
        _ => {
            let a = uniform_inclusive(rng, 3, 12) as i64;
            let p = uniform_inclusive(rng, 1, 3) as i64;
            let q_den = p + (uniform_inclusive(rng, 2, 4) as i64);
            // S_inf = a / (1 - p/q) = a * q / (q - p)
            let s_inf = frac(a * q_den, q_den - p);
            let target_str = to_canonical(&s_inf);
            let plus_r_trap = to_canonical(&frac(a * q_den, q_den + p));
            let mult_instead_div_trap = to_canonical(&frac(a * (q_den - p), q_den));
            let prompt_text = format!(
                "Find the exact sum S_∞ of the convergent infinite geometric series with first term a = {} and common ratio r = {}/{} (where |r| < 1).",
                a, p, q_den
            );
            let prompt_latex = format!(
                "a = {}, \\; r = \\frac{{{}}}{{{}}} \\implies S_\\infty = \\frac{{a}}{{1 - r}} = ?",
                a, p, q_den
            );
            let prompt_speech = format!(
                "Find the sum of the infinite geometric series with first term {} and common ratio {} over {}.",
                a, p, q_den
            );
            let metadata = serde_json::json!({
                "kind": "infinite_geometric_sum",
                "a": a,
                "p": p,
                "q": q_den,
                "plus_r_trap": plus_r_trap,
                "mult_instead_div_trap": mult_instead_div_trap,
                "target": target_str,
            });
            Ok((prompt_text, prompt_latex, prompt_speech, "rational".into(), None, Some(target_str), metadata, "rational-equality-v1".into()))
        }
    }
}

/// Template: `sequences.recursion_binomial` (Course S06)
fn generate_sequences_recursion_binomial<R: DrawSource + ?Sized>(
    rng: &mut R,
    difficulty: u32,
) -> Result<(String, String, String, String, Option<Vec<String>>, Option<String>, Value, String), GenerationFailure> {
    match difficulty {
        1 => {
            let a1 = uniform_inclusive(rng, 1, 4) as i64;
            let c = uniform_inclusive(rng, 2, 3) as i64;
            let d = uniform_inclusive(rng, 1, 5) as i64;
            let a2 = c * a1 + d;
            let a3 = c * a2 + d;
            let a4 = c * a3 + d;
            let stop_at_a3_trap = a3;
            let arithmetic_trap = a1 + 3 * d;
            let prompt_text = format!(
                "A sequence is defined recursively by a₁ = {} and a_n = {}a_(n-1) + {} for n ≥ 2. Compute the exact value of a₄.",
                a1, c, d
            );
            let prompt_latex = format!(
                "a_1 = {}, \\; a_n = {}a_{{n-1}} + {} \\implies a_4 = ?",
                a1, c, d
            );
            let prompt_speech = format!(
                "A sequence satisfies a 1 equals {} and a n equals {} a n minus 1 plus {}. Find a 4.",
                a1, c, d
            );
            let metadata = serde_json::json!({
                "kind": "linear_recurrence_a4",
                "a1": a1,
                "c": c,
                "d": d,
                "a2": a2,
                "a3": a3,
                "a4": a4,
                "stop_at_a3_trap": stop_at_a3_trap,
                "arithmetic_trap": arithmetic_trap,
                "target": a4,
            });
            Ok((prompt_text, prompt_latex, prompt_speech, "integer".into(), None, Some(a4.to_string()), metadata, "integer-exact-v1".into()))
        }
        2 => {
            let n = uniform_inclusive(rng, 4, 6) as u32;
            let k = uniform_inclusive(rng, 2, 3) as u32;
            let c = uniform_inclusive(rng, 2, 4) as i64;
            let mut binom = 1i64;
            for i in 1..=k {
                binom = binom * ((n - i + 1) as i64) / (i as i64);
            }
            let c_pow = c.pow(k);
            let coeff = binom * c_pow;
            let x_exp = (n - k) as i64;
            let binom_only_trap = binom;
            let power_only_trap = c_pow;
            let prompt_text = format!(
                "Using the Binomial Theorem, find the exact coefficient of x^{} in the expansion of (x + {})^{}.",
                x_exp, c, n
            );
            let prompt_latex = format!(
                "(x + {})^{{{}}} \\implies [x^{{{}}}] = \\binom{{{}}}{{{}}} \\cdot {}^{{{}}} = ?",
                c, n, x_exp, n, k, c, k
            );
            let prompt_speech = format!(
                "Find the coefficient of x to the power {} in the expansion of quantity x plus {} to the power {}.",
                x_exp, c, n
            );
            let metadata = serde_json::json!({
                "kind": "binomial_monic_coeff",
                "n": n as i64,
                "k": k as i64,
                "c": c,
                "x_exp": x_exp,
                "binom": binom,
                "c_pow": c_pow,
                "binom_only_trap": binom_only_trap,
                "power_only_trap": power_only_trap,
                "target": coeff,
            });
            Ok((prompt_text, prompt_latex, prompt_speech, "integer".into(), None, Some(coeff.to_string()), metadata, "integer-exact-v1".into()))
        }
        _ => {
            let n = 4u32;
            let k = 2u32;
            let a = uniform_inclusive(rng, 2, 3) as i64;
            let b = uniform_inclusive(rng, 2, 4) as i64;
            let binom = 6i64; // C(4, 2) = 6
            let a_pow = a.pow(n - k);
            let b_pow = b.pow(k);
            let coeff = binom * a_pow * b_pow;
            let forgot_a_pow_trap = binom * b_pow;
            let unexponentiated_ab_trap = binom * a * b;
            let prompt_text = format!(
                "Find the exact coefficient of x² in the binomial expansion of ({}x + {})⁴.",
                a, b
            );
            let prompt_latex = format!(
                "({}x + {})^4 \\implies [x^2] = \\binom{{4}}{{2}} ({})^2 ({})^2 = ?",
                a, b, a, b
            );
            let prompt_speech = format!(
                "Find the coefficient of x squared in the expansion of {} x plus {} to the fourth power.",
                a, b
            );
            let metadata = serde_json::json!({
                "kind": "binomial_general_coeff",
                "n": 4,
                "k": 2,
                "a": a,
                "b": b,
                "binom": binom,
                "a_pow": a_pow,
                "b_pow": b_pow,
                "forgot_a_pow_trap": forgot_a_pow_trap,
                "unexponentiated_ab_trap": unexponentiated_ab_trap,
                "target": coeff,
            });
            Ok((prompt_text, prompt_latex, prompt_speech, "integer".into(), None, Some(coeff.to_string()), metadata, "integer-exact-v1".into()))
        }
    }
}

/// Template: `calculus.limits_rational` (Course S07)
fn generate_calculus_limits_rational<R: DrawSource + ?Sized>(
    rng: &mut R,
    difficulty: u32,
) -> Result<(String, String, String, String, Option<Vec<String>>, Option<String>, Value, String), GenerationFailure> {
    match difficulty {
        1 => {
            let r = uniform_inclusive(rng, 3, 9) as i64;
            let s = uniform_inclusive(rng, -6, 2) as i64;
            // (x - r)(x - s) / (x - r) = x - s -> r - s as x -> r
            let sum_rs = r + s;
            let prod_rs = r * s;
            let lim_val = r - s;
            let zero_num_trap = 0i64;
            let sign_factor_trap = r + s;
            let mid_str = if sum_rs >= 0 {
                format!("- {}x", sum_rs)
            } else {
                format!("+ {}x", -sum_rs)
            };
            let const_str = if prod_rs >= 0 {
                format!("+ {}", prod_rs)
            } else {
                format!("- {}", -prod_rs)
            };
            let prompt_text = format!(
                "Evaluate the exact limit of the rational function with a removable discontinuity: lim_(x → {}) (x² {} {}) / (x - {}).",
                r, mid_str, const_str, r
            );
            let prompt_latex = format!(
                "\\lim_{{x \\to {}}} \\frac{{x^2 {} {}}}{{x - {}}} = ?",
                r, mid_str, const_str, r
            );
            let prompt_speech = format!(
                "Evaluate the limit as x approaches {} of x squared {} {} divided by x minus {}.",
                r, mid_str, const_str, r
            );
            let metadata = serde_json::json!({
                "kind": "removable_limit_quadratic",
                "r": r,
                "s": s,
                "sum_rs": sum_rs,
                "prod_rs": prod_rs,
                "zero_num_trap": zero_num_trap,
                "sign_factor_trap": sign_factor_trap,
                "target": lim_val,
            });
            Ok((prompt_text, prompt_latex, prompt_speech, "integer".into(), None, Some(lim_val.to_string()), metadata, "integer-exact-v1".into()))
        }
        2 => {
            let a = uniform_inclusive(rng, 2, 9) as i64;
            let mut d = uniform_inclusive(rng, 2, 9) as i64;
            if a == d {
                d += 1;
            }
            let b = uniform_inclusive(rng, 3, 11) as i64;
            let c = uniform_inclusive(rng, 4, 15) as i64;
            let e = uniform_inclusive(rng, 2, 8) as i64;
            let f = uniform_inclusive(rng, 3, 13) as i64;
            let lim_inf = frac(a, d);
            let target_str = to_canonical(&lim_inf);
            let const_ratio_trap = to_canonical(&frac(c, f));
            let inverted_lead_trap = to_canonical(&frac(d, a));
            let prompt_text = format!(
                "Evaluate the exact limit at infinity: lim_(x → ∞) ({}x² + {}x + {}) / ({}x² - {}x + {}).",
                a, b, c, d, e, f
            );
            let prompt_latex = format!(
                "\\lim_{{x \\to \\infty}} \\frac{{{}x^2 + {}x + {}}}{{{}x^2 - {}x + {}}} = ?",
                a, b, c, d, e, f
            );
            let prompt_speech = format!(
                "Evaluate the limit as x approaches infinity of {} x squared plus {} x plus {} over {} x squared minus {} x plus {}.",
                a, b, c, d, e, f
            );
            let metadata = serde_json::json!({
                "kind": "limit_at_infinity_rational",
                "a": a,
                "b": b,
                "c": c,
                "d": d,
                "e": e,
                "f": f,
                "const_ratio_trap": const_ratio_trap,
                "inverted_lead_trap": inverted_lead_trap,
                "target": target_str,
            });
            Ok((prompt_text, prompt_latex, prompt_speech, "rational".into(), None, Some(target_str), metadata, "rational-equality-v1".into()))
        }
        _ => {
            let a = uniform_inclusive(rng, 2, 6) as i64;
            let b = uniform_inclusive(rng, 1, 9) as i64;
            // Left limit of (x^2 - a^2)/(x - a) as x -> a is 2a. Set k*a + b = 2a => k = (2a - b)/a
            let left_lim = 2 * a;
            let k_val = frac(left_lim - b, a);
            let target_str = to_canonical(&k_val);
            let zero_limit_trap = to_canonical(&frac(-b, a));
            let unscaled_k_trap = to_canonical(&int(left_lim - b));
            let prompt_text = format!(
                "For x ≠ {}, f(x) = (x² - {}) / (x - {}), and at x = {}, f({}) = k·({}) + {}. Find the exact rational value of k that makes f(x) continuous at x = {}.",
                a, a * a, a, a, a, a, b, a
            );
            let prompt_latex = format!(
                "\\lim_{{x \\to {}}} \\frac{{x^2 - {}}}{{x - {}}} = {}k + {} \\implies k = ?",
                a, a * a, a, a, b
            );
            let prompt_speech = format!(
                "Find the exact rational value of k so that x squared minus {} over x minus {} is continuous at x equals {} when f of {} equals {} k plus {}.",
                a * a, a, a, a, a, b
            );
            let metadata = serde_json::json!({
                "kind": "continuity_param_k",
                "a": a,
                "b": b,
                "left_lim": left_lim,
                "zero_limit_trap": zero_limit_trap,
                "unscaled_k_trap": unscaled_k_trap,
                "target": target_str,
            });
            Ok((prompt_text, prompt_latex, prompt_speech, "rational".into(), None, Some(target_str), metadata, "rational-equality-v1".into()))
        }
    }
}

/// Template: `calculus.derivatives_tangent` (Course S07)
fn generate_calculus_derivatives_tangent<R: DrawSource + ?Sized>(
    rng: &mut R,
    difficulty: u32,
) -> Result<(String, String, String, String, Option<Vec<String>>, Option<String>, Value, String), GenerationFailure> {
    match difficulty {
        1 => {
            let a = uniform_inclusive(rng, 1, 3) as i64;
            let b = uniform_inclusive(rng, -5, 5) as i64;
            let c = uniform_inclusive(rng, 2, 8) as i64;
            let d = uniform_inclusive(rng, 3, 11) as i64;
            let x0 = uniform_inclusive(rng, 1, 3) as i64;
            // f(x) = a x^3 + b x^2 + c x + d => f'(x0) = 3a x0^2 + 2b x0 + c
            let deriv = 3 * a * x0 * x0 + 2 * b * x0 + c;
            let kept_const_trap = deriv + d;
            let func_eval_trap = a * x0 * x0 * x0 + b * x0 * x0 + c * x0 + d;
            let b_str = if b >= 0 { format!("+ {}x²", b) } else { format!("- {}x²", -b) };
            let prompt_text = format!(
                "Differentiate f(x) = {}x³ {} + {}x + {} using the power rule and evaluate the derivative f'({}).",
                a, b_str, c, d, x0
            );
            let prompt_latex = format!(
                "f(x) = {}x^3 {} + {}x + {} \\implies f'({}) = ?",
                a, b_str, c, d, x0
            );
            let prompt_speech = format!(
                "Differentiate f of x equals {} x cubed {} plus {} x plus {} and evaluate f prime of {}.",
                a, b_str, c, d, x0
            );
            let metadata = serde_json::json!({
                "kind": "power_rule_eval",
                "a": a,
                "b": b,
                "c": c,
                "d": d,
                "x0": x0,
                "kept_const_trap": kept_const_trap,
                "func_eval_trap": func_eval_trap,
                "target": deriv,
            });
            Ok((prompt_text, prompt_latex, prompt_speech, "integer".into(), None, Some(deriv.to_string()), metadata, "integer-exact-v1".into()))
        }
        2 => {
            let a = uniform_inclusive(rng, 1, 3) as i64;
            let b = uniform_inclusive(rng, 2, 6) as i64;
            let c = uniform_inclusive(rng, 3, 10) as i64;
            let x0 = uniform_inclusive(rng, 2, 4) as i64;
            let y0 = a * x0 * x0 + b * x0 + c;
            let m_tan = 2 * a * x0 + b;
            // Tangent line y = m_tan * (x - x0) + y0 => y-intercept B = y0 - m_tan * x0 = c - a * x0^2
            let tan_intercept = y0 - m_tan * x0;
            let slope_only_trap = m_tan;
            let curve_intercept_trap = c;
            let prompt_text = format!(
                "Find the exact y-intercept B of the line tangent to the parabola f(x) = {}x² + {}x + {} at x = {} (where the tangent line is written as y = mx + B).",
                a, b, c, x0
            );
            let prompt_latex = format!(
                "f(x) = {}x^2 + {}x + {}, \\; x_0 = {} \\implies B = f(x_0) - f'(x_0)x_0 = ?",
                a, b, c, x0
            );
            let prompt_speech = format!(
                "Find the y intercept B of the line tangent to {} x squared plus {} x plus {} at x equals {}.",
                a, b, c, x0
            );
            let metadata = serde_json::json!({
                "kind": "tangent_line_intercept",
                "a": a,
                "b": b,
                "c": c,
                "x0": x0,
                "y0": y0,
                "m_tan": m_tan,
                "slope_only_trap": slope_only_trap,
                "curve_intercept_trap": curve_intercept_trap,
                "target": tan_intercept,
            });
            Ok((prompt_text, prompt_latex, prompt_speech, "integer".into(), None, Some(tan_intercept.to_string()), metadata, "integer-exact-v1".into()))
        }
        _ => {
            let a = uniform_inclusive(rng, 2, 5) as i64;
            let b = uniform_inclusive(rng, 3, 11) as i64;
            let c = uniform_inclusive(rng, 1, 9) as i64;
            // f(x) = a x^2 - b x + c => f'(x) = 2a x - b = 0 => x = b / (2a)
            let x_crit = frac(b, 2 * a);
            let target_str = to_canonical(&x_crit);
            let forgot_two_deriv_trap = to_canonical(&frac(b, a));
            let sign_crit_trap = to_canonical(&frac(-b, 2 * a));
            let prompt_text = format!(
                "Find the exact rational critical point x where the function f(x) = {}x² - {}x + {} has a horizontal tangent line (f'(x) = 0).",
                a, b, c
            );
            let prompt_latex = format!(
                "f(x) = {}x^2 - {}x + {}, \\; f'(x) = 0 \\implies x = \\frac{{{}}}{{2 \\cdot {}}} = ?",
                a, b, c, b, a
            );
            let prompt_speech = format!(
                "Find the critical point x where {} x squared minus {} x plus {} has a horizontal tangent line.",
                a, b, c
            );
            let metadata = serde_json::json!({
                "kind": "critical_point_quadratic",
                "a": a,
                "b": b,
                "c": c,
                "forgot_two_deriv_trap": forgot_two_deriv_trap,
                "sign_crit_trap": sign_crit_trap,
                "target": target_str,
            });
            Ok((prompt_text, prompt_latex, prompt_speech, "rational".into(), None, Some(target_str), metadata, "rational-equality-v1".into()))
        }
    }
}

/// Template: `calculus.definite_integral_ftc` (Course S07)
fn generate_calculus_definite_integral_ftc<R: DrawSource + ?Sized>(
    rng: &mut R,
    difficulty: u32,
) -> Result<(String, String, String, String, Option<Vec<String>>, Option<String>, Value, String), GenerationFailure> {
    match difficulty {
        1 => {
            let a = uniform_inclusive(rng, 1, 4) as i64;
            let b = uniform_inclusive(rng, 2, 7) as i64;
            let t = uniform_inclusive(rng, 2, 5) as i64;
            // int_0^t (2a x + b) dx = [a x^2 + b x]_0^t = a t^2 + b t
            let area = a * t * t + b * t;
            let unhalved_antideriv_trap = 2 * a * t * t + b * t;
            let integrand_eval_trap = 2 * a * t + b;
            let prompt_text = format!(
                "Use the Fundamental Theorem of Calculus to evaluate the exact definite integral ∫₀^{} ({}x + {}) dx.",
                t, 2 * a, b
            );
            let prompt_latex = format!(
                "\\int_0^{{{}}} ({}x + {})\\,dx = \\left[{}x^2 + {}x\\right]_0^{{{}}} = ?",
                t, 2 * a, b, a, b, t
            );
            let prompt_speech = format!(
                "Evaluate the definite integral from 0 to {} of {} x plus {} d x.",
                t, 2 * a, b
            );
            let metadata = serde_json::json!({
                "kind": "definite_integral_linear",
                "a": a,
                "b": b,
                "t": t,
                "unhalved_antideriv_trap": unhalved_antideriv_trap,
                "integrand_eval_trap": integrand_eval_trap,
                "target": area,
            });
            Ok((prompt_text, prompt_latex, prompt_speech, "integer".into(), None, Some(area.to_string()), metadata, "integer-exact-v1".into()))
        }
        2 => {
            let a = uniform_inclusive(rng, 1, 5) as i64;
            let c = uniform_inclusive(rng, 1, 6) as i64;
            let b = uniform_inclusive(rng, 2, 4) as i64;
            // int_0^b (a x^2 + c) dx = a b^3 / 3 + c b = (a b^3 + 3 c b) / 3
            let val = frac(a * b * b * b + 3 * c * b, 3);
            let target_str = to_canonical(&val);
            let forgot_div_3_trap = to_canonical(&int(a * b * b * b + c * b));
            let forgot_const_antideriv_trap = to_canonical(&frac(a * b * b * b + 3 * c, 3));
            let prompt_text = format!(
                "Evaluate the exact definite integral ∫₀^{} ({}x² + {}) dx over ℚ.",
                b, a, c
            );
            let prompt_latex = format!(
                "\\int_0^{{{}}} ({}x^2 + {})\\,dx = \\left[\\frac{{{}}}{{3}}x^3 + {}x\\right]_0^{{{}}} = ?",
                b, a, c, a, c, b
            );
            let prompt_speech = format!(
                "Evaluate the definite integral from 0 to {} of {} x squared plus {} d x.",
                b, a, c
            );
            let metadata = serde_json::json!({
                "kind": "definite_integral_quadratic_q",
                "a": a,
                "b": b,
                "c": c,
                "b_cubed": b * b * b,
                "forgot_div_3_trap": forgot_div_3_trap,
                "forgot_const_antideriv_trap": forgot_const_antideriv_trap,
                "target": target_str,
            });
            Ok((prompt_text, prompt_latex, prompt_speech, "rational".into(), None, Some(target_str), metadata, "rational-equality-v1".into()))
        }
        _ => {
            let s0 = uniform_inclusive(rng, 5, 18) as i64;
            let a = uniform_inclusive(rng, 1, 3) as i64;
            let b = uniform_inclusive(rng, 1, 4) as i64;
            let t = uniform_inclusive(rng, 2, 3) as i64;
            // v(t) = 3a t^2 + 2b t => disp = a t^3 + b t^2, s(t) = s0 + disp
            let disp = a * t * t * t + b * t * t;
            let st = s0 + disp;
            let disp_only_trap = disp;
            let prompt_text = format!(
                "A particle moves along a line with velocity v(t) = {}t² + {}t and initial position s(0) = {}. By the Net Change Theorem s({}) = s(0) + ∫₀^{} v(t) dt, find the exact position s({}).",
                3 * a, 2 * b, s0, t, t, t
            );
            let prompt_latex = format!(
                "s(0) = {}, \\; v(t) = {}t^2 + {}t \\implies s({}) = s(0) + \\int_0^{{{}}} v(t)\\,dt = ?",
                s0, 3 * a, 2 * b, t, t
            );
            let prompt_speech = format!(
                "A particle has velocity {} t squared plus {} t and initial position s of 0 equals {}. Find its position s of {}.",
                3 * a, 2 * b, s0, t
            );
            let metadata = serde_json::json!({
                "kind": "net_change_initial_value",
                "s0": s0,
                "a": a,
                "b": b,
                "t": t,
                "disp": disp,
                "disp_only_trap": disp_only_trap,
                "target": st,
            });
            Ok((prompt_text, prompt_latex, prompt_speech, "integer".into(), None, Some(st.to_string()), metadata, "integer-exact-v1".into()))
        }
    }
}

/// Template: `proof.sets_and_cardinality` (Course P01)
fn generate_proof_sets_and_cardinality<R: DrawSource + ?Sized>(
    rng: &mut R,
    difficulty: u32,
) -> Result<(String, String, String, String, Option<Vec<String>>, Option<String>, Value, String), GenerationFailure> {
    match difficulty {
        1 => {
            let cap_ab = uniform_inclusive(rng, 4, 14) as i64;
            let only_a = uniform_inclusive(rng, 6, 18) as i64;
            let only_b = uniform_inclusive(rng, 5, 17) as i64;
            let card_a = only_a + cap_ab;
            let card_b = only_b + cap_ab;
            let union_ab = card_a + card_b - cap_ab;
            let double_count_trap = card_a + card_b;
            let add_inter_trap = card_a + card_b + cap_ab;
            let prompt_text = format!(
                "Finite sets A and B have cardinalities |A| = {}, |B| = {}, and intersection |A ∩ B| = {}. By the Principle of Inclusion-Exclusion, find the exact cardinality of the union |A ∪ B|.",
                card_a, card_b, cap_ab
            );
            let prompt_latex = format!(
                "|A| = {}, \\; |B| = {}, \\; |A \\cap B| = {} \\implies |A \\cup B| = |A| + |B| - |A \\cap B| = ?",
                card_a, card_b, cap_ab
            );
            let prompt_speech = format!(
                "Sets A and B have sizes {} and {}, and intersection size {}. Find the size of A union B.",
                card_a, card_b, cap_ab
            );
            let metadata = serde_json::json!({
                "kind": "two_set_inclusion_exclusion",
                "card_a": card_a,
                "card_b": card_b,
                "cap_ab": cap_ab,
                "double_count_trap": double_count_trap,
                "add_inter_trap": add_inter_trap,
                "target": union_ab,
            });
            Ok((prompt_text, prompt_latex, prompt_speech, "integer".into(), None, Some(union_ab.to_string()), metadata, "integer-exact-v1".into()))
        }
        2 => {
            let n = uniform_inclusive(rng, 4, 8) as u32;
            let pow_n = 2i64.pow(n);
            let proper_nonempty = pow_n - 2;
            let all_subsets_trap = pow_n;
            let proper_only_trap = pow_n - 1;
            let prompt_text = format!(
                "A finite set A has |A| = {} elements, so its power set P(A) has 2^{} subsets. How many non-empty proper subsets does A have (i.e., subsets S ⊆ A such that S ≠ ∅ and S ≠ A)?",
                n, n
            );
            let prompt_latex = format!(
                "|A| = {} \\implies |\\{{S \\subseteq A : S \\neq \\emptyset \\text{{ and }} S \\neq A\\}}| = 2^{{{}}} - 2 = ?",
                n, n
            );
            let prompt_speech = format!(
                "A finite set A has {} elements. How many non-empty proper subsets does A have?",
                n
            );
            let metadata = serde_json::json!({
                "kind": "nonempty_proper_subsets",
                "n": n as i64,
                "pow_n": pow_n,
                "all_subsets_trap": all_subsets_trap,
                "proper_only_trap": proper_only_trap,
                "target": proper_nonempty,
            });
            Ok((prompt_text, prompt_latex, prompt_speech, "integer".into(), None, Some(proper_nonempty.to_string()), metadata, "integer-exact-v1".into()))
        }
        _ => {
            let abc = uniform_inclusive(rng, 2, 7) as i64;
            let ab = abc + (uniform_inclusive(rng, 2, 6) as i64);
            let ac = abc + (uniform_inclusive(rng, 2, 6) as i64);
            let bc = abc + (uniform_inclusive(rng, 2, 6) as i64);
            let a = ab + ac - abc + (uniform_inclusive(rng, 4, 10) as i64);
            let b = ab + bc - abc + (uniform_inclusive(rng, 4, 10) as i64);
            let c = ac + bc - abc + (uniform_inclusive(rng, 4, 10) as i64);
            let single_sum = a + b + c;
            let pair_sum = ab + ac + bc;
            let union_abc = single_sum - pair_sum + abc;
            let subtract_triple_trap = single_sum - pair_sum - abc;
            let omit_triple_trap = single_sum - pair_sum;
            let prompt_text = format!(
                "For three finite sets A, B, C, suppose |A| = {}, |B| = {}, |C| = {}, pairwise intersections |A ∩ B| = {}, |A ∩ C| = {}, |B ∩ C| = {}, and triple intersection |A ∩ B ∩ C| = {}. Find |A ∪ B ∪ C|.",
                a, b, c, ab, ac, bc, abc
            );
            let prompt_latex = format!(
                "|A\\cup B\\cup C| = ({} + {} + {}) - ({} + {} + {}) + {} = ?",
                a, b, c, ab, ac, bc, abc
            );
            let prompt_speech = format!(
                "Find the size of A union B union C given sizes {}, {}, {}, pairwise intersections {}, {}, {}, and triple intersection {}.",
                a, b, c, ab, ac, bc, abc
            );
            let metadata = serde_json::json!({
                "kind": "three_set_inclusion_exclusion",
                "a": a,
                "b": b,
                "c": c,
                "ab": ab,
                "ac": ac,
                "bc": bc,
                "abc": abc,
                "single_sum": single_sum,
                "pair_sum": pair_sum,
                "subtract_triple_trap": subtract_triple_trap,
                "omit_triple_trap": omit_triple_trap,
                "target": union_abc,
            });
            Ok((prompt_text, prompt_latex, prompt_speech, "integer".into(), None, Some(union_abc.to_string()), metadata, "integer-exact-v1".into()))
        }
    }
}

/// Template: `proof.logic_truth_quantifiers` (Course P01)
fn generate_proof_logic_truth_quantifiers<R: DrawSource + ?Sized>(
    rng: &mut R,
    difficulty: u32,
) -> Result<(String, String, String, String, Option<Vec<String>>, Option<String>, Value, String), GenerationFailure> {
    match difficulty {
        1 => {
            let a = uniform_inclusive(rng, 2, 4) as i64;
            let b = uniform_inclusive(rng, 2, 3) as i64;
            let ab = a * b;
            let mult = uniform_inclusive(rng, 3, 5) as i64;
            let n_max = ab * mult;
            let count_p = n_max / a;
            let count_q = n_max / ab;
            let false_count = count_p - count_q;
            let true_count = n_max - false_count;
            let non_vacuous_only_trap = count_q;
            let falsifying_count_trap = false_count;
            let prompt_text = format!(
                "On the finite domain D = {{1, 2, ..., {}}}, consider the implication P(x) ⇒ Q(x), where P(x) is \"{} divides x\" and Q(x) is \"{} divides x\". Counting both elements where P(x) and Q(x) are both true AND elements where P(x) is false (vacuous truth), how many x ∈ D make P(x) ⇒ Q(x) true?",
                n_max, a, ab
            );
            let prompt_latex = format!(
                "D = \\{{1,\\dots,{}\\}}, \\; ({} \\mid x \\implies {} \\mid x) \\implies |\\{{x \\in D : P(x) \\implies Q(x)\\}}| = ?",
                n_max, a, ab
            );
            let prompt_speech = format!(
                "On the domain 1 through {}, how many integers x make the implication if {} divides x then {} divides x true?",
                n_max, a, ab
            );
            let metadata = serde_json::json!({
                "kind": "finite_domain_implication_count",
                "a": a,
                "ab": ab,
                "n_max": n_max,
                "count_p": count_p,
                "count_q": count_q,
                "false_count": false_count,
                "non_vacuous_only_trap": non_vacuous_only_trap,
                "falsifying_count_trap": falsifying_count_trap,
                "target": true_count,
            });
            Ok((prompt_text, prompt_latex, prompt_speech, "integer".into(), None, Some(true_count.to_string()), metadata, "integer-exact-v1".into()))
        }
        2 => {
            let a = uniform_inclusive(rng, 2, 6) as i64;
            let k = uniform_inclusive(rng, 4, 12) as i64;
            let b = uniform_inclusive(rng, 3, 15) as i64;
            let c = a * k + b;
            // Negation of "for all x in Z, a x + b < c" is "there exists x in Z such that a x + b >= c", i.e. x >= k
            let min_witness = k;
            let strict_inequality_trap = k + 1;
            let unshifted_trap = (c + b) / a;
            let prompt_text = format!(
                "To disprove the universal claim \"∀x ∈ ℤ, {}x + {} < {}\", we form its logical negation \"∃x ∈ ℤ, {}x + {} ≥ {}\". Find the smallest integer witness x that satisfies this negated inequality.",
                a, b, c, a, b, c
            );
            let prompt_latex = format!(
                "\\neg(\\forall x \\in \\mathbb{{Z}},\\; {}x + {} < {}) \\equiv (\\exists x \\in \\mathbb{{Z}},\\; {}x + {} \\ge {}) \\implies x_{{\\min}} = ?",
                a, b, c, a, b, c
            );
            let prompt_speech = format!(
                "Find the smallest integer witness x satisfying the negation of for all x, {} x plus {} is less than {}.",
                a, b, c
            );
            let metadata = serde_json::json!({
                "kind": "quantifier_negation_threshold",
                "a": a,
                "b": b,
                "c": c,
                "k": k,
                "strict_inequality_trap": strict_inequality_trap,
                "unshifted_trap": unshifted_trap,
                "target": min_witness,
            });
            Ok((prompt_text, prompt_latex, prompt_speech, "integer".into(), None, Some(min_witness.to_string()), metadata, "integer-exact-v1".into()))
        }
        _ => {
            let n = uniform_inclusive(rng, 4, 6) as u32;
            let k = uniform_inclusive(rng, 2, (n - 1) as i128) as u32;
            let total_rows = 2i64.pow(n);
            let free_vars = n - k - 1;
            let false_rows = 2i64.pow(free_vars);
            let true_rows = total_rows - false_rows;
            let minus_one_trap = total_rows - 1;
            let false_rows_trap = false_rows;
            let prompt_text = format!(
                "In a truth table with {} independent propositional variables P₁, P₂, ..., P_{}, how many of the 2^{} = {} rows make the implication (P₁ ∧ ... ∧ P_{}) ⇒ P_{} true?",
                n, n, n, total_rows, k, k + 1
            );
            let prompt_latex = format!(
                "n = {}, \\; (P_1 \\land \\dots \\land P_{{{}}}) \\implies P_{{{}}} \\;\\text{{is true in }} 2^{{{}}} - 2^{{{}}} = ? \\text{{ rows}}",
                n, k, k + 1, n, free_vars
            );
            let prompt_speech = format!(
                "In a truth table with {} variables, how many rows make P 1 through P {} implies P {} true?",
                n, k, k + 1
            );
            let metadata = serde_json::json!({
                "kind": "truth_table_implication_rows",
                "n": n as i64,
                "k": k as i64,
                "total_rows": total_rows,
                "free_vars": free_vars as i64,
                "false_rows": false_rows,
                "minus_one_trap": minus_one_trap,
                "false_rows_trap": false_rows_trap,
                "target": true_rows,
            });
            Ok((prompt_text, prompt_latex, prompt_speech, "integer".into(), None, Some(true_rows.to_string()), metadata, "integer-exact-v1".into()))
        }
    }
}

/// Template: `proof.relations_and_bijections` (Course P01)
fn generate_proof_relations_and_bijections<R: DrawSource + ?Sized>(
    rng: &mut R,
    difficulty: u32,
) -> Result<(String, String, String, String, Option<Vec<String>>, Option<String>, Value, String), GenerationFailure> {
    match difficulty {
        1 => {
            let m = uniform_inclusive(rng, 5, 13) as i64;
            let q = uniform_inclusive(rng, 2, 6) as i64;
            let r = uniform_inclusive(rng, 1, (m - 1) as i128) as i64;
            let a = -(q * m) + r;
            let neg_rem_trap = a % m; // negative remainder in Rust
            let complement_trap = m - r;
            let prompt_text = format!(
                "Congruence modulo {} partitions ℤ into equivalence classes [0], [1], ..., [{}]. Find the unique canonical representative r ∈ {{0, 1, ..., {}}} of the equivalence class [{}]_{}.",
                m, m - 1, m - 1, a, m
            );
            let prompt_latex = format!(
                "{} \\equiv r \\pmod{{{}}}, \\quad 0 \\le r < {} \\implies r = ?",
                a, m, m
            );
            let prompt_speech = format!(
                "Find the least non-negative remainder r when {} is divided by {}.",
                a, m
            );
            let metadata = serde_json::json!({
                "kind": "equivalence_class_rep",
                "a": a,
                "m": m,
                "q": -q,
                "r": r,
                "neg_rem_trap": neg_rem_trap,
                "complement_trap": complement_trap,
                "target": r,
            });
            Ok((prompt_text, prompt_latex, prompt_speech, "integer".into(), None, Some(r.to_string()), metadata, "integer-exact-v1".into()))
        }
        2 => {
            let k = uniform_inclusive(rng, 2, 3) as u32;
            let n = uniform_inclusive(rng, 4, 7) as i64;
            let mut inj = 1i64;
            let mut fact_k = 1i64;
            for i in 0..k {
                inj *= n - (i as i64);
                fact_k *= (i as i64) + 1;
            }
            let all_funcs_trap = n.pow(k);
            let subsets_trap = inj / fact_k;
            let prompt_text = format!(
                "Let A be a finite domain with |A| = {} elements and B be a codomain with |B| = {} elements. How many injective (one-to-one) functions f: A → B exist?",
                k, n
            );
            let prompt_latex = format!(
                "|A| = {}, \\; |B| = {} \\implies |\\{{f: A \\to B \\mid f \\text{{ is injective}}\\}}| = \\frac{{{}!}}{{({} - {})!}} = ?",
                k, n, n, n, k
            );
            let prompt_speech = format!(
                "How many injective functions exist from a set of size {} to a set of size {}?",
                k, n
            );
            let metadata = serde_json::json!({
                "kind": "injective_functions_count",
                "k": k as i64,
                "n": n,
                "all_funcs_trap": all_funcs_trap,
                "subsets_trap": subsets_trap,
                "target": inj,
            });
            Ok((prompt_text, prompt_latex, prompt_speech, "integer".into(), None, Some(inj.to_string()), metadata, "integer-exact-v1".into()))
        }
        _ => {
            let s1 = uniform_inclusive(rng, 2, 5) as i64;
            let s2 = uniform_inclusive(rng, 2, 5) as i64;
            let s3 = uniform_inclusive(rng, 1, 4) as i64;
            let total_pairs = s1 * s1 + s2 * s2 + s3 * s3;
            let unordered_no_refl_trap = s1 * (s1 - 1) / 2 + s2 * (s2 - 1) / 2 + s3 * (s3 - 1) / 2;
            let full_cartesian_trap = (s1 + s2 + s3) * (s1 + s2 + s3);
            let prompt_text = format!(
                "An equivalence relation R on a finite set S partitions S into three disjoint equivalence classes C₁, C₂, C₃ of sizes |C₁| = {}, |C₂| = {}, and |C₃| = {}. How many ordered pairs (x, y) ∈ S × S belong to the equivalence relation R?",
                s1, s2, s3
            );
            let prompt_latex = format!(
                "|R| = |C_1 \\times C_1| + |C_2 \\times C_2| + |C_3 \\times C_3| = {}^2 + {}^2 + {}^2 = ?",
                s1, s2, s3
            );
            let prompt_speech = format!(
                "An equivalence relation partitions S into classes of sizes {}, {}, and {}. How many ordered pairs belong to R?",
                s1, s2, s3
            );
            let metadata = serde_json::json!({
                "kind": "equivalence_relation_pairs",
                "s1": s1,
                "s2": s2,
                "s3": s3,
                "unordered_no_refl_trap": unordered_no_refl_trap,
                "full_cartesian_trap": full_cartesian_trap,
                "target": total_pairs,
            });
            Ok((prompt_text, prompt_latex, prompt_speech, "integer".into(), None, Some(total_pairs.to_string()), metadata, "integer-exact-v1".into()))
        }
    }
}

/// Template: `proof.direct_and_contrapositive` (Course P02)
fn generate_proof_direct_and_contrapositive<R: DrawSource + ?Sized>(
    rng: &mut R,
    difficulty: u32,
) -> Result<(String, String, String, String, Option<Vec<String>>, Option<String>, Value, String), GenerationFailure> {
    match difficulty {
        1 => {
            let k = uniform_inclusive(rng, 2, 9) as i64;
            let n = 2 * k + 1;
            // n^2 - 1 = (2k + 1)^2 - 1 = 4k^2 + 4k = 4 * k(k + 1)
            let m_witness = k * (k + 1);
            let unfactored_trap = n * n - 1;
            let forgot_linear_k_trap = k * k;
            let prompt_text = format!(
                "In a direct proof that the square of any odd integer n = 2k + 1 satisfies 4 | (n² - 1), we expand (2k + 1)² - 1 = 4k² + 4k = 4M. For k = {} (so n = {}), find the exact integer witness M such that n² - 1 = 4M.",
                k, n
            );
            let prompt_latex = format!(
                "n = 2({}) + 1 = {} \\implies n^2 - 1 = 4M = 4({}^2 + {}) \\implies M = ?",
                k, n, k, k
            );
            let prompt_speech = format!(
                "For odd integer n equals {}, find the integer witness M such that n squared minus 1 equals 4 M.",
                n
            );
            let metadata = serde_json::json!({
                "kind": "direct_parity_witness",
                "k": k,
                "n": n,
                "unfactored_trap": unfactored_trap,
                "forgot_linear_k_trap": forgot_linear_k_trap,
                "target": m_witness,
            });
            Ok((prompt_text, prompt_latex, prompt_speech, "integer".into(), None, Some(m_witness.to_string()), metadata, "integer-exact-v1".into()))
        }
        2 => {
            let a = uniform_inclusive(rng, 3, 7) as i64;
            let b = uniform_inclusive(rng, 2, 11) as i64;
            let k = uniform_inclusive(rng, 4, 10) as i64;
            // Contrapositive of "if a*n + b > M then n >= k" for n in Z is "if n <= k - 1 then a*n + b <= M"
            let max_contra = a * (k - 1) + b;
            let boundary_k_trap = a * k + b;
            let sign_b_trap = a * (k - 1) - b;
            let prompt_text = format!(
                "To prove the implication \"For all n ∈ ℤ, if {}n + {} > M then n ≥ {}\" via its contrapositive \"If n < {} (so n ≤ {}), then {}n + {} ≤ M\", find the exact tightest integer bound M = {}({} - 1) + {}.",
                a, b, k, k, k - 1, a, b, a, k, b
            );
            let prompt_latex = format!(
                "n \\le {} - 1 \\implies {}n + {} \\le {}({} - 1) + {} = M = ?",
                k, a, b, a, k, b
            );
            let prompt_speech = format!(
                "Using the contrapositive for integers n less than {}, evaluate the maximum value M equals {} times {} minus 1 plus {}.",
                k, a, k, b
            );
            let metadata = serde_json::json!({
                "kind": "contrapositive_bound_proof",
                "a": a,
                "b": b,
                "k": k,
                "boundary_k_trap": boundary_k_trap,
                "sign_b_trap": sign_b_trap,
                "target": max_contra,
            });
            Ok((prompt_text, prompt_latex, prompt_speech, "integer".into(), None, Some(max_contra.to_string()), metadata, "integer-exact-v1".into()))
        }
        _ => {
            let m = uniform_inclusive(rng, 4, 10) as i64;
            let k = uniform_inclusive(rng, 3, 8) as i64;
            let min_items = m * (k - 1) + 1;
            let max_avoid_trap = m * (k - 1);
            let naive_product_trap = m * k;
            let prompt_text = format!(
                "By proof by contradiction (Generalized Pigeonhole Principle), if N items are placed into {} boxes and every box held at most {} - 1 = {} items, the total would be at most {} × {} = {}. What is the minimum number of items N that forces at least one box to contain at least {} items?",
                m, k, k - 1, m, k - 1, m * (k - 1), k
            );
            let prompt_latex = format!(
                "m = {}, \\; k = {} \\implies N_{{\\min}} = m(k - 1) + 1 = {}({} - 1) + 1 = ?",
                m, k, m, k
            );
            let prompt_speech = format!(
                "With {} boxes, what is the minimum number of items N needed to guarantee at least {} items in the same box?",
                m, k
            );
            let metadata = serde_json::json!({
                "kind": "pigeonhole_contradiction_threshold",
                "m": m,
                "k": k,
                "max_avoid_trap": max_avoid_trap,
                "naive_product_trap": naive_product_trap,
                "target": min_items,
            });
            Ok((prompt_text, prompt_latex, prompt_speech, "integer".into(), None, Some(min_items.to_string()), metadata, "integer-exact-v1".into()))
        }
    }
}

/// Template: `proof.mathematical_induction` (Course P02)
fn generate_proof_mathematical_induction<R: DrawSource + ?Sized>(
    rng: &mut R,
    difficulty: u32,
) -> Result<(String, String, String, String, Option<Vec<String>>, Option<String>, Value, String), GenerationFailure> {
    match difficulty {
        1 => {
            let a = uniform_inclusive(rng, 2, 6) as i64;
            let k = uniform_inclusive(rng, 4, 11) as i64;
            // S_k = a k^2, S_{k+1} = a (k+1)^2 => T_{k+1} = a(2k + 1)
            let term_k_plus_1 = a * (2 * k + 1);
            let kth_term_trap = a * (2 * k - 1);
            let full_sum_trap = a * (k + 1) * (k + 1);
            let prompt_text = format!(
                "In an induction proof that ∑_(i=1)^n {}(2i - 1) = {}n², the inductive step transitions from S_k = {}k² to S_(k+1) = {}(k + 1)² by adding the (k + 1)-th term T_(k+1) = S_(k+1) - S_k. For k = {}, compute the exact value of T_({}).",
                a, a, a, a, k, k + 1
            );
            let prompt_latex = format!(
                "S_k = {}k^2, \\; k = {} \\implies T_{{{}}} = S_{{{}}} - S_{{{}}} = {}(2\\cdot {} + 1) = ?",
                a, k, k + 1, k + 1, k, a, k
            );
            let prompt_speech = format!(
                "For S k equals {} k squared at k equals {}, find the k plus 1 term added in the inductive step.",
                a, k
            );
            let metadata = serde_json::json!({
                "kind": "induction_step_difference",
                "a": a,
                "k": k,
                "kth_term_trap": kth_term_trap,
                "full_sum_trap": full_sum_trap,
                "target": term_k_plus_1,
            });
            Ok((prompt_text, prompt_latex, prompt_speech, "integer".into(), None, Some(term_k_plus_1.to_string()), metadata, "integer-exact-v1".into()))
        }
        2 => {
            let n = uniform_inclusive(rng, 5, 12) as i64;
            let sum_sq = n * (n + 1) * (2 * n + 1) / 6;
            let linear_sum_trap = n * (n + 1) / 2;
            let wrong_factor_trap = n * (n + 1) * (n + 2) / 6;
            let prompt_text = format!(
                "By mathematical induction, the sum of the first n squares is ∑_(i=1)^n i² = n(n + 1)(2n + 1) / 6. Evaluate this exact sum for n = {}.",
                n
            );
            let prompt_latex = format!(
                "\\sum_{{i=1}}^{{{}}} i^2 = \\frac{{{}({} + 1)(2\\cdot {} + 1)}}{{6}} = ?",
                n, n, n, n
            );
            let prompt_speech = format!(
                "Evaluate the sum of i squared from i equals 1 to {}.",
                n
            );
            let metadata = serde_json::json!({
                "kind": "sum_of_squares_induction",
                "n": n,
                "linear_sum_trap": linear_sum_trap,
                "wrong_factor_trap": wrong_factor_trap,
                "target": sum_sq,
            });
            Ok((prompt_text, prompt_latex, prompt_speech, "integer".into(), None, Some(sum_sq.to_string()), metadata, "integer-exact-v1".into()))
        }
        _ => {
            let a = uniform_inclusive(rng, 1, 7) as i64;
            let n = uniform_inclusive(rng, 4, 15) as i64;
            let val = frac(a * n, n + 1);
            let target_str = to_canonical(&val);
            let forgot_n_num_trap = to_canonical(&frac(a, n + 1));
            let inverted_trap = to_canonical(&frac(a * (n + 1), n));
            let prompt_text = format!(
                "By mathematical induction (and telescoping partial fractions), ∑_(i=1)^n {} / [i(i + 1)] = ({}n) / (n + 1). Evaluate the exact rational sum for n = {}.",
                a, a, n
            );
            let prompt_latex = format!(
                "\\sum_{{i=1}}^{{{}}} \\frac{{{}}}{{i(i + 1)}} = \\frac{{{} \\cdot {}}}{{{} + 1}} = ?",
                n, a, a, n, n
            );
            let prompt_speech = format!(
                "Evaluate the sum from i equals 1 to {} of {} over i times i plus 1.",
                n, a
            );
            let metadata = serde_json::json!({
                "kind": "telescoping_fraction_induction",
                "a": a,
                "n": n,
                "forgot_n_num_trap": forgot_n_num_trap,
                "inverted_trap": inverted_trap,
                "target": target_str,
            });
            Ok((prompt_text, prompt_latex, prompt_speech, "rational".into(), None, Some(target_str), metadata, "rational-equality-v1".into()))
        }
    }
}

/// Template: `proof.counterexamples_and_extremal` (Course P02)
fn generate_proof_counterexamples_and_extremal<R: DrawSource + ?Sized>(
    rng: &mut R,
    difficulty: u32,
) -> Result<(String, String, String, String, Option<Vec<String>>, Option<String>, Value, String), GenerationFailure> {
    match difficulty {
        1 => {
            let primes = [3i64, 5, 11, 17, 41];
            let idx = uniform_inclusive(rng, 0, 4) as usize;
            let p = primes[idx];
            // For Euler's lucky primes p in {3, 5, 11, 17, 41}, P(n) = n^2 + n + p is prime for 1 <= n <= p - 2,
            // and at n = p - 1, P(p - 1) = (p - 1)^2 + (p - 1) + p = p^2 is composite!
            let min_n = p - 1;
            let obvious_p_trap = p;
            let composite_val_trap = p * p;
            let prompt_text = format!(
                "Consider the false universal claim: \"For every integer n ≥ 1, P(n) = n² + n + {} is prime.\" Notice that P(n) = n(n + 1) + {}, so at n = {} - 1, P(n) = ({} - 1){} + {} = {}². Find the smallest positive integer counterexample n ≥ 1 where P(n) is composite.",
                p, p, p, p, p, p, p
            );
            let prompt_latex = format!(
                "P(n) = n^2 + n + {} \\implies P({} - 1) = {}^2 \\implies n_{{\\min}} = ?",
                p, p, p
            );
            let prompt_speech = format!(
                "Find the smallest positive integer counterexample n where n squared plus n plus {} is composite.",
                p
            );
            let metadata = serde_json::json!({
                "kind": "minimal_counterexample_polynomial",
                "p": p,
                "obvious_p_trap": obvious_p_trap,
                "composite_val_trap": composite_val_trap,
                "target": min_n,
            });
            Ok((prompt_text, prompt_latex, prompt_speech, "integer".into(), None, Some(min_n.to_string()), metadata, "integer-exact-v1".into()))
        }
        2 => {
            let b = uniform_inclusive(rng, 5, 12) as i64;
            let k = uniform_inclusive(rng, 3, 9) as i64;
            let r = uniform_inclusive(rng, 1, (b - 1) as i128) as i64;
            let a = -(b * k) + r;
            let q = -k;
            let trunc_zero_trap = -(k - 1);
            let pos_q_trap = k;
            let prompt_text = format!(
                "By the Division Algorithm, for any integer a and divisor b > 0 there exist unique integers q and r such that a = bq + r with 0 ≤ r < b. For the negative dividend a = {} and divisor b = {}, find the exact Euclidean quotient q.",
                a, b
            );
            let prompt_latex = format!(
                "{} = {}q + r, \\quad 0 \\le r < {} \\implies q = ?",
                a, b, b
            );
            let prompt_speech = format!(
                "In the Euclidean division of {} by {} with non-negative remainder r less than {}, find the quotient q.",
                a, b, b
            );
            let metadata = serde_json::json!({
                "kind": "euclid_division_quotient",
                "a": a,
                "b": b,
                "q": q,
                "r": r,
                "trunc_zero_trap": trunc_zero_trap,
                "pos_q_trap": pos_q_trap,
                "target": q,
            });
            Ok((prompt_text, prompt_latex, prompt_speech, "integer".into(), None, Some(q.to_string()), metadata, "integer-exact-v1".into()))
        }
        _ => {
            let d = uniform_inclusive(rng, 2, 7) as i64;
            let a = 3 * d;
            let b = 5 * d;
            // gcd(3d, 5d) = d. Since 2*(3d) + (-1)*(5d) = d, or (-3)*(3d) + 2*(5d) = d:
            let y0 = uniform_inclusive(rng, 1, 4) as i64;
            // We want a * x0 + b * y0 = c_rhs where c_rhs is a known multiple of d:
            let x0 = uniform_inclusive(rng, -6, -2) as i64;
            let rhs = a * x0 + b * y0;
            let sign_x_trap = -x0;
            let unscaled_trap = a * x0;
            let prompt_text = format!(
                "By the Well-Ordering Principle and Bézout's Identity, gcd({}, {}) = {} is the smallest positive linear combination of {} and {}. Given the linear combination {}x + {}({}) = {}, find the exact integer coefficient x.",
                a, b, d, a, b, a, b, y0, rhs
            );
            let prompt_latex = format!(
                "{}x + {}({}) = {} \\implies x = \\frac{{{} - {}}}{{{}}} = ?",
                a, b, y0, rhs, rhs, b * y0, a
            );
            let prompt_speech = format!(
                "Solve {} x plus {} times {} equals {} for the integer coefficient x.",
                a, b, y0, rhs
            );
            let metadata = serde_json::json!({
                "kind": "bezout_linear_combination",
                "a": a,
                "b": b,
                "d": d,
                "y0": y0,
                "rhs": rhs,
                "sign_x_trap": sign_x_trap,
                "unscaled_trap": unscaled_trap,
                "target": x0,
            });
            Ok((prompt_text, prompt_latex, prompt_speech, "integer".into(), None, Some(x0.to_string()), metadata, "integer-exact-v1".into()))
        }
    }
}

/// Template: `proof.definitions_and_biconditionals` (Course P03)
fn generate_proof_definitions_and_biconditionals<R: DrawSource + ?Sized>(
    rng: &mut R,
    difficulty: u32,
) -> Result<(String, String, String, String, Option<Vec<String>>, Option<String>, Value, String), GenerationFailure> {
    match difficulty {
        1 => {
            let m = uniform_inclusive(rng, 1, 3) as u32;
            let gap = uniform_inclusive(rng, 1, 3) as u32;
            let n = m + gap;
            let p = (uniform_inclusive(rng, 1, 4) as i64) * 2 + 1;
            let q = (uniform_inclusive(rng, 1, 4) as i64) * 2 + 1;
            let scale = 2i64.pow(gap);
            let num_a = p * scale + q;
            let unscaled_sum_trap = p + q;
            let full_pow_trap = p * 2i64.pow(n) + q;
            let prompt_text = format!(
                "Definition: A rational number r is dyadic if r = A / 2^n for some integers A ∈ ℤ and n ≥ 0. To prove directly from the definition that {}/2^{} + {}/2^{} is dyadic with denominator 2^{}, find the exact integer witness A such that {}/2^{} + {}/2^{} = A / 2^{}.",
                p, m, q, n, n, p, m, q, n, n
            );
            let prompt_latex = format!(
                "\\frac{{{}}}{{2^{{{}}}}} + \\frac{{{}}}{{2^{{{}}}}} = \\frac{{A}}{{2^{{{}}}}} \\implies A = {} \\cdot 2^{{{} - {}}} + {} = ?",
                p, m, q, n, n, p, n, m, q
            );
            let prompt_speech = format!(
                "Find the integer numerator A such that {} over 2 to the {} plus {} over 2 to the {} equals A over 2 to the {}.",
                p, m, q, n, n
            );
            let metadata = serde_json::json!({
                "kind": "definition_witness_dyadic",
                "m": m as i64,
                "n": n as i64,
                "p": p,
                "q": q,
                "scale": scale,
                "unscaled_sum_trap": unscaled_sum_trap,
                "full_pow_trap": full_pow_trap,
                "target": num_a,
            });
            Ok((prompt_text, prompt_latex, prompt_speech, "integer".into(), None, Some(num_a.to_string()), metadata, "integer-exact-v1".into()))
        }
        2 => {
            let a = uniform_inclusive(rng, 2, 5) as i64;
            let b = uniform_inclusive(rng, 2, 4) as i64;
            let ab = a * b;
            let k = uniform_inclusive(rng, 3, 6) as i64;
            let n_max = ab * k;
            let nec_count = n_max / a;
            let suff_count = n_max / ab;
            let gap_count = nec_count - suff_count;
            let suff_only_trap = suff_count;
            let nec_only_trap = nec_count;
            let prompt_text = format!(
                "On D = {{1, 2, ..., {}}}, condition S(x): \"{} | x\" is sufficient for N(x): \"{} | x\", while N(x) is necessary for S(x). How many elements x ∈ D satisfy the necessary condition N(x) WITHOUT satisfying the sufficient condition S(x) (proving N(x) ⇏ S(x))?",
                n_max, ab, a
            );
            let prompt_latex = format!(
                "|\\{{x \\in \\{{1,\\dots,{}\\}} : {} \\mid x \\text{{ and }} {} \\nmid x\\}}| = \\frac{{{}}}{{{}}} - \\frac{{{}}}{{{}}} = ?",
                n_max, a, ab, n_max, a, n_max, ab
            );
            let prompt_speech = format!(
                "Between 1 and {}, how many integers are divisible by {} but not divisible by {}?",
                n_max, a, ab
            );
            let metadata = serde_json::json!({
                "kind": "necessary_vs_sufficient_gap",
                "a": a,
                "ab": ab,
                "n_max": n_max,
                "nec_count": nec_count,
                "suff_count": suff_count,
                "suff_only_trap": suff_only_trap,
                "nec_only_trap": nec_only_trap,
                "target": gap_count,
            });
            Ok((prompt_text, prompt_latex, prompt_speech, "integer".into(), None, Some(gap_count.to_string()), metadata, "integer-exact-v1".into()))
        }
        _ => {
            let a = uniform_inclusive(rng, 2, 6) as i64;
            let b = uniform_inclusive(rng, 3, 11) as i64;
            let c = uniform_inclusive(rng, 2, 7) as i64;
            // ax = b <=> cx = d requires d = b*c / a
            let d_val = frac(b * c, a);
            let target_str = to_canonical(&d_val);
            let additive_shift_trap = to_canonical(&int(b + c - a));
            let inverted_ratio_trap = to_canonical(&frac(a * c, b));
            let prompt_text = format!(
                "For the biconditional statement \"{}x = {} ⇔ {}x = d\" to hold for all real and rational x, both equations must have the exact same solution set. Find the exact rational parameter d.",
                a, b, c
            );
            let prompt_latex = format!(
                "({}x = {} \\iff {}x = d) \\implies d = \\frac{{{} \\cdot {}}}{{{}}} = ?",
                a, b, c, b, c, a
            );
            let prompt_speech = format!(
                "Find the exact rational value of d so that {} x equals {} if and only if {} x equals d.",
                a, b, c
            );
            let metadata = serde_json::json!({
                "kind": "biconditional_parameter_match",
                "a": a,
                "b": b,
                "c": c,
                "additive_shift_trap": additive_shift_trap,
                "inverted_ratio_trap": inverted_ratio_trap,
                "target": target_str,
            });
            Ok((prompt_text, prompt_latex, prompt_speech, "rational".into(), None, Some(target_str), metadata, "rational-equality-v1".into()))
        }
    }
}

/// Template: `proof.hypotheses_and_lemmas` (Course P03)
fn generate_proof_hypotheses_and_lemmas<R: DrawSource + ?Sized>(
    rng: &mut R,
    difficulty: u32,
) -> Result<(String, String, String, String, Option<Vec<String>>, Option<String>, Value, String), GenerationFailure> {
    match difficulty {
        1 => {
            let r = uniform_inclusive(rng, 2, 7) as i64;
            let c = uniform_inclusive(rng, 2, 5) as i64;
            let a = c + 1; // so a - c = 1
            let b = uniform_inclusive(rng, 3, 9) as i64;
            let d = uniform_inclusive(rng, 2, 8) as i64;
            // (x - r)(a x - b) = (x - r)(c x + d) => x = r or (a - c)x = b + d => x = b + d
            let root2 = b + d;
            let sum_roots = r + root2;
            let divided_out_root_trap = root2;
            let only_cancelled_root_trap = r;
            let prompt_text = format!(
                "In solving (x - {})({}x - {}) = (x - {})({}x + {}), dividing by (x - {}) without checking the domain hypothesis x ≠ {} loses the root x = {}. Find the exact sum of BOTH roots of the equation.",
                r, a, b, r, c, d, r, r, r
            );
            let prompt_latex = format!(
                "(x - {})({}x - {}) = (x - {})({}x + {}) \\implies x \\in \\{{{},\\; {} + {}\\}} \\implies \\sum x = ?",
                r, a, b, r, c, d, r, b, d
            );
            let prompt_speech = format!(
                "Find the sum of both roots of quantity x minus {} times {} x minus {} equals quantity x minus {} times {} x plus {}.",
                r, a, b, r, c, d
            );
            let metadata = serde_json::json!({
                "kind": "missing_hypothesis_roots",
                "r": r,
                "a": a,
                "b": b,
                "c": c,
                "d": d,
                "root2": root2,
                "divided_out_root_trap": divided_out_root_trap,
                "only_cancelled_root_trap": only_cancelled_root_trap,
                "target": sum_roots,
            });
            Ok((prompt_text, prompt_latex, prompt_speech, "integer".into(), None, Some(sum_roots.to_string()), metadata, "integer-exact-v1".into()))
        }
        2 => {
            // Triples (b, m, k) with b^k == 1 (mod m)
            let triples = [(2i64, 7i64, 3i64), (3, 8, 2), (2, 15, 4), (3, 10, 4)];
            let idx = uniform_inclusive(rng, 0, 3) as usize;
            let (b, m, k) = triples[idx];
            let q = uniform_inclusive(rng, 5, 12) as i64;
            let r = uniform_inclusive(rng, 1, (k - 1) as i128) as i64;
            let exp_e = k * q + r;
            let rem = b.pow(r as u32) % m;
            let exp_rem_trap = r;
            let mult_trap = (b * r) % m;
            let prompt_text = format!(
                "Lemma 1 states that {}^{} ≡ 1 (mod {}). Using Lemma 1 to reduce the exponent {} = {}×{} + {}, find the exact remainder in {{0, ..., {}}} when {}^{} is divided by {}.",
                b, k, m, exp_e, k, q, r, m - 1, b, exp_e, m
            );
            let prompt_latex = format!(
                "{}^{{{}}} \\equiv 1 \\pmod{{{}}} \\implies {}^{{{}}} = ({}^{{{}}})^{{{}}} \\cdot {}^{{{}}} \\equiv ? \\pmod{{{}}}",
                b, k, m, b, exp_e, b, k, q, b, r, m
            );
            let prompt_speech = format!(
                "Using the lemma {} to the {} is congruent to 1 modulo {}, find the remainder of {} to the {} modulo {}.",
                b, k, m, b, exp_e, m
            );
            let metadata = serde_json::json!({
                "kind": "lemma_chain_modular_power",
                "b": b,
                "m": m,
                "k": k,
                "q": q,
                "r": r,
                "exp_e": exp_e,
                "exp_rem_trap": exp_rem_trap,
                "mult_trap": mult_trap,
                "target": rem,
            });
            Ok((prompt_text, prompt_latex, prompt_speech, "integer".into(), None, Some(rem.to_string()), metadata, "integer-exact-v1".into()))
        }
        _ => {
            let a = uniform_inclusive(rng, 2, 6) as i64;
            let b = uniform_inclusive(rng, 2, 6) as i64;
            let c = uniform_inclusive(rng, 3, 7) as i64;
            // |a(x - x0) + b(y - y0)| <= a * (1 / (2a)) + b * (1 / (c*b)) = 1/2 + 1/c = (c + 2)/(2c)
            let bound_val = frac(c + 2, 2 * c);
            let target_str = to_canonical(&bound_val);
            let unscaled_lemma_trap = to_canonical(&(frac(1, 2 * a) + frac(1, c * b)));
            let add_denom_trap = to_canonical(&frac(2, 2 + c));
            let prompt_text = format!(
                "By the Triangle Inequality Lemma, |{}(x - x₀) + {}(y - y₀)| ≤ {}|x - x₀| + {}|y - y₀|. If Lemma A gives |x - x₀| ≤ 1/{} and Lemma B gives |y - y₀| ≤ 1/{}, find the exact combined rational upper bound.",
                a, b, a, b, 2 * a, c * b
            );
            let prompt_latex = format!(
                "{}|x - x_0| + {}|y - y_0| \\le {} \\cdot \\frac{{1}}{{{}}} + {} \\cdot \\frac{{1}}{{{}}} = \\frac{{1}}{{2}} + \\frac{{1}}{{{}}} = ?",
                a, b, a, 2 * a, b, c * b, c
            );
            let prompt_speech = format!(
                "Using the triangle inequality with {} times 1 over {} plus {} times 1 over {}, find the exact rational upper bound.",
                a, 2 * a, b, c * b
            );
            let metadata = serde_json::json!({
                "kind": "triangle_inequality_lemma_bound",
                "a": a,
                "b": b,
                "c": c,
                "unscaled_lemma_trap": unscaled_lemma_trap,
                "add_denom_trap": add_denom_trap,
                "target": target_str,
            });
            Ok((prompt_text, prompt_latex, prompt_speech, "rational".into(), None, Some(target_str), metadata, "rational-equality-v1".into()))
        }
    }
}

/// Template: `proof.proof_audit_and_revision` (Course P03)
fn generate_proof_proof_audit_and_revision<R: DrawSource + ?Sized>(
    rng: &mut R,
    difficulty: u32,
) -> Result<(String, String, String, String, Option<Vec<String>>, Option<String>, Value, String), GenerationFailure> {
    match difficulty {
        1 => {
            let r = uniform_inclusive(rng, 2, 6) as i64;
            let s = r + (uniform_inclusive(rng, 2, 5) as i64);
            let sum_rs = r + s;
            let prod_rs = r * s;
            let hypothesis_r_trap = r;
            let sum_roots_trap = sum_rs;
            let prompt_text = format!(
                "Audit this converse fallacy: \"Theorem: If x = {}, then x² - {}x + {} = 0. Proof that x = {}: We are given x² - {}x + {} = 0; therefore by the Theorem, x = {}.\" What is the exact other root x ≠ {} of x² - {}x + {} = 0 that refutes this invalid converse step?",
                r, sum_rs, prod_rs, r, sum_rs, prod_rs, r, r, sum_rs, prod_rs
            );
            let prompt_latex = format!(
                "x^2 - {}x + {} = (x - {})(x - {}) = 0, \\; x \\neq {} \\implies x = ?",
                sum_rs, prod_rs, r, s, r
            );
            let prompt_speech = format!(
                "Find the other root x not equal to {} of x squared minus {} x plus {} equals 0.",
                r, sum_rs, prod_rs
            );
            let metadata = serde_json::json!({
                "kind": "converse_fallacy_audit",
                "r": r,
                "s": s,
                "sum_rs": sum_rs,
                "prod_rs": prod_rs,
                "hypothesis_r_trap": hypothesis_r_trap,
                "sum_roots_trap": sum_roots_trap,
                "target": s,
            });
            Ok((prompt_text, prompt_latex, prompt_speech, "integer".into(), None, Some(s.to_string()), metadata, "integer-exact-v1".into()))
        }
        2 => {
            let a = uniform_inclusive(rng, 2, 6) as i64;
            let b = uniform_inclusive(rng, 3, 12) as i64;
            let m = uniform_inclusive(rng, 5, 11) as i64;
            let min_uniform_y = a * m + b;
            let min_x1_trap = a + b;
            let forgot_b_trap = a * m;
            let prompt_text = format!(
                "Compare quantifier orders on X = {{1, 2, ..., {}}}: whereas \"∀x ∈ X, ∃y ∈ ℤ, y ≥ {}x + {}\" lets y depend on x, the uniform statement \"∃Y ∈ ℤ, ∀x ∈ X, Y ≥ {}x + {}\" requires a single integer Y that bounds {}x + {} for every x ∈ X simultaneously. Find the minimum such integer Y.",
                m, a, b, a, b, a, b
            );
            let prompt_latex = format!(
                "\\exists Y \\in \\mathbb{{Z}}, \\; \\forall x \\in \\{{1,\\dots,{}\\}}, \\; Y \\ge {}x + {} \\implies Y_{{\\min}} = {}({}) + {} = ?",
                m, a, b, a, m, b
            );
            let prompt_speech = format!(
                "Find the minimum integer Y such that for all x in 1 through {}, Y is at least {} x plus {}.",
                m, a, b
            );
            let metadata = serde_json::json!({
                "kind": "quantifier_order_uniform_bound",
                "a": a,
                "b": b,
                "m": m,
                "min_x1_trap": min_x1_trap,
                "forgot_b_trap": forgot_b_trap,
                "target": min_uniform_y,
            });
            Ok((prompt_text, prompt_latex, prompt_speech, "integer".into(), None, Some(min_uniform_y.to_string()), metadata, "integer-exact-v1".into()))
        }
        _ => {
            let k = uniform_inclusive(rng, 3, 8) as i64;
            let offset_c = uniform_inclusive(rng, 4, 15) as i64;
            let true_sum = k * (k + 1);
            let claimed_val = true_sum + offset_c;
            let true_sum_trap = true_sum;
            let claimed_val_trap = claimed_val;
            let prompt_text = format!(
                "A flawed induction proof claims ∑_(i=1)^n 2i = n(n + 1) + C and verifies the inductive step (adding 2(k + 1) yields (k + 1)(k + 2) + C), but skips the base case n = 1. At n = {}, the student's formula gives {} whereas the true sum is {}({} + 1) = {}. Find the exact constant error C.",
                k, claimed_val, k, k, true_sum
            );
            let prompt_latex = format!(
                "k(k + 1) + C = {} \\text{{ at }} k = {} \\implies C = {} - {} = ?",
                claimed_val, k, claimed_val, true_sum
            );
            let prompt_speech = format!(
                "At k equals {}, the flawed induction formula gives {} while the true sum is {}. Find the constant error C.",
                k, claimed_val, true_sum
            );
            let metadata = serde_json::json!({
                "kind": "induction_base_case_audit",
                "k": k,
                "true_sum": true_sum,
                "claimed_val": claimed_val,
                "true_sum_trap": true_sum_trap,
                "claimed_val_trap": claimed_val_trap,
                "target": offset_c,
            });
            Ok((prompt_text, prompt_latex, prompt_speech, "integer".into(), None, Some(offset_c.to_string()), metadata, "integer-exact-v1".into()))
        }
    }
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

    #[test]
    fn multiplication_and_division_generation() {
        let pm = generate("arithmetic.whole.multiplication", "1.1.0", TEST_SEED, 1, MANIFEST_HASH).unwrap();
        assert_eq!(pm.answer_kind, "integer");
        assert!(pm.target_value.is_some());
        assert!(pm.metadata.get("factor1").is_some());

        let pd = generate("arithmetic.whole.division", "1.1.0", TEST_SEED, 1, MANIFEST_HASH).unwrap();
        assert_eq!(pd.answer_kind, "integer");
        assert!(pd.target_value.is_some());
        assert!(pd.metadata.get("quotient").is_some());
    }

    #[test]
    fn integers_and_divisibility_generation() {
        let p_order = generate("integers.order", "1.1.0", TEST_SEED, 1, MANIFEST_HASH).unwrap();
        assert_eq!(p_order.answer_kind, "enum");
        assert!(p_order.target_value.is_some());
        assert!(p_order.metadata.get("a").is_some());
        assert_eq!(p_order.replay_identity.checker_version, "integer-order-v2");

        let p_factors = generate("divisibility.factors.find", "1.1.0", TEST_SEED, 1, MANIFEST_HASH).unwrap();
        assert_eq!(p_factors.answer_kind, "integer");
        assert!(p_factors.target_value.is_some());
        assert!(p_factors.metadata.get("greatest_proper_divisor").is_some());

        let p_gcd = generate("divisibility.gcd_lcm", "1.1.0", TEST_SEED, 1, MANIFEST_HASH).unwrap();
        assert_eq!(p_gcd.answer_kind, "integer");
        assert!(p_gcd.target_value.is_some());
        assert_eq!(p_gcd.metadata.get("kind").and_then(|v| v.as_str()), Some("gcd"));

        let p_lcm = generate("divisibility.gcd_lcm", "1.1.0", TEST_SEED, 3, MANIFEST_HASH).unwrap();
        assert_eq!(p_lcm.answer_kind, "integer");
        assert!(p_lcm.target_value.is_some());
        assert_eq!(p_lcm.metadata.get("kind").and_then(|v| v.as_str()), Some("lcm"));
    }
    #[test]
    fn learning_to_learn_generation() {
        let p_reading = generate("learning.problem_reading", "1.1.0", TEST_SEED, 1, MANIFEST_HASH).unwrap();
        assert_eq!(p_reading.answer_kind, "integer");
        assert!(p_reading.target_value.is_some());
        assert_eq!(p_reading.replay_identity.checker_version, "integer-exact-v2");

        let p_balance = generate("learning.equality_balance", "1.1.0", TEST_SEED, 1, MANIFEST_HASH).unwrap();
        assert_eq!(p_balance.answer_kind, "integer");
        assert!(p_balance.target_value.is_some());
        assert_eq!(p_balance.replay_identity.checker_version, "integer-exact-v2");

        let p_bounds = generate("learning.checking_bounds", "1.1.0", TEST_SEED, 1, MANIFEST_HASH).unwrap();
        assert_eq!(p_bounds.answer_kind, "integer");
        assert!(p_bounds.target_value.is_some());
        assert_eq!(p_bounds.replay_identity.checker_version, "integer-exact-v2");
    }

    #[test]
    fn f05_decimals_percentages_ratios_generation() {
        let p_dec = generate("decimals.fraction_convert", "1.1.0", TEST_SEED, 1, MANIFEST_HASH).unwrap();
        assert_eq!(p_dec.answer_kind, "rational");
        assert!(p_dec.target_value.is_some());
        assert_eq!(p_dec.replay_identity.checker_version, "rational-equality-v2");

        let p_pct = generate("percentages.benchmark_calc", "1.1.0", TEST_SEED, 1, MANIFEST_HASH).unwrap();
        assert_eq!(p_pct.answer_kind, "integer");
        assert!(p_pct.target_value.is_some());
        assert_eq!(p_pct.replay_identity.checker_version, "integer-exact-v2");

        let p_ratio = generate("ratios.scaling_solve", "1.1.0", TEST_SEED, 1, MANIFEST_HASH).unwrap();
        assert_eq!(p_ratio.answer_kind, "integer");
        assert!(p_ratio.target_value.is_some());
        assert_eq!(p_ratio.replay_identity.checker_version, "integer-exact-v2");
    }

    #[test]
    fn f06_powers_and_roots_generation() {
        let p_pow1 = generate("powers.integer_eval", "1.1.0", TEST_SEED, 1, MANIFEST_HASH).unwrap();
        assert_eq!(p_pow1.answer_kind, "integer");
        assert!(p_pow1.target_value.is_some());
        assert_eq!(p_pow1.replay_identity.checker_version, "integer-exact-v2");

        let p_pow2 = generate("powers.integer_eval", "1.1.0", TEST_SEED, 2, MANIFEST_HASH).unwrap();
        assert!(p_pow2.target_value.is_some());

        let p_sci = generate("powers.scientific_notation", "1.1.0", TEST_SEED, 1, MANIFEST_HASH).unwrap();
        assert_eq!(p_sci.answer_kind, "integer");
        assert!(p_sci.target_value.is_some());
        assert_eq!(p_sci.replay_identity.checker_version, "integer-exact-v2");

        let p_root = generate("roots.radical_eval", "1.1.0", TEST_SEED, 1, MANIFEST_HASH).unwrap();
        assert_eq!(p_root.answer_kind, "integer");
        assert!(p_root.target_value.is_some());
        assert_eq!(p_root.replay_identity.checker_version, "integer-exact-v2");

        let p_root2 = generate("roots.radical_eval", "1.1.0", TEST_SEED, 2, MANIFEST_HASH).unwrap();
        assert_eq!(p_root2.answer_kind, "rational");
        assert!(p_root2.target_value.is_some());
        assert_eq!(p_root2.replay_identity.checker_version, "rational-equality-v2");
    }

    #[test]
    fn f07_geometry_generation() {
        let p_geom1 = generate("geometry.perimeter_and_area", "1.1.0", TEST_SEED, 1, MANIFEST_HASH).unwrap();
        assert_eq!(p_geom1.answer_kind, "integer");
        assert!(p_geom1.target_value.is_some());
        assert_eq!(p_geom1.replay_identity.checker_version, "integer-exact-v2");

        let p_geom2 = generate("geometry.perimeter_and_area", "1.1.0", TEST_SEED, 2, MANIFEST_HASH).unwrap();
        assert_eq!(p_geom2.answer_kind, "integer");
        assert!(p_geom2.target_value.is_some());

        let p_ang1 = generate("geometry.angles_triangles", "1.1.0", TEST_SEED, 1, MANIFEST_HASH).unwrap();
        assert_eq!(p_ang1.answer_kind, "integer");
        assert!(p_ang1.target_value.is_some());
        assert_eq!(p_ang1.replay_identity.checker_version, "integer-exact-v2");

        let p_coord1 = generate("geometry.coordinate_distance", "1.1.0", TEST_SEED, 1, MANIFEST_HASH).unwrap();
        assert_eq!(p_coord1.answer_kind, "integer");
        assert!(p_coord1.target_value.is_some());
        assert_eq!(p_coord1.replay_identity.checker_version, "integer-exact-v2");

        let p_coord2 = generate("geometry.coordinate_distance", "1.1.0", TEST_SEED, 2, MANIFEST_HASH).unwrap();
        assert_eq!(p_coord2.answer_kind, "integer");
        assert!(p_coord2.target_value.is_some());
    }

    #[test]
    fn f08_data_counting_chance_generation() {
        for band in 1..=3 {
            let p_data = generate("data.mean_median_calc", "1.1.0", TEST_SEED, band, MANIFEST_HASH).unwrap();
            assert_eq!(p_data.answer_kind, "integer");
            assert!(p_data.target_value.is_some());
            assert_eq!(p_data.replay_identity.checker_version, "integer-exact-v2");

            let p_count = generate("counting.permutations_combinations", "1.1.0", TEST_SEED, band, MANIFEST_HASH).unwrap();
            assert_eq!(p_count.answer_kind, "integer");
            assert!(p_count.target_value.is_some());
            assert_eq!(p_count.replay_identity.checker_version, "integer-exact-v2");

            let p_prob = generate("probability.simple_exact", "1.1.0", TEST_SEED, band, MANIFEST_HASH).unwrap();
            assert_eq!(p_prob.answer_kind, "rational");
            assert!(p_prob.target_value.is_some());
            assert_eq!(p_prob.replay_identity.checker_version, "rational-equality-v2");
        }
    }

    #[test]
    fn s01_algebra_generation() {
        for band in 1..=3 {
            let p_expr = generate("algebra.expression_eval", "1.1.0", TEST_SEED, band, MANIFEST_HASH).unwrap();
            assert_eq!(p_expr.answer_kind, "integer");
            assert!(p_expr.target_value.is_some());
            assert_eq!(p_expr.replay_identity.checker_version, "integer-exact-v2");

            let p_eq = generate("algebra.linear_equation_solve", "1.1.0", TEST_SEED, band, MANIFEST_HASH).unwrap();
            assert!(p_eq.target_value.is_some());

            let p_sys = generate("algebra.linear_system_solve", "1.1.0", TEST_SEED, band, MANIFEST_HASH).unwrap();
            assert_eq!(p_sys.answer_kind, "integer");
            assert!(p_sys.target_value.is_some());
            assert_eq!(p_sys.replay_identity.checker_version, "integer-exact-v2");
        }
    }

    #[test]
    fn s02_functions_generation() {
        for band in 1..=3 {
            let p_eval = generate("functions.evaluate_and_range", "1.1.0", TEST_SEED, band, MANIFEST_HASH).unwrap();
            assert_eq!(p_eval.answer_kind, "integer");
            assert!(p_eval.target_value.is_some());

            let p_comp = generate("functions.composition_inverse", "1.1.0", TEST_SEED, band, MANIFEST_HASH).unwrap();
            assert!(p_comp.target_value.is_some());

            let p_trans = generate("functions.transformations_vertex", "1.1.0", TEST_SEED, band, MANIFEST_HASH).unwrap();
            assert_eq!(p_trans.answer_kind, "integer");
            assert!(p_trans.target_value.is_some());
        }
    }

    #[test]
    fn s03_algebra2_generation() {
        for band in 1..=3 {
            let p_poly = generate("algebra2.polynomial_factor_roots", "1.1.0", TEST_SEED, band, MANIFEST_HASH).unwrap();
            assert!(p_poly.target_value.is_some());

            let p_disc = generate("algebra2.quadratic_discriminant", "1.1.0", TEST_SEED, band, MANIFEST_HASH).unwrap();
            assert!(p_disc.target_value.is_some());

            let p_log = generate("algebra2.exp_log_eval", "1.1.0", TEST_SEED, band, MANIFEST_HASH).unwrap();
            assert!(p_log.target_value.is_some());
        }
    }

    #[test]
    fn s04_geometry_generation() {
        for band in 1..=3 {
            let p_lines = generate("sec_geom.slope_and_lines", "1.1.0", TEST_SEED, band, MANIFEST_HASH).unwrap();
            assert!(p_lines.target_value.is_some());

            let p_circ = generate("sec_geom.circle_equations_theorems", "1.1.0", TEST_SEED, band, MANIFEST_HASH).unwrap();
            assert!(p_circ.target_value.is_some());

            let p_vec = generate("sec_geom.planar_vectors_dot", "1.1.0", TEST_SEED, band, MANIFEST_HASH).unwrap();
            assert!(p_vec.target_value.is_some());
        }
    }

    #[test]
    fn s05_trig_generation() {
        for band in 1..=3 {
            let p_rad = generate("trig.radians_unit_circle", "1.1.0", TEST_SEED, band, MANIFEST_HASH).unwrap();
            assert!(p_rad.target_value.is_some());

            let p_id = generate("trig.identities_double_angle", "1.1.0", TEST_SEED, band, MANIFEST_HASH).unwrap();
            assert!(p_id.target_value.is_some());

            let p_law = generate("trig.law_of_sines_cosines", "1.1.0", TEST_SEED, band, MANIFEST_HASH).unwrap();
            assert!(p_law.target_value.is_some());
        }
    }

    #[test]
    fn s06_sequences_generation() {
        for band in 1..=3 {
            let p_arith = generate("sequences.arithmetic_progression", "1.1.0", TEST_SEED, band, MANIFEST_HASH).unwrap();
            assert!(p_arith.target_value.is_some());

            let p_geom = generate("sequences.geometric_and_infinite", "1.1.0", TEST_SEED, band, MANIFEST_HASH).unwrap();
            assert!(p_geom.target_value.is_some());

            let p_rec = generate("sequences.recursion_binomial", "1.1.0", TEST_SEED, band, MANIFEST_HASH).unwrap();
            assert!(p_rec.target_value.is_some());
        }
    }

    #[test]
    fn s07_calculus_generation() {
        for band in 1..=3 {
            let p_lim = generate("calculus.limits_rational", "1.1.0", TEST_SEED, band, MANIFEST_HASH).unwrap();
            assert!(p_lim.target_value.is_some());

            let p_deriv = generate("calculus.derivatives_tangent", "1.1.0", TEST_SEED, band, MANIFEST_HASH).unwrap();
            assert!(p_deriv.target_value.is_some());

            let p_int = generate("calculus.definite_integral_ftc", "1.1.0", TEST_SEED, band, MANIFEST_HASH).unwrap();
            assert!(p_int.target_value.is_some());
        }
    }

    #[test]
    fn p01_p02_p03_proof_bridge_generation() {
        for band in 1..=3 {
            for tmpl in [
                "proof.sets_and_cardinality",
                "proof.logic_truth_quantifiers",
                "proof.relations_and_bijections",
                "proof.direct_and_contrapositive",
                "proof.mathematical_induction",
                "proof.counterexamples_and_extremal",
                "proof.definitions_and_biconditionals",
                "proof.hypotheses_and_lemmas",
                "proof.proof_audit_and_revision",
            ] {
                let p = generate(tmpl, "1.1.0", TEST_SEED, band, MANIFEST_HASH).unwrap();
                assert!(p.target_value.is_some());
            }
        }
    }
}
