fn main() {
    let seed = "0000000000000000000000000000000000000000000000000000000000000000";
    let hash = "0000000000000000000000000000000000000000000000000000000000000000";
    let templates = ["fractions.compare.positive", "fractions.equivalent.find", "fractions.unit.identify", "arithmetic.whole.addition", "arithmetic.column.addition", "arithmetic.column.subtraction", "place_value.decompose", "abacus.read.state", "abacus.target.setting"];
    let artifacts: Vec<_> = templates.iter().map(|template| axiom_math::generate(template, "1.1.0", seed, 1, hash).unwrap()).collect();
    println!("{}", serde_json::to_string(&artifacts).unwrap());
}
