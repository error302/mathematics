//! Bounded, exact soroban teaching traces. UI renders these Rust-owned states/actions.
use crate::{AbacusState, AbacusAction, apply_abacus_action, GenerationFailure};
use crate::number::{int, to_canonical};
use crate::rng::{DrawSource, uniform_inclusive};
use serde::{Serialize, Deserialize};
use serde_json::{Value, json};

pub type ExerciseParts = (String, String, String, String, Option<Vec<String>>, Option<String>, Value, String);

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TeachingFrame {
    pub state: AbacusState,
    pub caption: String,
    pub value: String,
    pub action: Option<AbacusAction>,
    pub highlight_rod: usize,
}

fn failure(message: impl ToString) -> GenerationFailure { GenerationFailure::Serialization(message.to_string()) }
fn state(value: i64, rods: usize) -> Result<AbacusState, GenerationFailure> {
    AbacusState::from_value(&int(value), rods, 0).map_err(failure)
}
fn frame(state: AbacusState, caption: String, action: Option<AbacusAction>, rod: usize) -> TeachingFrame {
    TeachingFrame { value: to_canonical(&state.compute_value()), state, caption, action, highlight_rod: rod }
}
fn step(frames: &mut Vec<TeachingFrame>, action: AbacusAction, caption: String, rod: usize) -> Result<(), GenerationFailure> {
    let next = apply_abacus_action(&frames.last().ok_or_else(|| failure("missing starting frame"))?.state, &action).map_err(failure)?;
    frames.push(frame(next, caption, Some(action), rod)); Ok(())
}

