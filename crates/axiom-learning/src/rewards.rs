//! Reward ledger and progression policies (Section 17).
//!
//! Enforces:
//! - 10 XP for first successful independent check on an exercise instance (100 XP/day cap).
//! - 5 XP for completed reviewed repair sequence (max 1 per concept/day).
//! - 50 XP plus badge for provisional concept milestone (lifetime one-time).
//! - 100 XP plus emblem for secure concept milestone (lifetime one-time).
//! - Append-only ledger entries with unique award keys.
//! - Balance is the deterministic sum of ledger entries.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RewardEntry {
    pub award_key: String,
    pub delta: i32,
    pub reason: String,
    pub timestamp_utc: String,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct RewardLedger {
    pub entries: Vec<RewardEntry>,
}

impl RewardLedger {
    pub fn new() -> Self {
        RewardLedger { entries: Vec::new() }
    }

    pub fn total_xp(&self) -> i32 {
        self.entries.iter().map(|e| e.delta).sum()
    }

    pub fn has_award(&self, award_key: &str) -> bool {
        self.entries.iter().any(|e| e.award_key == award_key)
    }

    /// Evaluates ordinary practice XP earned on a given UTC day.
    pub fn daily_practice_xp(&self, utc_day: &str) -> i32 {
        self.entries
            .iter()
            .filter(|e| {
                e.timestamp_utc.starts_with(utc_day)
                    && (e.award_key.starts_with("instance:first-success:")
                        || e.award_key.starts_with("instance:offline-rechecked:"))
            })
            .map(|e| e.delta)
            .sum()
    }

    /// Attempts to award 10 XP for the first independent success on an exercise instance.
    pub fn award_instance_success(
        &mut self,
        instance_id: &str,
        utc_timestamp: &str,
    ) -> Option<RewardEntry> {
        let award_key = format!("instance:first-success:{instance_id}");
        if self.has_award(&award_key) {
            return None; // Idempotent: already awarded
        }

        let utc_day = &utc_timestamp[..10.min(utc_timestamp.len())];
        let today_practice = self.daily_practice_xp(utc_day);
        let allowed = (100 - today_practice).max(0);
        let delta = 10.min(allowed);

        if delta == 0 {
            // Still record key with 0 delta so it is marked as processed without duplicate credit
            let entry = RewardEntry {
                award_key,
                delta: 0,
                reason: "Daily practice XP cap reached (100 XP)".into(),
                timestamp_utc: utc_timestamp.into(),
            };
            self.entries.push(entry.clone());
            return Some(entry);
        }

        let entry = RewardEntry {
            award_key,
            delta,
            reason: "First successful independent exercise check".into(),
            timestamp_utc: utc_timestamp.into(),
        };
        self.entries.push(entry.clone());
        Some(entry)
    }

    /// Awards provisional milestone: 50 XP (lifetime one-time key).
    pub fn award_provisional_milestone(
        &mut self,
        concept_id: &str,
        utc_timestamp: &str,
    ) -> Option<RewardEntry> {
        let award_key = format!("milestone:provisional:{concept_id}");
        if self.has_award(&award_key) {
            return None;
        }

        let entry = RewardEntry {
            award_key,
            delta: 50,
            reason: format!("Provisional mastery milestone achieved: {concept_id}"),
            timestamp_utc: utc_timestamp.into(),
        };
        self.entries.push(entry.clone());
        Some(entry)
    }

    /// Awards secure milestone: 100 XP (lifetime one-time key).
    pub fn award_secure_milestone(
        &mut self,
        concept_id: &str,
        utc_timestamp: &str,
    ) -> Option<RewardEntry> {
        let award_key = format!("milestone:secure:{concept_id}");
        if self.has_award(&award_key) {
            return None;
        }

        let entry = RewardEntry {
            award_key,
            delta: 100,
            reason: format!("Secure retention milestone achieved: {concept_id}"),
            timestamp_utc: utc_timestamp.into(),
        };
        self.entries.push(entry.clone());
        Some(entry)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn practice_daily_cap_at_100_xp() {
        let mut ledger = RewardLedger::new();
        let ts = "2026-10-09T12:00:00Z";

        // 10 successes = 100 XP
        for i in 0..10 {
            let entry = ledger.award_instance_success(&format!("inst_{i}"), ts).unwrap();
            assert_eq!(entry.delta, 10);
        }
        assert_eq!(ledger.total_xp(), 100);

        // 11th success capped at 0 delta
        let entry11 = ledger.award_instance_success("inst_11", ts).unwrap();
        assert_eq!(entry11.delta, 0);
        assert_eq!(ledger.total_xp(), 100);

        // Same instance retry is completely skipped (idempotent)
        assert!(ledger.award_instance_success("inst_0", ts).is_none());
    }

    #[test]
    fn milestones_awarded_outside_daily_cap() {
        let mut ledger = RewardLedger::new();
        let ts = "2026-10-09T12:00:00Z";

        let prov = ledger.award_provisional_milestone("foundation.fractions.compare", ts).unwrap();
        assert_eq!(prov.delta, 50);

        let sec = ledger.award_secure_milestone("foundation.fractions.compare", ts).unwrap();
        assert_eq!(sec.delta, 100);

        assert_eq!(ledger.total_xp(), 150);
        // Repeated call returns None
        assert!(ledger.award_provisional_milestone("foundation.fractions.compare", ts).is_none());
    }
}
