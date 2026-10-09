//! Spaced retrieval review scheduling and activity recommender (Section 16.3 & 16.4).

use serde::{Deserialize, Serialize};

/// Review interval ladder in days: [1, 3, 7, 14, 30, 60, 120]
pub const INTERVAL_LADDER_DAYS: &[u32] = &[1, 3, 7, 14, 30, 60, 120];

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReviewSchedule {
    pub concept_id: String,
    pub ladder_step: usize,
    pub last_reviewed_epoch_days: i64,
    pub next_due_epoch_days: i64,
}

impl ReviewSchedule {
    pub fn new(concept_id: &str, current_epoch_days: i64) -> Self {
        let first_interval = INTERVAL_LADDER_DAYS[0] as i64;
        ReviewSchedule {
            concept_id: concept_id.into(),
            ladder_step: 0,
            last_reviewed_epoch_days: current_epoch_days,
            next_due_epoch_days: current_epoch_days + first_interval,
        }
    }

    pub fn is_due(&self, current_epoch_days: i64) -> bool {
        current_epoch_days >= self.next_due_epoch_days
    }

    /// Advances the schedule upon a successful independent review.
    pub fn on_success(&mut self, current_epoch_days: i64) {
        if self.ladder_step + 1 < INTERVAL_LADDER_DAYS.len() {
            self.ladder_step += 1;
        }
        let interval = INTERVAL_LADDER_DAYS[self.ladder_step] as i64;
        self.last_reviewed_epoch_days = current_epoch_days;
        self.next_due_epoch_days = current_epoch_days + interval;
    }

    /// Resets the schedule upon an incorrect review, recommending immediate repair.
    pub fn on_failure(&mut self, current_epoch_days: i64) {
        self.ladder_step = 0;
        let interval = INTERVAL_LADDER_DAYS[0] as i64;
        self.last_reviewed_epoch_days = current_epoch_days;
        self.next_due_epoch_days = current_epoch_days + interval;
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ActivityKind {
    DueReview,
    NextLearning,
    RepairTransfer,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecommendedActivity {
    pub kind: ActivityKind,
    pub concept_id: String,
    pub template_id: String,
    pub reason: String,
}

/// Deterministic next-activity selection order (Section 16.4):
/// 1. Unresolved repair
/// 2. Overdue review
/// 3. Next ready learning concept
pub fn select_next_activity(
    due_reviews: &[ReviewSchedule],
    active_concept: &str,
    has_repair_needed: bool,
) -> RecommendedActivity {
    if has_repair_needed {
        return RecommendedActivity {
            kind: ActivityKind::RepairTransfer,
            concept_id: active_concept.into(),
            template_id: "fractions.equivalent.find".into(),
            reason: "Targeted repair: strengthen common denominators to support fraction comparisons.".into(),
        };
    }

    if let Some(due) = due_reviews.first() {
        return RecommendedActivity {
            kind: ActivityKind::DueReview,
            concept_id: due.concept_id.clone(),
            template_id: "fractions.compare.positive".into(),
            reason: format!("Scheduled retrieval review for {}", due.concept_id),
        };
    }

    RecommendedActivity {
        kind: ActivityKind::NextLearning,
        concept_id: active_concept.into(),
        template_id: "fractions.compare.positive".into(),
        reason: "Continue with the fraction comparison learning journey.".into(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ladder_progression_and_reset() {
        let mut sched = ReviewSchedule::new("foundation.fractions.compare", 100);
        assert_eq!(sched.ladder_step, 0);
        assert_eq!(sched.next_due_epoch_days, 101); // 100 + 1

        // Success advances to 3 days
        sched.on_success(101);
        assert_eq!(sched.ladder_step, 1);
        assert_eq!(sched.next_due_epoch_days, 104); // 101 + 3

        // Failure resets to 1 day
        sched.on_failure(104);
        assert_eq!(sched.ladder_step, 0);
        assert_eq!(sched.next_due_epoch_days, 105); // 104 + 1
    }
}
