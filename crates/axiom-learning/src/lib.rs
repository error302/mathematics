//! AXIOM Learning Policies (Sections 16 & 17)
//!
//! Provides adaptive mastery evidence modeling, review scheduling,
//! append-only reward ledger, and deterministic next-activity selection.

pub mod mastery;
pub mod rewards;
pub mod schedule;

pub use mastery::{ConceptMastery, Dimension, DimensionScore, MasteryState};
pub use rewards::{RewardEntry, RewardLedger};
pub use schedule::{select_next_activity, ActivityKind, RecommendedActivity, ReviewSchedule, INTERVAL_LADDER_DAYS};
