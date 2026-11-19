use axiom_math::{check, generate, MathBudget};
fn main() {
    let mut fixtures = Vec::new();
    for id in [
        "puzzles.arithmetic.grid",
        "reasoning.analysis",
        "reasoning.logic",
        "reasoning.comprehension",
        "reasoning.pattern",
        "visual.rectangle",
        "visual.linear",
        "visual.quadratic",
    ] {
        for band in 1..=3 {
            let p = generate(id, "1.1.0", &"12".repeat(32), band, "challenge-fixture").unwrap();
            let outcome = check(&p, p.target_value.as_ref().unwrap(), &MathBudget::default());
            fixtures.push(serde_json::json!({"problem":p,"outcome":outcome}));
        }
    }
    println!("{}", serde_json::to_string(&fixtures).unwrap());
}