/// Generate a teaching trace with explicit legal moves, including intermediate five/ten exchanges.
pub fn script(stage: &str, a: i64, b: i64, c: i64) -> Result<(String, Vec<TeachingFrame>), GenerationFailure> {
    let rods = if stage == "ten" || stage == "ten_sub" || stage == "compound" { 2 } else { 1 };
    let max_a = if rods == 2 { 18 } else { 9 };
    if !(0..=max_a).contains(&a) || !(0..=9).contains(&b) || !(0..=9).contains(&c) { return Err(failure("teaching values outside bounds")); }
    let ones = rods-1;
    let start = if stage == "number" {0} else {a};
    let mut frames = vec![frame(state(start, rods)?, format!("Start at {start}. Only beads touching the middle bar count."), None, ones)];
    let equation = match stage {
        "number" => {
            if a >= 5 {step(&mut frames, AbacusAction::ToggleUpper {rod_index:ones}, "Move the upper bead down to the bar. It counts as five.".into(), ones)?;}
            let lower = (a%5) as u8;
            for count in 1..=lower {step(&mut frames, AbacusAction::SetLower {rod_index:ones,count}, format!("Move one more lower bead up to the bar. Now {count} lower beads count."), ones)?;}
            format!("Make {a}")
        }
        "add" | "mental" => {
            if b<1 || a%5+b>4 || a+b>9 {return Err(failure("addition needs available lower beads"));}
            step(&mut frames, AbacusAction::PushLower {rod_index:ones,count:b as u8},format!("Add {b}: move {b} lower bead(s) up to the bar."),ones)?;
            if stage=="mental" {
                if c<1 || c>a%5+b {return Err(failure("mental subtraction needs counted lower beads"));}
                step(&mut frames, AbacusAction::PullLower {rod_index:ones,count:c as u8},format!("Then subtract {c}: move {c} lower bead(s) away from the bar."),ones)?;
                format!("{a} + {b} − {c}")
            } else {format!("{a} + {b}")}
        }
        "subtract" => {
            if b<1 || b>a%5 {return Err(failure("subtraction needs counted lower beads"));}
            step(&mut frames, AbacusAction::PullLower {rod_index:ones,count:b as u8},format!("Subtract {b}: move {b} lower bead(s) down, away from the bar."),ones)?;
            format!("{a} − {b}")
        }
        "five" => {
            if !(1..=4).contains(&a) || !(1..=4).contains(&b) || a+b<5 {return Err(failure("five exchange needs crossing five"));}
            step(&mut frames, AbacusAction::ToggleUpper {rod_index:ones},format!("To add {b}, first add five by moving the upper bead to the bar."),ones)?;
            let remove=5-b;
            step(&mut frames, AbacusAction::PullLower {rod_index:ones,count:remove as u8},format!("Take away {remove} lower beads. Adding five and taking away {remove} adds {b}."),ones)?;
            format!("{a} + {b} = {a} + 5 − {remove}")
        }
        "five_sub" => {
            if !(5..=8).contains(&a) || !(1..=4).contains(&b) || (a % 5) >= b { return Err(failure("five_sub needs borrowing from five")); }
            step(&mut frames, AbacusAction::ToggleUpper { rod_index: ones }, format!("To subtract {b}, first remove five by moving the upper bead away from the bar."), ones)?;
            let add_back = 5 - b;
            step(&mut frames, AbacusAction::PushLower { rod_index: ones, count: add_back as u8 }, format!("Add back {add_back} lower beads. Removing five and adding {add_back} subtracts {b}."), ones)?;
            format!("{a} − {b} = {a} − 5 + {add_back}")
        }
        "ten" => {
            if a<5 || b<1 || a+b<10 || a+b>18 {return Err(failure("ten exchange needs a carry"));}
            step(&mut frames, AbacusAction::SetLower {rod_index:0,count:1},format!("To add {b}, first add one ten on the Tens rod."),0)?;
            let remove=10-b;
            let lower=a%5;
            if remove<=lower {
                step(&mut frames,AbacusAction::PullLower{rod_index:ones,count:remove as u8},format!("Remove {remove} ones. Adding ten and removing {remove} adds {b}."),ones)?;
            } else if remove>=5 {
                step(&mut frames,AbacusAction::ToggleUpper{rod_index:ones},"Remove five ones by moving the upper bead away from the bar.".into(),ones)?;
                if remove>5 {step(&mut frames,AbacusAction::PullLower{rod_index:ones,count:(remove-5) as u8},format!("Remove {} more ones to finish taking away {remove}.",remove-5),ones)?;}
            } else {
                step(&mut frames,AbacusAction::ToggleUpper{rod_index:ones},format!("To remove {remove} ones, first remove five using the upper bead."),ones)?;
                step(&mut frames,AbacusAction::PushLower{rod_index:ones,count:(5-remove) as u8},format!("Add back {} ones. Removing five and adding back {} removes {remove}.",5-remove,5-remove),ones)?;
            }
            format!("{a} + {b} = {a} + 10 − {remove}")
        }
        "ten_sub" => {
            let ones_val = a % 10;
            if !(10..=18).contains(&a) || !(1..=9).contains(&b) || ones_val >= b { return Err(failure("ten_sub needs borrowing from ten")); }
            step(&mut frames, AbacusAction::PullLower { rod_index: 0, count: 1 }, format!("To subtract {b}, first borrow one ten from the Tens rod."), 0)?;
            let complement = 10 - b;
            let t = a - b;
            let curr_u = ones_val >= 5;
            let curr_l = (ones_val % 5) as u8;
            let tgt_u = t >= 5;
            let tgt_l = (t % 5) as u8;
            if curr_u != tgt_u {
                if tgt_u {
                    step(&mut frames, AbacusAction::ToggleUpper { rod_index: ones }, "Add five using the upper bead on the Ones rod.".into(), ones)?;
                } else {
                    step(&mut frames, AbacusAction::ToggleUpper { rod_index: ones }, "Remove five using the upper bead on the Ones rod.".into(), ones)?;
                }
            }
            if tgt_l > curr_l {
                let diff = tgt_l - curr_l;
                step(&mut frames, AbacusAction::PushLower { rod_index: ones, count: diff }, format!("Add {diff} more lower bead(s) up to the bar."), ones)?;
            } else if tgt_l < curr_l {
                let diff = curr_l - tgt_l;
                step(&mut frames, AbacusAction::PullLower { rod_index: ones, count: diff }, format!("Move {diff} lower bead(s) away from the bar."), ones)?;
            }
            format!("{a} − {b} = {a} − 10 + {complement}")
        }
        "compound" => {
            if a <= 9 {
                let rem_lower = a % 5;
                let need_lower = 10 - b;
                if !(5..=8).contains(&a) || !(6..=9).contains(&b) || rem_lower >= need_lower { return Err(failure("compound addition requires mixed trade")); }
                step(&mut frames, AbacusAction::SetLower { rod_index: 0, count: 1 }, "Add one ten on the Tens rod.".into(), 0)?;
                step(&mut frames, AbacusAction::ToggleUpper { rod_index: ones }, "Remove five on the Ones rod.".into(), ones)?;
                let add_ones = (b - 5) as u8;
                step(&mut frames, AbacusAction::PushLower { rod_index: ones, count: add_ones }, format!("Add {add_ones} lower beads on the Ones rod. (+10 − 5 + {add_ones} = +{b})."), ones)?;
                format!("{a} + {b} = {a} + 10 − 5 + {add_ones}")
            } else {
                let remove_ones = (b - 5) as u8;
                if !(11..=14).contains(&a) || !(6..=9).contains(&b) || (a - 10) + (10 - b) <= 4 { return Err(failure("compound subtraction requires mixed trade")); }
                step(&mut frames, AbacusAction::PullLower { rod_index: 0, count: 1 }, "Borrow one ten from the Tens rod.".into(), 0)?;
                step(&mut frames, AbacusAction::ToggleUpper { rod_index: ones }, "Add five on the Ones rod.".into(), ones)?;
                step(&mut frames, AbacusAction::PullLower { rod_index: ones, count: remove_ones }, format!("Remove {remove_ones} lower beads on the Ones rod. (−10 + 5 − {remove_ones} = −{b})."), ones)?;
                format!("{a} − {b} = {a} − 10 + 5 − {remove_ones}")
            }
        }
        _ => return Err(GenerationFailure::TemplateNotFound(stage.into())),
    };
    Ok((equation,frames))
}

