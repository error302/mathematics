//! Exact, bounded help material. Separate from grading/replay so feedback never changes a result.
use crate::{ProblemArtifact, AbacusState, number::{from_canonical, to_canonical, int}, abacus_tutor::TeachingFrame};
use serde::{Serialize,Deserialize};
use serde_json::{Value,json};

#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct CoachingMaterial {
    pub version: String,
    pub hint: String,
    pub steps: Vec<String>,
    pub conclusion: String,
    pub success_note: String,
    pub visual: Option<Value>,
}
fn integer(p:&ProblemArtifact,key:&str)->Result<i64,String>{
    let v=&p.metadata[key];
    let n=v.as_i64().or_else(||v.as_str().filter(|s|s.len()<20).and_then(|s|s.parse().ok())).ok_or_else(||format!("Missing bounded field: {key}"))?;
    if !(0..=999_999_999).contains(&n){return Err("Value outside teaching bounds".into())}Ok(n)
}
fn signed_integer(p:&ProblemArtifact,key:&str)->Result<i64,String>{
    let v=&p.metadata[key];
    let n=v.as_i64().or_else(||v.as_str().filter(|s|s.len()<20).and_then(|s|s.parse().ok())).ok_or_else(||format!("Missing bounded field: {key}"))?;
    if !(-999_999_999..=999_999_999).contains(&n){return Err("Value outside teaching bounds".into())}Ok(n)
}
fn fraction(p:&ProblemArtifact,key:&str)->Result<crate::Q,String>{
    let s=p.metadata[key].as_str().filter(|s|s.len()<64).ok_or_else(||format!("Missing fraction: {key}"))?;
    from_canonical(s).ok_or_else(||"Invalid reference fraction".into())
}
fn place(index:usize)->String {match index{0=>"Ones".into(),1=>"Tens".into(),2=>"Hundreds".into(),3=>"Thousands".into(),_=>format!("10^{index} place")}}
fn addition(a:i64,b:i64)->Vec<String>{
    let mut result=vec!["Line up the numbers so ones meet ones, tens meet tens, and so on.".into()];
    let(mut remaining_a,mut remaining_b,mut carry,mut column)=(a,b,0,0);
    loop{let x=remaining_a%10;let y=remaining_b%10;let total=x+y+carry;let incoming=carry;carry=total/10;
        result.push(format!("{}: {x} + {y} + {incoming} carried = {total}. Write {}; carry {carry} to the next column.",place(column),total%10));
        remaining_a/=10;remaining_b/=10;column+=1;if remaining_a==0&&remaining_b==0&&carry==0{break}}
    result.push(format!("Read the result: {a} + {b} = {}.",a+b));result
}
fn subtraction(a:i64,b:i64)->Result<Vec<String>,String>{
    if a<b{return Err("Negative subtraction not in this teaching family".into())}
    let mut digits:Vec<i64>=a.to_string().chars().rev().map(|c|c.to_digit(10).unwrap() as i64).collect();
    let mut other:Vec<i64>=b.to_string().chars().rev().map(|c|c.to_digit(10).unwrap() as i64).collect();other.resize(digits.len(),0);
    let mut result=vec!["Line up the place-value columns. Work from the ones toward the left.".into()];
    for i in 0..digits.len(){if digits[i]<other[i]{let j=((i+1)..digits.len()).find(|&j|digits[j]>0).ok_or_else(||"Invalid borrow reference".to_string())?;digits[j]-=1;for digit in &mut digits[i+1..j]{*digit=9}digits[i]+=10;
        result.push(format!("Regroup from the {} column into smaller units. Any zero columns crossed become nine; the {} column now contains {}.",place(j),place(i),digits[i]));}
        result.push(format!("{}: {} − {} = {}. Write {} in this column.",place(i),digits[i],other[i],digits[i]-other[i],digits[i]-other[i]));}
    result.push(format!("Read the result: {a} − {b} = {}.",a-b));Ok(result)
}

