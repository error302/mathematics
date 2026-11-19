use axiom_math::{check, generate, MathBudget};
use axiom_math::checker::Disposition;

const SEED: &str = "0000000000000000000000000000000000000000000000000000000000000000";
const HASH: &str = "0000000000000000000000000000000000000000000000000000000000000000";

#[test]
fn missing_rational_target_cannot_accept_zero() {
    let mut problem = generate("fractions.unit.identify", "1.0.0", SEED, 1, HASH).unwrap();
    problem.target_value = None;
    assert_eq!(check(&problem, "0", &MathBudget::default()).disposition, Disposition::Unsupported);
}

#[test]
fn abacus_invalid_lower_count_cannot_masquerade_as_digit() {
    let mut problem = generate("abacus.target.setting", "1.0.0", SEED, 1, HASH).unwrap();
    problem.target_value = Some("5".into());
    let invalid = r#"{"schema_version":1,"rod_count":1,"rightmost_exponent":0,"sign":1,"rods":[{"upper":false,"lower":5}]}"#;
    assert_eq!(check(&problem, invalid, &MathBudget::default()).disposition, Disposition::Malformed);
}

#[test]
fn malformed_integer_suffix_is_not_accepted() {
    let problem = generate("arithmetic.whole.addition", "1.0.0", SEED, 1, HASH).unwrap();
    let answer = format!("{}junk", problem.target_value.as_ref().unwrap());
    assert_eq!(check(&problem, &answer, &MathBudget::default()).disposition, Disposition::Malformed);
}

#[test]
fn equivalent_unit_fraction_values_are_accepted() {
    let problem = generate("fractions.unit.identify", "1.0.0", SEED, 1, HASH).unwrap();
    let answer = format!("({}) * 2 / 2", problem.target_value.as_ref().unwrap());
    assert_eq!(check(&problem, &answer, &MathBudget::default()).disposition, Disposition::Correct);
}

#[test]
fn huge_even_exponent_of_minus_one_preserves_parity() {
    let budget = MathBudget::default();
    let ast = axiom_math::parse_scalar("(-1)^4294967296", &budget).unwrap();
    let result = axiom_math::evaluate(&ast, &budget).unwrap();
    assert_eq!(axiom_math::to_canonical(&result), "1");
}

#[test]
fn enormous_exponent_is_budgeted_without_panicking() {
    let budget = MathBudget::default();
    let raw = format!("2^{}", "9".repeat(100));
    let ast = axiom_math::parse_scalar(&raw, &budget).unwrap();
    let result = std::panic::catch_unwind(|| axiom_math::evaluate(&ast, &budget));
    assert!(result.is_ok(), "Untrusted scalar input must not panic.");
    assert!(result.unwrap().unwrap_err().budget);
}

#[test]
fn mathematically_valid_fractional_power_is_unsupported_not_wrong() {
    let mut problem = generate("arithmetic.whole.addition", "1.1.0", SEED, 1, HASH).unwrap();
    problem.target_value = Some("2".into());
    assert_eq!(check(&problem, "4^(1/2)", &MathBudget::default()).disposition, Disposition::Unsupported);
}
