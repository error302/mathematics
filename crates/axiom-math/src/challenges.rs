//! Original, bounded practice quests. Exact mathematics and clue validation live in Rust.
use crate::{
    checker::GradeOutcome,
    coaching::CoachingMaterial,
    exercise::{GenerationFailure, ProblemArtifact},
    number::{int, to_canonical},
    parse::{evaluate, parse_scalar, MathBudget},
    rng::{shuffle, uniform_inclusive, DrawSource},
    Q,
};
use serde_json::{json, Value};
type Parts = (
    String,
    String,
    String,
    String,
    Option<Vec<String>>,
    Option<String>,
    Value,
    String,
);
const CHECKER: &str = "academy-challenge-v1";

fn apply(a: Q, b: Q, op: &str) -> Option<Q> {
    Some(match op {
        "+" => a + b,
        "-" => a - b,
        "*" => a * b,
        "/" => {
            if b == int(0) {
                return None;
            }
            a / b
        }
        _ => return None,
    })
}
pub fn line(values: [i64; 3], ops: &[String]) -> Option<Q> {
    if ops.len() != 2 {
        return None;
    }
    let [a, b, c] = values.map(int);
    if matches!(ops[1].as_str(), "*" | "/") && !matches!(ops[0].as_str(), "*" | "/") {
        apply(a, apply(b, c, &ops[1])?, &ops[0])
    } else {
        apply(apply(a, b, &ops[0])?, c, &ops[1])
    }
}
fn totals(cells: &[i64], ops: &[Vec<String>]) -> Option<Vec<String>> {
    if cells.len() != 9 || ops.len() != 6 {
        return None;
    }
    (0..6)
        .map(|i| {
            let values = if i < 3 {
                [cells[i * 3], cells[i * 3 + 1], cells[i * 3 + 2]]
            } else {
                let c = i - 3;
                [cells[c], cells[c + 3], cells[c + 6]]
            };
            line(values, &ops[i]).map(|q| to_canonical(&q))
        })
        .collect()
}
fn count_solutions(
    cells: &mut [i64],
    blanks: &[usize],
    pool: &mut [i64],
    ops: &[Vec<String>],
    targets: &[String],
    depth: usize,
) -> usize {
    if depth == blanks.len() {
        return usize::from(totals(cells, ops).as_deref() == Some(targets));
    }
    let mut count = 0;
    for i in depth..pool.len() {
        pool.swap(depth, i);
        cells[blanks[depth]] = pool[depth];
        count += count_solutions(cells, blanks, pool, ops, targets, depth + 1);
        pool.swap(depth, i);
        if count > 1 {
            break;
        }
    }
    count
}
fn grid<R: DrawSource + ?Sized>(rng: &mut R, band: u32) -> Result<Parts, GenerationFailure> {
    for _ in 0..4096 {
        let mut cells = vec![1, 2, 3, 4, 6, 8, 9, 12, 18];
        shuffle(rng, &mut cells);
        let allowed = if band == 1 {
            vec!["+", "-"]
        } else if band == 2 {
            vec!["+", "-", "*"]
        } else {
            vec!["+", "-", "*", "/"]
        };
        let mut ops: Vec<Vec<String>> = (0..6)
            .map(|_| {
                (0..2)
                    .map(|_| {
                        allowed[uniform_inclusive(rng, 0, (allowed.len() - 1) as i128) as usize]
                            .into()
                    })
                    .collect()
            })
            .collect();
        // Hard puzzles always contain all four operations, including exact division.
        if band == 3 {
            ops[0] = vec!["*".into(), "+".into()];
            ops[1] = vec!["/".into(), "-".into()];
            ops[2] = vec!["+".into(), "*".into()]
        }
        let Some(targets) = totals(&cells, &ops) else {
            continue;
        };
        if targets
            .iter()
            .any(|s| s.contains('/') || s.parse::<i64>().map_or(true, |v| !(0..=999).contains(&v)))
        {
            continue;
        }
        let blanks = if band == 3 {
            vec![0, 1, 4, 8]
        } else {
            vec![0, 4, 8]
        };
        let solution: Vec<i64> = blanks.iter().map(|&i| cells[i]).collect();
        let mut pool = solution.clone();
        shuffle(rng, &mut pool);
        if count_solutions(
            &mut cells.clone(),
            &blanks,
            &mut pool.clone(),
            &ops,
            &targets,
            0,
        ) != 1
        {
            continue;
        }
        let shown: Vec<Value> = cells
            .iter()
            .enumerate()
            .map(|(i, &v)| {
                if blanks.contains(&i) {
                    Value::Null
                } else {
                    json!(v)
                }
            })
            .collect();
        let steps: Vec<String> = (0..6)
            .map(|i| {
                let v = if i < 3 {
                    vec![cells[i * 3], cells[i * 3 + 1], cells[i * 3 + 2]]
                } else {
                    let c = i - 3;
                    vec![cells[c], cells[c + 3], cells[c + 6]]
                };
                format!(
                    "{} {}: {} {} {} {} {} = {}.",
                    if i < 3 { "Row" } else { "Column" },
                    i % 3 + 1,
                    v[0],
                    ops[i][0],
                    v[1],
                    ops[i][1],
                    v[2],
                    targets[i]
                )
            })
            .collect();
        let prompt="Fill the empty squares. Use each number in the pool once. Every row and column must match its total.".to_string();
        return Ok((
            prompt.clone(),
            String::new(),
            prompt,
            "number_grid".into(),
            None,
            Some(serde_json::to_string(&solution).unwrap()),
            json!({"kind":"grid","cells":shown,"blanks":blanks,"pool":pool,"ops":ops,"totals":targets,"hint":"Start with a row or column with one empty square. Undo addition with subtraction, or multiplication with division. Multiply and divide before adding and subtracting.","steps":steps,"conclusion":"All six equations agree. The number pool has exactly one solution for this grid.","success_note":"You connected the row and column constraints. Now explain which clue you used first."}),
            CHECKER.into(),
        ));
    }
    Err(GenerationFailure::ConstraintExhaustion { draws: 4096 })
}