pub fn material(p:&ProblemArtifact)->Result<CoachingMaterial,String>{
    if p.replay_identity.checker_version == "academy-challenge-v1" { return crate::challenges::coaching(p); }
    let target=p.target_value.as_ref().filter(|s|s.len()<64).ok_or_else(||"Missing reference answer".to_string())?;
    let mut output=CoachingMaterial{version:"coaching-v1".into(),hint:String::new(),steps:Vec::new(),conclusion:String::new(),success_note:String::new(),visual:None};
    let template=p.replay_identity.template_id.as_str();
    if template.starts_with("abacus.trainer."){
        let frames:Vec<TeachingFrame>=serde_json::from_value(p.metadata["trace"].clone()).map_err(|_|"Invalid teaching trace")?;
        if frames.is_empty()||frames.len()>12{return Err("Teaching trace outside bounds".into())}
        for frame in &frames{frame.state.validate().map_err(|e|e.to_string())?;if frame.state.rod_count>2{return Err("Teaching frame too wide".into())}if from_canonical(&frame.value)!=Some(frame.state.compute_value()){return Err("Teaching caption value mismatch".into())}}
        for pair in frames.windows(2){let action=pair[1].action.as_ref().ok_or("Missing teaching move")?;if crate::apply_abacus_action(&pair[0].state,action).map_err(|e|e.to_string())?!=pair[1].state{return Err("Teaching move mismatch".into())}}
        let final_frame=frames.last().unwrap();if from_canonical(target)!=Some(final_frame.state.compute_value()){return Err("Teaching trace does not match target".into())}
        output.hint=frames.get(1).unwrap_or(&frames[0]).caption.clone();
        output.steps=frames.iter().map(|f|format!("{} Shown value: {}.",f.caption,f.value)).collect();output.conclusion=format!("The final number is {target}.");
        output.success_note="Your final number matches the exact check. Try describing the bead changes in your own words.".into();
        output.visual=Some(json!({"kind":"abacus","state":final_frame.state}));return Ok(output)
    }
    match template{
        "arithmetic.whole.addition"|"arithmetic.column.addition"=>{
            let(a,b)=(integer(p,"operand1")?,integer(p,"operand2")?);if from_canonical(target)!=Some(int(a+b)){return Err("Addition reference mismatch".into())}
            output.hint="Start in the ones column. If a column makes ten or more, regroup ten units into one unit in the next column.".into();output.steps=addition(a,b);output.conclusion=format!("{a} + {b} = {}.",a+b);output.success_note="Your sum is correct. Place-value columns give you a way to explain and check it.".into();output.visual=Some(json!({"kind":"columns","rows":[a.to_string(),format!("+ {b}"),(a+b).to_string()]}));
        }
        "arithmetic.whole.multiplication"=>{
            let(a,b)=(integer(p,"factor1")?,integer(p,"factor2")?);if from_canonical(target)!=Some(int(a*b)){return Err("Multiplication reference mismatch".into())}
            output.hint="Multiplication represents equal groups or a rectangular array. Decompose into place values or add repeated groups.".into();
            output.steps=if a<10&&b<10{vec![format!("Think of {a} groups of {b}, or a rectangular array with {a} rows and {b} columns."),format!("{a} × {b} = {}.",a*b)]}else{let tens=(a/10)*10;let ones=a%10;vec![format!("Decompose {a} by place value: {a} = {tens} + {ones}."),format!("Multiply each part: {tens} × {b} = {} and {ones} × {b} = {}.",tens*b,ones*b),format!("Combine partial products: {} + {} = {}.",tens*b,ones*b,a*b)]};
            output.conclusion=format!("{a} × {b} = {}.",a*b);output.success_note="Your product is correct. Arrays and place value explain why multiplication works.".into();
            output.visual=Some(json!({"kind":"columns","rows":[a.to_string(),format!("× {b}"),(a*b).to_string()]}));
        }
        "arithmetic.whole.division"=>{
            let(dividend,divisor)=(integer(p,"dividend")?,integer(p,"divisor")?);let quotient=integer(p,"quotient")?;
            if divisor==0||from_canonical(target)!=Some(int(quotient)){return Err("Division reference mismatch".into())}
            output.hint="Division splits an amount into equal groups. Check your answer by multiplying the quotient by the divisor.".into();
            output.steps=vec![format!("We want to find how many groups of {divisor} fit into {dividend}."),format!("Think: what number × {divisor} = {dividend}?"),format!("Since {quotient} × {divisor} = {dividend}, {dividend} ÷ {divisor} = {quotient}.")];
            output.conclusion=format!("{dividend} ÷ {divisor} = {quotient}.");output.success_note="Your quotient is correct. Multiplication confirms that {quotient} × {divisor} = {dividend}.".into();
            output.visual=Some(json!({"kind":"columns","rows":[format!("{dividend} ÷ {divisor}"),format!("= {quotient}"),format!("Check: {quotient} × {divisor} = {dividend}")]}));
        }
        "arithmetic.column.subtraction"=>{
            let(a,b)=(integer(p,"minuend")?,integer(p,"subtrahend")?);if from_canonical(target)!=Some(int(a-b)){return Err("Subtraction reference mismatch".into())}
            output.hint="Compare the ones digits first. If the top digit is too small, regroup one higher-place unit into ten smaller units before subtracting.".into();output.steps=subtraction(a,b)?;output.conclusion=format!("{a} − {b} = {}.",a-b);output.success_note="Your difference is correct. Add it back to the amount taken away to check the starting number.".into();output.visual=Some(json!({"kind":"columns","rows":[a.to_string(),format!("− {b}"),(a-b).to_string()]}));
        }
        "fractions.compare.positive"=>{
            let(q1,q2)=(fraction(p,"frac1")?,fraction(p,"frac2")?);let d=integer(p,"common_denom")?;if d<1||d>10000{return Err("Common denominator outside bounds".into())}
            let n1=&q1*int(d);let n2=&q2*int(d);if !n1.is_integer()||!n2.is_integer(){return Err("Invalid common denominator".into())}
            let relation=if q1<q2{"less"}else if q1>q2{"greater"}else{"equal"};if target!=relation{return Err("Comparison reference mismatch".into())}
            output.hint="Compare equal-sized parts of the same whole. Rewrite both fractions using a common denominator, then compare their numerators.".into();
            output.steps=vec![format!("Use {d} equal-sized parts in each whole."),format!("{} = {}/{d}; {} = {}/{d}.",to_canonical(&q1),to_canonical(&n1),to_canonical(&q2),to_canonical(&n2)),format!("With the same denominator, compare {} and {}.",to_canonical(&n1),to_canonical(&n2))];output.conclusion=format!("{} is {relation}{} {}.",to_canonical(&q1),if relation=="equal"{" to"}else{" than"},to_canonical(&q2));output.success_note="Your comparison is correct. Equal-sized parts explain why it works.".into();
            let num1=to_canonical(&n1).parse::<i64>().map_err(|_|"Visual numerator out of bounds")?;let num2=to_canonical(&n2).parse::<i64>().map_err(|_|"Visual numerator out of bounds")?;
            if !(0..=d).contains(&num1)||!(0..=d).contains(&num2){return Err("Fraction strip outside the whole".into())}
            output.visual=Some(json!({"kind":"fractions","rows":[{"label":format!("{} = {num1}/{d}",to_canonical(&q1)),"numerator":num1,"denominator":d},{"label":format!("{} = {num2}/{d}",to_canonical(&q2)),"numerator":num2,"denominator":d}]}));
        }
        "fractions.equivalent.find"=>{
            let q=fraction(p,"base_frac")?;let(d,n)=(integer(p,"target_denominator")?,integer(p,"target_numerator")?);if d<1||from_canonical(target)!=Some(crate::number::frac(n,d))||q!=crate::number::frac(n,d){return Err("Equivalent fraction reference mismatch".into())}
            let scale=int(d)/int(q.denom().to_string().parse::<i64>().map_err(|_|"Base denominator too large")?);
            output.hint="Find the factor that changes the original denominator into the requested denominator. Multiply the numerator by that same factor.".into();output.steps=vec![format!("Start with {} and the requested denominator {d}.",to_canonical(&q)),format!("The denominator is multiplied by {}. Multiply the numerator by {} too.",to_canonical(&scale),to_canonical(&scale)),format!("Write {n}/{d}; changing both by the same factor preserves the value.")];output.conclusion=format!("The requested form is {n}/{d}.");output.success_note="Your fraction has both the correct value and the requested denominator.".into();
        }
        "fractions.unit.identify"=>{
            let d=integer(p,"denominator")?;if d<1||d>100||from_canonical(target)!=Some(crate::number::frac(1,d)){return Err("Unit fraction reference mismatch".into())}
            output.hint="The numerator counts the pieces you chose. The denominator counts the equal-sized pieces in the whole.".into();output.steps=vec![format!("The whole has {d} equal pieces."),"You chose one piece, so the numerator is 1.".into(),format!("Put the piece count below it: 1/{d}.")];output.conclusion=format!("One piece is 1/{d} of the whole.");output.success_note="You correctly named one equal-sized part of the whole.".into();output.visual=Some(json!({"kind":"fractions","rows":[{"label":format!("1/{d}"),"numerator":1,"denominator":d}]}));
        }
        "place_value.decompose"=>{
            let n=integer(p,"number")?;let digit=integer(p,"digit")?;let value=integer(p,"place_value")?;if from_canonical(target)!=Some(int(value)){return Err("Place-value reference mismatch".into())}
            let name=p.metadata["place_name"].as_str().filter(|s|s.len()<30).ok_or("Missing place name")?;
            let weight=match name{"units"=>1,"tens"=>10,"hundreds"=>100,"thousands"=>1000,_=>return Err("Unknown place name".into())};if digit!=(n/weight)%10||value!=digit*weight{return Err("Digit and place-value reference mismatch".into())}
            output.hint="Count positions from the right: ones, tens, hundreds. The digit’s value is its count multiplied by the size of that place.".into();output.steps=vec![format!("Locate the {name} place in {n}."),format!("The digit there is {digit}."),format!("That digit contributes {value} to the number, rather than only its face value.")];output.conclusion=format!("The digit in the {name} place has value {value}.");output.success_note="You distinguished the digit itself from the value of its place.".into();
        }
        "abacus.read.state"|"abacus.target.setting"=>{
            let state:AbacusState=if template=="abacus.read.state"{serde_json::from_value(p.metadata["abacus_state"].clone()).map_err(|_|"Missing abacus state")?}else{AbacusState::from_value(&from_canonical(target).ok_or("Invalid target")?,2,0).map_err(|e|e.to_string())?};state.validate().map_err(|e|e.to_string())?;if state.rod_count>3||from_canonical(target)!=Some(state.compute_value()){return Err("Abacus reference mismatch".into())}
            output.hint="Count only beads at the middle bar. Each upper bead is five units of its rod; each lower bead is one. Read ones separately from tens.".into();output.steps=vec![state.describe_state(),"Combine the rod contributions, keeping their place values.".into()];output.conclusion=format!("The abacus represents {target}.");output.success_note="Your final value matches the abacus check. Place value makes each rod meaningful.".into();output.visual=Some(json!({"kind":"abacus","state":state}));
        }
        "integers.order"=>{
            let(a,b)=(signed_integer(p,"a")?,signed_integer(p,"b")?);
            let expected_rel=if a<b{"less"}else if a>b{"greater"}else{"equal"};
            if target!=expected_rel{return Err("Comparison reference mismatch".into())}
            let rel_word=if target=="less"{"less than"}else if target=="greater"{"greater than"}else{"equal to"};
            output.hint="On the signed number line, numbers further to the right are always greater, while numbers to the left are smaller.".into();
            output.steps=vec![
                format!("Plot {a} and {b} on the signed number line with zero at the center."),
                if a<b{
                    format!("{a} lies to the left of {b}. Negative numbers with larger absolute values are further to the left.")
                }else if a>b{
                    format!("{a} lies to the right of {b}. Values increase as you move towards positive numbers.")
                }else{
                    format!("{a} and {b} share the exact same location on the number line.")
                },
                format!("Therefore, {a} is {rel_word} {b}.")
            ];
            output.conclusion=format!("{a} is {rel_word} {b}.");
            output.success_note="Your comparison is correct. The number line gives a reliable visual ground for ordering positive and negative numbers.".into();
            output.visual=Some(json!({"kind":"number_line","a":a,"b":b,"min":a.min(b)-2,"max":a.max(b)+2}));
        }
        "divisibility.factors.find"=>{
            let n=integer(p,"number")?;
            let greatest=integer(p,"greatest_proper_divisor")?;
            if n<=1||n%greatest!=0||greatest>=n{return Err("Divisor reference mismatch".into())}
            output.hint="A proper divisor is a positive integer strictly less than the number that divides it with zero remainder.".into();
            output.steps=vec![
                format!("To find the greatest proper divisor of {n}, find the smallest positive integer > 1 that divides {n}."),
                format!("The smallest factor greater than 1 of {n} is {}.",n/greatest),
                format!("{n} ÷ {} = {greatest}, which is the largest factor strictly smaller than {n}.",n/greatest)
            ];
            output.conclusion=format!("The greatest proper divisor of {n} is {greatest}.");
            output.success_note="Your proper divisor is correct. Dividing by the smallest prime factor yields the greatest proper divisor.".into();
            output.visual=Some(json!({"kind":"factors","number":n,"proper_divisor":greatest}));
        }
        "divisibility.gcd_lcm"=>{
            let kind=p.metadata["kind"].as_str().ok_or("Missing operation kind")?;
            let(a,b)=(integer(p,"operand1")?,integer(p,"operand2")?);
            let result=integer(p,"result")?;
            if kind=="gcd"{
                if a%result!=0||b%result!=0{return Err("GCD reference mismatch".into())}
                output.hint="The Greatest Common Divisor (GCD) is the largest integer that divides both numbers without a remainder.".into();
                output.steps=vec![
                    format!("Find the divisors that {a} and {b} share in common."),
                    format!("The common divisors of {a} and {b} have maximum value {result}."),
                    format!("Check: {a} ÷ {result} = {} and {b} ÷ {result} = {}.",a/result,b/result)
                ];
                output.conclusion=format!("gcd({a}, {b}) = {result}.");
                output.success_note="Your GCD is correct. Common prime factors determine the greatest shared divisor.".into();
                output.visual=Some(json!({"kind":"gcd_venn","a":a,"b":b,"gcd":result}));
            }else if kind=="lcm"{
                let g=integer(p,"gcd")?;
                if (a*b)/g!=result{return Err("LCM reference mismatch".into())}
                output.hint="The Least Common Multiple (LCM) is the smallest positive integer divisible by both numbers.".into();
                output.steps=vec![
                    format!("Use the identity: lcm(a, b) = (a × b) / gcd(a, b)."),
                    format!("The GCD of {a} and {b} is {g}."),
                    format!("Calculate ({a} × {b}) / {g} = {} / {g} = {result}.",a*b)
                ];
                output.conclusion=format!("lcm({a}, {b}) = {result}.");
                output.success_note="Your LCM is correct. Connecting LCM to GCD through their product guarantees the exact multiple.".into();
                output.visual=Some(json!({"kind":"lcm_overlap","a":a,"b":b,"lcm":result}));
            }
        }
        "learning.problem_reading"=>{
            let target_n = integer(p, "target")?;
            if from_canonical(target) != Some(int(target_n)) {
                return Err("Problem reading reference mismatch".into());
            }
            output.hint = "Deconstruct the question: identify the given facts, formulate an equation for the unknown x, and solve.".into();
            output.steps = vec![
                "Identify what is known and what unknown x represents from the problem statement.".into(),
                "Formulate the relation connecting the quantities into an equation.".into(),
                format!("Solve for x: x = {target_n}."),
                format!("Check by verifying that {target_n} satisfies the problem context."),
            ];
            output.conclusion = format!("The unknown quantity is x = {target_n}.");
            output.success_note = "Your solution is correct. Formulating a word problem into an exact equation clarifies what is known and what must be found.".into();
            output.visual = Some(json!({"kind": "problem_breakdown", "target": target_n}));
        }
        "learning.equality_balance"=>{
            let target_n = integer(p, "target")?;
            if from_canonical(target) != Some(int(target_n)) {
                return Err("Equality balance reference mismatch".into());
            }
            output.hint = "The equals sign (=) represents a balance scale. To isolate x while maintaining equality, perform the exact same operations on both sides.".into();
            let b_type = p.metadata["type"].as_str().unwrap_or("balance");
            if b_type == "additive_balance" {
                let a = integer(p, "left_const")?;
                let sum_r = integer(p, "right_sum")?;
                output.steps = vec![
                    format!("The left pan has {a} + x and the right pan has total value {sum_r}."),
                    format!("Subtract {a} from both pans: ({a} + x) − {a} = {sum_r} − {a}."),
                    format!("x = {sum_r} − {a} = {target_n}."),
                    format!("Verify: {a} + {target_n} = {sum_r}, in perfect balance.")
                ];
            } else if b_type == "multiplicative_balance" {
                let coeff = integer(p, "coeff")?;
                let sum_r = integer(p, "right_sum")?;
                output.steps = vec![
                    format!("The left side has {coeff} × x and the right side has total value {sum_r}."),
                    format!("Divide both sides by {coeff}: ({coeff} × x) ÷ {coeff} = {sum_r} ÷ {coeff}."),
                    format!("x = {sum_r} ÷ {coeff} = {target_n}."),
                    format!("Verify: {coeff} × {target_n} = {sum_r}.")
                ];
            } else {
                let a = integer(p, "left_coeff")?;
                let b = integer(p, "left_const")?;
                let c = integer(p, "right_coeff")?;
                let d = integer(p, "right_const")?;
                output.steps = vec![
                    format!("Balance equation: {a}x + {b} = {c}x + {d}."),
                    format!("Subtract {c}x from both sides: {}x + {b} = {d}.", a - c),
                    format!("Subtract {b} from both sides: {}x = {}.", a - c, d - b),
                    format!("Divide by {}: x = {target_n}.", a - c),
                    format!("Verify: {a}({target_n}) + {b} = {} and {c}({target_n}) + {d} = {}.", a * target_n + b, c * target_n + d)
                ];
            }
            output.conclusion = format!("x = {target_n}.");
            output.success_note = "Your solution is correct. Treating equations as balances is foundational to algebraic reasoning.".into();
            output.visual = Some(json!({"kind": "balance_scale", "target": target_n}));
        }
        "learning.checking_bounds"=>{
            let target_n = integer(p, "target")?;
            if from_canonical(target) != Some(int(target_n)) {
                return Err("Checking bounds reference mismatch".into());
            }
            output.hint = "Checking bounds, parity invariants, or testing counterexamples lets you verify conclusions and detect arithmetic errors.".into();
            let kind = p.metadata["kind"].as_str().unwrap_or("bounds");
            if kind == "parity_check" {
                let word = p.metadata["parity_word"].as_str().unwrap_or("parity");
                output.steps = vec![
                    "Parity arithmetic: odd × odd = odd, odd × even = even, even × even = even.".into(),
                    "Adding an even number preserves parity.".into(),
                    format!("The expression evaluates to an {word} number ({target_n})."),
                ];
            } else if kind == "bounds_estimation" {
                let a_down = integer(p, "a_down")?;
                let b_down = integer(p, "b_down")?;
                output.steps = vec![
                    "Rounding down provides a strict lower bound for products of positive numbers.".into(),
                    format!("{a_down} × {b_down} = {target_n}."),
                    format!("Any true product must be strictly greater than {target_n}."),
                ];
            } else {
                let p_val = integer(p, "p")?;
                output.steps = vec![
                    format!("Substitute n = {p_val} into the quadratic expression:"),
                    format!("{p_val}² − {p_val} + {p_val} = {p_val}² = {}.", p_val * p_val),
                    format!("Since {} = {p_val} × {p_val}, it is divisible by {p_val} and composite.", p_val * p_val),
                    format!("Therefore n = {p_val} disproves the conjecture as the smallest counterexample."),
                ];
            }
            output.conclusion = format!("The result is {target_n}.");
            output.success_note = "Your check is correct. Rigorous verification and counterexamples distinguish mathematical certainty from speculation.".into();
            output.visual = Some(json!({"kind": "bounds_check", "target": target_n}));
        }
        "decimals.fraction_convert" => {
            let canon = p.metadata["canonical"].as_str().ok_or("Missing canonical in metadata")?;
            if target != canon {
                return Err("Decimals fraction convert reference mismatch".into());
            }
            output.hint = "Decimals represent place values with powers-of-ten denominators (1/10, 1/100, 1/1000).".into();
            let kind = p.metadata["kind"].as_str().unwrap_or("conversion");
            if kind == "tenths" {
                let n = integer(p, "numerator")?;
                output.steps = vec![
                    format!("The fraction is {n}/10."),
                    "1/10 represents one tenth, written as 0.1.".into(),
                    format!("{n} tenths is written as 0.{n} (or simplified: {target})."),
                ];
            } else if kind == "hundredths" {
                let n = integer(p, "numerator")?;
                output.steps = vec![
                    format!("The decimal is 0.{:02}.", n),
                    format!("Two decimal places indicate hundredths: {n}/100."),
                    format!("Dividing numerator and denominator by gcd gives {target}."),
                ];
            } else if kind == "powers_of_ten_factors" {
                let n = integer(p, "numerator")?;
                let d = integer(p, "denominator")?;
                let scale = 100 / d;
                output.steps = vec![
                    format!("The fraction is {n}/{d}."),
                    format!("Scale the denominator to 100 by multiplying by {scale}: ({n} × {scale}) / ({d} × {scale}) = {}/100.", n * scale),
                    format!("{}/100 = {}.", n * scale, target),
                ];
            } else {
                let w = integer(p, "whole")?;
                let n = integer(p, "numerator")?;
                let d = integer(p, "denominator")?;
                output.steps = vec![
                    format!("The mixed number is {w} + {n}/{d}."),
                    format!("Convert the fractional part {n}/{d} to decimals: {}/{} = {}.", n, d, target),
                    format!("Add the whole number part: {w} + fraction = {target}."),
                ];
            }
            output.conclusion = format!("The exact value is {target}.");
            output.success_note = "Your conversion is exact. Base-ten decimals and rational fractions represent the same exact quantities.".into();
            output.visual = Some(json!({"kind": "decimal_grid", "target": target}));
        }
        "percentages.benchmark_calc" => {
            let target_n = integer(p, "target")?;
            if from_canonical(target) != Some(int(target_n)) {
                return Err("Percentages benchmark reference mismatch".into());
            }
            output.hint = "Percentages mean parts per hundred (p% = p/100). Use benchmarks like 10%, 25%, and 50% to decompose calculations.".into();
            let kind = p.metadata["kind"].as_str().unwrap_or("benchmark");
            if kind == "benchmark_simple" {
                let p_val = integer(p, "percent")?;
                let total = integer(p, "total")?;
                output.steps = vec![
                    format!("Find {p_val}% of {total}."),
                    format!("{p_val}% = {p_val}/100."),
                    format!("({p_val}/100) × {total} = ({p_val} × {total}) / 100 = {target_n}."),
                ];
            } else if kind == "benchmark_composite" {
                let p_val = integer(p, "percent")?;
                let total = integer(p, "total")?;
                let ten_pct = total / 10;
                let five_pct = total / 20;
                output.steps = vec![
                    format!("Decompose {p_val}% into benchmark components:"),
                    format!("10% of {total} = {ten_pct}, and 5% of {total} = {five_pct}."),
                    format!("Summing the benchmark parts gives {target_n}."),
                ];
            } else {
                let p_val = integer(p, "percent")?;
                let part = integer(p, "part")?;
                output.steps = vec![
                    format!("{p_val}% of whole = {part}."),
                    format!("Since {p_val}% = {p_val}/100, whole = ({part} × 100) / {p_val}."),
                    format!("whole = {target_n}."),
                    format!("Verify: {p_val}% of {target_n} = {part}."),
                ];
            }
            output.conclusion = format!("The result is {target_n}.");
            output.success_note = "Your percentage calculation is correct. Decomposing percentages into benchmark parts ensures fast, accurate mental arithmetic.".into();
            output.visual = Some(json!({"kind": "percent_bar", "target": target_n}));
        }
        "ratios.scaling_solve" => {
            let target_n = integer(p, "target")?;
            if from_canonical(target) != Some(int(target_n)) {
                return Err("Ratios scaling reference mismatch".into());
            }
            output.hint = "Ratios represent multiplicative comparisons. Equivalent ratios scale by multiplying both terms by the same nonzero factor k.".into();
            let a = integer(p, "term_a")?;
            let b = integer(p, "term_b")?;
            let c = integer(p, "term_c")?;
            let k = integer(p, "scale_factor")?;
            output.steps = vec![
                format!("Set up the proportion: {a}/{b} = {c}/x."),
                format!("Determine the scale factor between the first terms: k = {c} ÷ {a} = {k}."),
                format!("Multiply the second term by the scale factor: x = {b} × {k} = {target_n}."),
                format!("Verify cross products: {a} × {target_n} = {} and {b} × {c} = {}.", a * target_n, b * c),
            ];
            output.conclusion = format!("x = {target_n}.");
            output.success_note = "Your proportion solution is correct. Multiplicative scaling preserves the exact relationship between quantities.".into();
            output.visual = Some(json!({"kind": "double_number_line", "target": target_n}));
        }
        "powers.integer_eval" => {
            let kind = p.metadata.get("kind").and_then(|v| v.as_str()).unwrap_or("positive_power");
            match kind {
                "positive_power" => {
                    let b = integer(p, "base")?;
                    let e = integer(p, "exponent")?;
                    let target_n = integer(p, "target")?;
                    if from_canonical(target) != Some(int(target_n)) {
                        return Err("Power evaluation reference mismatch".into());
                    }
                    output.hint = format!("An exponent indicates repeated multiplication: {b}^{e} means {b} multiplied by itself {e} times.");
                    output.steps = vec![
                        format!("Identify the base ({b}) and the exponent ({e})."),
                        format!("Write out the repeated multiplication: {}.", (0..e).map(|_| b.to_string()).collect::<Vec<_>>().join(" × ")),
                        format!("Calculate the product: {b}^{e} = {target_n}."),
                    ];
                    output.conclusion = format!("{b}^{e} = {target_n}.");
                    output.success_note = "Your power evaluation is correct. Exponents scale multiplicatively by the base.".into();
                    output.visual = Some(json!({"kind": "powers", "base": b, "exponent": e, "result": target_n}));
                }
                "zero_power" => {
                    let b = integer(p, "base")?;
                    if target != "1" {
                        return Err("Zero power reference mismatch".into());
                    }
                    output.hint = "Any non-zero base raised to the power 0 equals 1.".into();
                    output.steps = vec![
                        format!("Observe the quotient rule: {b}^n ÷ {b}^n = {b}^(n-n) = {b}^0."),
                        format!("Any non-zero quantity divided by itself equals 1: {b}^n ÷ {b}^n = 1."),
                        format!("Therefore, by exponent laws, {b}^0 = 1."),
                    ];
                    output.conclusion = format!("{b}^0 = 1.");
                    output.success_note = "Your evaluation is correct. The zero exponent is defined by consistent quotient behavior.".into();
                    output.visual = Some(json!({"kind": "powers", "base": b, "exponent": 0, "result": 1}));
                }
                "negative_power" => {
                    let b = integer(p, "base")?;
                    let exp = integer(p, "exponent")?;
                    let den = integer(p, "denominator")?;
                    let pos_e = exp.abs();
                    if target != &format!("1/{den}") {
                        return Err("Negative power reference mismatch".into());
                    }
                    output.hint = format!("A negative exponent represents the reciprocal of the positive power: {b}^{{{exp}}} = 1 / ({b}^{{{pos_e}}}).");
                    output.steps = vec![
                        format!("Apply the negative exponent definition: {b}^{{{exp}}} = 1 / ({b}^{{{pos_e}}})."),
                        format!("Evaluate the positive power in the denominator: {b}^{{{pos_e}}} = {den}."),
                        format!("Combine to form the reciprocal fraction: 1/{den}."),
                    ];
                    output.conclusion = format!("{b}^{{{exp}}} = 1/{den}.");
                    output.success_note = "Your negative exponent evaluation is correct. Negative exponents mean repeated division or reciprocal scaling.".into();
                    output.visual = Some(json!({"kind": "powers", "base": b, "exponent": exp, "result": format!("1/{den}")}));
                }
                _ => {
                    let b = integer(p, "base")?;
                    let p_exp = integer(p, "p")?;
                    let q_exp = integer(p, "q")?;
                    let r_exp = integer(p, "r")?;
                    let net = integer(p, "net_exponent")?;
                    let target_n = integer(p, "target")?;
                    if from_canonical(target) != Some(int(target_n)) {
                        return Err("Exponent laws reference mismatch".into());
                    }
                    output.hint = "Combine exponents with the same base: add exponents for products, subtract for quotients.".into();
                    output.steps = vec![
                        format!("In the numerator, apply the product rule: {b}^{{{p_exp}}} × {b}^{{{q_exp}}} = {b}^{{{p_exp} + {q_exp}}} = {b}^{{{}}}.", p_exp + q_exp),
                        format!("Apply the quotient rule: {b}^{{{}}} ÷ {b}^{{{r_exp}}} = {b}^{{{} - {r_exp}}} = {b}^{{{net}}}.", p_exp + q_exp, p_exp + q_exp),
                        format!("Evaluate the simplified power: {b}^{{{net}}} = {target_n}."),
                    ];
                    output.conclusion = format!("Result = {target_n}.");
                    output.success_note = "Your exponent law simplification is correct. Working with exponents directly preserves exact values.".into();
                    output.visual = Some(json!({"kind": "powers", "base": b, "exponent": net, "result": target_n}));
                }
            }
        }
        "powers.scientific_notation" => {
            let kind = p.metadata.get("kind").and_then(|v| v.as_str()).unwrap_or("integer_scientific");
            match kind {
                "integer_scientific" => {
                    let m = integer(p, "significand")?;
                    let k = integer(p, "exponent")?;
                    let target_n = integer(p, "target")?;
                    if from_canonical(target) != Some(int(target_n)) {
                        return Err("Scientific notation reference mismatch".into());
                    }
                    output.hint = format!("10^{k} represents 1 followed by {k} zeros. Multiply {m} by this power of ten.");
                    output.steps = vec![
                        format!("Identify the power of ten: 10^{k} = {}.", 10i64.pow(k as u32)),
                        format!("Multiply the significand: {m} × {} = {target_n}.", 10i64.pow(k as u32)),
                    ];
                    output.conclusion = format!("{m} × 10^{k} = {target_n}.");
                    output.success_note = "Your scientific notation conversion is correct. Powers of ten indicate the magnitude shift.".into();
                    output.visual = Some(json!({"kind": "scientific", "significand": m, "exponent": k, "target": target_n}));
                }
                "decimal_scientific" => {
                    let w = integer(p, "whole")?;
                    let t = integer(p, "tenth")?;
                    let k = integer(p, "exponent")?;
                    let target_n = integer(p, "target")?;
                    if from_canonical(target) != Some(int(target_n)) {
                        return Err("Decimal scientific notation reference mismatch".into());
                    }
                    output.hint = format!("Multiplying {w}.{t} by 10^{k} shifts the decimal point {k} places to the right.");
                    output.steps = vec![
                        format!("Shift the decimal point 1 place right using one factor of ten: {w}.{t} × 10 = {}.", w * 10 + t),
                        format!("Multiply by the remaining 10^{{{}}} ({} zeros): ({} × {}) = {target_n}.", k - 1, k - 1, w * 10 + t, 10i64.pow((k - 1) as u32)),
                    ];
                    output.conclusion = format!("{w}.{t} × 10^{k} = {target_n}.");
                    output.success_note = "Your scientific notation evaluation is correct. Place-value shifts preserve the exact number.".into();
                    output.visual = Some(json!({"kind": "scientific", "whole": w, "tenth": t, "exponent": k, "target": target_n}));
                }
                _ => {
                    let a = integer(p, "a")?;
                    let b = integer(p, "b")?;
                    let p_exp = integer(p, "p")?;
                    let q_exp = integer(p, "q")?;
                    let target_n = integer(p, "target")?;
                    if from_canonical(target) != Some(int(target_n)) {
                        return Err("Multiply scientific notation reference mismatch".into());
                    }
                    output.hint = "Group the numerical coefficients and group the powers of ten separately.".into();
                    output.steps = vec![
                        format!("Regroup factors: ({a} × {b}) × (10^{{{p_exp}}} × 10^{{{q_exp}}})."),
                        format!("Multiply coefficients: {a} × {b} = {}.", a * b),
                        format!("Add exponents: 10^{{{p_exp}}} × 10^{{{q_exp}}} = 10^{{{p_exp} + {q_exp}}} = 10^{{{}}}.", p_exp + q_exp),
                        format!("Evaluate the total: {} × 10^{{{}}} = {target_n}.", a * b, p_exp + q_exp),
                    ];
                    output.conclusion = format!("Result = {target_n}.");
                    output.success_note = "Your scientific multiplication is correct. Commutativity allows grouping coefficients and powers separately.".into();
                    output.visual = Some(json!({"kind": "scientific", "a": a, "b": b, "target": target_n}));
                }
            }
        }
        "roots.radical_eval" => {
            let kind = p.metadata.get("kind").and_then(|v| v.as_str()).unwrap_or("perfect_square");
            match kind {
                "perfect_square" => {
                    let s = integer(p, "radicand")?;
                    let n = integer(p, "root")?;
                    if from_canonical(target) != Some(int(n)) {
                        return Err("Perfect square reference mismatch".into());
                    }
                    output.hint = format!("The principal square root of {s} is the non-negative integer whose square equals {s}.");
                    output.steps = vec![
                        format!("Find a number that, when multiplied by itself, produces {s}."),
                        format!("Test squares: {n} × {n} = {s}."),
                        format!("Therefore, √{s} = {n}."),
                    ];
                    output.conclusion = format!("√{s} = {n}.");
                    output.success_note = "Your square root evaluation is correct. Square roots invert the squaring operation.".into();
                    output.visual = Some(json!({"kind": "roots", "radicand": s, "root": n}));
                }
                "rational_root" => {
                    let sq_a = integer(p, "num_sq")?;
                    let sq_b = integer(p, "den_sq")?;
                    let a = integer(p, "root_num")?;
                    let b = integer(p, "root_den")?;
                    if target != &format!("{a}/{b}") {
                        return Err("Rational root reference mismatch".into());
                    }
                    output.hint = "Apply the quotient rule for radicals: √(a/b) = √a / √b.".into();
                    output.steps = vec![
                        format!("Split numerator and denominator: √({sq_a}/{sq_b}) = √{sq_a} / √{sq_b}."),
                        format!("Evaluate the numerator root: √{sq_a} = {a} (since {a}² = {sq_a})."),
                        format!("Evaluate the denominator root: √{sq_b} = {b} (since {b}² = {sq_b})."),
                        format!("Form the simplified rational root: {a}/{b}."),
                    ];
                    output.conclusion = format!("√({sq_a}/{sq_b}) = {a}/{b}.");
                    output.success_note = "Your rational root evaluation is correct. Radical rules distribute across exact fractions.".into();
                    output.visual = Some(json!({"kind": "roots", "num_sq": sq_a, "den_sq": sq_b, "root_num": a, "root_den": b}));
                }
                _ => {
                    let sq_a = integer(p, "sq_a")?;
                    let sq_b = integer(p, "sq_b")?;
                    let a = integer(p, "root_a")?;
                    let b = integer(p, "root_b")?;
                    let target_n = integer(p, "target")?;
                    if from_canonical(target) != Some(int(target_n)) {
                        return Err("Radical sum reference mismatch".into());
                    }
                    output.hint = "Evaluate each square root separately before adding. Square roots do not distribute across addition: √(A+B) ≠ √A + √B.".into();
                    output.steps = vec![
                        format!("Evaluate the first square root: √{sq_a} = {a}."),
                        format!("Evaluate the second square root: √{sq_b} = {b}."),
                        format!("Add the evaluated roots: {a} + {b} = {target_n}."),
                    ];
                    output.conclusion = format!("√{sq_a} + √{sq_b} = {target_n}.");
                    output.success_note = "Your radical sum is correct. Evaluating each radical avoids the common distributive fallacy.".into();
                    output.visual = Some(json!({"kind": "roots", "sq_a": sq_a, "sq_b": sq_b, "target": target_n}));
                }
            }
        }
        "geometry.perimeter_and_area" => {
            let kind = p.metadata.get("kind").and_then(|v| v.as_str()).unwrap_or("rectangle_perimeter");
            match kind {
                "rectangle_perimeter" => {
                    let w = integer(p, "width")?;
                    let h = integer(p, "height")?;
                    let target_n = integer(p, "target")?;
                    if from_canonical(target) != Some(int(target_n)) { return Err("Rectangle perimeter reference mismatch".into()); }
                    output.hint = "The perimeter of a rectangle is the total distance around its four boundary edges: P = 2w + 2h.".into();
                    output.steps = vec![
                        format!("Identify the dimensions: width = {w}, height = {h}."),
                        format!("Apply the perimeter formula: P = 2 × {w} + 2 × {h}."),
                        format!("Calculate: 2 × {w} = {} and 2 × {h} = {}.", 2 * w, 2 * h),
                        format!("Sum the terms: {} + {} = {target_n}.", 2 * w, 2 * h),
                    ];
                    output.conclusion = format!("Perimeter = {target_n}.");
                    output.success_note = "Your perimeter calculation is correct. Perimeter measures the one-dimensional boundary length.".into();
                    output.visual = Some(json!({"kind": "geometry_rectangle", "width": w, "height": h, "target": target_n}));
                }
                "rectangle_area" => {
                    let w = integer(p, "width")?;
                    let h = integer(p, "height")?;
                    let target_n = integer(p, "target")?;
                    if from_canonical(target) != Some(int(target_n)) { return Err("Rectangle area reference mismatch".into()); }
                    output.hint = "The area of a rectangle is the amount of two-dimensional surface it covers: Area = width × height.".into();
                    output.steps = vec![
                        format!("Identify the dimensions: width = {w}, height = {h}."),
                        format!("Apply the area formula: Area = width × height = {w} × {h}."),
                        format!("Calculate the product: {w} × {h} = {target_n}."),
                    ];
                    output.conclusion = format!("Area = {target_n}.");
                    output.success_note = "Your area calculation is correct. Area counts the square units covering the interior.".into();
                    output.visual = Some(json!({"kind": "geometry_rectangle", "width": w, "height": h, "target": target_n}));
                }
                "triangle_area" => {
                    let b = integer(p, "base")?;
                    let h = integer(p, "height")?;
                    let target_n = integer(p, "target")?;
                    if from_canonical(target) != Some(int(target_n)) { return Err("Triangle area reference mismatch".into()); }
                    output.hint = "The area of a triangle is half the area of a bounding rectangle with the same base and perpendicular height: Area = (1/2) × base × height.".into();
                    output.steps = vec![
                        format!("Identify base b = {b} and perpendicular height h = {h}."),
                        format!("Multiply base by height: {b} × {h} = {}.", b * h),
                        format!("Take half of the product: (1/2) × {} = {target_n}.", b * h),
                    ];
                    output.conclusion = format!("Area = {target_n}.");
                    output.success_note = "Your triangle area calculation is correct. Every triangle is exactly half of a parallelogram with the same base and height.".into();
                    output.visual = Some(json!({"kind": "geometry_triangle", "base": b, "height": h, "target": target_n}));
                }
                _ => {
                    let w = integer(p, "width")?;
                    let h = integer(p, "height")?;
                    let area = integer(p, "area")?;
                    let target_n = integer(p, "target")?;
                    if from_canonical(target) != Some(int(target_n)) { return Err("Missing dimension reference mismatch".into()); }
                    output.hint = "Use the given area and width to find the unknown height first (height = Area ÷ width), then calculate perimeter.".into();
                    output.steps = vec![
                        format!("Find the missing height: height = Area ÷ width = {area} ÷ {w} = {h}."),
                        format!("Now calculate perimeter: P = 2 × (width + height) = 2 × ({w} + {h})."),
                        format!("Add dimensions: {w} + {h} = {}.", w + h),
                        format!("Multiply by 2: 2 × {} = {target_n}.", w + h),
                    ];
                    output.conclusion = format!("Perimeter = {target_n}.");
                    output.success_note = "Your calculation is correct. Finding the unstated dimension links area and perimeter reasoning.".into();
                    output.visual = Some(json!({"kind": "geometry_rectangle", "width": w, "height": h, "target": target_n}));
                }
            }
        }
        "geometry.angles_triangles" => {
            let kind = p.metadata.get("kind").and_then(|v| v.as_str()).unwrap_or("triangle_angle_sum");
            match kind {
                "triangle_angle_sum" => {
                    let a = integer(p, "angle_a")?;
                    let b = integer(p, "angle_b")?;
                    let target_n = integer(p, "target")?;
                    if from_canonical(target) != Some(int(target_n)) { return Err("Triangle angle sum reference mismatch".into()); }
                    output.hint = "The interior angles of any triangle in the Euclidean plane always sum to exactly 180°.".into();
                    output.steps = vec![
                        format!("Sum the two known angles: {a}° + {b}° = {}°.", a + b),
                        format!("Subtract the sum from 180°: 180° − {}° = {target_n}°.", a + b),
                    ];
                    output.conclusion = format!("Third angle = {target_n}°.");
                    output.success_note = "Your angle calculation is correct. The triangle sum theorem follows directly from parallel lines and alternate interior angles.".into();
                    output.visual = Some(json!({"kind": "angles_triangle", "angle_a": a, "angle_b": b, "target": target_n}));
                }
                "supplementary_angle" => {
                    let theta = integer(p, "theta")?;
                    let target_n = integer(p, "target")?;
                    if from_canonical(target) != Some(int(target_n)) { return Err("Supplementary angle reference mismatch".into()); }
                    output.hint = "Supplementary angles form a straight line and sum to 180°.".into();
                    output.steps = vec![
                        format!("A straight angle is 180°: θ + {theta}° = 180°."),
                        format!("Solve for the supplement: θ = 180° − {theta}° = {target_n}°."),
                    ];
                    output.conclusion = format!("Supplementary angle = {target_n}°.");
                    output.success_note = "Your supplementary angle calculation is correct. Linear pairs always sum to a straight rotation of 180°.".into();
                    output.visual = Some(json!({"kind": "angles_linear", "theta": theta, "target": target_n}));
                }
                "complementary_angle" => {
                    let theta = integer(p, "theta")?;
                    let target_n = integer(p, "target")?;
                    if from_canonical(target) != Some(int(target_n)) { return Err("Complementary angle reference mismatch".into()); }
                    output.hint = "Complementary angles form a right angle and sum to 90°.".into();
                    output.steps = vec![
                        format!("A right angle is 90°: θ + {theta}° = 90°."),
                        format!("Solve for the complement: θ = 90° − {theta}° = {target_n}°."),
                    ];
                    output.conclusion = format!("Complementary angle = {target_n}°.");
                    output.success_note = "Your complementary angle calculation is correct. Complementary angles partition a right angle into two parts.".into();
                    output.visual = Some(json!({"kind": "angles_right", "theta": theta, "target": target_n}));
                }
                _ => {
                    let v = integer(p, "vertex_angle")?;
                    let target_n = integer(p, "target")?;
                    if from_canonical(target) != Some(int(target_n)) { return Err("Isosceles base angle reference mismatch".into()); }
                    output.hint = "In an isosceles triangle, the angles opposite the equal sides are equal. The three angles sum to 180°.".into();
                    output.steps = vec![
                        format!("Subtract the vertex angle from 180°: 180° − {v}° = {}°.", 180 - v),
                        format!("Divide the remaining angle equally between the two base angles: {}° ÷ 2 = {target_n}°.", 180 - v),
                    ];
                    output.conclusion = format!("Each base angle = {target_n}°.");
                    output.success_note = "Your isosceles angle calculation is correct. Symmetry guarantees that base angles opposite equal sides are congruent.".into();
                    output.visual = Some(json!({"kind": "angles_isosceles", "vertex": v, "target": target_n}));
                }
            }
        }
        "geometry.coordinate_distance" => {
            let kind = p.metadata.get("kind").and_then(|v| v.as_str()).unwrap_or("axis_aligned_distance");
            match kind {
                "axis_aligned_distance" => {
                    let x1 = signed_integer(p, "x1")?;
                    let y1 = signed_integer(p, "y1")?;
                    let x2 = signed_integer(p, "x2")?;
                    let y2 = signed_integer(p, "y2")?;
                    let target_n = integer(p, "target")?;
                    if from_canonical(target) != Some(int(target_n)) { return Err("Axis aligned distance reference mismatch".into()); }
                    output.hint = "Points sharing a coordinate lie along a line parallel to an axis. Distance is the absolute difference between the other coordinates.".into();
                    output.steps = vec![
                        format!("Points are ({x1}, {y1}) and ({x2}, {y2}). Since y-coordinates are equal ({y1}), distance is horizontal: |x2 − x1|."),
                        format!("Calculate: |{x2} − ({x1})| = |{}| = {target_n}.", x2 - x1),
                    ];
                    output.conclusion = format!("Distance = {target_n}.");
                    output.success_note = "Your coordinate distance calculation is correct. Distance along a coordinate axis is direct absolute displacement.".into();
                    output.visual = Some(json!({"kind": "coords_axis", "x1": x1, "y1": y1, "x2": x2, "y2": y2, "target": target_n}));
                }
                "pythagorean_distance" => {
                    let x1 = signed_integer(p, "x1")?;
                    let y1 = signed_integer(p, "y1")?;
                    let x2 = signed_integer(p, "x2")?;
                    let y2 = signed_integer(p, "y2")?;
                    let dx = integer(p, "dx")?;
                    let dy = integer(p, "dy")?;
                    let target_n = integer(p, "target")?;
                    if from_canonical(target) != Some(int(target_n)) { return Err("Pythagorean distance reference mismatch".into()); }
                    output.hint = "Apply the Pythagorean distance formula: d = √((Δx)² + (Δy)²).".into();
                    output.steps = vec![
                        format!("Find horizontal displacement: |Δx| = |{x2} − ({x1})| = {dx}."),
                        format!("Find vertical displacement: |Δy| = |{y2} − ({y1})| = {dy}."),
                        format!("Square displacements: {dx}² = {} and {dy}² = {}.", dx * dx, dy * dy),
                        format!("Sum the squares: {} + {} = {}.", dx * dx, dy * dy, (dx * dx) + (dy * dy)),
                        format!("Take the square root: √{} = {target_n}.", (dx * dx) + (dy * dy)),
                    ];
                    output.conclusion = format!("Distance = {target_n}.");
                    output.success_note = "Your Euclidean distance calculation is correct. The distance formula is the Pythagorean theorem on the coordinate grid.".into();
                    output.visual = Some(json!({"kind": "coords_pythagoras", "x1": x1, "y1": y1, "x2": x2, "y2": y2, "dx": dx, "dy": dy, "target": target_n}));
                }
                _ => {
                    let x1 = signed_integer(p, "x1")?;
                    let y1 = signed_integer(p, "y1")?;
                    let x2 = signed_integer(p, "x2")?;
                    let y2 = signed_integer(p, "y2")?;
                    let w = integer(p, "width")?;
                    let h = integer(p, "height")?;
                    let target_n = integer(p, "target")?;
                    if from_canonical(target) != Some(int(target_n)) { return Err("Coordinate rectangle area reference mismatch".into()); }
                    output.hint = "Find width = |x2 − x1| and height = |y2 − y1|, then multiply: Area = width × height.".into();
                    output.steps = vec![
                        format!("Find width from x-coordinates: |{x2} − ({x1})| = {w}."),
                        format!("Find height from y-coordinates: |{y2} − ({y1})| = {h}."),
                        format!("Multiply width by height: Area = {w} × {h} = {target_n}."),
                    ];
                    output.conclusion = format!("Area = {target_n}.");
                    output.success_note = "Your coordinate rectangle area calculation is correct. Subtracting coordinates gives side lengths on the grid.".into();
                    output.visual = Some(json!({"kind": "coords_rect", "x1": x1, "y1": y1, "x2": x2, "y2": y2, "width": w, "height": h, "target": target_n}));
                }
            }
        }
        "data.mean_median_calc" => {
            let kind = p.metadata.get("kind").and_then(|v| v.as_str()).unwrap_or("mean_balance");
            match kind {
                "mean_balance" => {
                    let v1 = integer(p, "v1")?;
                    let v2 = integer(p, "v2")?;
                    let v3 = integer(p, "v3")?;
                    let v4 = integer(p, "v4")?;
                    let v5 = integer(p, "v5")?;
                    let sum = integer(p, "sum")?;
                    let target_n = integer(p, "target")?;
                    if from_canonical(target) != Some(int(target_n)) || v1 + v2 + v3 + v4 + v5 != sum || sum != 5 * target_n {
                        return Err("Mean balance reference mismatch".into());
                    }
                    output.hint = "Add all five values together to find their total sum, then divide by 5 (the number of observations).".into();
                    output.steps = vec![
                        format!("Sum the five data values: {v1} + {v2} + {v3} + {v4} + {v5} = {sum}."),
                        format!("Divide the sum by the count of values (5): {sum} ÷ 5 = {target_n}."),
                    ];
                    output.conclusion = format!("Arithmetic mean = {target_n}.");
                    output.success_note = "Your mean calculation is correct. At the balance point, positive and negative deviations cancel to zero.".into();
                    output.visual = Some(json!({"kind": "data_mean", "values": [v1, v2, v3, v4, v5], "mean": target_n}));
                }
                "median_unsorted" => {
                    let u1 = integer(p, "u1")?;
                    let u2 = integer(p, "u2")?;
                    let u3 = integer(p, "u3")?;
                    let u4 = integer(p, "u4")?;
                    let u5 = integer(p, "u5")?;
                    let s1 = integer(p, "s1")?;
                    let s2 = integer(p, "s2")?;
                    let s3 = integer(p, "s3")?;
                    let s4 = integer(p, "s4")?;
                    let s5 = integer(p, "s5")?;
                    let target_n = integer(p, "target")?;
                    if from_canonical(target) != Some(int(target_n)) || s3 != target_n {
                        return Err("Median unsorted reference mismatch".into());
                    }
                    output.hint = "Sort the numbers from least to greatest FIRST before selecting the middle (3rd) value. Do not pick the middle number of the unsorted list.".into();
                    output.steps = vec![
                        format!("Start with the unsorted dataset: {u1}, {u2}, {u3}, {u4}, {u5}."),
                        format!("Order the values from least to greatest: {s1}, {s2}, {s3}, {s4}, {s5}."),
                        format!("Select the middle (3rd of 5) value in the sorted list: {s3}."),
                    ];
                    output.conclusion = format!("Median = {target_n}.");
                    output.success_note = "Your median is correct. Sorting first ensures the median splits the dataset into lower and upper halves.".into();
                    output.visual = Some(json!({"kind": "data_median", "sorted": [s1, s2, s3, s4, s5], "median": target_n}));
                }
                _ => {
                    let x1 = integer(p, "x1")?;
                    let x2 = integer(p, "x2")?;
                    let x3 = integer(p, "x3")?;
                    let x4 = integer(p, "x4")?;
                    let known_sum = integer(p, "known_sum")?;
                    let m = integer(p, "target_mean")?;
                    let total_needed = integer(p, "total_needed")?;
                    let target_n = integer(p, "target")?;
                    if from_canonical(target) != Some(int(target_n)) || total_needed - known_sum != target_n {
                        return Err("Missing value for mean reference mismatch".into());
                    }
                    output.hint = "If 5 scores have a mean of M, their total sum must equal 5 × M. Subtract the sum of the first 4 scores from 5 × M.".into();
                    output.steps = vec![
                        format!("Find the total sum required for 5 scores to average {m}: 5 × {m} = {total_needed}."),
                        format!("Sum the 4 known scores: {x1} + {x2} + {x3} + {x4} = {known_sum}."),
                        format!("Subtract the known sum from the required total: {total_needed} − {known_sum} = {target_n}."),
                    ];
                    output.conclusion = format!("Required 5th score = {target_n}.");
                    output.success_note = "Your missing-value calculation is correct. Reasoning via the total sum turns a mean constraint into direct subtraction.".into();
                    output.visual = Some(json!({"kind": "data_missing", "known": [x1, x2, x3, x4], "target_mean": m, "target": target_n}));
                }
            }
        }
        "counting.permutations_combinations" => {
            let kind = p.metadata.get("kind").and_then(|v| v.as_str()).unwrap_or("multiplication_principle");
            match kind {
                "multiplication_principle" => {
                    let a = integer(p, "a")?;
                    let b = integer(p, "b")?;
                    let c = integer(p, "c")?;
                    let target_n = integer(p, "target")?;
                    if from_canonical(target) != Some(int(target_n)) || a * b * c != target_n {
                        return Err("Multiplication principle reference mismatch".into());
                    }
                    output.hint = "By the fundamental counting principle, multiply the number of independent choices at each stage (do not add them).".into();
                    output.steps = vec![
                        format!("Stage 1 has {a} choices and Stage 2 has {b} choices: {a} × {b} = {} pairs.", a * b),
                        format!("Multiply by the {c} choices in Stage 3: {} × {c} = {target_n} total configurations.", a * b),
                    ];
                    output.conclusion = format!("Total configurations = {target_n}.");
                    output.success_note = "Your count is correct. Each branch of Stage 1 splits into all branches of Stage 2 and Stage 3.".into();
                    output.visual = Some(json!({"kind": "counting_tree", "a": a, "b": b, "c": c, "target": target_n}));
                }
                "ordered_permutation" => {
                    let n = integer(p, "n")?;
                    let target_n = integer(p, "target")?;
                    if from_canonical(target) != Some(int(target_n)) || n * (n - 1) * (n - 2) != target_n {
                        return Err("Permutation reference mismatch".into());
                    }
                    output.hint = "Order matters and no competitor can occupy two places: multiply n × (n − 1) × (n − 2).".into();
                    output.steps = vec![
                        format!("There are {n} choices for 1st place."),
                        format!("Once 1st place is decided, {} competitors remain for 2nd place, and {} remain for 3rd place.", n - 1, n - 2),
                        format!("Multiply the sequential choices: {n} × {} × {} = {target_n}.", n - 1, n - 2),
                    ];
                    output.conclusion = format!("P({n}, 3) = {target_n}.");
                    output.success_note = "Your permutation count is correct. Without replacement, each selection reduces the remaining pool by 1.".into();
                    output.visual = Some(json!({"kind": "counting_permutation", "n": n, "k": 3, "target": target_n}));
                }
                _ => {
                    let n = integer(p, "n")?;
                    let k = integer(p, "k")?;
                    let ord = integer(p, "ordered_count")?;
                    let fact_k = integer(p, "factorial_k")?;
                    let target_n = integer(p, "target")?;
                    if from_canonical(target) != Some(int(target_n)) || ord / fact_k != target_n {
                        return Err("Combination reference mismatch".into());
                    }
                    output.hint = "In a committee, order does NOT matter. First count the ordered selections, then divide by k! to remove duplicate orderings of the same members.".into();
                    output.steps = vec![
                        format!("Count ordered ways to pick {k} people from {n}: {ord}."),
                        format!("Each group of {k} members can be rearranged in {k}! = {fact_k} ways."),
                        format!("Divide by {fact_k} to count unordered committees: {ord} ÷ {fact_k} = {target_n}."),
                    ];
                    output.conclusion = format!("C({n}, {k}) = {target_n}.");
                    output.success_note = "Your combination count is correct. Dividing by k! collapses equivalent reorderings into a single subset.".into();
                    output.visual = Some(json!({"kind": "counting_combination", "n": n, "k": k, "target": target_n}));
                }
            }
        }
        "probability.simple_exact" => {
            let kind = p.metadata.get("kind").and_then(|v| v.as_str()).unwrap_or("single_event_probability");
            match kind {
                "single_event_probability" => {
                    let r = integer(p, "red")?;
                    let b = integer(p, "blue")?;
                    let g = integer(p, "green")?;
                    let total = integer(p, "total")?;
                    let q = crate::number::frac(r, total);
                    if from_canonical(target) != Some(q.clone()) {
                        return Err("Single event probability reference mismatch".into());
                    }
                    let canon = to_canonical(&q);
                    output.hint = "Probability is (favorable outcomes) ÷ (total outcomes in the sample space), not favorable ÷ unfavorable.".into();
                    output.steps = vec![
                        format!("Find the total number of tokens in the sample space: {r} + {b} + {g} = {total}."),
                        format!("Identify favorable outcomes (red tokens): {r}."),
                        format!("Form the exact probability ratio and simplify: {r}/{total} = {canon}."),
                    ];
                    output.conclusion = format!("P(red) = {canon}.");
                    output.success_note = "Your exact probability is correct. Every outcome in the sample space is counted in the denominator.".into();
                    output.visual = Some(json!({"kind": "prob_single", "favorable": r, "total": total, "target": canon}));
                }
                "complementary_probability" => {
                    let fav = integer(p, "event_outcomes")?;
                    let comp = integer(p, "complement_outcomes")?;
                    let total = integer(p, "total")?;
                    let q = crate::number::frac(comp, total);
                    if from_canonical(target) != Some(q.clone()) {
                        return Err("Complementary probability reference mismatch".into());
                    }
                    let canon = to_canonical(&q);
                    output.hint = "The probability that E does NOT occur is P(not E) = 1 − P(E) = (total − favorable) / total.".into();
                    output.steps = vec![
                        format!("Subtract the {fav} outcomes where E occurs from the {total} total outcomes: {total} − {fav} = {comp}."),
                        format!("Divide by the total sample space size {total} and simplify: {comp}/{total} = {canon}."),
                    ];
                    output.conclusion = format!("P(not E) = {canon}.");
                    output.success_note = "Your complementary probability is correct. P(E) + P(not E) always equals 1.".into();
                    output.visual = Some(json!({"kind": "prob_complement", "fav": fav, "comp": comp, "total": total, "target": canon}));
                }
                _ => {
                    let gold = integer(p, "gold")?;
                    let silver = integer(p, "silver")?;
                    let total = integer(p, "total")?;
                    let num = integer(p, "num")?;
                    let den = integer(p, "den")?;
                    let q = crate::number::frac(num, den);
                    if from_canonical(target) != Some(q.clone()) {
                        return Err("Two-step probability reference mismatch".into());
                    }
                    let canon = to_canonical(&q);
                    output.hint = "Without replacement, the first gold draw reduces both the remaining gold coins and the total remaining coins by 1.".into();
                    output.steps = vec![
                        format!("Total coins initially: {gold} + {silver} = {total}. Probability 1st coin is gold: {gold}/{total}."),
                        format!("After drawing 1 gold coin without replacement, {} gold coins remain out of {} total coins.", gold - 1, total - 1),
                        format!("Multiply the two stage probabilities: ({gold}/{total}) × ({}/{}) = {num}/{den} = {canon}.", gold - 1, total - 1),
                    ];
                    output.conclusion = format!("P(both gold) = {canon}.");
                    output.success_note = "Your compound probability is correct. Updating the sample space after the first draw accounts for conditional dependence.".into();
                    output.visual = Some(json!({"kind": "prob_compound", "gold": gold, "total": total, "target": canon}));
                }
            }
        }
        "algebra.expression_eval" => {
            let kind = p.metadata.get("kind").and_then(|v| v.as_str()).unwrap_or("quadratic_substitution");
            match kind {
                "quadratic_substitution" => {
                    let a = signed_integer(p, "a")?;
                    let b = signed_integer(p, "b")?;
                    let c = signed_integer(p, "c")?;
                    let x = signed_integer(p, "x")?;
                    let target_n = signed_integer(p, "target")?;
                    if from_canonical(target) != Some(int(target_n)) || a * x * x + b * x + c != target_n {
                        return Err("Quadratic substitution reference mismatch".into());
                    }
                    output.hint = "When substituting a negative number for x, wrap it in parentheses: x² = (x) × (x) is strictly positive.".into();
                    output.steps = vec![
                        format!("Substitute x = {x} in parentheses: {a}({x})² + {b}({x}) + ({c})."),
                        format!("Evaluate the square first: ({x})² = {}, so {a} × {} = {}.", x * x, x * x, a * x * x),
                        format!("Evaluate the linear term: {b} × ({x}) = {}.", b * x),
                        format!("Combine all terms: {} + ({}) + ({c}) = {target_n}.", a * x * x, b * x),
                    ];
                    output.conclusion = format!("Value = {target_n}.");
                    output.success_note = "Your evaluation is correct. Parentheses around negative inputs preserve exponent precedence.".into();
                    output.visual = Some(json!({"kind": "algebra_eval", "a": a, "b": b, "c": c, "x": x, "target": target_n}));
                }
                "two_variable_eval" => {
                    let p_coeff = signed_integer(p, "p")?;
                    let q_coeff = signed_integer(p, "q")?;
                    let x = signed_integer(p, "x")?;
                    let y = signed_integer(p, "y")?;
                    let target_n = signed_integer(p, "target")?;
                    if from_canonical(target) != Some(int(target_n)) || p_coeff * (x - y) + q_coeff * x * y != target_n {
                        return Err("Two-variable evaluation reference mismatch".into());
                    }
                    output.hint = "Be careful with x − y when y is negative: subtracting a negative number adds its magnitude.".into();
                    output.steps = vec![
                        format!("Evaluate the difference in parentheses: x − y = {x} − ({y}) = {}.", x - y),
                        format!("Multiply by {p_coeff}: {p_coeff} × {} = {}.", x - y, p_coeff * (x - y)),
                        format!("Evaluate the product term: {q_coeff} × {x} × ({y}) = {}.", q_coeff * x * y),
                        format!("Add the two parts: {} + ({}) = {target_n}.", p_coeff * (x - y), q_coeff * x * y),
                    ];
                    output.conclusion = format!("Value = {target_n}.");
                    output.success_note = "Your two-variable substitution is correct. Tracking signs inside parentheses avoids double-negative errors.".into();
                    output.visual = Some(json!({"kind": "algebra_two_var", "x": x, "y": y, "target": target_n}));
                }
                _ => {
                    let a = signed_integer(p, "a")?;
                    let b = signed_integer(p, "b")?;
                    let c = signed_integer(p, "c")?;
                    let d = signed_integer(p, "d")?;
                    let e = signed_integer(p, "e")?;
                    let f = signed_integer(p, "f")?;
                    let x0 = signed_integer(p, "x0")?;
                    let net_x = signed_integer(p, "net_x_coeff")?;
                    let net_c = signed_integer(p, "net_const")?;
                    let target_n = signed_integer(p, "target")?;
                    if from_canonical(target) != Some(int(target_n)) || net_x * x0 + net_c != target_n {
                        return Err("Distributive simplify reference mismatch".into());
                    }
                    output.hint = "Distribute the minus sign across BOTH terms of the second parenthesis: −d(ex − f) = −dex + df.".into();
                    output.steps = vec![
                        format!("Expand the first group: {a}({b}x + {c}) = {}x + {}.", a * b, a * c),
                        format!("Expand the second group carefully (negative times negative is positive): −{d}({e}x − {f}) = −{}x + {}.", d * e, d * f),
                        format!("Combine like terms: ({} − {})x + ({} + {}) = {net_x}x + {net_c}.", a * b, d * e, a * c, d * f),
                        format!("Evaluate at x = {x0}: {net_x}({x0}) + {net_c} = {} + {net_c} = {target_n}.", net_x * x0),
                    ];
                    output.conclusion = format!("Value = {target_n}.");
                    output.success_note = "Your expansion and evaluation are correct. Distributing −d across −f yields +df.".into();
                    output.visual = Some(json!({"kind": "algebra_distribute", "net_x": net_x, "net_c": net_c, "x0": x0, "target": target_n}));
                }
            }
        }
        "algebra.linear_equation_solve" => {
            let kind = p.metadata.get("kind").and_then(|v| v.as_str()).unwrap_or("two_step_linear");
            match kind {
                "two_step_linear" => {
                    let a = signed_integer(p, "a")?;
                    let b = signed_integer(p, "b")?;
                    let c = signed_integer(p, "c")?;
                    let target_n = signed_integer(p, "target")?;
                    if from_canonical(target) != Some(int(target_n)) || a * target_n + b != c {
                        return Err("Two-step linear reference mismatch".into());
                    }
                    output.hint = "Undo addition first by subtracting the constant term from both sides, then divide both sides by the coefficient of x.".into();
                    output.steps = vec![
                        format!("Subtract {b} from both sides of {a}x + {b} = {c}: {a}x = {c} − {b} = {}.", c - b),
                        format!("Divide both sides by {a}: x = {} ÷ {a} = {target_n}.", c - b),
                        format!("Check by substitution: {a}({target_n}) + {b} = {} + {b} = {c}.", a * target_n),
                    ];
                    output.conclusion = format!("x = {target_n}.");
                    output.success_note = "Your solution is correct and verified by substitution.".into();
                    output.visual = Some(json!({"kind": "algebra_linear", "a": a, "b": b, "c": c, "target": target_n}));
                }
                "variables_both_sides" => {
                    let a = signed_integer(p, "a")?;
                    let b = signed_integer(p, "b")?;
                    let c = signed_integer(p, "c")?;
                    let d = signed_integer(p, "d")?;
                    let net_coeff = signed_integer(p, "net_coeff")?;
                    let net_rhs = signed_integer(p, "net_rhs")?;
                    let target_n = signed_integer(p, "target")?;
                    if from_canonical(target) != Some(int(target_n)) || (a - c) * target_n != d - b {
                        return Err("Variables on both sides reference mismatch".into());
                    }
                    output.hint = "Collect all variable terms on the left by subtracting cx from both sides, and collect constants on the right by subtracting b from both sides.".into();
                    output.steps = vec![
                        format!("Subtract {c}x from both sides: ({a} − {c})x + {b} = {d} ⇒ {net_coeff}x + {b} = {d}."),
                        format!("Subtract {b} from both sides: {net_coeff}x = {d} − {b} = {net_rhs}."),
                        format!("Divide both sides by {net_coeff}: x = {net_rhs} ÷ {net_coeff} = {target_n}."),
                    ];
                    output.conclusion = format!("x = {target_n}.");
                    output.success_note = "Your solution is correct. Performing identical operations on both sides preserves equivalence.".into();
                    output.visual = Some(json!({"kind": "algebra_both_sides", "a": a, "b": b, "c": c, "d": d, "target": target_n}));
                }
                _ => {
                    let k = signed_integer(p, "k")?;
                    let a = signed_integer(p, "a")?;
                    let b = signed_integer(p, "b")?;
                    let c = signed_integer(p, "c")?;
                    let d = signed_integer(p, "d")?;
                    let lhs_x = signed_integer(p, "lhs_x_coeff")?;
                    let lhs_c = signed_integer(p, "lhs_const")?;
                    let net_coeff = signed_integer(p, "net_coeff")?;
                    let net_rhs = signed_integer(p, "net_rhs")?;
                    let q = crate::number::frac(net_rhs, net_coeff);
                    if from_canonical(target) != Some(q.clone()) {
                        return Err("Distributive rational linear reference mismatch".into());
                    }
                    let canon = to_canonical(&q);
                    output.hint = "First distribute the outer multiplier across BOTH terms inside the parentheses: k(ax + b) = kax + kb.".into();
                    output.steps = vec![
                        format!("Expand the left side: {k}({a}x + {b}) = {lhs_x}x + {lhs_c}."),
                        format!("Subtract {c}x and {lhs_c} from both sides: ({lhs_x} − {c})x = {d} − {lhs_c} ⇒ {net_coeff}x = {net_rhs}."),
                        format!("Divide by {net_coeff} and simplify over Q: x = {net_rhs}/{net_coeff} = {canon}."),
                    ];
                    output.conclusion = format!("x = {canon}.");
                    output.success_note = "Your exact solution is correct. Keeping rational results as exact fractions avoids rounding error.".into();
                    output.visual = Some(json!({"kind": "algebra_dist_eq", "k": k, "a": a, "b": b, "c": c, "d": d, "target": canon}));
                }
            }
        }
        "algebra.linear_system_solve" => {
            let kind = p.metadata.get("kind").and_then(|v| v.as_str()).unwrap_or("sum_difference_system");
            match kind {
                "sum_difference_system" => {
                    let s = signed_integer(p, "s")?;
                    let d = signed_integer(p, "d")?;
                    let y = signed_integer(p, "y")?;
                    let target_n = signed_integer(p, "target")?;
                    if from_canonical(target) != Some(int(target_n)) || target_n + y != s || target_n - y != d {
                        return Err("Sum-difference system reference mismatch".into());
                    }
                    output.hint = "Add the two equations (x + y = s) and (x − y = d) together: the +y and −y terms cancel out, leaving 2x = s + d.".into();
                    output.steps = vec![
                        format!("Add the left sides and right sides of the two equations: (x + y) + (x − y) = {s} + {d}."),
                        format!("Simplify: 2x = {}.", s + d),
                        format!("Divide by 2: x = {} ÷ 2 = {target_n} (and y = {y}).", s + d),
                    ];
                    output.conclusion = format!("x = {target_n}.");
                    output.success_note = "Your system solution is correct. Adding equations with opposite y-coefficients eliminates y immediately.".into();
                    output.visual = Some(json!({"kind": "algebra_system", "x": target_n, "y": y}));
                }
                "substitution_system" => {
                    let m = signed_integer(p, "m")?;
                    let b = signed_integer(p, "b")?;
                    let c = signed_integer(p, "c")?;
                    let d = signed_integer(p, "d")?;
                    let e = signed_integer(p, "e")?;
                    let comb = signed_integer(p, "combined_x_coeff")?;
                    let const_t = signed_integer(p, "const_term")?;
                    let target_n = signed_integer(p, "target")?;
                    if from_canonical(target) != Some(int(target_n)) || comb * target_n + const_t != e {
                        return Err("Substitution system reference mismatch".into());
                    }
                    output.hint = "Substitute the expression for y from the first equation (y = mx + b) into the second equation so only x remains.".into();
                    output.steps = vec![
                        format!("Replace y with ({m}x + {b}) in the second equation: {c}x + {d}({m}x + {b}) = {e}."),
                        format!("Expand and combine x terms: {c}x + {}x + {const_t} = {e} ⇒ {comb}x + {const_t} = {e}.", d * m),
                        format!("Subtract {const_t} and divide by {comb}: {comb}x = {} ⇒ x = {target_n}.", e - const_t),
                    ];
                    output.conclusion = format!("x = {target_n}.");
                    output.success_note = "Your substitution solution is correct. Expressing one variable in terms of the other reduces a 2×2 system to a single linear equation.".into();
                    output.visual = Some(json!({"kind": "algebra_system_sub", "x": target_n}));
                }
                _ => {
                    let a1 = signed_integer(p, "a1")?;
                    let b1 = signed_integer(p, "b1")?;
                    let c1 = signed_integer(p, "c1")?;
                    let a2 = signed_integer(p, "a2")?;
                    let b2 = signed_integer(p, "b2")?;
                    let c2 = signed_integer(p, "c2")?;
                    let elim_coeff = signed_integer(p, "elim_coeff")?;
                    let elim_rhs = signed_integer(p, "elim_rhs")?;
                    let target_n = signed_integer(p, "target")?;
                    if from_canonical(target) != Some(int(target_n)) || elim_coeff * target_n != elim_rhs {
                        return Err("Elimination system reference mismatch".into());
                    }
                    output.hint = "Multiply the first equation by 2 so the y-coefficients become opposites (+2b₁y and −2b₁y), then add the equations to eliminate y.".into();
                    output.steps = vec![
                        format!("Multiply the first equation ({a1}x + {b1}y = {c1}) by 2: {}x + {}y = {}.", 2 * a1, 2 * b1, 2 * c1),
                        format!("Add the second equation ({a2}x − {}y = {c2}): ({} + {a2})x = {} + {c2} ⇒ {elim_coeff}x = {elim_rhs}.", -b2, 2 * a1, 2 * c1),
                        format!("Divide by {elim_coeff}: x = {elim_rhs} ÷ {elim_coeff} = {target_n}."),
                    ];
                    output.conclusion = format!("x = {target_n}.");
                    output.success_note = "Your elimination solution is correct. Scaling equations to create opposite coefficients eliminates a variable cleanly.".into();
                    output.visual = Some(json!({"kind": "algebra_system_elim", "x": target_n}));
                }
            }
        }
        "functions.evaluate_and_range" => {
            let kind = p.metadata.get("kind").and_then(|v| v.as_str()).unwrap_or("preimage_linear");
            match kind {
                "preimage_linear" => {
                    let m = signed_integer(p, "m")?;
                    let b = signed_integer(p, "b")?;
                    let y0 = signed_integer(p, "y0")?;
                    let target_n = signed_integer(p, "target")?;
                    if from_canonical(target) != Some(int(target_n)) || m * target_n + b != y0 {
                        return Err("Linear preimage reference mismatch".into());
                    }
                    output.hint = "Finding x such that f(x) = y₀ means setting the formula mx + b equal to y₀ and solving for x (do not plug y₀ in for x).".into();
                    output.steps = vec![
                        format!("Set the function rule equal to the target output: {m}x + ({b}) = {y0}."),
                        format!("Subtract ({b}) from both sides: {m}x = {} − ({b}) = {}.", y0, y0 - b),
                        format!("Divide by {m}: x = {} ÷ {m} = {target_n}.", y0 - b),
                    ];
                    output.conclusion = format!("x = {target_n}.");
                    output.success_note = "Your pre-image input x is correct. Solving f(x) = y₀ reverses the mapping from codomain back to domain.".into();
                    output.visual = Some(json!({"kind": "func_preimage", "m": m, "b": b, "y0": y0, "target": target_n}));
                }
                "piecewise_evaluation" => {
                    let a = signed_integer(p, "a")?;
                    let b = signed_integer(p, "b")?;
                    let c = signed_integer(p, "c")?;
                    let d = signed_integer(p, "d")?;
                    let e = signed_integer(p, "e")?;
                    let x0 = signed_integer(p, "x0")?;
                    let target_n = signed_integer(p, "target")?;
                    let expected = if x0 >= c { d * x0 * x0 + e } else { a * x0 + b };
                    if from_canonical(target) != Some(int(target_n)) || expected != target_n {
                        return Err("Piecewise evaluation reference mismatch".into());
                    }
                    let branch_desc = if x0 >= c {
                        format!("Since x = {x0} satisfies {x0} ≥ {c}, select the second branch: f(x) = {d}x² − {}.", -e)
                    } else {
                        format!("Since x = {x0} satisfies {x0} < {c}, select the first branch: f(x) = {a}x + {b}.")
                    };
                    output.hint = "First check which domain inequality (x < c or x ≥ c) the input satisfies, then substitute into ONLY that branch.".into();
                    output.steps = vec![
                        branch_desc,
                        format!("Substitute x = {x0} into the active branch to obtain f({x0}) = {target_n}."),
                    ];
                    output.conclusion = format!("f({x0}) = {target_n}.");
                    output.success_note = "Your piecewise evaluation is correct. Testing the domain condition first selects the unique active rule.".into();
                    output.visual = Some(json!({"kind": "func_piecewise", "c": c, "x0": x0, "target": target_n}));
                }
                _ => {
                    let x1 = signed_integer(p, "x1")?;
                    let x2 = signed_integer(p, "x2")?;
                    let fx1 = signed_integer(p, "fx1")?;
                    let fx2 = signed_integer(p, "fx2")?;
                    let target_n = signed_integer(p, "target")?;
                    if from_canonical(target) != Some(int(target_n)) || (fx2 - fx1) != target_n * (x2 - x1) {
                        return Err("Average rate of change reference mismatch".into());
                    }
                    output.hint = "The average rate of change is the slope of the secant line: (f(x₂) − f(x₁)) ÷ (x₂ − x₁).".into();
                    output.steps = vec![
                        format!("Evaluate the function at both endpoints: f({x1}) = {fx1} and f({x2}) = {fx2}."),
                        format!("Compute the change in output: Δy = {fx2} − {fx1} = {}.", fx2 - fx1),
                        format!("Divide by the change in input Δx = {x2} − {x1} = {}: {} ÷ {} = {target_n}.", x2 - x1, fx2 - fx1, x2 - x1),
                    ];
                    output.conclusion = format!("Average rate of change = {target_n}.");
                    output.success_note = "Your difference quotient is correct. It measures the exact secant slope across the interval.".into();
                    output.visual = Some(json!({"kind": "func_secant", "x1": x1, "x2": x2, "fx1": fx1, "fx2": fx2, "target": target_n}));
                }
            }
        }
        "functions.composition_inverse" => {
            let kind = p.metadata.get("kind").and_then(|v| v.as_str()).unwrap_or("compose_linear_quadratic");
            match kind {
                "compose_linear_quadratic" => {
                    let a = signed_integer(p, "a")?;
                    let b = signed_integer(p, "b")?;
                    let c = signed_integer(p, "c")?;
                    let x0 = signed_integer(p, "x0")?;
                    let gx0 = signed_integer(p, "gx0")?;
                    let target_n = signed_integer(p, "target")?;
                    if from_canonical(target) != Some(int(target_n)) || x0 * x0 + c != gx0 || a * gx0 + b != target_n {
                        return Err("Function composition reference mismatch".into());
                    }
                    output.hint = "Work from the inside out: evaluate the inner function g(x₀) first, then feed that number into the outer function f.".into();
                    output.steps = vec![
                        format!("Evaluate the inner function g({x0}) = {x0}² + {c} = {} + {c} = {gx0}.", x0 * x0),
                        format!("Substitute u = {gx0} into the outer function f(u) = {a}u + ({b}): {a}({gx0}) + ({b}) = {target_n}."),
                    ];
                    output.conclusion = format!("f(g({x0})) = {target_n}.");
                    output.success_note = "Your composite evaluation is correct. Evaluating from inside out respects the chain of mappings.".into();
                    output.visual = Some(json!({"kind": "func_compose", "x0": x0, "gx0": gx0, "target": target_n}));
                }
                "inverse_affine_q" => {
                    let a = signed_integer(p, "a")?;
                    let b = signed_integer(p, "b")?;
                    let y0 = signed_integer(p, "y0")?;
                    let q = crate::number::frac(y0 - b, a);
                    if from_canonical(target) != Some(q.clone()) {
                        return Err("Affine inverse reference mismatch".into());
                    }
                    let canon = to_canonical(&q);
                    output.hint = "f⁻¹(y₀) is the unique input x such that f(x) = y₀. Solve ax + b = y₀ for x.".into();
                    output.steps = vec![
                        format!("Write the equation f(x) = {y0}: {a}x + {b} = {y0}."),
                        format!("Subtract {b} from both sides: {a}x = {y0} − {b} = {}.", y0 - b),
                        format!("Divide by {a} and simplify over Q: x = {}/{a} = {canon}.", y0 - b),
                    ];
                    output.conclusion = format!("f⁻¹({y0}) = {canon}.");
                    output.success_note = "Your exact inverse value is correct. The inverse function undoes the operations of f in reverse order.".into();
                    output.visual = Some(json!({"kind": "func_inverse", "a": a, "b": b, "y0": y0, "target": canon}));
                }
                _ => {
                    let a = signed_integer(p, "a")?;
                    let b = signed_integer(p, "b")?;
                    let c = signed_integer(p, "c")?;
                    let d = signed_integer(p, "d")?;
                    let k = signed_integer(p, "k")?;
                    let gx = signed_integer(p, "gx")?;
                    let target_n = signed_integer(p, "target")?;
                    if from_canonical(target) != Some(int(target_n)) || a * gx + b != k || c * target_n - d != gx {
                        return Err("Composite equation reference mismatch".into());
                    }
                    output.hint = "Peel the outer function first by solving f(u) = k for u = g(x), then solve g(x) = u for x.".into();
                    output.steps = vec![
                        format!("Let u = g(x). Solve f(u) = {a}u + {b} = {k} ⇒ {a}u = {} ⇒ u = {gx}.", k - b),
                        format!("Now solve g(x) = {c}x − {d} = {gx} ⇒ {c}x = {} ⇒ x = {target_n}.", gx + d),
                    ];
                    output.conclusion = format!("x = {target_n}.");
                    output.success_note = "Your solution to the composite equation is correct.".into();
                    output.visual = Some(json!({"kind": "func_comp_solve", "gx": gx, "target": target_n}));
                }
            }
        }
        "functions.transformations_vertex" => {
            let kind = p.metadata.get("kind").and_then(|v| v.as_str()).unwrap_or("vertex_form_coords");
            match kind {
                "vertex_form_coords" => {
                    let a = signed_integer(p, "a")?;
                    let h = signed_integer(p, "h")?;
                    let k = signed_integer(p, "k")?;
                    let target_n = signed_integer(p, "target")?;
                    if from_canonical(target) != Some(int(target_n)) || target_n != h {
                        return Err("Vertex form coordinate reference mismatch".into());
                    }
                    output.hint = "In vertex form a(x − h)² + k, the squared term (x − h)² equals 0 when x = +h.".into();
                    output.steps = vec![
                        format!("Set the squared binomial (x − {h}) equal to zero: x − {h} = 0 ⇒ x = {h}."),
                        format!("At x = {h}, g({h}) = {a}(0)² + {k} = {k}, so the vertex is ({h}, {k})."),
                    ];
                    output.conclusion = format!("x_vertex = {h}.");
                    output.success_note = "Your vertex x-coordinate is correct. Subtracting h inside f(x − h) shifts the graph h units to the right.".into();
                    output.visual = Some(json!({"kind": "func_vertex", "a": a, "h": h, "k": k}));
                }
                "point_transformation" => {
                    let x0 = signed_integer(p, "x0")?;
                    let y0 = signed_integer(p, "y0")?;
                    let a = signed_integer(p, "a")?;
                    let h = signed_integer(p, "h")?;
                    let k = signed_integer(p, "k")?;
                    let new_x = signed_integer(p, "new_x")?;
                    let target_n = signed_integer(p, "target")?;
                    if from_canonical(target) != Some(int(target_n)) || a * y0 + k != target_n {
                        return Err("Point transformation reference mismatch".into());
                    }
                    output.hint = "Substitute x = new_x into g(x) = a·f(x − h) + k, then replace f(x₀) with its given value y₀.".into();
                    output.steps = vec![
                        format!("Substitute x = {new_x}: g({new_x}) = {a}·f({new_x} − {h}) + ({k}) = {a}·f({x0}) + ({k})."),
                        format!("Since f({x0}) = {y0}, compute {a} × {y0} + ({k}) = {} + ({k}) = {target_n}.", a * y0),
                    ];
                    output.conclusion = format!("g({new_x}) = {target_n}.");
                    output.success_note = "Your transformed output is correct. Vertical stretch multiplies y₀ by a before the vertical shift k is added.".into();
                    output.visual = Some(json!({"kind": "func_point_trans", "x0": x0, "y0": y0, "new_x": new_x, "target": target_n}));
                }
                _ => {
                    let a = signed_integer(p, "a")?;
                    let h = signed_integer(p, "h")?;
                    let k = signed_integer(p, "k")?;
                    let x0 = signed_integer(p, "x0")?;
                    let d = signed_integer(p, "d")?;
                    let target_n = signed_integer(p, "target")?;
                    if from_canonical(target) != Some(int(target_n)) || a * d * d + k != target_n {
                        return Err("Constructed transformation reference mismatch".into());
                    }
                    output.hint = "Shifting right by h replaces x with (x − h); stretching vertically by a multiplies by a; shifting up by k adds k.".into();
                    output.steps = vec![
                        format!("Write the transformed function: g(x) = {a}(x − {h})² + {k}."),
                        format!("Substitute x = {x0}: g({x0}) = {a}({x0} − {h})² + {k} = {a}({d})² + {k}."),
                        format!("Evaluate: {a} × {} + {k} = {} + {k} = {target_n}.", d * d, a * d * d),
                    ];
                    output.conclusion = format!("g({x0}) = {target_n}.");
                    output.success_note = "Your formula construction and evaluation are both correct.".into();
                    output.visual = Some(json!({"kind": "func_construct", "a": a, "h": h, "k": k, "x0": x0, "target": target_n}));
                }
            }
        }
        "algebra2.polynomial_factor_roots" => {
            let kind = p.metadata.get("kind").and_then(|v| v.as_str()).unwrap_or("monic_quadratic_larger_root");
            match kind {
                "monic_quadratic_larger_root" => {
                    let r1 = signed_integer(p, "r1")?;
                    let r2 = signed_integer(p, "r2")?;
                    let sum_r = signed_integer(p, "sum_r")?;
                    let prod_r = signed_integer(p, "prod_r")?;
                    let target_n = signed_integer(p, "target")?;
                    if from_canonical(target) != Some(int(target_n)) || target_n != r2 || r1 + r2 != sum_r || r1 * r2 != prod_r {
                        return Err("Monic quadratic roots reference mismatch".into());
                    }
                    output.hint = "Find two numbers whose sum is b and whose product is c: x² − (r₁ + r₂)x + r₁r₂ = (x − r₁)(x − r₂) = 0.".into();
                    output.steps = vec![
                        format!("Find two integers that multiply to {prod_r} and add to {sum_r}: {r1} and {r2}."),
                        format!("Factor the quadratic: (x − {r1})(x − {r2}) = 0, giving roots x = {r1} and x = {r2}."),
                        format!("Select the larger root: max({r1}, {r2}) = {r2}."),
                    ];
                    output.conclusion = format!("Larger root = {r2}.");
                    output.success_note = "Your factored root is correct. Setting (x − r) = 0 yields x = +r.".into();
                    output.visual = Some(json!({"kind": "poly_roots", "r1": r1, "r2": r2}));
                }
                "remainder_theorem_eval" => {
                    let a = signed_integer(p, "a")?;
                    let b = signed_integer(p, "b")?;
                    let c = signed_integer(p, "c")?;
                    let r = signed_integer(p, "r")?;
                    let target_n = signed_integer(p, "target")?;
                    if from_canonical(target) != Some(int(target_n)) || r * r * r + a * r * r - b * r + c != target_n {
                        return Err("Remainder theorem reference mismatch".into());
                    }
                    output.hint = "By the Polynomial Remainder Theorem, the remainder when P(x) is divided by (x − r) is P(+r).".into();
                    output.steps = vec![
                        format!("Since P(x) = (x − {r})Q(x) + R, setting x = {r} gives R = P({r})."),
                        format!("Substitute x = {r}: P({r}) = {r}³ + {a}({r})² − {b}({r}) + {c}."),
                        format!("Compute: {} + {} − {} + {c} = {target_n}.", r * r * r, a * r * r, b * r),
                    ];
                    output.conclusion = format!("Remainder P({r}) = {target_n}.");
                    output.success_note = "Your remainder calculation is correct. Evaluating P(r) finds the remainder directly without long division.".into();
                    output.visual = Some(json!({"kind": "poly_remainder", "r": r, "target": target_n}));
                }
                _ => {
                    let a = signed_integer(p, "a")?;
                    let b = signed_integer(p, "b")?;
                    let q = crate::number::frac(b, a);
                    if from_canonical(target) != Some(q.clone()) {
                        return Err("Vieta sum reference mismatch".into());
                    }
                    let canon = to_canonical(&q);
                    output.hint = "For Ax² + Bx + C = 0, Vieta's formula states that the sum of the roots is r₁ + r₂ = −B / A.".into();
                    output.steps = vec![
                        format!("Identify the coefficients in {a}x² − {b}x + c = 0: A = {a} and B = −{b}."),
                        format!("Apply Vieta's sum formula: r₁ + r₂ = −B / A = −(−{b}) / {a} = {b}/{a}."),
                        format!("Simplify over Q: {b}/{a} = {canon}."),
                    ];
                    output.conclusion = format!("r₁ + r₂ = {canon}.");
                    output.success_note = "Your Vieta sum is correct. Dividing by the leading coefficient A normalizes the polynomial to monic form.".into();
                    output.visual = Some(json!({"kind": "poly_vieta", "a": a, "b": b, "target": canon}));
                }
            }
        }
        "algebra2.quadratic_discriminant" => {
            let kind = p.metadata.get("kind").and_then(|v| v.as_str()).unwrap_or("discriminant_calc");
            match kind {
                "discriminant_calc" => {
                    let a = signed_integer(p, "a")?;
                    let b = signed_integer(p, "b")?;
                    let c = signed_integer(p, "c")?;
                    let target_n = signed_integer(p, "target")?;
                    if from_canonical(target) != Some(int(target_n)) || b * b - 4 * a * c != target_n {
                        return Err("Discriminant reference mismatch".into());
                    }
                    output.hint = "Substitute a, b, and c (with its sign!) into Δ = b² − 4ac. Watch the sign when c is negative.".into();
                    output.steps = vec![
                        format!("Square b: b² = {b}² = {}.", b * b),
                        format!("Compute 4ac: 4 × {a} × ({c}) = {}.", 4 * a * c),
                        format!("Subtract: Δ = {} − ({}) = {target_n}.", b * b, 4 * a * c),
                    ];
                    output.conclusion = format!("Δ = {target_n}.");
                    output.success_note = "Your discriminant is correct. Its sign determines whether the roots are real and distinct (Δ > 0), repeated (Δ = 0), or complex conjugates (Δ < 0).".into();
                    output.visual = Some(json!({"kind": "quad_disc", "a": a, "b": b, "c": c, "target": target_n}));
                }
                "quadratic_formula_positive_root" => {
                    let a = signed_integer(p, "a")?;
                    let b = signed_integer(p, "b")?;
                    let disc = signed_integer(p, "disc")?;
                    let sqrt_disc = signed_integer(p, "sqrt_disc")?;
                    let p_num = signed_integer(p, "p")?;
                    let q = crate::number::frac(p_num, a);
                    if from_canonical(target) != Some(q.clone()) {
                        return Err("Quadratic formula positive root reference mismatch".into());
                    }
                    let canon = to_canonical(&q);
                    output.hint = "Apply x = (−b + √(b² − 4ac)) / (2a), remembering to divide by 2a (not just a or 2).".into();
                    output.steps = vec![
                        format!("Compute the discriminant: Δ = b² − 4ac = {disc}, so √Δ = {sqrt_disc}."),
                        format!("Take the positive branch: x = (−({b}) + {sqrt_disc}) / (2 × {a}) = {} / {}.", -b + sqrt_disc, 2 * a),
                        format!("Simplify the fraction over Q: x = {canon}."),
                    ];
                    output.conclusion = format!("x = {canon}.");
                    output.success_note = "Your exact root from the quadratic formula is correct.".into();
                    output.visual = Some(json!({"kind": "quad_formula", "a": a, "b": b, "target": canon}));
                }
                _ => {
                    let a = signed_integer(p, "a")?;
                    let b = signed_integer(p, "b")?;
                    let q = crate::number::frac(b * b, 4 * a);
                    if from_canonical(target) != Some(q.clone()) {
                        return Err("Tangent double root reference mismatch".into());
                    }
                    let canon = to_canonical(&q);
                    output.hint = "A quadratic has a single repeated root precisely when its discriminant Δ = b² − 4ac equals 0.".into();
                    output.steps = vec![
                        format!("Set the discriminant equal to zero: {b}² − 4({a})c = 0 ⇒ {} − {}c = 0.", b * b, 4 * a),
                        format!("Solve for c over Q: c = {} / {} = {canon}.", b * b, 4 * a),
                    ];
                    output.conclusion = format!("c = {canon}.");
                    output.success_note = "Your parameter value is correct. At Δ = 0, the parabola is tangent to the x-axis at its vertex.".into();
                    output.visual = Some(json!({"kind": "quad_tangent", "a": a, "b": b, "target": canon}));
                }
            }
        }
        "algebra2.exp_log_eval" => {
            let kind = p.metadata.get("kind").and_then(|v| v.as_str()).unwrap_or("log_exact_power");
            match kind {
                "log_exact_power" => {
                    let base = signed_integer(p, "base")?;
                    let n = signed_integer(p, "n")?;
                    let is_recip = p.metadata.get("is_reciprocal").and_then(|v| v.as_bool()).unwrap_or(false);
                    let target_n = signed_integer(p, "target")?;
                    if from_canonical(target) != Some(int(target_n)) {
                        return Err("Exact logarithm reference mismatch".into());
                    }
                    output.hint = "log_b(y) asks: 'To what exponent must we raise base b to obtain y?' Recall that 1 / b^k = b^(−k).".into();
                    output.steps = if is_recip {
                        vec![
                            format!("Express {n} as a power of {base}: {n} = {base}^{}.", -target_n),
                            format!("Use the negative exponent law: 1/{n} = {base}^({target_n})."),
                            format!("Therefore log_{base}(1/{n}) = {target_n}."),
                        ]
                    } else {
                        vec![
                            format!("Express {n} as a power of {base}: {n} = {base}^{target_n}."),
                            format!("By the inverse definition of logarithm, log_{base}({base}^{target_n}) = {target_n}."),
                        ]
                    };
                    output.conclusion = format!("Value = {target_n}.");
                    output.success_note = "Your exact logarithm is correct. A logarithm extracts the exponent to which the base is raised.".into();
                    output.visual = Some(json!({"kind": "exp_log_basic", "base": base, "n": n, "target": target_n}));
                }
                "log_laws_combine" => {
                    let base = signed_integer(p, "base")?;
                    let cap_a = signed_integer(p, "A")?;
                    let cap_b = signed_integer(p, "B")?;
                    let cap_c = signed_integer(p, "C")?;
                    let comb = signed_integer(p, "combined_arg")?;
                    let target_n = signed_integer(p, "target")?;
                    if from_canonical(target) != Some(int(target_n)) || (cap_a * cap_b) / cap_c != comb {
                        return Err("Log laws combine reference mismatch".into());
                    }
                    output.hint = "Combine into a single logarithm first: log_b(A) + log_b(B) − log_b(C) = log_b((A × B) / C).".into();
                    output.steps = vec![
                        format!("Apply the product and quotient laws of logarithms: log_{base}(({cap_a} × {cap_b}) / {cap_c})."),
                        format!("Simplify the argument inside the logarithm: {} / {cap_c} = {comb}.", cap_a * cap_b),
                        format!("Evaluate the exact power of {base}: log_{base}({comb}) = log_{base}({base}^{target_n}) = {target_n}."),
                    ];
                    output.conclusion = format!("Value = {target_n}.");
                    output.success_note = "Your logarithmic simplification is correct. Combining factors inside a single log cancels the non-power factor cleanly.".into();
                    output.visual = Some(json!({"kind": "exp_log_laws", "base": base, "comb": comb, "target": target_n}));
                }
                _ => {
                    let base = signed_integer(p, "base")?;
                    let m = signed_integer(p, "m")?;
                    let big_b = signed_integer(p, "big_b")?;
                    let a = signed_integer(p, "a")?;
                    let c = signed_integer(p, "c")?;
                    let d = signed_integer(p, "d")?;
                    let e = signed_integer(p, "e")?;
                    let net_coeff = signed_integer(p, "net_coeff")?;
                    let net_rhs = signed_integer(p, "net_rhs")?;
                    let q = crate::number::frac(net_rhs, net_coeff);
                    if from_canonical(target) != Some(q.clone()) {
                        return Err("Common base exponential reference mismatch".into());
                    }
                    let canon = to_canonical(&q);
                    output.hint = "Rewrite the right-hand base as a power of the left-hand base (B = b^m), then multiply the outer exponent by m before equating exponents.".into();
                    output.steps = vec![
                        format!("Rewrite {big_b} as {base}^{m}: ({base}^{m})^({d}x + {e}) = {base}^({m}({d}x + {e})) = {base}^({}x + {}).", m * d, m * e),
                        format!("Since the base {base} > 1 is injective, equate the exponents: {a}x + {c} = {}x + {}.", m * d, m * e),
                        format!("Solve for x over Q: {net_coeff}x = {net_rhs} ⇒ x = {canon}."),
                    ];
                    output.conclusion = format!("x = {canon}.");
                    output.success_note = "Your exponential equation solution is correct. Matching bases reduces an exponential equation to a linear equation in the exponents.".into();
                    output.visual = Some(json!({"kind": "exp_common_base", "base": base, "m": m, "target": canon}));
                }
            }
        }
        "sec_geom.slope_and_lines" => {
            let kind = p.metadata.get("kind").and_then(|v| v.as_str()).unwrap_or("slope_exact_rational");
            match kind {
                "slope_exact_rational" => {
                    let x1 = signed_integer(p, "x1")?;
                    let y1 = signed_integer(p, "y1")?;
                    let x2 = signed_integer(p, "x2")?;
                    let y2 = signed_integer(p, "y2")?;
                    let dx = signed_integer(p, "dx")?;
                    let dy = signed_integer(p, "dy")?;
                    let q = crate::number::frac(dy, dx);
                    if from_canonical(target) != Some(q.clone()) {
                        return Err("Slope exact rational reference mismatch".into());
                    }
                    let canon = to_canonical(&q);
                    output.hint = "Slope is rise over run: m = (y₂ − y₁) / (x₂ − x₁). Subtract coordinates in the same order.".into();
                    output.steps = vec![
                        format!("Compute the vertical change (rise): y₂ − y₁ = {y2} − ({y1}) = {dy}."),
                        format!("Compute the horizontal change (run): x₂ − x₁ = {x2} − ({x1}) = {dx}."),
                        format!("Divide rise by run over Q: m = {dy} / {dx} = {canon}."),
                    ];
                    output.conclusion = format!("m = {canon}.");
                    output.success_note = "Your exact slope is correct. Dividing vertical change by horizontal change gives the constant rate of inclination.".into();
                    output.visual = Some(json!({"kind": "geom_slope", "x1": x1, "y1": y1, "x2": x2, "y2": y2, "target": canon}));
                }
                "perpendicular_line_intercept" => {
                    let p_num = signed_integer(p, "p")?;
                    let q_den = signed_integer(p, "q")?;
                    let x0 = signed_integer(p, "x0")?;
                    let y0 = signed_integer(p, "y0")?;
                    let m_perp = crate::number::frac(-q_den, p_num);
                    let b_val = crate::number::frac(y0 * p_num + q_den * x0, p_num);
                    if from_canonical(target) != Some(b_val.clone()) {
                        return Err("Perpendicular line intercept reference mismatch".into());
                    }
                    let m_perp_str = to_canonical(&m_perp);
                    let canon = to_canonical(&b_val);
                    output.hint = "A line perpendicular to slope m₁ = p/q has negative-reciprocal slope m_⊥ = −q/p. Then solve y₀ = m_⊥ x₀ + b for b.".into();
                    output.steps = vec![
                        format!("Take the negative reciprocal of slope {p_num}/{q_den}: m_⊥ = {m_perp_str}."),
                        format!("Substitute ({x0}, {y0}) into b = y₀ − m_⊥ x₀ = {y0} − ({m_perp_str})({x0})."),
                        format!("Combine over common denominator {p_num}: b = ({y0}×{p_num} + {q_den}×{x0}) / {p_num} = {canon}."),
                    ];
                    output.conclusion = format!("b = {canon}.");
                    output.success_note = "Your perpendicular line intercept is correct. Negative-reciprocal slopes guarantee m₁ · m_⊥ = −1.".into();
                    output.visual = Some(json!({"kind": "geom_perp", "m_perp": m_perp_str, "x0": x0, "y0": y0, "target": canon}));
                }
                _ => {
                    let x1 = signed_integer(p, "x1")?;
                    let x2 = signed_integer(p, "x2")?;
                    let m = signed_integer(p, "m")?;
                    let n = signed_integer(p, "n")?;
                    let xp = crate::number::frac(n * x1 + m * x2, m + n);
                    if from_canonical(target) != Some(xp.clone()) {
                        return Err("Section formula reference mismatch".into());
                    }
                    let canon = to_canonical(&xp);
                    output.hint = "In the section formula x_P = (n x₁ + m x₂) / (m + n), weight A's coordinate x₁ by n and B's coordinate x₂ by m.".into();
                    output.steps = vec![
                        format!("Compute the weighted numerator: n x₁ + m x₂ = {n}({x1}) + {m}({x2}) = {}.", n * x1 + m * x2),
                        format!("Divide by the total ratio parts m + n = {m} + {n} = {}: x_P = {} / {} = {canon}.", m + n, n * x1 + m * x2, m + n),
                    ];
                    output.conclusion = format!("x_P = {canon}.");
                    output.success_note = "Your section formula coordinate is correct. Moving m/(m+n) of the way from A to B gives x₁ + (m/(m+n))(x₂ − x₁) = (n x₁ + m x₂)/(m + n).".into();
                    output.visual = Some(json!({"kind": "geom_section", "x1": x1, "x2": x2, "m": m, "n": n, "target": canon}));
                }
            }
        }
        "sec_geom.circle_equations_theorems" => {
            let kind = p.metadata.get("kind").and_then(|v| v.as_str()).unwrap_or("inscribed_angle_theorem");
            match kind {
                "inscribed_angle_theorem" => {
                    let central = signed_integer(p, "central")?;
                    let inscribed = signed_integer(p, "inscribed")?;
                    if from_canonical(target) != Some(int(inscribed)) || central != 2 * inscribed {
                        return Err("Inscribed angle reference mismatch".into());
                    }
                    output.hint = "By the Inscribed Angle Theorem, an inscribed angle ∠ACB is always half the central angle ∠AOB subtending the same arc.".into();
                    output.steps = vec![
                        format!("Identify the central angle subtending minor arc AB: ∠AOB = {central}°."),
                        format!("Halve the central angle: ∠ACB = {central}° / 2 = {inscribed}°."),
                    ];
                    output.conclusion = format!("∠ACB = {inscribed}°.");
                    output.success_note = "Your inscribed angle is correct. Splitting ∠ACB through the center O into two isosceles triangles proves ∠AOB = 2∠ACB.".into();
                    output.visual = Some(json!({"kind": "circle_inscribed", "central": central, "inscribed": inscribed}));
                }
                "circle_complete_square_radius" => {
                    let h = signed_integer(p, "h")?;
                    let k = signed_integer(p, "k")?;
                    let r_sq = signed_integer(p, "r_sq")?;
                    let f_const = signed_integer(p, "f_const")?;
                    if from_canonical(target) != Some(int(r_sq)) || h * h + k * k - f_const != r_sq {
                        return Err("Circle complete square reference mismatch".into());
                    }
                    output.hint = "Group the x and y terms, add h² = (D/2)² and k² = (E/2)² to both sides, and move the constant F to the right-hand side.".into();
                    output.steps = vec![
                        format!("Complete the square on x: x² − {}x + {h}² = (x − {h})² (adding {} to both sides).", 2 * h, h * h),
                        format!("Complete the square on y: y² − {}y + {k}² = (y − {k})² (adding {} to both sides).", 2 * k, k * k),
                        format!("Combine on the right-hand side: R² = {h}² + {k}² − ({f_const}) = {} + {} − ({f_const}) = {r_sq}.", h * h, k * k),
                    ];
                    output.conclusion = format!("R² = {r_sq}.");
                    output.success_note = "Your squared radius is correct. Completing the square reveals both the center (h, k) and R².".into();
                    output.visual = Some(json!({"kind": "circle_radius_sq", "h": h, "k": k, "r_sq": r_sq}));
                }
                _ => {
                    let a = signed_integer(p, "a")?;
                    let b = signed_integer(p, "b")?;
                    let c = signed_integer(p, "c")?;
                    let pd = crate::number::frac(a * b, c);
                    if from_canonical(target) != Some(pd.clone()) {
                        return Err("Power of a point reference mismatch".into());
                    }
                    let canon = to_canonical(&pd);
                    output.hint = "By the Intersecting Chords Theorem (Power of a Point), the products of the chord segments are equal: PA · PB = PC · PD.".into();
                    output.steps = vec![
                        format!("Multiply the segments of chord AB: PA · PB = {a} × {b} = {}.", a * b),
                        format!("Set equal to PC · PD and divide by PC = {c}: PD = {} / {c} = {canon}.", a * b),
                    ];
                    output.conclusion = format!("PD = {canon}.");
                    output.success_note = "Your chord segment length is correct. Similar triangles △PAC ∼ △PDB force PA/PC = PD/PB.".into();
                    output.visual = Some(json!({"kind": "circle_chords", "a": a, "b": b, "c": c, "target": canon}));
                }
            }
        }
        "sec_geom.planar_vectors_dot" => {
            let kind = p.metadata.get("kind").and_then(|v| v.as_str()).unwrap_or("dot_product_eval");
            match kind {
                "dot_product_eval" => {
                    let u1 = signed_integer(p, "u1")?;
                    let u2 = signed_integer(p, "u2")?;
                    let v1 = signed_integer(p, "v1")?;
                    let v2 = signed_integer(p, "v2")?;
                    let target_n = signed_integer(p, "target")?;
                    if from_canonical(target) != Some(int(target_n)) || u1 * v1 + u2 * v2 != target_n {
                        return Err("Dot product eval reference mismatch".into());
                    }
                    output.hint = "Multiply corresponding x-components and y-components, then add the two products: u · v = u₁v₁ + u₂v₂.".into();
                    output.steps = vec![
                        format!("Multiply the first components: u₁v₁ = {u1} × {v1} = {}.", u1 * v1),
                        format!("Multiply the second components: u₂v₂ = ({u2}) × {v2} = {}.", u2 * v2),
                        format!("Add the products: u · v = {} + ({}) = {target_n}.", u1 * v1, u2 * v2),
                    ];
                    output.conclusion = format!("u · v = {target_n}.");
                    output.success_note = "Your dot product is correct.".into();
                    output.visual = Some(json!({"kind": "vec_dot", "u1": u1, "u2": u2, "v1": v1, "v2": v2, "target": target_n}));
                }
                "orthogonal_vector_param" => {
                    let a = signed_integer(p, "a")?;
                    let b = signed_integer(p, "b")?;
                    let c = signed_integer(p, "c")?;
                    let q = crate::number::frac(-a * c, b);
                    if from_canonical(target) != Some(q.clone()) {
                        return Err("Orthogonal vector parameter reference mismatch".into());
                    }
                    let canon = to_canonical(&q);
                    output.hint = "Two vectors are orthogonal (perpendicular) if and only if their dot product is zero: a·c + b·k = 0.".into();
                    output.steps = vec![
                        format!("Write the orthogonality equation: ⟨{a}, {b}⟩ · ⟨{c}, k⟩ = {a}({c}) + {b}k = {} + {b}k = 0.", a * c),
                        format!("Subtract {} and divide by {b}: k = −{} / {b} = {canon}.", a * c, a * c),
                    ];
                    output.conclusion = format!("k = {canon}.");
                    output.success_note = "Your parameter value is correct. At u · v = 0, cos(θ) = 0 so the angle between the vectors is 90°.".into();
                    output.visual = Some(json!({"kind": "vec_ortho", "a": a, "b": b, "c": c, "target": canon}));
                }
                _ => {
                    let u1 = signed_integer(p, "u1")?;
                    let u2 = signed_integer(p, "u2")?;
                    let v1 = signed_integer(p, "v1")?;
                    let v2 = signed_integer(p, "v2")?;
                    let norm_v = signed_integer(p, "norm_v")?;
                    let dot = signed_integer(p, "dot")?;
                    let q = crate::number::frac(dot, norm_v);
                    if from_canonical(target) != Some(q.clone()) || u1 * v1 + u2 * v2 != dot {
                        return Err("Scalar projection reference mismatch".into());
                    }
                    let canon = to_canonical(&q);
                    output.hint = "Compute the dot product u · v first, then divide by the magnitude ‖v‖ = √(v₁² + v₂²).".into();
                    output.steps = vec![
                        format!("Compute the dot product: u · v = {u1}({v1}) + {u2}({v2}) = {dot}."),
                        format!("Verify the magnitude of v: ‖v‖ = √({v1}² + {v2}²) = {norm_v}."),
                        format!("Divide by ‖v‖ over Q: comp_v(u) = {dot} / {norm_v} = {canon}."),
                    ];
                    output.conclusion = format!("comp_v(u) = {canon}.");
                    output.success_note = "Your scalar projection is correct. Dividing by ‖v‖ projects u onto the unit direction vector v / ‖v‖.".into();
                    output.visual = Some(json!({"kind": "vec_proj", "dot": dot, "norm_v": norm_v, "target": canon}));
                }
            }
        }
        "trig.radians_unit_circle" => {
            let kind = p.metadata.get("kind").and_then(|v| v.as_str()).unwrap_or("deg_to_rad_multiplier");
            match kind {
                "deg_to_rad_multiplier" => {
                    let deg = signed_integer(p, "deg")?;
                    let q = crate::number::frac(deg, 180);
                    if from_canonical(target) != Some(q.clone()) {
                        return Err("Degree to radian reference mismatch".into());
                    }
                    let canon = to_canonical(&q);
                    output.hint = "Since 180° = π radians, divide the degree measure by 180 and reduce the fraction over Q.".into();
                    output.steps = vec![
                        format!("Use the conversion identity 1° = π / 180 radians: {deg}° = ({deg} / 180)π."),
                        format!("Reduce {deg}/180 to lowest terms over Q: k = {canon}."),
                    ];
                    output.conclusion = format!("k = {canon}.");
                    output.success_note = "Your radian multiplier is correct.".into();
                    output.visual = Some(json!({"kind": "trig_rad", "deg": deg, "target": canon}));
                }
                "quadrant_two_cosine" => {
                    let a = signed_integer(p, "a")?;
                    let b = signed_integer(p, "b")?;
                    let c = signed_integer(p, "c")?;
                    let q = crate::number::frac(-b, c);
                    if from_canonical(target) != Some(q.clone()) || a * a + b * b != c * c {
                        return Err("Quadrant II cosine reference mismatch".into());
                    }
                    let canon = to_canonical(&q);
                    output.hint = "Use cos²(θ) = 1 − sin²(θ) and remember that in Quadrant II (x < 0, y > 0), cos(θ) is negative.".into();
                    output.steps = vec![
                        format!("Apply the Pythagorean identity: cos²(θ) = 1 − ({a}/{c})² = ({c}² − {a}²) / {c}² = {b}² / {c}²."),
                        format!("Because θ lies in Quadrant II (π/2 < θ < π), cos(θ) < 0, so cos(θ) = −{b}/{c} = {canon}."),
                    ];
                    output.conclusion = format!("cos(θ) = {canon}.");
                    output.success_note = "Your Quadrant II cosine is correct. Both magnitude (from x² + y² = 1) and sign (from the quadrant) are essential.".into();
                    output.visual = Some(json!({"kind": "trig_q2", "a": a, "b": b, "c": c, "target": canon}));
                }
                _ => {
                    let omega = signed_integer(p, "omega")?;
                    let q = crate::number::frac(2, omega);
                    if from_canonical(target) != Some(q.clone()) {
                        return Err("Sinusoid period reference mismatch".into());
                    }
                    let canon = to_canonical(&q);
                    output.hint = "A cosine wave cos(B t) completes one full cycle when its argument increases by 2π, so T = 2π / B.".into();
                    output.steps = vec![
                        format!("Identify the angular frequency coefficient B = {omega}π."),
                        format!("Compute the fundamental period: T = 2π / ({omega}π) = 2 / {omega} = {canon}."),
                    ];
                    output.conclusion = format!("T = {canon}.");
                    output.success_note = "Your fundamental period is correct. Amplitude and vertical midline shift do not alter the period.".into();
                    output.visual = Some(json!({"kind": "trig_period", "omega": omega, "target": canon}));
                }
            }
        }
        "trig.identities_double_angle" => {
            let kind = p.metadata.get("kind").and_then(|v| v.as_str()).unwrap_or("pythagorean_tangent_acute");
            match kind {
                "pythagorean_tangent_acute" => {
                    let a = signed_integer(p, "a")?;
                    let b = signed_integer(p, "b")?;
                    let c = signed_integer(p, "c")?;
                    let q = crate::number::frac(a, b);
                    if from_canonical(target) != Some(q.clone()) {
                        return Err("Pythagorean tangent reference mismatch".into());
                    }
                    let canon = to_canonical(&q);
                    output.hint = "Find sin(θ) = √(1 − cos²(θ)) = a/c first, then divide sin(θ) by cos(θ) = b/c so the hypotenuse c cancels.".into();
                    output.steps = vec![
                        format!("From sin²(θ) + ({b}/{c})² = 1 and θ ∈ (0, π/2), sin(θ) = {a}/{c}."),
                        format!("Compute the ratio: tan(θ) = sin(θ) / cos(θ) = ({a}/{c}) / ({b}/{c}) = {a}/{b} = {canon}."),
                    ];
                    output.conclusion = format!("tan(θ) = {canon}.");
                    output.success_note = "Your tangent ratio is correct.".into();
                    output.visual = Some(json!({"kind": "trig_tan", "a": a, "b": b, "c": c, "target": canon}));
                }
                "double_angle_sine" => {
                    let a = signed_integer(p, "a")?;
                    let b = signed_integer(p, "b")?;
                    let c = signed_integer(p, "c")?;
                    let q = crate::number::frac(2 * a * b, c * c);
                    if from_canonical(target) != Some(q.clone()) {
                        return Err("Double angle sine reference mismatch".into());
                    }
                    let canon = to_canonical(&q);
                    output.hint = "Apply the double-angle identity sin(2θ) = 2 sin(θ) cos(θ); do not just double sin(θ).".into();
                    output.steps = vec![
                        format!("Substitute sin(θ) = {a}/{c} and cos(θ) = {b}/{c} into sin(2θ) = 2 sin(θ) cos(θ)."),
                        format!("Multiply numerators and denominators: 2 × ({a}/{c}) × ({b}/{c}) = {} / {} = {canon}.", 2 * a * b, c * c),
                    ];
                    output.conclusion = format!("sin(2θ) = {canon}.");
                    output.success_note = "Your double-angle sine calculation is correct.".into();
                    output.visual = Some(json!({"kind": "trig_double_sin", "a": a, "b": b, "c": c, "target": canon}));
                }
                _ => {
                    let a1 = signed_integer(p, "a1")?;
                    let b1 = signed_integer(p, "b1")?;
                    let c1 = signed_integer(p, "c1")?;
                    let a2 = signed_integer(p, "a2")?;
                    let b2 = signed_integer(p, "b2")?;
                    let c2 = signed_integer(p, "c2")?;
                    let num = signed_integer(p, "num")?;
                    let den = signed_integer(p, "den")?;
                    let q = crate::number::frac(num, den);
                    if from_canonical(target) != Some(q.clone()) {
                        return Err("Angle sum cosine reference mismatch".into());
                    }
                    let canon = to_canonical(&q);
                    output.hint = "Use cos(α + β) = cos(α)cos(β) − sin(α)sin(β). Note the minus sign between the two products.".into();
                    output.steps = vec![
                        format!("Compute cos(α)cos(β) = ({b1}/{c1})({b2}/{c2}) = {} / {den}.", b1 * b2),
                        format!("Compute sin(α)sin(β) = ({a1}/{c1})({a2}/{c2}) = {} / {den}.", a1 * a2),
                        format!("Subtract over the common denominator {den}: cos(α + β) = ({} − {}) / {den} = {num}/{den} = {canon}.", b1 * b2, a1 * a2),
                    ];
                    output.conclusion = format!("cos(α + β) = {canon}.");
                    output.success_note = "Your angle-sum cosine value is correct.".into();
                    output.visual = Some(json!({"kind": "trig_angle_sum", "num": num, "den": den, "target": canon}));
                }
            }
        }
        "trig.law_of_sines_cosines" => {
            let kind = p.metadata.get("kind").and_then(|v| v.as_str()).unwrap_or("inverse_trig_principal_branch");
            match kind {
                "inverse_trig_principal_branch" => {
                    let m = signed_integer(p, "m")?;
                    let n = signed_integer(p, "n")?;
                    let q = crate::number::frac(1, n);
                    if from_canonical(target) != Some(q.clone()) {
                        return Err("Inverse trig principal branch reference mismatch".into());
                    }
                    let canon = to_canonical(&q);
                    output.hint = "Because arcsin(y) only outputs angles in [−π/2, π/2], find the acute reference angle π − (mπ/n) in [−π/2, π/2] that has the same sine.".into();
                    output.steps = vec![
                        format!("Observe that {m}π/{n} lies in Quadrant II (outside [−π/2, π/2])."),
                        format!("By symmetry sin(π − θ) = sin(θ), so sin({m}π/{n}) = sin(π − {m}π/{n}) = sin(π/{n})."),
                        format!("Since π/{n} ∈ [−π/2, π/2], arcsin(sin({m}π/{n})) = (1/{n})π, giving k = {canon}."),
                    ];
                    output.conclusion = format!("k = {canon}.");
                    output.success_note = "Your principal-branch inverse sine is correct. Restricted domains make inverse trig functions well-defined.".into();
                    output.visual = Some(json!({"kind": "trig_arcsin", "m": m, "n": n, "target": canon}));
                }
                "law_of_sines_side" => {
                    let side_a = signed_integer(p, "side_a")?;
                    let p1 = signed_integer(p, "p1")?;
                    let q1 = signed_integer(p, "q1")?;
                    let p2 = signed_integer(p, "p2")?;
                    let q2 = signed_integer(p, "q2")?;
                    let sin_a = crate::number::frac(p1, q1);
                    let sin_b = crate::number::frac(p2, q2);
                    let q = crate::number::frac(side_a, 1) * sin_b / sin_a;
                    if from_canonical(target) != Some(q.clone()) {
                        return Err("Law of sines reference mismatch".into());
                    }
                    let canon = to_canonical(&q);
                    output.hint = "Solve a / sin(A) = b / sin(B) for b: multiply a by sin(B) and divide by sin(A).".into();
                    output.steps = vec![
                        format!("Rearrange the Law of Sines: b = a × sin(B) / sin(A) = {side_a} × ({p2}/{q2}) ÷ ({p1}/{q1})."),
                        format!("Multiply by the reciprocal of sin(A): b = {side_a} × ({p2}/{q2}) × ({q1}/{p1}) = {canon}."),
                    ];
                    output.conclusion = format!("b = {canon}.");
                    output.success_note = "Your Law of Sines calculation is correct.".into();
                    output.visual = Some(json!({"kind": "trig_sine_law", "side_a": side_a, "target": canon}));
                }
                _ => {
                    let a = signed_integer(p, "a")?;
                    let b = signed_integer(p, "b")?;
                    let c = signed_integer(p, "c")?;
                    let num = signed_integer(p, "num")?;
                    let den = signed_integer(p, "den")?;
                    let q = crate::number::frac(num, den);
                    if from_canonical(target) != Some(q.clone()) {
                        return Err("Law of cosines reference mismatch".into());
                    }
                    let canon = to_canonical(&q);
                    output.hint = "Rearrange c² = a² + b² − 2ab cos(C) to isolate cos(C) = (a² + b² − c²) / (2ab).".into();
                    output.steps = vec![
                        format!("Compute a² + b² − c² = {a}² + {b}² − {c}² = {} + {} − {} = {num}.", a * a, b * b, c * c),
                        format!("Compute the denominator 2ab = 2 × {a} × {b} = {den}."),
                        format!("Divide over Q: cos(C) = {num} / {den} = {canon}."),
                    ];
                    output.conclusion = format!("cos(C) = {canon}.");
                    output.success_note = "Your Law of Cosines calculation is correct. Notice that cos(C) > 0 indicates C is acute, while cos(C) < 0 indicates C is obtuse.".into();
                    output.visual = Some(json!({"kind": "trig_cosine_law", "a": a, "b": b, "c": c, "target": canon}));
                }
            }
        }
        "sequences.arithmetic_progression" => {
            let kind = p.metadata.get("kind").and_then(|v| v.as_str()).unwrap_or("arithmetic_nth_term");
            match kind {
                "arithmetic_nth_term" => {
                    let a1 = signed_integer(p, "a1")?;
                    let d = signed_integer(p, "d")?;
                    let n = signed_integer(p, "n")?;
                    let target_n = signed_integer(p, "target")?;
                    if from_canonical(target) != Some(int(target_n)) || a1 + (n - 1) * d != target_n {
                        return Err("Arithmetic nth term reference mismatch".into());
                    }
                    output.hint = "Moving from the 1st term to the n-th term takes (n − 1) steps of size d, so a_n = a₁ + (n − 1)d.".into();
                    output.steps = vec![
                        format!("Count the number of common-difference steps from a₁ to a_{n}: n − 1 = {n} − 1 = {}.", n - 1),
                        format!("Multiply the step count by d = {d}: {} × {d} = {}.", n - 1, (n - 1) * d),
                        format!("Add to the first term a₁ = {a1}: a_{n} = {a1} + {} = {target_n}.", (n - 1) * d),
                    ];
                    output.conclusion = format!("a_{n} = {target_n}.");
                    output.success_note = "Your arithmetic n-th term is correct. Counting steps (n − 1) avoids the fencepost error of adding n copies of d.".into();
                    output.visual = Some(json!({"kind": "seq_arith_nth", "a1": a1, "d": d, "n": n, "target": target_n}));
                }
                "arithmetic_series_sum" => {
                    let a1 = signed_integer(p, "a1")?;
                    let d = signed_integer(p, "d")?;
                    let n = signed_integer(p, "n")?;
                    let an = signed_integer(p, "an")?;
                    let target_n = signed_integer(p, "target")?;
                    if from_canonical(target) != Some(int(target_n)) || n * (a1 + an) / 2 != target_n {
                        return Err("Arithmetic series sum reference mismatch".into());
                    }
                    output.hint = "Find the last term a_n = a₁ + (n − 1)d first, then pair the first and last terms using Gauss's formula S_n = n(a₁ + a_n) / 2.".into();
                    output.steps = vec![
                        format!("Find the {n}-th term: a_{n} = {a1} + ({n} − 1)×{d} = {an}."),
                        format!("Sum the first and last terms: a₁ + a_{n} = {a1} + {an} = {}.", a1 + an),
                        format!("Multiply by n/2 = {n}/2 = {}: S_{n} = {} × {} = {target_n}.", n / 2, n / 2, a1 + an),
                    ];
                    output.conclusion = format!("S_{n} = {target_n}.");
                    output.success_note = "Your arithmetic series sum is correct. Reversing the sum and adding term-by-term forms n identical pairs of (a₁ + a_n).".into();
                    output.visual = Some(json!({"kind": "seq_arith_sum", "a1": a1, "an": an, "n": n, "target": target_n}));
                }
                _ => {
                    let p_idx = signed_integer(p, "p")?;
                    let q_idx = signed_integer(p, "q")?;
                    let ap = signed_integer(p, "ap")?;
                    let aq = signed_integer(p, "aq")?;
                    let step_gap = signed_integer(p, "step_gap")?;
                    let delta_val = signed_integer(p, "delta_val")?;
                    let q_val = crate::number::frac(delta_val, step_gap);
                    if from_canonical(target) != Some(q_val.clone()) || q_idx - p_idx != step_gap || aq - ap != delta_val {
                        return Err("Arithmetic distant terms reference mismatch".into());
                    }
                    let canon = to_canonical(&q_val);
                    output.hint = "Since a_q − a_p = (q − p)d, divide the value change (a_q − a_p) by the index gap (q − p).".into();
                    output.steps = vec![
                        format!("Compute the change in term value: a_{q_idx} − a_{p_idx} = {aq} − {ap} = {delta_val}."),
                        format!("Compute the number of steps between indices: {q_idx} − {p_idx} = {step_gap}."),
                        format!("Divide over Q: d = {delta_val} / {step_gap} = {canon}."),
                    ];
                    output.conclusion = format!("d = {canon}.");
                    output.success_note = "Your common difference is correct. Any arithmetic progression has constant slope d = (a_q − a_p)/(q − p).".into();
                    output.visual = Some(json!({"kind": "seq_arith_d", "p": p_idx, "q": q_idx, "ap": ap, "aq": aq, "target": canon}));
                }
            }
        }
        "sequences.geometric_and_infinite" => {
            let kind = p.metadata.get("kind").and_then(|v| v.as_str()).unwrap_or("geometric_nth_term");
            match kind {
                "geometric_nth_term" => {
                    let a = signed_integer(p, "a")?;
                    let r = signed_integer(p, "r")?;
                    let n = signed_integer(p, "n")?;
                    let target_n = signed_integer(p, "target")?;
                    if from_canonical(target) != Some(int(target_n)) || a * r.pow((n - 1) as u32) != target_n {
                        return Err("Geometric nth term reference mismatch".into());
                    }
                    output.hint = "In a geometric sequence, each step multiplies by r, so g_n = g₁ · r^(n−1) (exponentiate r first, then multiply by g₁).".into();
                    output.steps = vec![
                        format!("Compute the exponent n − 1 = {n} − 1 = {}.", n - 1),
                        format!("Evaluate the power of the common ratio: {r}^{} = {}.", n - 1, r.pow((n - 1) as u32)),
                        format!("Multiply by the first term g₁ = {a}: g_{n} = {a} × {} = {target_n}.", r.pow((n - 1) as u32)),
                    ];
                    output.conclusion = format!("g_{n} = {target_n}.");
                    output.success_note = "Your geometric n-th term is correct.".into();
                    output.visual = Some(json!({"kind": "seq_geom_nth", "a": a, "r": r, "n": n, "target": target_n}));
                }
                "geometric_finite_sum" => {
                    let a = signed_integer(p, "a")?;
                    let r = signed_integer(p, "r")?;
                    let n = signed_integer(p, "n")?;
                    let r_pow_n = signed_integer(p, "r_pow_n")?;
                    let target_n = signed_integer(p, "target")?;
                    if from_canonical(target) != Some(int(target_n)) || a * (r_pow_n - 1) / (r - 1) != target_n {
                        return Err("Geometric finite sum reference mismatch".into());
                    }
                    output.hint = "Use the finite geometric series formula S_n = a(r^n − 1) / (r − 1), remembering to divide by (r − 1).".into();
                    output.steps = vec![
                        format!("Compute r^n = {r}^{n} = {r_pow_n}, so r^n − 1 = {}.", r_pow_n - 1),
                        format!("Multiply by a = {a} and divide by r − 1 = {}: S_{n} = {a} × {} / {} = {target_n}.", r - 1, r_pow_n - 1, r - 1),
                    ];
                    output.conclusion = format!("S_{n} = {target_n}.");
                    output.success_note = "Your finite geometric series sum is correct. Subtracting S_n from r·S_n telescopes all middle terms.".into();
                    output.visual = Some(json!({"kind": "seq_geom_sum", "a": a, "r": r, "n": n, "target": target_n}));
                }
                _ => {
                    let a = signed_integer(p, "a")?;
                    let p_num = signed_integer(p, "p")?;
                    let q_den = signed_integer(p, "q")?;
                    let q_val = crate::number::frac(a * q_den, q_den - p_num);
                    if from_canonical(target) != Some(q_val.clone()) {
                        return Err("Infinite geometric sum reference mismatch".into());
                    }
                    let canon = to_canonical(&q_val);
                    output.hint = "Because |r| < 1, r^n → 0 as n → ∞, so S_∞ = a / (1 − r). Subtract r from 1 over a common denominator first.".into();
                    output.steps = vec![
                        format!("Compute the denominator 1 − r = 1 − {p_num}/{q_den} = {} / {q_den}.", q_den - p_num),
                        format!("Divide a = {a} by ({} / {q_den}): S_∞ = {a} × {q_den} / {} = {canon}.", q_den - p_num, q_den - p_num),
                    ];
                    output.conclusion = format!("S_∞ = {canon}.");
                    output.success_note = "Your infinite geometric series sum is correct.".into();
                    output.visual = Some(json!({"kind": "seq_geom_inf", "a": a, "p": p_num, "q": q_den, "target": canon}));
                }
            }
        }
        "sequences.recursion_binomial" => {
            let kind = p.metadata.get("kind").and_then(|v| v.as_str()).unwrap_or("linear_recurrence_a4");
            match kind {
                "linear_recurrence_a4" => {
                    let a1 = signed_integer(p, "a1")?;
                    let c = signed_integer(p, "c")?;
                    let d = signed_integer(p, "d")?;
                    let a2 = signed_integer(p, "a2")?;
                    let a3 = signed_integer(p, "a3")?;
                    let a4 = signed_integer(p, "a4")?;
                    if from_canonical(target) != Some(int(a4)) || c * a3 + d != a4 {
                        return Err("Linear recurrence reference mismatch".into());
                    }
                    output.hint = "Step forward iteratively one index at a time: use a₁ to find a₂, then a₂ to find a₃, and finally a₃ to find a₄.".into();
                    output.steps = vec![
                        format!("Compute a₂ = {c}a₁ + {d} = {c}({a1}) + {d} = {a2}."),
                        format!("Compute a₃ = {c}a₂ + {d} = {c}({a2}) + {d} = {a3}."),
                        format!("Compute a₄ = {c}a₃ + {d} = {c}({a3}) + {d} = {a4}."),
                    ];
                    output.conclusion = format!("a₄ = {a4}.");
                    output.success_note = "Your recursive evaluation is correct.".into();
                    output.visual = Some(json!({"kind": "seq_recurrence", "a1": a1, "a2": a2, "a3": a3, "a4": a4}));
                }
                "binomial_monic_coeff" => {
                    let n = signed_integer(p, "n")?;
                    let k = signed_integer(p, "k")?;
                    let c = signed_integer(p, "c")?;
                    let x_exp = signed_integer(p, "x_exp")?;
                    let binom = signed_integer(p, "binom")?;
                    let c_pow = signed_integer(p, "c_pow")?;
                    let target_n = signed_integer(p, "target")?;
                    if from_canonical(target) != Some(int(target_n)) || binom * c_pow != target_n {
                        return Err("Binomial monic coeff reference mismatch".into());
                    }
                    output.hint = "In (x + c)^n, the term with x^(n−k) is C(n, k) · x^(n−k) · c^k. Multiply the binomial coefficient C(n, k) by c^k.".into();
                    output.steps = vec![
                        format!("Identify k = n − {x_exp} = {n} − {x_exp} = {k}, and compute C({n}, {k}) = {binom}."),
                        format!("Compute the constant power c^k = {c}^{k} = {c_pow}."),
                        format!("Multiply the two factors: {binom} × {c_pow} = {target_n}."),
                    ];
                    output.conclusion = format!("[x^{x_exp}] = {target_n}.");
                    output.success_note = "Your binomial coefficient calculation is correct.".into();
                    output.visual = Some(json!({"kind": "seq_binom_monic", "n": n, "k": k, "c": c, "target": target_n}));
                }
                _ => {
                    let a = signed_integer(p, "a")?;
                    let b = signed_integer(p, "b")?;
                    let binom = signed_integer(p, "binom")?;
                    let a_pow = signed_integer(p, "a_pow")?;
                    let b_pow = signed_integer(p, "b_pow")?;
                    let target_n = signed_integer(p, "target")?;
                    if from_canonical(target) != Some(int(target_n)) || binom * a_pow * b_pow != target_n {
                        return Err("Binomial general coeff reference mismatch".into());
                    }
                    output.hint = "In (ax + b)⁴, the x² term is C(4, 2)(ax)²b² = 6 · a² · b² · x². Remember to square both a and b.".into();
                    output.steps = vec![
                        format!("Compute the binomial number C(4, 2) = {binom}."),
                        format!("Square both coefficients: a² = {a}² = {a_pow} and b² = {b}² = {b_pow}."),
                        format!("Multiply all three factors: {binom} × {a_pow} × {b_pow} = {target_n}."),
                    ];
                    output.conclusion = format!("[x²] = {target_n}.");
                    output.success_note = "Your general binomial expansion coefficient is correct.".into();
                    output.visual = Some(json!({"kind": "seq_binom_gen", "a": a, "b": b, "target": target_n}));
                }
            }
        }
        "calculus.limits_rational" => {
            let kind = p.metadata.get("kind").and_then(|v| v.as_str()).unwrap_or("removable_limit_quadratic");
            match kind {
                "removable_limit_quadratic" => {
                    let r = signed_integer(p, "r")?;
                    let s = signed_integer(p, "s")?;
                    let target_n = signed_integer(p, "target")?;
                    if from_canonical(target) != Some(int(target_n)) || r - s != target_n {
                        return Err("Removable limit reference mismatch".into());
                    }
                    output.hint = "Factor the numerator as (x − r)(x − s), cancel the common factor (x − r) for x ≠ r, and then substitute x = r.".into();
                    output.steps = vec![
                        format!("Factor the quadratic numerator into (x − {r})(x − ({s}))."),
                        format!("For x ≠ {r}, cancel (x − {r}) with the denominator to leave x − ({s})."),
                        format!("Evaluate the simplified expression at x = {r}: {r} − ({s}) = {target_n}."),
                    ];
                    output.conclusion = format!("lim = {target_n}.");
                    output.success_note = "Your limit evaluation is correct. Removing the 0/0 indeterminate factor reveals the continuous extension at x = r.".into();
                    output.visual = Some(json!({"kind": "calc_lim_hole", "r": r, "s": s, "target": target_n}));
                }
                "limit_at_infinity_rational" => {
                    let a = signed_integer(p, "a")?;
                    let d = signed_integer(p, "d")?;
                    let q_val = crate::number::frac(a, d);
                    if from_canonical(target) != Some(q_val.clone()) {
                        return Err("Limit at infinity reference mismatch".into());
                    }
                    let canon = to_canonical(&q_val);
                    output.hint = "Divide every term in the numerator and denominator by the highest power x². As x → ∞, the 1/x and 1/x² terms vanish, leaving the ratio of leading coefficients.".into();
                    output.steps = vec![
                        format!("Divide numerator and denominator by x² so lower-degree terms become b/x, c/x², e/x, and f/x²."),
                        format!("Take the limit as x → ∞: all reciprocal terms approach 0, leaving {a} / {d} = {canon}."),
                    ];
                    output.conclusion = format!("lim = {canon}.");
                    output.success_note = "Your limit at infinity is correct. Equal-degree rational functions approach the ratio of their leading coefficients.".into();
                    output.visual = Some(json!({"kind": "calc_lim_inf", "a": a, "d": d, "target": canon}));
                }
                _ => {
                    let a = signed_integer(p, "a")?;
                    let b = signed_integer(p, "b")?;
                    let left_lim = signed_integer(p, "left_lim")?;
                    let q_val = crate::number::frac(left_lim - b, a);
                    if from_canonical(target) != Some(q_val.clone()) || 2 * a != left_lim {
                        return Err("Continuity parameter reference mismatch".into());
                    }
                    let canon = to_canonical(&q_val);
                    output.hint = "Continuity at x = a requires f(a) = lim_(x→a) f(x). Factor x² − a² = (x − a)(x + a) to find the limit 2a, then solve a·k + b = 2a.".into();
                    output.steps = vec![
                        format!("Evaluate the limit: lim_(x→{a}) (x² − {})/(x − {a}) = lim_(x→{a}) (x + {a}) = {left_lim}.", a * a),
                        format!("Set f({a}) equal to the limit: {a}k + {b} = {left_lim} ⇒ {a}k = {}.", left_lim - b),
                        format!("Divide by {a} over Q: k = {} / {a} = {canon}.", left_lim - b),
                    ];
                    output.conclusion = format!("k = {canon}.");
                    output.success_note = "Your continuity parameter k is correct. Matching the point value f(a) to the limit heals the removable discontinuity.".into();
                    output.visual = Some(json!({"kind": "calc_continuity", "a": a, "b": b, "left_lim": left_lim, "target": canon}));
                }
            }
        }
        "calculus.derivatives_tangent" => {
            let kind = p.metadata.get("kind").and_then(|v| v.as_str()).unwrap_or("power_rule_eval");
            match kind {
                "power_rule_eval" => {
                    let a = signed_integer(p, "a")?;
                    let b = signed_integer(p, "b")?;
                    let c = signed_integer(p, "c")?;
                    let x0 = signed_integer(p, "x0")?;
                    let target_n = signed_integer(p, "target")?;
                    if from_canonical(target) != Some(int(target_n)) || 3 * a * x0 * x0 + 2 * b * x0 + c != target_n {
                        return Err("Power rule eval reference mismatch".into());
                    }
                    output.hint = "Apply d/dx(x^n) = n x^(n−1) term by term (noting that the derivative of the constant term d is 0), then substitute x = x₀.".into();
                    output.steps = vec![
                        format!("Differentiate term-by-term: f'(x) = {}x² + ({})x + {c}.", 3 * a, 2 * b),
                        format!("Substitute x = {x0}: f'({x0}) = {}({x0}²) + ({})({x0}) + {c} = {target_n}.", 3 * a, 2 * b),
                    ];
                    output.conclusion = format!("f'({x0}) = {target_n}.");
                    output.success_note = "Your derivative evaluation is correct.".into();
                    output.visual = Some(json!({"kind": "calc_deriv_eval", "a": a, "b": b, "c": c, "x0": x0, "target": target_n}));
                }
                "tangent_line_intercept" => {
                    let a = signed_integer(p, "a")?;
                    let b = signed_integer(p, "b")?;
                    let c = signed_integer(p, "c")?;
                    let x0 = signed_integer(p, "x0")?;
                    let y0 = signed_integer(p, "y0")?;
                    let m_tan = signed_integer(p, "m_tan")?;
                    let target_n = signed_integer(p, "target")?;
                    if from_canonical(target) != Some(int(target_n)) || y0 - m_tan * x0 != target_n {
                        return Err("Tangent line intercept reference mismatch".into());
                    }
                    output.hint = "Find the point y₀ = f(x₀) and the tangent slope m = f'(x₀) = 2ax₀ + b, then expand y − y₀ = m(x − x₀) to get B = y₀ − m x₀.".into();
                    output.steps = vec![
                        format!("Evaluate the point on the parabola: y₀ = f({x0}) = {a}({x0}²) + {b}({x0}) + {c} = {y0}."),
                        format!("Evaluate the tangent slope: m = f'({x0}) = 2({a})({x0}) + {b} = {m_tan}."),
                        format!("Compute the y-intercept: B = y₀ − m·x₀ = {y0} − {m_tan}({x0}) = {target_n}."),
                    ];
                    output.conclusion = format!("B = {target_n}.");
                    output.success_note = "Your tangent line y-intercept is correct.".into();
                    output.visual = Some(json!({"kind": "calc_tangent", "x0": x0, "y0": y0, "m_tan": m_tan, "target": target_n}));
                }
                _ => {
                    let a = signed_integer(p, "a")?;
                    let b = signed_integer(p, "b")?;
                    let q_val = crate::number::frac(b, 2 * a);
                    if from_canonical(target) != Some(q_val.clone()) {
                        return Err("Critical point reference mismatch".into());
                    }
                    let canon = to_canonical(&q_val);
                    output.hint = "A horizontal tangent occurs where f'(x) = 0. Differentiate f(x) = ax² − bx + c to get 2ax − b = 0 and solve for x.".into();
                    output.steps = vec![
                        format!("Differentiate the quadratic: f'(x) = {}x − {b}.", 2 * a),
                        format!("Set f'(x) = 0 and solve over Q: {}x = {b} ⇒ x = {b} / {} = {canon}.", 2 * a, 2 * a),
                    ];
                    output.conclusion = format!("x = {canon}.");
                    output.success_note = "Your critical point is correct. Notice that f'(x) = 0 recovers the parabola's vertex x = b/(2a).".into();
                    output.visual = Some(json!({"kind": "calc_crit_pt", "a": a, "b": b, "target": canon}));
                }
            }
        }
        "calculus.definite_integral_ftc" => {
            let kind = p.metadata.get("kind").and_then(|v| v.as_str()).unwrap_or("definite_integral_linear");
            match kind {
                "definite_integral_linear" => {
                    let a = signed_integer(p, "a")?;
                    let b = signed_integer(p, "b")?;
                    let t = signed_integer(p, "t")?;
                    let target_n = signed_integer(p, "target")?;
                    if from_canonical(target) != Some(int(target_n)) || a * t * t + b * t != target_n {
                        return Err("Definite integral linear reference mismatch".into());
                    }
                    output.hint = "Find an antiderivative F(x) = ax² + bx of f(x) = 2ax + b, then evaluate F(t) − F(0) by the Fundamental Theorem of Calculus.".into();
                    output.steps = vec![
                        format!("Antidifferentiate ({}x + {b}): F(x) = {a}x² + {b}x.", 2 * a),
                        format!("Apply FTC from 0 to {t}: F({t}) − F(0) = {a}({t}²) + {b}({t}) − 0 = {} + {} = {target_n}.", a * t * t, b * t),
                    ];
                    output.conclusion = format!("∫ = {target_n}.");
                    output.success_note = "Your definite integral is correct.".into();
                    output.visual = Some(json!({"kind": "calc_int_linear", "a": a, "b": b, "t": t, "target": target_n}));
                }
                "definite_integral_quadratic_q" => {
                    let a = signed_integer(p, "a")?;
                    let b = signed_integer(p, "b")?;
                    let c = signed_integer(p, "c")?;
                    let b_cubed = signed_integer(p, "b_cubed")?;
                    let q_val = crate::number::frac(a * b_cubed + 3 * c * b, 3);
                    if from_canonical(target) != Some(q_val.clone()) {
                        return Err("Definite integral quadratic reference mismatch".into());
                    }
                    let canon = to_canonical(&q_val);
                    output.hint = "Use the reverse power rule ∫ x² dx = x³/3 and ∫ c dx = cx, then combine over a common denominator of 3.".into();
                    output.steps = vec![
                        format!("Antidifferentiate ({a}x² + {c}): F(x) = ({a}/3)x³ + {c}x."),
                        format!("Evaluate at x = {b}: F({b}) = ({a} × {b_cubed})/3 + {} = ({} + {}) / 3 = {canon}.", c * b, a * b_cubed, 3 * c * b),
                    ];
                    output.conclusion = format!("∫ = {canon}.");
                    output.success_note = "Your exact rational definite integral is correct.".into();
                    output.visual = Some(json!({"kind": "calc_int_quad", "a": a, "b": b, "c": c, "target": canon}));
                }
                _ => {
                    let s0 = signed_integer(p, "s0")?;
                    let a = signed_integer(p, "a")?;
                    let b = signed_integer(p, "b")?;
                    let t = signed_integer(p, "t")?;
                    let disp = signed_integer(p, "disp")?;
                    let target_n = signed_integer(p, "target")?;
                    if from_canonical(target) != Some(int(target_n)) || s0 + disp != target_n {
                        return Err("Net change theorem reference mismatch".into());
                    }
                    output.hint = "Integrate v(t) from 0 to t to find the net displacement, then add the initial position s(0).".into();
                    output.steps = vec![
                        format!("Integrate velocity: ∫₀^{t} ({}t² + {}t) dt = [{a}t³ + {b}t²]₀^{t} = {disp}.", 3 * a, 2 * b),
                        format!("Add the initial position s(0) = {s0}: s({t}) = {s0} + {disp} = {target_n}."),
                    ];
                    output.conclusion = format!("s({t}) = {target_n}.");
                    output.success_note = "Your Net Change Theorem calculation is correct. Final position equals initial position plus net displacement.".into();
                    output.visual = Some(json!({"kind": "calc_net_change", "s0": s0, "disp": disp, "t": t, "target": target_n}));
                }
            }
        }
        "proof.sets_and_cardinality" => {
            let kind = p.metadata.get("kind").and_then(|v| v.as_str()).unwrap_or("two_set_inclusion_exclusion");
            match kind {
                "two_set_inclusion_exclusion" => {
                    let card_a = signed_integer(p, "card_a")?;
                    let card_b = signed_integer(p, "card_b")?;
                    let cap_ab = signed_integer(p, "cap_ab")?;
                    let target_n = signed_integer(p, "target")?;
                    if from_canonical(target) != Some(int(target_n)) || card_a + card_b - cap_ab != target_n {
                        return Err("Two-set inclusion-exclusion reference mismatch".into());
                    }
                    output.hint = "Adding |A| + |B| counts every element in the overlap A ∩ B twice; subtract |A ∩ B| once so each element in A ∪ B is counted exactly once.".into();
                    output.steps = vec![
                        format!("Sum the individual cardinalities: |A| + |B| = {card_a} + {card_b} = {}.", card_a + card_b),
                        format!("Subtract the overlap |A ∩ B| = {cap_ab}: |A ∪ B| = {} − {cap_ab} = {target_n}.", card_a + card_b),
                    ];
                    output.conclusion = format!("|A ∪ B| = {target_n}.");
                    output.success_note = "Your Inclusion-Exclusion calculation is correct.".into();
                    output.visual = Some(json!({"kind": "proof_sets_2", "a": card_a, "b": card_b, "ab": cap_ab, "target": target_n}));
                }
                "nonempty_proper_subsets" => {
                    let n = signed_integer(p, "n")?;
                    let pow_n = signed_integer(p, "pow_n")?;
                    let target_n = signed_integer(p, "target")?;
                    if from_canonical(target) != Some(int(target_n)) || pow_n - 2 != target_n {
                        return Err("Nonempty proper subsets reference mismatch".into());
                    }
                    output.hint = "The power set P(A) has 2^n subsets in total. Exclude the empty set ∅ (non-empty) and the full set A itself (proper), subtracting 2.".into();
                    output.steps = vec![
                        format!("Compute the total number of subsets in P(A): 2^{n} = {pow_n}."),
                        format!("Subtract 2 (excluding ∅ and A): {pow_n} − 2 = {target_n}."),
                    ];
                    output.conclusion = format!("2^{n} − 2 = {target_n}.");
                    output.success_note = "Your non-empty proper subset count is correct.".into();
                    output.visual = Some(json!({"kind": "proof_power_set", "n": n, "pow_n": pow_n, "target": target_n}));
                }
                _ => {
                    let single_sum = signed_integer(p, "single_sum")?;
                    let pair_sum = signed_integer(p, "pair_sum")?;
                    let abc = signed_integer(p, "abc")?;
                    let target_n = signed_integer(p, "target")?;
                    if from_canonical(target) != Some(int(target_n)) || single_sum - pair_sum + abc != target_n {
                        return Err("Three-set inclusion-exclusion reference mismatch".into());
                    }
                    output.hint = "Add the 3 single sets, subtract the 3 pairwise intersections, and add back the triple intersection |A ∩ B ∩ C| (which was added 3 times and subtracted 3 times).".into();
                    output.steps = vec![
                        format!("Sum the single set sizes: |A| + |B| + |C| = {single_sum}."),
                        format!("Subtract the pairwise intersections: {single_sum} − {pair_sum} = {}.", single_sum - pair_sum),
                        format!("Add back the triple intersection |A ∩ B ∩ C| = {abc}: {} + {abc} = {target_n}.", single_sum - pair_sum),
                    ];
                    output.conclusion = format!("|A ∪ B ∪ C| = {target_n}.");
                    output.success_note = "Your 3-set Inclusion-Exclusion calculation is correct. Since 1 − 3 + 3 − 1 = 0, adding |A ∩ B ∩ C| restores the triple overlap count to 1.".into();
                    output.visual = Some(json!({"kind": "proof_sets_3", "single": single_sum, "pair": pair_sum, "abc": abc, "target": target_n}));
                }
            }
        }
        "proof.logic_truth_quantifiers" => {
            let kind = p.metadata.get("kind").and_then(|v| v.as_str()).unwrap_or("finite_domain_implication_count");
            match kind {
                "finite_domain_implication_count" => {
                    let a = signed_integer(p, "a")?;
                    let ab = signed_integer(p, "ab")?;
                    let n_max = signed_integer(p, "n_max")?;
                    let count_p = signed_integer(p, "count_p")?;
                    let count_q = signed_integer(p, "count_q")?;
                    let false_count = signed_integer(p, "false_count")?;
                    let target_n = signed_integer(p, "target")?;
                    if from_canonical(target) != Some(int(target_n)) || n_max - false_count != target_n {
                        return Err("Finite domain implication reference mismatch".into());
                    }
                    output.hint = "An implication P(x) ⇒ Q(x) is false ONLY when P(x) is true and Q(x) is false. Count those counterexamples and subtract from |D|.".into();
                    output.steps = vec![
                        format!("Count x ∈ D where hypothesis P(x) ({a} | x) is true: {n_max}/{a} = {count_p} elements."),
                        format!("Count x ∈ D where both P(x) and Q(x) ({ab} | x) are true: {n_max}/{ab} = {count_q} elements."),
                        format!("Subtract the {false_count} falsifying elements ({count_p} − {count_q}) from |D| = {n_max}: {n_max} − {false_count} = {target_n}."),
                    ];
                    output.conclusion = format!("{target_n} elements.");
                    output.success_note = "Your implication truth count is correct. Every x where P(x) is false makes P(x) ⇒ Q(x) vacuously true.".into();
                    output.visual = Some(json!({"kind": "proof_implication", "n_max": n_max, "false_count": false_count, "target": target_n}));
                }
                "quantifier_negation_threshold" => {
                    let a = signed_integer(p, "a")?;
                    let b = signed_integer(p, "b")?;
                    let c = signed_integer(p, "c")?;
                    let k = signed_integer(p, "k")?;
                    if from_canonical(target) != Some(int(k)) || a * k + b != c {
                        return Err("Quantifier negation reference mismatch".into());
                    }
                    output.hint = "Negating '∀x, ax + b < c' flips ∀ to ∃ and negates strict '<' into weak '≥': solve ax + b ≥ c.".into();
                    output.steps = vec![
                        format!("Write the negated predicate: {a}x + {b} ≥ {c}."),
                        format!("Subtract {b} and divide by {a}: {a}x ≥ {} ⇒ x ≥ {k}, so the smallest integer witness is x = {k}.", c - b),
                    ];
                    output.conclusion = format!("x_min = {k}.");
                    output.success_note = "Your negated quantifier witness is correct. Because the negation of '<' is '≥' (not '>'), the boundary value x = k is already a valid counterexample.".into();
                    output.visual = Some(json!({"kind": "proof_quant_neg", "a": a, "b": b, "c": c, "target": k}));
                }
                _ => {
                    let n = signed_integer(p, "n")?;
                    let k = signed_integer(p, "k")?;
                    let total_rows = signed_integer(p, "total_rows")?;
                    let free_vars = signed_integer(p, "free_vars")?;
                    let false_rows = signed_integer(p, "false_rows")?;
                    let target_n = signed_integer(p, "target")?;
                    if from_canonical(target) != Some(int(target_n)) || total_rows - false_rows != target_n {
                        return Err("Truth table implication reference mismatch".into());
                    }
                    output.hint = "The implication (P₁ ∧ ... ∧ P_k) ⇒ P_(k+1) is false only when P₁=...=P_k=T and P_(k+1)=F, leaving n − (k + 1) unconstrained variables.".into();
                    output.steps = vec![
                        format!("Total rows in an {n}-variable truth table: 2^{n} = {total_rows}."),
                        format!("Falsifying rows fix {k} + 1 variables and leave {free_vars} free variables: 2^{free_vars} = {false_rows} false rows."),
                        format!("Subtract from total rows: {total_rows} − {false_rows} = {target_n} true rows."),
                    ];
                    output.conclusion = format!("{target_n} rows.");
                    output.success_note = "Your truth-table row count is correct.".into();
                    output.visual = Some(json!({"kind": "proof_truth_rows", "total": total_rows, "false_rows": false_rows, "target": target_n}));
                }
            }
        }
        "proof.relations_and_bijections" => {
            let kind = p.metadata.get("kind").and_then(|v| v.as_str()).unwrap_or("equivalence_class_rep");
            match kind {
                "equivalence_class_rep" => {
                    let a = signed_integer(p, "a")?;
                    let m = signed_integer(p, "m")?;
                    let q = signed_integer(p, "q")?;
                    let r = signed_integer(p, "r")?;
                    if from_canonical(target) != Some(int(r)) || m * q + r != a {
                        return Err("Equivalence class representative reference mismatch".into());
                    }
                    output.hint = "Add a big enough multiple of the modulus m to the negative integer a so the result lies in {0, 1, ..., m − 1}.".into();
                    output.steps = vec![
                        format!("Express {a} using the Euclidean division algorithm modulo {m}: {a} = ({q})({m}) + {r}."),
                        format!("Since 0 ≤ {r} < {m}, the canonical representative of [{a}]_{m} is r = {r}."),
                    ];
                    output.conclusion = format!("r = {r}.");
                    output.success_note = "Your equivalence class representative is correct.".into();
                    output.visual = Some(json!({"kind": "proof_mod_rep", "a": a, "m": m, "r": r}));
                }
                "injective_functions_count" => {
                    let k = signed_integer(p, "k")?;
                    let n = signed_integer(p, "n")?;
                    let target_n = signed_integer(p, "target")?;
                    if from_canonical(target) != Some(int(target_n)) {
                        return Err("Injective functions reference mismatch".into());
                    }
                    output.hint = "An injection maps the 1st domain element to any of n choices, the 2nd to any of (n − 1) remaining choices, and so on: n(n − 1)...(n − k + 1).".into();
                    output.steps = vec![
                        format!("Choose distinct images in B sequentially for the {k} elements of A."),
                        format!("Multiply the falling factorial factors starting at {n}: P({n}, {k}) = {target_n}."),
                    ];
                    output.conclusion = format!("{target_n} injections.");
                    output.success_note = "Your injective function count is correct.".into();
                    output.visual = Some(json!({"kind": "proof_injections", "k": k, "n": n, "target": target_n}));
                }
                _ => {
                    let s1 = signed_integer(p, "s1")?;
                    let s2 = signed_integer(p, "s2")?;
                    let s3 = signed_integer(p, "s3")?;
                    let target_n = signed_integer(p, "target")?;
                    if from_canonical(target) != Some(int(target_n)) || s1 * s1 + s2 * s2 + s3 * s3 != target_n {
                        return Err("Equivalence relation pairs reference mismatch".into());
                    }
                    output.hint = "Each equivalence class C_i of size s_i contributes every ordered pair in C_i × C_i, which is s_i² ordered pairs.".into();
                    output.steps = vec![
                        format!("Count ordered pairs in each class block: {s1}² = {}, {s2}² = {}, and {s3}² = {}.", s1 * s1, s2 * s2, s3 * s3),
                        format!("Sum across the disjoint blocks: {} + {} + {} = {target_n}.", s1 * s1, s2 * s2, s3 * s3),
                    ];
                    output.conclusion = format!("|R| = {target_n}.");
                    output.success_note = "Your equivalence relation cardinality is correct. Every equivalence relation is a disjoint union of Cartesian squares C_i × C_i.".into();
                    output.visual = Some(json!({"kind": "proof_equiv_pairs", "s1": s1, "s2": s2, "s3": s3, "target": target_n}));
                }
            }
        }
        "proof.direct_and_contrapositive" => {
            let kind = p.metadata.get("kind").and_then(|v| v.as_str()).unwrap_or("direct_parity_witness");
            match kind {
                "direct_parity_witness" => {
                    let k = signed_integer(p, "k")?;
                    let n = signed_integer(p, "n")?;
                    let target_n = signed_integer(p, "target")?;
                    if from_canonical(target) != Some(int(target_n)) || k * (k + 1) != target_n {
                        return Err("Direct parity witness reference mismatch".into());
                    }
                    output.hint = "Expand n² − 1 = (2k + 1)² − 1 = 4k² + 4k = 4k(k + 1), so the witness M in n² − 1 = 4M is M = k(k + 1).".into();
                    output.steps = vec![
                        format!("Compute n² − 1 = {n}² − 1 = {}.", n * n - 1),
                        format!("Factor out 4: {} / 4 = {k}({k} + 1) = {target_n}.", n * n - 1),
                    ];
                    output.conclusion = format!("M = {target_n}.");
                    output.success_note = "Your existential divisibility witness M is correct.".into();
                    output.visual = Some(json!({"kind": "proof_direct_witness", "k": k, "n": n, "target": target_n}));
                }
                "contrapositive_bound_proof" => {
                    let a = signed_integer(p, "a")?;
                    let b = signed_integer(p, "b")?;
                    let k = signed_integer(p, "k")?;
                    let target_n = signed_integer(p, "target")?;
                    if from_canonical(target) != Some(int(target_n)) || a * (k - 1) + b != target_n {
                        return Err("Contrapositive bound reference mismatch".into());
                    }
                    output.hint = "For an integer n, the negation of n ≥ k is n ≤ k − 1. Substitute n = k − 1 into a·n + b.".into();
                    output.steps = vec![
                        format!("Identify the largest integer satisfying n < {k}: n = {} = {}.", k, k - 1),
                        format!("Evaluate the linear expression at n = {}: {a}({}) + {b} = {} + {b} = {target_n}.", k - 1, k - 1, a * (k - 1)),
                    ];
                    output.conclusion = format!("M = {target_n}.");
                    output.success_note = "Your contrapositive threshold is correct.".into();
                    output.visual = Some(json!({"kind": "proof_contrapositive", "a": a, "b": b, "k": k, "target": target_n}));
                }
                _ => {
                    let m = signed_integer(p, "m")?;
                    let k = signed_integer(p, "k")?;
                    let target_n = signed_integer(p, "target")?;
                    if from_canonical(target) != Some(int(target_n)) || m * (k - 1) + 1 != target_n {
                        return Err("Pigeonhole contradiction reference mismatch".into());
                    }
                    output.hint = "If every box has at most k − 1 items, m boxes hold at most m(k − 1) items; adding 1 more item forces a contradiction.".into();
                    output.steps = vec![
                        format!("Compute the maximum items without any box reaching {k}: {m} × ({k} − 1) = {}.", m * (k - 1)),
                        format!("Add 1 to force at least one box to hold {k} items: {} + 1 = {target_n}.", m * (k - 1)),
                    ];
                    output.conclusion = format!("N_min = {target_n}.");
                    output.success_note = "Your Pigeonhole Principle threshold is correct.".into();
                    output.visual = Some(json!({"kind": "proof_pigeonhole", "m": m, "k": k, "target": target_n}));
                }
            }
        }
        "proof.mathematical_induction" => {
            let kind = p.metadata.get("kind").and_then(|v| v.as_str()).unwrap_or("induction_step_difference");
            match kind {
                "induction_step_difference" => {
                    let a = signed_integer(p, "a")?;
                    let k = signed_integer(p, "k")?;
                    let target_n = signed_integer(p, "target")?;
                    if from_canonical(target) != Some(int(target_n)) || a * (2 * k + 1) != target_n {
                        return Err("Induction step difference reference mismatch".into());
                    }
                    output.hint = "The inductive step adds the (k + 1)-th term T_(k+1) = S_(k+1) − S_k = a(k + 1)² − ak² = a(2k + 1).".into();
                    output.steps = vec![
                        format!("Substitute i = k + 1 = {} into the general term {a}(2i − 1): {a}(2({k}) + 1).", k + 1),
                        format!("Evaluate: {a} × {} = {target_n}.", 2 * k + 1),
                    ];
                    output.conclusion = format!("T_({}) = {target_n}.", k + 1);
                    output.success_note = "Your inductive step increment is correct.".into();
                    output.visual = Some(json!({"kind": "proof_ind_step", "a": a, "k": k, "target": target_n}));
                }
                "sum_of_squares_induction" => {
                    let n = signed_integer(p, "n")?;
                    let target_n = signed_integer(p, "target")?;
                    if from_canonical(target) != Some(int(target_n)) || n * (n + 1) * (2 * n + 1) / 6 != target_n {
                        return Err("Sum of squares induction reference mismatch".into());
                    }
                    output.hint = "Substitute n into the closed-form identity n(n + 1)(2n + 1) / 6.".into();
                    output.steps = vec![
                        format!("Compute the three factors: n = {n}, n + 1 = {}, and 2n + 1 = {}.", n + 1, 2 * n + 1),
                        format!("Multiply and divide by 6: ({n} × {} × {}) / 6 = {target_n}.", n + 1, 2 * n + 1),
                    ];
                    output.conclusion = format!("∑ i² = {target_n}.");
                    output.success_note = "Your sum-of-squares evaluation is correct.".into();
                    output.visual = Some(json!({"kind": "proof_sum_sq", "n": n, "target": target_n}));
                }
                _ => {
                    let a = signed_integer(p, "a")?;
                    let n = signed_integer(p, "n")?;
                    let q_val = crate::number::frac(a * n, n + 1);
                    if from_canonical(target) != Some(q_val.clone()) {
                        return Err("Telescoping fraction induction reference mismatch".into());
                    }
                    let canon = to_canonical(&q_val);
                    output.hint = "Since 1/[i(i + 1)] = 1/i − 1/(i + 1), the sum telescopes to a[1 − 1/(n + 1)] = (a·n) / (n + 1).".into();
                    output.steps = vec![
                        format!("Apply the inductive closed form (a·n)/(n + 1) with a = {a} and n = {n}: {} / {}.", a * n, n + 1),
                        format!("Reduce over Q: {canon}."),
                    ];
                    output.conclusion = format!("∑ = {canon}.");
                    output.success_note = "Your telescoping induction sum is correct.".into();
                    output.visual = Some(json!({"kind": "proof_telescope", "a": a, "n": n, "target": canon}));
                }
            }
        }
        "proof.counterexamples_and_extremal" => {
            let kind = p.metadata.get("kind").and_then(|v| v.as_str()).unwrap_or("minimal_counterexample_polynomial");
            match kind {
                "minimal_counterexample_polynomial" => {
                    let p_prime = signed_integer(p, "p")?;
                    let target_n = signed_integer(p, "target")?;
                    if from_canonical(target) != Some(int(target_n)) || p_prime - 1 != target_n {
                        return Err("Minimal counterexample polynomial reference mismatch".into());
                    }
                    output.hint = "Rewrite n² + n + p = n(n + 1) + p. When n + 1 = p (so n = p − 1), P(p − 1) = (p − 1)p + p = p² is composite!".into();
                    output.steps = vec![
                        format!("Factor the first two terms: P(n) = n(n + 1) + {p_prime}."),
                        format!("Set n + 1 = {p_prime} so that n = {p_prime} − 1 = {target_n}, giving P({target_n}) = {p_prime}² (composite)."),
                    ];
                    output.conclusion = format!("n_min = {target_n}.");
                    output.success_note = "Your minimal counterexample is correct. Notice that n = p − 1 occurs one step earlier than the obvious n = p.".into();
                    output.visual = Some(json!({"kind": "proof_euler_prime", "p": p_prime, "target": target_n}));
                }
                "euclid_division_quotient" => {
                    let a = signed_integer(p, "a")?;
                    let b = signed_integer(p, "b")?;
                    let q = signed_integer(p, "q")?;
                    let r = signed_integer(p, "r")?;
                    if from_canonical(target) != Some(int(q)) || b * q + r != a {
                        return Err("Euclidean division quotient reference mismatch".into());
                    }
                    output.hint = "Because the Euclidean remainder r must satisfy 0 ≤ r < b, for a negative dividend a you must step down to the greatest multiple b·q ≤ a.".into();
                    output.steps = vec![
                        format!("Find the largest multiple of {b} that does not exceed {a}: {b} × ({q}) = {}.", b * q),
                        format!("Verify the non-negative remainder: r = {a} − ({}) = {r} ∈ [0, {b}).", b * q),
                    ];
                    output.conclusion = format!("q = {q}.");
                    output.success_note = "Your Euclidean quotient is correct.".into();
                    output.visual = Some(json!({"kind": "proof_euclid_div", "a": a, "b": b, "q": q, "r": r}));
                }
                _ => {
                    let a = signed_integer(p, "a")?;
                    let b = signed_integer(p, "b")?;
                    let y0 = signed_integer(p, "y0")?;
                    let rhs = signed_integer(p, "rhs")?;
                    let target_n = signed_integer(p, "target")?;
                    if from_canonical(target) != Some(int(target_n)) || a * target_n + b * y0 != rhs {
                        return Err("Bezout linear combination reference mismatch".into());
                    }
                    output.hint = "Subtract b·y₀ from the right-hand side and divide by a to isolate the integer coefficient x.".into();
                    output.steps = vec![
                        format!("Compute {b} × {y0} = {} and subtract from {rhs}: {a}x = {rhs} − {} = {}.", b * y0, b * y0, rhs - b * y0),
                        format!("Divide by {a}: x = {} / {a} = {target_n}.", rhs - b * y0),
                    ];
                    output.conclusion = format!("x = {target_n}.");
                    output.success_note = "Your Bézout coefficient is correct.".into();
                    output.visual = Some(json!({"kind": "proof_bezout", "a": a, "b": b, "x": target_n, "y": y0, "rhs": rhs}));
                }
            }
        }
        "proof.definitions_and_biconditionals" => {
            let kind = p.metadata.get("kind").and_then(|v| v.as_str()).unwrap_or("definition_witness_dyadic");
            match kind {
                "definition_witness_dyadic" => {
                    let m = signed_integer(p, "m")?;
                    let n = signed_integer(p, "n")?;
                    let p_num = signed_integer(p, "p")?;
                    let q_num = signed_integer(p, "q")?;
                    let scale = signed_integer(p, "scale")?;
                    let target_n = signed_integer(p, "target")?;
                    if from_canonical(target) != Some(int(target_n)) || p_num * scale + q_num != target_n {
                        return Err("Definition witness dyadic reference mismatch".into());
                    }
                    output.hint = "Multiply the numerator and denominator of p / 2^m by 2^(n − m) so both fractions share denominator 2^n.".into();
                    output.steps = vec![
                        format!("Scale {p_num}/2^{m} by 2^({n}−{m}) = {scale}: ({p_num} × {scale}) / 2^{n} = {} / 2^{n}.", p_num * scale),
                        format!("Add the numerators over 2^{n}: A = {} + {q_num} = {target_n}.", p_num * scale),
                    ];
                    output.conclusion = format!("A = {target_n}.");
                    output.success_note = "Your definition witness A is correct.".into();
                    output.visual = Some(json!({"kind": "proof_dyadic", "p": p_num, "q": q_num, "scale": scale, "target": target_n}));
                }
                "necessary_vs_sufficient_gap" => {
                    let a = signed_integer(p, "a")?;
                    let ab = signed_integer(p, "ab")?;
                    let nec_count = signed_integer(p, "nec_count")?;
                    let suff_count = signed_integer(p, "suff_count")?;
                    let target_n = signed_integer(p, "target")?;
                    if from_canonical(target) != Some(int(target_n)) || nec_count - suff_count != target_n {
                        return Err("Necessary vs sufficient gap reference mismatch".into());
                    }
                    output.hint = "Count the multiples of a in D (satisfying N(x)) and subtract the multiples of ab in D (satisfying S(x)).".into();
                    output.steps = vec![
                        format!("Multiples of {a} in D: {nec_count}. Multiples of {ab} in D: {suff_count}."),
                        format!("Subtract to count counterexamples where {a} | x but {ab} ∤ x: {nec_count} − {suff_count} = {target_n}."),
                    ];
                    output.conclusion = format!("{target_n} counterexamples.");
                    output.success_note = "Your necessary-versus-sufficient counterexample count is correct.".into();
                    output.visual = Some(json!({"kind": "proof_nec_suff", "nec": nec_count, "suff": suff_count, "target": target_n}));
                }
                _ => {
                    let a = signed_integer(p, "a")?;
                    let b = signed_integer(p, "b")?;
                    let c = signed_integer(p, "c")?;
                    let q_val = crate::number::frac(b * c, a);
                    if from_canonical(target) != Some(q_val.clone()) {
                        return Err("Biconditional parameter reference mismatch".into());
                    }
                    let canon = to_canonical(&q_val);
                    output.hint = "Solve ax = b to get x = b/a, then substitute x = b/a into d = cx so both equations have the exact same root.".into();
                    output.steps = vec![
                        format!("From {a}x = {b}, the unique solution is x = {b}/{a}."),
                        format!("Substitute into d = {c}x: d = {c} × ({b}/{a}) = {} / {a} = {canon}.", b * c),
                    ];
                    output.conclusion = format!("d = {canon}.");
                    output.success_note = "Your biconditional equivalence parameter is correct.".into();
                    output.visual = Some(json!({"kind": "proof_bicond", "a": a, "b": b, "c": c, "target": canon}));
                }
            }
        }
        "proof.hypotheses_and_lemmas" => {
            let kind = p.metadata.get("kind").and_then(|v| v.as_str()).unwrap_or("missing_hypothesis_roots");
            match kind {
                "missing_hypothesis_roots" => {
                    let r = signed_integer(p, "r")?;
                    let b = signed_integer(p, "b")?;
                    let d = signed_integer(p, "d")?;
                    let root2 = signed_integer(p, "root2")?;
                    let target_n = signed_integer(p, "target")?;
                    if from_canonical(target) != Some(int(target_n)) || r + root2 != target_n {
                        return Err("Missing hypothesis roots reference mismatch".into());
                    }
                    output.hint = "Move everything to one side and factor out (x − r): either x − r = 0 (giving x = r) or ax − b = cx + d (giving x = b + d).".into();
                    output.steps = vec![
                        format!("Case 1 (the root lost when dividing without x ≠ {r}): x = {r}."),
                        format!("Case 2 (for x ≠ {r}): x = {b} + {d} = {root2}."),
                        format!("Sum both roots: {r} + {root2} = {target_n}."),
                    ];
                    output.conclusion = format!("Sum = {target_n}.");
                    output.success_note = "Your complete root sum is correct. Splitting into cases x = r and x ≠ r prevents losing solutions.".into();
                    output.visual = Some(json!({"kind": "proof_lost_root", "r": r, "root2": root2, "target": target_n}));
                }
                "lemma_chain_modular_power" => {
                    let b = signed_integer(p, "b")?;
                    let m = signed_integer(p, "m")?;
                    let k = signed_integer(p, "k")?;
                    let q = signed_integer(p, "q")?;
                    let r = signed_integer(p, "r")?;
                    let target_n = signed_integer(p, "target")?;
                    if from_canonical(target) != Some(int(target_n)) || (b.pow(r as u32) % m) != target_n {
                        return Err("Lemma chain modular power reference mismatch".into());
                    }
                    output.hint = "Apply Lemma 1 to replace (b^k)^q with 1^q ≡ 1 (mod m), then evaluate b^r mod m.".into();
                    output.steps = vec![
                        format!("Rewrite the power using the division of the exponent: ({b}^{k})^{q} · {b}^{r} ≡ 1^{q} · {b}^{r} (mod {m})."),
                        format!("Evaluate {b}^{r} mod {m} = {} mod {m} = {target_n}.", b.pow(r as u32)),
                    ];
                    output.conclusion = format!("Remainder = {target_n}.");
                    output.success_note = "Your lemma-reduced modular remainder is correct.".into();
                    output.visual = Some(json!({"kind": "proof_lemma_mod", "b": b, "m": m, "r": r, "target": target_n}));
                }
                _ => {
                    let a = signed_integer(p, "a")?;
                    let b = signed_integer(p, "b")?;
                    let c = signed_integer(p, "c")?;
                    let q_val = crate::number::frac(c + 2, 2 * c);
                    if from_canonical(target) != Some(q_val.clone()) {
                        return Err("Triangle inequality lemma reference mismatch".into());
                    }
                    let canon = to_canonical(&q_val);
                    output.hint = "Substitute the upper bounds from Lemma A and Lemma B into a|x − x₀| + b|y − y₀|, cancel a and b, and add 1/2 + 1/c.".into();
                    output.steps = vec![
                        format!("Scale Lemma A and Lemma B: {a} × (1/{}) + {b} × (1/{}) = 1/2 + 1/{c}.", 2 * a, c * b),
                        format!("Combine over common denominator {}: ({c} + 2) / {} = {canon}.", 2 * c, 2 * c),
                    ];
                    output.conclusion = format!("Bound = {canon}.");
                    output.success_note = "Your Triangle Inequality lemma bound is correct.".into();
                    output.visual = Some(json!({"kind": "proof_triangle_lemma", "c": c, "target": canon}));
                }
            }
        }
        "proof.proof_audit_and_revision" => {
            let kind = p.metadata.get("kind").and_then(|v| v.as_str()).unwrap_or("converse_fallacy_audit");
            match kind {
                "converse_fallacy_audit" => {
                    let r = signed_integer(p, "r")?;
                    let s = signed_integer(p, "s")?;
                    if from_canonical(target) != Some(int(s)) {
                        return Err("Converse fallacy audit reference mismatch".into());
                    }
                    output.hint = "Factor the quadratic as (x − r)(x − s) = 0. The second root x = s satisfies the conclusion without satisfying x = r.".into();
                    output.steps = vec![
                        format!("Factor the quadratic equation into (x − {r})(x − {s}) = 0."),
                        format!("Identify the other root x ≠ {r}: x = {s}."),
                    ];
                    output.conclusion = format!("x = {s}.");
                    output.success_note = "Your converse counterexample is correct. Knowing P ⇒ Q and Q does not imply P when another path leads to Q.".into();
                    output.visual = Some(json!({"kind": "proof_converse_audit", "r": r, "s": s}));
                }
                "quantifier_order_uniform_bound" => {
                    let a = signed_integer(p, "a")?;
                    let b = signed_integer(p, "b")?;
                    let m = signed_integer(p, "m")?;
                    let target_n = signed_integer(p, "target")?;
                    if from_canonical(target) != Some(int(target_n)) || a * m + b != target_n {
                        return Err("Quantifier order uniform bound reference mismatch".into());
                    }
                    output.hint = "For a single Y to satisfy Y ≥ ax + b for ALL x ∈ {1, ..., m}, Y must be at least the maximum value of ax + b on X (at x = m).".into();
                    output.steps = vec![
                        format!("Since {a} > 0, {a}x + {b} is strictly increasing and attains its maximum on X at x = {m}."),
                        format!("Evaluate at x = {m}: Y_min = {a}({m}) + {b} = {target_n}."),
                    ];
                    output.conclusion = format!("Y_min = {target_n}.");
                    output.success_note = "Your uniform quantifier bound is correct. Placing ∃Y before ∀x forces Y to cover the worst-case x.".into();
                    output.visual = Some(json!({"kind": "proof_quant_order", "a": a, "b": b, "m": m, "target": target_n}));
                }
                _ => {
                    let k = signed_integer(p, "k")?;
                    let true_sum = signed_integer(p, "true_sum")?;
                    let claimed_val = signed_integer(p, "claimed_val")?;
                    let target_n = signed_integer(p, "target")?;
                    if from_canonical(target) != Some(int(target_n)) || claimed_val - true_sum != target_n {
                        return Err("Induction base case audit reference mismatch".into());
                    }
                    output.hint = "Subtract the true sum k(k + 1) from the student's claimed value k(k + 1) + C to isolate the constant offset C.".into();
                    output.steps = vec![
                        format!("Compute the true sum at n = {k}: {k}({k} + 1) = {true_sum}."),
                        format!("Subtract from the claimed value {claimed_val}: C = {claimed_val} − {true_sum} = {target_n}."),
                    ];
                    output.conclusion = format!("C = {target_n}.");
                    output.success_note = "Your induction base-case audit is correct. An inductive step without a verified base case only proves a parallel shifted family of statements.".into();
                    output.visual = Some(json!({"kind": "proof_base_audit", "k": k, "true_sum": true_sum, "claimed": claimed_val, "target": target_n}));
                }
            }
        }
        _=>return Err("Worked help is not available for this exercise family".into()),
    }Ok(output)
}

