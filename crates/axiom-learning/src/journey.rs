//! A local study organizer. These milestones are practice loops, never certified mastery.
use crate::schedule::ReviewSchedule;
use axiom_math::{Disposition, GradeOutcome, ProblemArtifact};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
pub const POLICY: &str = "local-study-journey-v1";
#[derive(Clone, Debug, Serialize)]
pub struct StepDefinition {
    pub id: String,
    pub title: String,
    pub template_id: String,
    pub goal: String,
    pub prerequisite: Option<String>,
}
pub fn catalog() -> Vec<StepDefinition> {
    let rows = [
        (
            "foundation.place_value.decimal_expansion",
            "What each digit means",
            "place_value.decompose",
            "Connect a digit to its place and explain its contribution.",
        ),
        (
            "foundation.place_value.regrouping",
            "Trade ten small units for one larger unit",
            "arithmetic.column.addition",
            "Use regrouping to explain a carry, not just write the answer.",
        ),
        (
            "foundation.arithmetic.addition_disjoint",
            "Combine quantities",
            "arithmetic.whole.addition",
            "Add quantities and check what the sum represents.",
        ),
        (
            "foundation.arithmetic.column_addition_carry",
            "Addition with carries",
            "arithmetic.column.addition",
            "Line up place values and explain each regrouping.",
        ),
        (
            "foundation.arithmetic.subtraction_inverse",
            "Find a missing amount",
            "arithmetic.column.subtraction",
            "Use addition to check the difference.",
        ),
        (
            "foundation.arithmetic.column_subtraction_borrow",
            "Subtraction with regrouping",
            "arithmetic.column.subtraction",
            "Explain why a trade changes the digits but preserves the value.",
        ),
        (
            "foundation.fractions.unit_fractions",
            "Name an equal part",
            "fractions.unit.identify",
            "Name one piece of a whole split into equal parts.",
        ),
        (
            "foundation.fractions.compare",
            "Compare amounts, not appearances",
            "fractions.compare.positive",
            "Use equal-sized parts to explain a fraction comparison.",
        ),
    ];
    rows.iter()
        .enumerate()
        .map(|(i, (id, title, template, goal))| StepDefinition {
            id: (*id).into(),
            title: (*title).into(),
            template_id: (*template).into(),
            goal: (*goal).into(),
            prerequisite: if i == 0 {
                None
            } else {
                Some(rows[i - 1].0.into())
            },
        })
        .collect()
}
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct StepProgress {
    pub studied: bool,
    pub reflection_recorded: bool,
    pub success_hashes: Vec<String>,
    pub repair_needed: bool,
    pub review: Option<ReviewSchedule>,
    pub review_hashes: Vec<String>,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct JourneyState {
    pub schema_version: u32,
    pub policy_version: String,
    pub active_step: String,
    pub steps: BTreeMap<String, StepProgress>,
}
impl Default for JourneyState {
    fn default() -> Self {
        Self {
            schema_version: 1,
            policy_version: POLICY.into(),
            active_step: catalog()[0].id.clone(),
            steps: BTreeMap::new(),
        }
    }
}
fn valid_day(day: i64) -> Result<(), String> {
    if (0..=100_000).contains(&day) {
        Ok(())
    } else {
        Err("Device date is outside supported study-calendar bounds".into())
    }
}
fn hash(s: &str) -> bool {
    s.len() == 64
        && s.bytes()
            .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
}
pub fn validate(s: &JourneyState) -> Result<(), String> {
    let defs = catalog();
    if s.schema_version != 1
        || s.policy_version != POLICY
        || !defs.iter().any(|d| d.id == s.active_step)
        || s.steps.len() > defs.len()
    {
        return Err("Unsupported study history".into());
    }
    for (id, p) in &s.steps {
        if !defs.iter().any(|d| &d.id == id)
            || p.success_hashes.len() > 2
            || p.review_hashes.len() > 256
            || p.success_hashes
                .iter()
                .chain(&p.review_hashes)
                .any(|h| !hash(h))
        {
            return Err("Study history exceeds supported bounds".into());
        }
        let mut seen = std::collections::BTreeSet::new();
        if p.success_hashes
            .iter()
            .chain(&p.review_hashes)
            .any(|h| !seen.insert(h))
        {
            return Err("Repeated example identities in study history".into());
        }
        if let Some(r) = &p.review {
            if &r.concept_id != id
                || r.ladder_step >= crate::INTERVAL_LADDER_DAYS.len()
                || r.next_due_epoch_days < r.last_reviewed_epoch_days
                || valid_day(r.next_due_epoch_days).is_err()
                || valid_day(r.last_reviewed_epoch_days).is_err()
                || !loop_complete(p)
            {
                return Err("Invalid local review schedule".into());
            }
        }
    }
    Ok(())
}
fn loop_complete(p: &StepProgress) -> bool {
    p.studied && p.reflection_recorded && p.success_hashes.len() == 2
}
fn ensure_review(id: &str, p: &mut StepProgress, day: i64) {
    if loop_complete(p) && p.review.is_none() {
        p.review = Some(ReviewSchedule::new(id, day))
    }
}
#[derive(Debug, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum StudyAction {
    Open { step_id: String },
    AcknowledgeLesson { step_id: String },
    Reflection { step_id: String, recorded: bool },
}
pub fn action(mut s: JourneyState, a: StudyAction, day: i64) -> Result<JourneyState, String> {
    validate(&s)?;
    valid_day(day)?;
    let id = match &a {
        StudyAction::Open { step_id }
        | StudyAction::AcknowledgeLesson { step_id }
        | StudyAction::Reflection { step_id, .. } => step_id,
    };
    if !catalog().iter().any(|d| &d.id == id) {
        return Err("This lesson is outside the current guided foundation path".into());
    }
    let id = id.clone();
    s.active_step = id.clone();
    let p = s.steps.entry(id.clone()).or_default();
    match a {
        StudyAction::Open { .. } => {}
        StudyAction::AcknowledgeLesson { .. } => p.studied = true,
        StudyAction::Reflection { recorded, .. } => {
            p.reflection_recorded = recorded;
            if !recorded {
                p.review = None
            }
        }
    };
    ensure_review(&id, p, day);
    validate(&s)?;
    Ok(s)
}
/// Grade first; only a regenerated, matching artifact may update local practice history.
pub fn record(
    mut s: JourneyState,
    p: &ProblemArtifact,
    outcome: &GradeOutcome,
    assisted: bool,
    day: i64,
) -> Result<JourneyState, String> {
    validate(&s)?;
    valid_day(day)?;
    let def = catalog()
        .into_iter()
        .find(|d| d.id == s.active_step)
        .unwrap();
    if p.replay_identity.template_id != def.template_id {
        return Ok(s);
    }
    let r = &p.replay_identity;
    let original = axiom_math::generate(
        &r.template_id,
        &r.template_version,
        &r.seed_hex,
        r.difficulty_band,
        &r.content_manifest_hash,
    )
    .map_err(|e| e.to_string())?;
    if &original != p {
        return Err(
            "The example no longer matches its replay identity; study history was kept".into(),
        );
    }
    let progress = s.steps.entry(def.id.clone()).or_default();
    match outcome.disposition {
        Disposition::Incorrect => {
            progress.repair_needed = true;
            if let Some(review) = &mut progress.review {
                if day >= review.last_reviewed_epoch_days {
                    review.on_failure(day)
                }
            }
        }
        Disposition::Correct if !assisted => {
            let seen = progress.success_hashes.contains(&p.problem_hash)
                || progress.review_hashes.contains(&p.problem_hash);
            if !seen {
                progress.repair_needed = false;
                if progress.success_hashes.len() < 2 {
                    progress.success_hashes.push(p.problem_hash.clone())
                } else {
                    if progress.review_hashes.len() == 256 {
                        return Err("The local review record is full. Export your study before extending the record format.".into());
                    }
                    progress.review_hashes.push(p.problem_hash.clone());
                    if let Some(review) = &mut progress.review {
                        if day >= review.next_due_epoch_days
                            && day >= review.last_reviewed_epoch_days
                        {
                            review.on_success(day)
                        }
                    }
                }
                ensure_review(&def.id, progress, day);
            }
        }
        _ => {}
    }
    validate(&s)?;
    Ok(s)
}
#[derive(Clone, Debug, Serialize)]
pub struct StepView {
    #[serde(flatten)]
    pub definition: StepDefinition,
    pub studied: bool,
    pub reflection_recorded: bool,
    pub independent_examples: usize,
    pub practice_complete: bool,
    pub repair_needed: bool,
    pub due: bool,
    pub next_due_day: Option<i64>,
    pub recommended_ready: bool,
}
#[derive(Debug, Serialize)]
pub struct JourneyView {
    pub policy_version: String,
    pub active_step: String,
    pub steps: Vec<StepView>,
    pub completed_practice_loops: usize,
    pub recommendation: Recommendation,
}
#[derive(Debug, Serialize)]
pub struct Recommendation {
    pub step_id: String,
    pub action: String,
    pub reason: String,
}
pub fn snapshot(s: &JourneyState, day: i64) -> Result<JourneyView, String> {
    validate(s)?;
    valid_day(day)?;
    let mut ready = true;
    let mut views = Vec::new();
    for def in catalog() {
        let p = s.steps.get(&def.id).cloned().unwrap_or_default();
        let complete = loop_complete(&p);
        let due = p.review.as_ref().is_some_and(|r| r.is_due(day));
        views.push(StepView {
            definition: def,
            studied: p.studied,
            reflection_recorded: p.reflection_recorded,
            independent_examples: p.success_hashes.len(),
            practice_complete: complete,
            repair_needed: p.repair_needed,
            due,
            next_due_day: p.review.map(|r| r.next_due_epoch_days),
            recommended_ready: ready,
        });
        ready = ready && complete && !p.repair_needed;
    }
    let target = views
        .iter()
        .find(|v| v.repair_needed)
        .or_else(|| {
            views
                .iter()
                .filter(|v| v.due)
                .min_by_key(|v| v.next_due_day)
        })
        .or_else(|| {
            views
                .iter()
                .find(|v| v.recommended_ready && !v.practice_complete)
        })
        .unwrap_or_else(|| views.last().unwrap());
    let (action, reason) = if target.repair_needed {
        (
            "repair",
            "Revisit the worked explanation, then try a fresh example. Your earlier work is kept.",
        )
    } else if target.due {
        ("review","This idea is due for retrieval using your device’s study calendar. Try a fresh example before looking at help.")
    } else if !target.studied {
        (
            "lesson",
            "Build the next foundation with its explanation and worked example.",
        )
    } else if !target.reflection_recorded {
        ("reflection","Record how the idea works in your own words. This reflection is saved, not independently graded.")
    } else if !target.practice_complete {
        ("practice","Try two different examples without requesting hints or worked solutions. A correct result is only one part of learning.")
    } else {
        ("enrichment","Your current foundation practice loops are complete. Revisit an explanation, try a puzzle, or return for scheduled review.")
    };
    Ok(JourneyView {
        policy_version: POLICY.into(),
        active_step: s.active_step.clone(),
        completed_practice_loops: views.iter().filter(|v| v.practice_complete).count(),
        recommendation: Recommendation {
            step_id: target.definition.id.clone(),
            action: action.into(),
            reason: reason.into(),
        },
        steps: views,
    })
}
#[cfg(test)]
mod tests {
    use super::*;
    fn example(seed: u32) -> ProblemArtifact {
        axiom_math::generate(
            "place_value.decompose",
            "1.1.0",
            &format!("{seed:064x}"),
            1,
            "test",
        )
        .unwrap()
    }
    fn checked(s: JourneyState, seed: u32, assisted: bool, day: i64) -> JourneyState {
        let p = example(seed);
        let o = axiom_math::check(&p, p.target_value.as_ref().unwrap(), &Default::default());
        record(s, &p, &o, assisted, day).unwrap()
    }
    fn complete(day: i64) -> JourneyState {
        let id = catalog()[0].id.clone();
        let s = action(
            Default::default(),
            StudyAction::AcknowledgeLesson {
                step_id: id.clone(),
            },
            day,
        )
        .unwrap();
        let s = action(
            s,
            StudyAction::Reflection {
                step_id: id,
                recorded: true,
            },
            day,
        )
        .unwrap();
        checked(checked(s, 1, false, day), 2, false, day)
    }
    #[test]
    fn real_checks_and_reflection_required() {
        let s = checked(Default::default(), 1, true, 100);
        assert_eq!(snapshot(&s, 100).unwrap().steps[0].independent_examples, 0);
        let s = checked(checked(s, 1, false, 100), 1, false, 100);
        assert_eq!(snapshot(&s, 100).unwrap().steps[0].independent_examples, 1);
        assert!(!snapshot(&s, 100).unwrap().steps[0].practice_complete);
        let s = complete(100);
        let v = snapshot(&s, 100).unwrap();
        assert_eq!(v.completed_practice_loops, 1);
        assert_eq!(v.recommendation.step_id, catalog()[1].id);
        assert_eq!(v.steps[0].next_due_day, Some(101))
    }
    #[test]
    fn reviews_require_due_date_and_unseen_unassisted_item() {
        let s = complete(100);
        let s = checked(s, 3, false, 100);
        assert_eq!(snapshot(&s, 100).unwrap().steps[0].next_due_day, Some(101));
        let s = checked(s, 1, false, 101);
        assert!(snapshot(&s, 101).unwrap().steps[0].due);
        let s = checked(s, 4, true, 101);
        assert!(snapshot(&s, 101).unwrap().steps[0].due);
        let s = checked(s, 4, false, 101);
        assert_eq!(snapshot(&s, 101).unwrap().steps[0].next_due_day, Some(104));
        let s = checked(s, 4, false, 104);
        assert!(snapshot(&s, 104).unwrap().steps[0].due)
    }
    #[test]
    fn failures_prioritize_repair_and_preserve_progress() {
        let s = complete(100);
        let p = example(5);
        let o = axiom_math::check(&p, "-1", &Default::default());
        let s = record(s, &p, &o, false, 101).unwrap();
        let v = snapshot(&s, 101).unwrap();
        assert_eq!(v.recommendation.action, "repair");
        assert_eq!(v.completed_practice_loops, 1);
        let s = checked(s, 6, false, 101);
        assert!(!snapshot(&s, 101).unwrap().steps[0].repair_needed)
    }
    #[test]
    fn rejects_corrupt_or_misattributed_evidence() {
        let s = Default::default();
        let mut p = example(1);
        p.target_value = Some("999999999".into());
        let o = GradeOutcome::correct("test", "fake");
        assert!(record(s, &p, &o, false, 100).is_err());
        let mut s = complete(100);
        s.steps
            .values_mut()
            .next()
            .unwrap()
            .review
            .as_mut()
            .unwrap()
            .ladder_step = 99;
        assert!(validate(&s).is_err());
        assert!(snapshot(&Default::default(), -1).is_err())
    }
}