pub fn generate_training<R: DrawSource + ?Sized>(rng: &mut R, template: &str) -> Result<ExerciseParts,GenerationFailure> {
    let stage=template.strip_prefix("abacus.trainer.").ok_or_else(|| failure("invalid trainer template"))?;
    let (a,b,c)=match stage {
        "number" => (uniform_inclusive(rng,1,9) as i64,0,0),
        "add" => {let a=uniform_inclusive(rng,1,3) as i64;(a,uniform_inclusive(rng,1,(4-a) as i128) as i64,0)},
        "subtract" => {let a=uniform_inclusive(rng,2,4) as i64;(a,uniform_inclusive(rng,1,(a-1) as i128) as i64,0)},
        "five" => {let a=uniform_inclusive(rng,1,4) as i64;(a,uniform_inclusive(rng,(5-a) as i128,4) as i64,0)},
        "five_sub" => {let a=uniform_inclusive(rng,5,8) as i64;(a,uniform_inclusive(rng,(a%5+1) as i128,4) as i64,0)},
        "ten" => {let a=uniform_inclusive(rng,5,9) as i64;(a,uniform_inclusive(rng,(10-a) as i128,9) as i64,0)},
        "ten_sub" => {let a=uniform_inclusive(rng,10,14) as i64;(a,uniform_inclusive(rng,(a%10+1) as i128,9) as i64,0)},
        "compound" => {
            if uniform_inclusive(rng, 0, 1) == 0 {
                let a = uniform_inclusive(rng, 5, 8) as i64;
                let b = uniform_inclusive(rng, 6, (14 - a) as i128) as i64;
                (a, b, 0)
            } else {
                let a = uniform_inclusive(rng, 11, 14) as i64;
                let b = uniform_inclusive(rng, 6, (a - 5) as i128) as i64;
                (a, b, 0)
            }
        },
        "mental" => {let a=uniform_inclusive(rng,1,3) as i64;let b=uniform_inclusive(rng,1,(4-a) as i128) as i64;(a,b,uniform_inclusive(rng,1,(a+b) as i128) as i64)},
        _ => return Err(GenerationFailure::TemplateNotFound(template.into())),
    };
    let (equation,trace)=script(stage,a,b,c)?;
    let target=trace.last().ok_or_else(|| failure("empty trace"))?.value.clone();
    let is_sub = stage == "subtract" || stage == "five_sub" || stage == "ten_sub" || (stage == "compound" && a > 9);
    let prompt = if stage == "number" {
        format!("Make {a} on the Ones rod.")
    } else if stage == "mental" {
        format!("Start with {a}. Add {b}. Then subtract {c}. What number do you finish with?")
    } else {
        format!("Start with {a}. {} {b}. What number do you finish with?", if is_sub { "Subtract" } else { "Add" })
    };
    let metadata=json!({"stage":stage,"operands":[a,b,c],"equation":equation,"initial_state":trace[0].state,"trace":trace,"task_mode":if stage=="mental" {"mental_answer"} else {"virtual_trace"}});
    let kind=if stage=="mental" {"integer"} else {"soroban_state"};
    Ok((prompt.clone(),String::new(),prompt,kind.into(),None,Some(target),metadata,if stage=="mental" {"integer-exact-v2".into()} else {"abacus-state-v2".into()}))
}

