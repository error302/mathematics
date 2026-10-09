//! Adaptive mastery evidence model and state machine (Section 16).

use num_rational::BigRational;
use num_traits::{One, ToPrimitive, Zero};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Dimension {
    Conceptual,
    Procedural,
    Reasoning,
    Transfer,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MasteryState {
    NotStarted,
    Learning,
    Provisional,
    Secure,
    ReviewDue,
    NeedsRepair,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DimensionScore {
    pub dimension: Dimension,
    pub alpha: BigRational,
    pub beta: BigRational,
    pub eligible_units: u32,
    pub distinct_families: Vec<String>,
}

impl DimensionScore {
    pub fn new(dimension: Dimension) -> Self {
        DimensionScore {
            dimension,
            alpha: BigRational::one(),
            beta: BigRational::one(),
            eligible_units: 0,
            distinct_families: Vec::new(),
        }
    }

    /// Internal evidence index m = alpha / (alpha + beta)
    pub fn evidence_index(&self) -> f64 {
        let total = &self.alpha + &self.beta;
        if total.is_zero() {
            return 0.5;
        }
        let ratio = &self.alpha / &total;
        ratio.to_f64().unwrap_or(0.5)
    }

    pub fn record_evidence(&mut self, score: BigRational, weight: BigRational, family: &str) {
        let one = BigRational::one();
        self.alpha += &weight * &score;
        self.beta += &weight * (one - score);
        self.eligible_units += 1;
        if !self.distinct_families.iter().any(|f| f == family) {
            self.distinct_families.push(family.to_string());
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConceptMastery {
    pub concept_id: String,
    pub state: MasteryState,
    pub scores: Vec<DimensionScore>,
    pub provisional_reached: bool,
    pub secure_reached: bool,
    pub successful_retrievals: u32,
    pub transfer_passed: bool,
}

impl ConceptMastery {
    pub fn new(concept_id: &str) -> Self {
        ConceptMastery {
            concept_id: concept_id.into(),
            state: MasteryState::NotStarted,
            scores: vec![
                DimensionScore::new(Dimension::Conceptual),
                DimensionScore::new(Dimension::Procedural),
                DimensionScore::new(Dimension::Reasoning),
                DimensionScore::new(Dimension::Transfer),
            ],
            provisional_reached: false,
            secure_reached: false,
            successful_retrievals: 0,
            transfer_passed: false,
        }
    }

    pub fn get_score_mut(&mut self, dim: Dimension) -> &mut DimensionScore {
        if let Some(pos) = self.scores.iter().position(|s| s.dimension == dim) {
            &mut self.scores[pos]
        } else {
            self.scores.push(DimensionScore::new(dim));
            self.scores.last_mut().unwrap()
        }
    }

    pub fn evaluate_state(&mut self) -> MasteryState {
        if self.secure_reached {
            self.state = MasteryState::Secure;
            return self.state;
        }

        // Check provisional gate:
        // m >= 0.80 across required dimensions
        // >= 6 total units across all dimensions, >= 3 distinct families
        let total_units: u32 = self.scores.iter().map(|s| s.eligible_units).sum();
        let mut all_families = Vec::new();
        for s in &self.scores {
            for f in &s.distinct_families {
                if !all_families.contains(f) {
                    all_families.push(f.clone());
                }
            }
        }

        let procedural_idx = self.get_score_mut(Dimension::Procedural).evidence_index();
        let passes_provisional = procedural_idx >= 0.80
            && total_units >= 6
            && all_families.len() >= 3;

        if passes_provisional {
            self.provisional_reached = true;
            if self.successful_retrievals >= 2 && self.transfer_passed {
                self.secure_reached = true;
                self.state = MasteryState::Secure;
            } else {
                self.state = MasteryState::Provisional;
            }
        } else if total_units > 0 {
            self.state = MasteryState::Learning;
        } else {
            self.state = MasteryState::NotStarted;
        }

        self.state
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn evidence_prior_and_updates() {
        let mut score = DimensionScore::new(Dimension::Procedural);
        assert_eq!(score.evidence_index(), 0.5);

        // 6 successes: alpha becomes 1+6=7, beta remains 1. m = 7/8 = 0.875
        for i in 0..6 {
            score.record_evidence(BigRational::one(), BigRational::one(), &format!("fam_{i}"));
        }
        assert_eq!(score.evidence_index(), 0.875);
    }

    #[test]
    fn provisional_gate_logic() {
        let mut mastery = ConceptMastery::new("foundation.fractions.compare");
        assert_eq!(mastery.evaluate_state(), MasteryState::NotStarted);

        // Record 6 successes across 3 families
        for i in 0..6 {
            let fam = format!("fam_{}", i % 3);
            mastery.get_score_mut(Dimension::Procedural).record_evidence(
                BigRational::one(),
                BigRational::one(),
                &fam,
            );
        }

        assert_eq!(mastery.evaluate_state(), MasteryState::Provisional);
    }
}