pub fn generate<R: DrawSource + ?Sized>(
    rng: &mut R,
    id: &str,
    band: u32,
) -> Result<Parts, GenerationFailure> {
    if !(1..=3).contains(&band) {
        return Err(GenerationFailure::TemplateNotFound(
            "Challenge bands are 1–3".into(),
        ));
    }
    if id == "puzzles.arithmetic.grid" {
        return grid(rng, band);
    }
    let n = uniform_inclusive(rng, 2, 4 + band as i128 * 3) as i64;
    let k = uniform_inclusive(rng, 2, 3 + band as i128) as i64;
    let mut meta = json!({});
    let mut options = None;
    let (prompt, target, hint, steps, note): (String, String, String, Vec<String>, String) =
        match id {
            "reasoning.pattern" => {
                let xs: Vec<i64> = (0..4)
                    .map(|i| {
                        if band == 1 {
                            n + i * k
                        } else if band == 2 {
                            n * k.pow(i as u32)
                        } else {
                            n + i * i * k
                        }
                    })
                    .collect();
                let rule = if band == 1 {
                    format!("add {k} each time")
                } else if band == 2 {
                    format!("multiply by {k} each time")
                } else {
                    format!("term at index i is {n} + {k} × i², starting at i = 0")
                };
                let next = if band == 1 {
                    n + 4 * k
                } else if band == 2 {
                    n * k.pow(4)
                } else {
                    n + 16 * k
                };
                (format!("The rule is: {rule}. Terms: {}. What is the next term?",xs.iter().map(|x|x.to_string()).collect::<Vec<_>>().join(", ")),next.to_string(),"Use the stated rule, rather than guessing from a few terms. A finite list can fit many different rules.".into(),vec![format!("The first four terms follow: {rule}."),format!("Apply the rule once more to get {next}.")],"You applied a stated rule consistently. Try explaining why a list alone does not specify a unique pattern.".into())
            }
            "reasoning.logic" => {
                options = Some(vec!["always".into(), "sometimes".into(), "never".into()]);
                let variant = uniform_inclusive(rng, 0, 1);
                if band == 1 {
                    if variant == 0 {
                        ("A rectangle has four right angles. A square has four equal sides and four right angles. Does a square meet the rectangle definition? Choose always, sometimes, or never.".into(),"always".into(),"Use the definitions. Ask whether a square satisfies every condition required of a rectangle.".into(),vec!["A square has four right angles.".into(),"That satisfies the rectangle definition; equal sides are an extra condition.".into()],"You used a definition to test a logical implication.".into())
                    } else {
                        (format!("An even number can be divided into two equal whole-number groups. {} counters make two groups of {n} with one counter left over. Is this total even? Choose always, sometimes, or never.",2*n+1),"never".into(),"Use the given grouping: an even total leaves no counter left over.".into(),vec![format!("Two groups of {n} use {} counters.",2*n),"There is one counter left over, so the given total is odd and is not even.".into()],"You tested the example against the stated definition.".into())
                    }
                } else if band == 2 {
                    if variant == 0 {
                        (format!("Consider any rectangle. Does it have four equal sides? Choose always, sometimes, or never."),"sometimes".into(),format!("Test a {n} by {} rectangle and a {n} by {n} rectangle. Does the property hold in both?",n+1),vec![format!("A {n} by {} rectangle has unequal adjacent sides.",n+1),format!("A {n} by {n} rectangle has four equal sides. The property holds for some rectangles, but not all.")],"You used an example and a counterexample to distinguish sometimes from always.".into())
                    } else {
                        (format!("Consider any integer divisible by {k}. Is it also divisible by {}? Choose always, sometimes, or never.",2*k),"sometimes".into(),format!("Test {k} and {}. Both are divisible by {k}; which is divisible by {}?",2*k,2*k),vec![format!("{k} is divisible by {k}, but not by {}.",2*k),format!("{} is divisible by both. So the second property holds sometimes.",2*k)],"You tested the converse using an example and a counterexample.".into())
                    }
                } else {
                    (format!("Let n be any integer. If n is divisible by {}, is n divisible by {k}? Choose always, sometimes, or never.",2*k),"always".into(),"Write divisible as a multiplication by an integer. Can you regroup that product?".into(),vec![format!("If n is divisible by {}, write n = {} × t for an integer t.",2*k,2*k),format!("Then n = {k} × (2t). Since 2t is an integer, n is divisible by {k}.")],"You followed a general argument rather than relying only on examples.".into())
                }
            }
            "reasoning.comprehension" => {
                let paid = if band == 1 { n * k } else { n * k - k };
                let question = if band == 1 {
                    format!("You buy {n} notebooks at {k} credits each. What is the total cost in credits?")
                } else if band == 2 {
                    format!("You buy {n} notebooks at {k} credits each. One notebook is free. What do you pay in credits?")
                } else {
                    format!("A pack contains {n} notebooks and costs {} credits. You buy 2 packs and receive a discount of {k} credits on the whole purchase. What do you pay in credits?",n*k)
                };
                let ans = if band == 3 { 2 * n * k - k } else { paid };
                (question,ans.to_string(),"Identify the quantities, their units, and exactly what the question asks. Apply any discount once, to the stated purchase.".into(),vec![format!("Before the discount: {} credits.",if band==3{2*n*k}else{n*k}),format!("After the stated discount: {ans} credits.")],"You translated the story into quantities and operations. Check that your answer has the requested unit.".into())
            }
            "reasoning.analysis" => {
                let total = n * k;
                let remove = if band == 1 { 0 } else { k };
                let groups = if band == 3 { 2 } else { 1 };
                (format!("There are {n} trays with {k} counters each. Remove {remove} counters in total, then share the remaining counters equally between {groups} people. How many does each receive?"),to_canonical(&(int(total-remove)/int(groups))),"Make a plan: find the total, subtract the removed counters, then divide by the number of people. Fractions are allowed.".into(),vec![format!("Total counters: {n} × {k} = {total}."),format!("Remaining counters: {total} − {remove} = {}.",total-remove),format!("Each receives {} ÷ {groups} = {}.",total-remove,to_canonical(&(int(total-remove)/int(groups))))],"You organized a multi-step calculation. Check it by reversing the steps.".into())
            }
            "visual.rectangle" => {
                let w = n;
                let h = k;
                meta = json!({"kind":"rectangle","width":w,"height":h});
                let target = if band == 1 { w * h } else { 2 * (w + h) };
                let measure = if band == 1 {
                    "area in square units"
                } else {
                    "perimeter in units"
                };
                (
                    format!(
                        "A rectangle is {w} units wide and {h} units tall. Find its {measure}."
                    ),
                    target.to_string(),
                    if band == 1 {
                        "Area counts unit squares covering the surface. Multiply width by height."
                            .into()
                    } else {
                        "Perimeter follows the boundary. Include all four sides; do not multiply width by height.".into()
                    },
                    vec![
                        format!("Opposite sides have equal lengths: {w}, {h}, {w}, {h}."),
                        if band == 1 {
                            format!("Area = {w} × {h} = {target} square units.")
                        } else {
                            format!("Perimeter = 2 × ({w} + {h}) = {target} units.")
                        },
                    ],
                    "You connected a diagram, dimensions, and a measurement formula.".into(),
                )
            }
            "visual.linear" | "visual.quadratic" => {
                let a = if band == 1 { 1 } else { k };
                let b = if band == 3 { -n } else { n };
                let quadratic = id == "visual.quadratic";
                let x = if band == 1 {
                    1
                } else if band == 2 {
                    2
                } else {
                    -2
                };
                let y = |x: i64| if quadratic { a * x * x + b } else { a * x + b };
                let points: Vec<Value> = (-3..=3).map(|x| json!([x, y(x)])).collect();
                meta = json!({"kind":"graph","quadratic":quadratic,"a":a,"b":b,"points":points,"control":[0,b-9*a],"x_min":-3,"x_max":3,"y_min":if quadratic{b.min(0)-2}else{(b-3*a).min(0)-2},"y_max":if quadratic{(a*9+b).max(0)+2}else{(a*3+b).max(0)+2},"query_x":x});
                let eq = format!(
                    "y = {a}x{} {} {}",
                    if quadratic { "²" } else { "" },
                    if b < 0 { "−" } else { "+" },
                    b.abs()
                );
                (format!("For {eq}, what is y when x = {x}? Use the plot to predict, then calculate exactly."),y(x).to_string(),"Find the given x on the horizontal axis, move to the curve, then read y. Substitute x into the equation to confirm; for x², square before multiplying.".into(),vec![format!("Substitute x = {x} into {eq}."),format!("{} = {}.",if quadratic{format!("{a} × ({x})² + ({b})")}else{format!("{a} × ({x}) + ({b})")},y(x))],"You linked the equation to a coordinate on its graph. The exact substitution confirms your reading.".into())
            }
            _ => return Err(GenerationFailure::TemplateNotFound(id.into())),
        };
    meta["hint"] = json!(hint);
    meta["steps"] = json!(steps);
    meta["conclusion"] = json!(format!("The checked answer is {target}."));
    meta["success_note"] = json!(note);
    Ok((
        prompt.clone(),
        String::new(),
        prompt,
        if options.is_some() {
            "choice"
        } else {
            "scalar"
        }
        .into(),
        options,
        Some(target),
        meta,
        CHECKER.into(),
    ))
}