pub fn catalog() -> Result<Value,GenerationFailure> {
    let definitions=[
        ("number","Meet the beads","Read and make numbers from zero to nine.","Begin with one rod: the Ones rod. The upper bead counts as five; each lower bead counts as one. Beads count only when they touch the bar.",7,0,0),
        ("add","Add with beads","Picture adding before you calculate.","Start by making the first number. Move available lower beads toward the bar to add. We start with small sums that do not need an exchange.",2,1,0),
        ("subtract","Take beads away","Connect subtraction to removing counted beads.","Make the starting number, then move counted lower beads away from the bar. Their contribution disappears; the other beads keep their value.",4,2,0),
        ("five","Make a bridge over five","Learn why adding five and removing a partner works.","If there are not enough free lower beads, use the upper bead. Three and two make five, so adding three is the same as adding five and taking away two.",4,3,0),
        ("five_sub","Subtract across five","Master 5-complement borrowing: remove five and add back the partner.","When you do not have enough lower beads to subtract, move the upper bead away from the bar and push back the partner lower beads.",6,2,0),
        ("ten","Make a bridge over ten","Carry one ten, then adjust the ones.","The new left rod is Tens: one lower bead there counts as ten. To add seven, add ten and remove three. Eight plus seven becomes eight plus ten minus three.",8,7,0),
        ("ten_sub","Borrow across ten","Master 10-complement borrowing: borrow ten and add back the partner.","When the ones place has insufficient beads to subtract, borrow one ten on the Tens rod and adjust the ones place with the partner.",12,7,0),
        ("compound","Combined exchanges","Master five- and ten-complements together in compound carries and borrows.","Some calculations require both 10-carrying and 5-exchanges in sequence, such as 6 + 7 = 6 + 10 − 5 + 2 or 13 − 7 = 13 − 10 + 5 − 2.",6,7,0),
        ("mental","Picture the beads","Try short arithmetic sequences with less visual help.","First look at the starting beads. Hide them when you are ready. Picture the moves as you add and subtract. There is no timer; bring the beads back whenever you need support.",2,1,2),
    ];
    let mut lessons=Vec::new();
    for (id,title,goal,intro,a,b,c) in definitions {
        let (equation,trace)=script(id,a,b,c)?;
        lessons.push(json!({"id":id,"title":title,"goal":goal,"intro":intro,"template_id":format!("abacus.trainer.{id}"),"equation":equation,"trace":trace}));
    }
    Ok(json!({"version":"1.0.0","status":"draft","lessons":lessons}))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{generate, check, Disposition, MathBudget};
    #[test] fn teaching_frames_replay_legal_moves_and_exact_values() {
        for a in 5..=9 {for b in 10-a..=9 {let (_,frames)=script("ten",a,b,0).unwrap();assert_eq!(frames.last().unwrap().value,(a+b).to_string());for pair in frames.windows(2) {assert_eq!(apply_abacus_action(&pair[0].state,pair[1].action.as_ref().unwrap()).unwrap(),pair[1].state);pair[1].state.validate().unwrap();}}}
        for a in 1..=4 {for b in 5-a..=4 {assert_eq!(script("five",a,b,0).unwrap().1.last().unwrap().value,(a+b).to_string());}}
        for a in 5..=8 {for b in (a%5)+1..=4 {let (_,frames)=script("five_sub",a,b,0).unwrap();assert_eq!(frames.last().unwrap().value,(a-b).to_string());for pair in frames.windows(2) {assert_eq!(apply_abacus_action(&pair[0].state,pair[1].action.as_ref().unwrap()).unwrap(),pair[1].state);pair[1].state.validate().unwrap();}}}
        for a in 10..=18 {for b in (a%10)+1..=9 {let (_,frames)=script("ten_sub",a,b,0).unwrap();assert_eq!(frames.last().unwrap().value,(a-b).to_string());for pair in frames.windows(2) {assert_eq!(apply_abacus_action(&pair[0].state,pair[1].action.as_ref().unwrap()).unwrap(),pair[1].state);pair[1].state.validate().unwrap();}}}
        for a in 5..=8 {for b in 6..=(14-a) {let (_,frames)=script("compound",a,b,0).unwrap();assert_eq!(frames.last().unwrap().value,(a+b).to_string());for pair in frames.windows(2) {assert_eq!(apply_abacus_action(&pair[0].state,pair[1].action.as_ref().unwrap()).unwrap(),pair[1].state);pair[1].state.validate().unwrap();}}}
        for a in 11..=14 {for b in 6..=(a-5) {let (_,frames)=script("compound",a,b,0).unwrap();assert_eq!(frames.last().unwrap().value,(a-b).to_string());for pair in frames.windows(2) {assert_eq!(apply_abacus_action(&pair[0].state,pair[1].action.as_ref().unwrap()).unwrap(),pair[1].state);pair[1].state.validate().unwrap();}}}
    }
    #[test] fn trainer_generation_is_exact_replayable_and_checkable() {
        for stage in ["number","add","subtract","five","five_sub","ten","ten_sub","compound","mental"] {for seed in 0..24 {
            let seed=format!("{seed:064x}");let p=generate(&format!("abacus.trainer.{stage}"),"1.1.0",&seed,1,&"0".repeat(64)).unwrap();assert_eq!(p,generate(&format!("abacus.trainer.{stage}"),"1.1.0",&seed,1,&"0".repeat(64)).unwrap());
            let trace:Vec<TeachingFrame>=serde_json::from_value(p.metadata["trace"].clone()).unwrap();assert_eq!(p.target_value.as_ref().unwrap(),&trace.last().unwrap().value);
            let answer=if stage=="mental" {p.target_value.clone().unwrap()} else {serde_json::to_string(&trace.last().unwrap().state).unwrap()};assert_eq!(check(&p,&answer,&MathBudget::default()).disposition,Disposition::Correct);
        }}
    }
    #[test] fn invalid_teaching_inputs_are_rejected() {assert!(script("ten",1,1,0).is_err());assert!(script("mental",4,1,1).is_err());assert!(script("five",0,2,0).is_err());assert!(script("five_sub",4,2,0).is_err());assert!(script("ten_sub",9,5,0).is_err());assert!(script("number",100,0,0).is_err());}
}