#[cfg(test)]
mod tests{
    use super::*;use crate::{generate,check,MathBudget,Disposition};
    #[test]fn all_current_families_have_consistent_exact_worked_help(){
        for name in ["arithmetic.whole.addition","arithmetic.column.addition","arithmetic.whole.multiplication","arithmetic.whole.division","arithmetic.column.subtraction","integers.order","divisibility.factors.find","divisibility.gcd_lcm","learning.problem_reading","learning.equality_balance","learning.checking_bounds","decimals.fraction_convert","percentages.benchmark_calc","ratios.scaling_solve","fractions.compare.positive","fractions.equivalent.find","fractions.unit.identify","place_value.decompose","abacus.read.state","abacus.target.setting","abacus.trainer.number","abacus.trainer.add","abacus.trainer.subtract","abacus.trainer.five","abacus.trainer.ten","abacus.trainer.mental","powers.integer_eval","powers.scientific_notation","roots.radical_eval","geometry.perimeter_and_area","geometry.angles_triangles","geometry.coordinate_distance","data.mean_median_calc","counting.permutations_combinations","probability.simple_exact","algebra.expression_eval","algebra.linear_equation_solve","algebra.linear_system_solve","functions.evaluate_and_range","functions.composition_inverse","functions.transformations_vertex","algebra2.polynomial_factor_roots","algebra2.quadratic_discriminant","algebra2.exp_log_eval","sec_geom.slope_and_lines","sec_geom.circle_equations_theorems","sec_geom.planar_vectors_dot","trig.radians_unit_circle","trig.identities_double_angle","trig.law_of_sines_cosines","sequences.arithmetic_progression","sequences.geometric_and_infinite","sequences.recursion_binomial","calculus.limits_rational","calculus.derivatives_tangent","calculus.definite_integral_ftc","proof.sets_and_cardinality","proof.logic_truth_quantifiers","proof.relations_and_bijections","proof.direct_and_contrapositive","proof.mathematical_induction","proof.counterexamples_and_extremal","proof.definitions_and_biconditionals","proof.hypotheses_and_lemmas","proof.proof_audit_and_revision"]{for seed in 0..16{
            let p=generate(name,"1.1.0",&format!("{seed:064x}"),1,&"0".repeat(64)).unwrap();let help=material(&p).unwrap();assert!(!help.hint.is_empty()&&!help.steps.is_empty()&&!help.success_note.is_empty());assert_eq!(check(&p,p.target_value.as_ref().unwrap(),&MathBudget::default()).disposition,Disposition::Correct);
        }}
        for name in ["data.mean_median_calc","counting.permutations_combinations","probability.simple_exact","algebra.expression_eval","algebra.linear_equation_solve","algebra.linear_system_solve","functions.evaluate_and_range","functions.composition_inverse","functions.transformations_vertex","algebra2.polynomial_factor_roots","algebra2.quadratic_discriminant","algebra2.exp_log_eval","sec_geom.slope_and_lines","sec_geom.circle_equations_theorems","sec_geom.planar_vectors_dot","trig.radians_unit_circle","trig.identities_double_angle","trig.law_of_sines_cosines","sequences.arithmetic_progression","sequences.geometric_and_infinite","sequences.recursion_binomial","calculus.limits_rational","calculus.derivatives_tangent","calculus.definite_integral_ftc","proof.sets_and_cardinality","proof.logic_truth_quantifiers","proof.relations_and_bijections","proof.direct_and_contrapositive","proof.mathematical_induction","proof.counterexamples_and_extremal","proof.definitions_and_biconditionals","proof.hypotheses_and_lemmas","proof.proof_audit_and_revision"]{for band in 2..=3{for seed in 0..8{
            let p=generate(name,"1.1.0",&format!("{seed:064x}"),band,&"0".repeat(64)).unwrap();let help=material(&p).unwrap();assert!(!help.hint.is_empty()&&!help.steps.is_empty()&&!help.success_note.is_empty());assert_eq!(check(&p,p.target_value.as_ref().unwrap(),&MathBudget::default()).disposition,Disposition::Correct);
        }}}
    }
    #[test]fn borrowing_across_zero_columns_explains_each_regroup(){let s=subtraction(1002,587).unwrap();assert!(s.iter().any(|s|s.contains("zero columns")));assert_eq!(s.last().unwrap(),"Read the result: 1002 − 587 = 415.");}
    #[test]fn inconsistent_reference_never_receives_a_fabricated_solution(){let mut p=generate("arithmetic.whole.addition","1.1.0",&"0".repeat(64),1,&"0".repeat(64)).unwrap();p.target_value=Some("999".into());assert!(material(&p).is_err());}
}