fn validated(p: &ProblemArtifact) -> Result<ProblemArtifact, String> {
    let r = &p.replay_identity;
    let fresh = crate::generate(
        &r.template_id,
        &r.template_version,
        &r.seed_hex,
        r.difficulty_band,
        &r.content_manifest_hash,
    )
    .map_err(|e| e.to_string())?;
    if fresh != *p {
        return Err(
            "Exercise differs from its deterministic replay. Choose a fresh challenge.".into(),
        );
    }
    Ok(fresh)
}
pub fn coaching(p: &ProblemArtifact) -> Result<CoachingMaterial, String> {
    let p = validated(p)?;
    Ok(CoachingMaterial {
        version: "challenge-coaching-v1".into(),
        hint: p.metadata["hint"].as_str().ok_or("Missing hint")?.into(),
        steps: serde_json::from_value(p.metadata["steps"].clone()).map_err(|_| "Missing steps")?,
        conclusion: p.metadata["conclusion"]
            .as_str()
            .ok_or("Missing conclusion")?
            .into(),
        success_note: p.metadata["success_note"]
            .as_str()
            .ok_or("Missing success note")?
            .into(),
        visual: None,
    })
}
pub fn check(p: &ProblemArtifact, answer: &str, budget: &MathBudget) -> GradeOutcome {
    let p = match validated(p) {
        Ok(v) => v,
        Err(e) => return GradeOutcome::unsupported(CHECKER, "invalid_replay", e),
    };
    if p.answer_kind == "number_grid" {
        let values: Vec<i64> = match serde_json::from_str(answer) {
            Ok(v) => v,
            Err(_) => {
                return GradeOutcome::malformed(
                    CHECKER,
                    "grid_format",
                    "Choose a number for every empty square.",
                )
            }
        };
        let blanks: Vec<usize> = serde_json::from_value(p.metadata["blanks"].clone()).unwrap();
        let mut pool: Vec<i64> = serde_json::from_value(p.metadata["pool"].clone()).unwrap();
        let mut used = values.clone();
        pool.sort();
        used.sort();
        if used != pool {
            return GradeOutcome::malformed(CHECKER,"number_pool","Use every number from the pool exactly once. Check for repeats or missing squares.");
        }
        let mut cells: Vec<i64> = p.metadata["cells"]
            .as_array()
            .unwrap()
            .iter()
            .map(|v| v.as_i64().unwrap_or(0))
            .collect();
        for (i, &index) in blanks.iter().enumerate() {
            cells[index] = values[i]
        }
        let ops: Vec<Vec<String>> = serde_json::from_value(p.metadata["ops"].clone()).unwrap();
        let targets: Vec<String> = serde_json::from_value(p.metadata["totals"].clone()).unwrap();
        let got = match totals(&cells, &ops) {
            Some(v) => v,
            None => {
                return GradeOutcome::incorrect(
                    CHECKER,
                    "division_zero",
                    "A division has zero as its divisor. Recheck that column or row.",
                )
            }
        };
        if let Some(i) = (0..6).find(|&i| got[i] != targets[i]) {
            return GradeOutcome::incorrect(CHECKER,"clue_mismatch",format!("{} {} gives {}, but its total is {}. Multiply and divide first, then add and subtract; also check the crossing clues.",if i<3{"Row"}else{"Column"},i%3+1,got[i],targets[i]));
        }
    } else if p.answer_kind == "choice" {
        let answer = answer.trim().to_lowercase();
        if !p.options.as_ref().unwrap().contains(&answer) {
            return GradeOutcome::malformed(
                CHECKER,
                "choice_format",
                "Choose always, sometimes, or never.",
            );
        }
        if Some(&answer) != p.target_value.as_ref() {
            return GradeOutcome::incorrect(CHECKER,"logical_claim","Revisit the definition or a counterexample. One example alone cannot establish an always claim.");
        }
    } else {
        let ast = match parse_scalar(answer, budget) {
            Ok(v) => v,
            Err(e) => {
                return if e.budget || e.code == "non_integer_exponent" {
                    GradeOutcome::unsupported(CHECKER, &e.code, e.message)
                } else {
                    GradeOutcome::malformed(CHECKER, &e.code, e.message)
                }
            }
        };
        let q = match evaluate(&ast, budget) {
            Ok(v) => v,
            Err(e) => {
                return if e.budget || e.code == "non_integer_exponent" {
                    GradeOutcome::unsupported(CHECKER, &e.code, e.message)
                } else {
                    GradeOutcome::malformed(CHECKER, &e.code, e.message)
                }
            }
        };
        if Some(to_canonical(&q)) != p.target_value {
            return GradeOutcome::incorrect(CHECKER,"quantity_mismatch","The value does not yet match. Use the hint to check the rule, quantities, units, and order of operations.");
        }
    }
    GradeOutcome::correct(CHECKER, p.metadata["success_note"].as_str().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{generate, Disposition};
    #[test]
    fn replay_unique_and_exact() {
        for band in 1..=3 {
            for seed in 0..32 {
                let s = format!("{seed:064x}");
                let p = generate("puzzles.arithmetic.grid", "1.1.0", &s, band, "test").unwrap();
                assert_eq!(
                    p,
                    generate("puzzles.arithmetic.grid", "1.1.0", &s, band, "test").unwrap()
                );
                assert_eq!(
                    check(&p, p.target_value.as_ref().unwrap(), &MathBudget::default()).disposition,
                    Disposition::Correct
                );
                let blanks: Vec<usize> =
                    serde_json::from_value(p.metadata["blanks"].clone()).unwrap();
                let mut cells: Vec<i64> = p.metadata["cells"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(|v| v.as_i64().unwrap_or(0))
                    .collect();
                let mut pool =
                    serde_json::from_value::<Vec<i64>>(p.metadata["pool"].clone()).unwrap();
                let ops =
                    serde_json::from_value::<Vec<Vec<String>>>(p.metadata["ops"].clone()).unwrap();
                let targets =
                    serde_json::from_value::<Vec<String>>(p.metadata["totals"].clone()).unwrap();
                assert_eq!(
                    count_solutions(&mut cells, &blanks, &mut pool, &ops, &targets, 0),
                    1
                );
                if band == 3 {
                    for op in ["+", "-", "*", "/"] {
                        assert!(ops.iter().flatten().any(|s| s == op))
                    }
                }
            }
        }
    }
    #[test]
    fn rejects_bad_pool_and_tampered_reference() {
        let mut p = generate(
            "puzzles.arithmetic.grid",
            "1.1.0",
            &"00".repeat(32),
            1,
            "test",
        )
        .unwrap();
        assert_eq!(
            check(&p, "[1,1,1]", &MathBudget::default()).disposition,
            Disposition::Malformed
        );
        p.target_value = Some("[1,1,1]".into());
        assert_eq!(
            check(&p, "[1,1,1]", &MathBudget::default()).disposition,
            Disposition::Unsupported
        );
        assert!(coaching(&p).is_err())
    }
    #[test]
    fn precedence_and_exact_division() {
        assert_eq!(
            line([2, 3, 4], &vec!["+".into(), "*".into()]),
            Some(int(14))
        );
        assert_eq!(line([8, 4, 2], &vec!["/".into(), "*".into()]), Some(int(4)));
        assert_eq!(
            line([1, 2, 3], &vec!["/".into(), "+".into()]),
            Some(crate::number::frac(7, 2))
        )
    }
    #[test]
    fn unsupported_math_is_not_wrong() {
        let p = generate("reasoning.analysis", "1.1.0", &"01".repeat(32), 3, "test").unwrap();
        assert_eq!(
            check(&p, "2^1000000000", &MathBudget::default()).disposition,
            Disposition::Unsupported
        );
        assert_eq!(
            check(&p, "4^(1/2)", &MathBudget::default()).disposition,
            Disposition::Unsupported
        );
        assert_eq!(
            check(&p, "1/0", &MathBudget::default()).disposition,
            Disposition::Malformed
        )
    }
    #[test]
    fn graph_viewports_and_continuous_curve_are_exact() {
        for id in ["visual.linear", "visual.quadratic"] {
            for band in 1..=3 {
                for seed in 0..128 {
                    let p = generate(id, "1.1.0", &format!("{seed:064x}"), band, "test").unwrap();
                    let m = &p.metadata;
                    let lo = m["y_min"].as_i64().unwrap();
                    let hi = m["y_max"].as_i64().unwrap();
                    assert!(lo < 0 && hi > 0);
                    for xy in m["points"].as_array().unwrap() {
                        assert!((lo..=hi).contains(&xy[1].as_i64().unwrap()))
                    }
                    if id == "visual.quadratic" {
                        let a = int(m["a"].as_i64().unwrap());
                        let b = int(m["b"].as_i64().unwrap());
                        let start = int(m["points"][0][1].as_i64().unwrap());
                        let end = int(m["points"][6][1].as_i64().unwrap());
                        let control = int(m["control"][1].as_i64().unwrap());
                        for i in 0..=4 {
                            let t = crate::number::frac(i, 4);
                            let u = int(1) - &t;
                            let x = int(-3) + int(6) * &t;
                            let curve =
                                &u * &u * &start + int(2) * &u * &t * &control + &t * &t * &end;
                            assert_eq!(curve, &a * &x * &x + &b)
                        }
                    }
                }
            }
        }
    }
    #[test]
    fn reasoning_visuals_and_bounds() {
        for id in [
            "reasoning.pattern",
            "reasoning.logic",
            "reasoning.comprehension",
            "reasoning.analysis",
            "visual.rectangle",
            "visual.linear",
            "visual.quadratic",
        ] {
            for band in 1..=3 {
                let p = generate(id, "1.1.0", &"ab".repeat(32), band, "test").unwrap();
                assert_eq!(
                    check(&p, p.target_value.as_ref().unwrap(), &MathBudget::default()).disposition,
                    Disposition::Correct
                );
                assert!(!coaching(&p).unwrap().steps.is_empty());
                if p.metadata["kind"] == "graph" {
                    let a = p.metadata["a"].as_i64().unwrap();
                    let b = p.metadata["b"].as_i64().unwrap();
                    for xy in p.metadata["points"].as_array().unwrap() {
                        let x = xy[0].as_i64().unwrap();
                        assert_eq!(
                            xy[1].as_i64().unwrap(),
                            if id == "visual.quadratic" {
                                a * x * x + b
                            } else {
                                a * x + b
                            }
                        )
                    }
                }
            }
            assert!(generate(id, "1.1.0", &"ab".repeat(32), 4, "test").is_err())
        }
    }
}
