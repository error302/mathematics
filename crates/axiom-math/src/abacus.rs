//! Soroban abacus model and legal transition reducer (Section 10).
//!
//! Complies with the League of Japan Abacus Associations 1:4 bead design:
//! 1 upper bead (value 5) and 4 lower beads (value 1 each).
//! Each rod has exactly ten valid digit states (0 through 9).
//! No gaps permitted among engaged lower beads.
//! Canonical state serialization matches Section 10.1.

use crate::number::{frac, to_canonical, Q};
use num_bigint::BigInt;
use num_traits::{One, Signed, Zero};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct RodState {
    pub upper: bool,
    pub lower: u8,
}

impl RodState {
    pub fn new(upper: bool, lower: u8) -> Result<Self, TransitionFailure> {
        if lower > 4 {
            return Err(TransitionFailure::InvalidRodState {
                message: format!("lower beads count must be 0..=4, got {lower}"),
            });
        }
        Ok(RodState { upper, lower })
    }

    #[inline]
    pub fn digit(self) -> u8 {
        (if self.upper { 5 } else { 0 }) + self.lower
    }

    pub fn from_digit(d: u8) -> Result<Self, TransitionFailure> {
        if d > 9 {
            return Err(TransitionFailure::InvalidRodState {
                message: format!("digit must be 0..=9, got {d}"),
            });
        }
        Ok(RodState {
            upper: d >= 5,
            lower: d % 5,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AbacusState {
    pub schema_version: u32,
    pub rod_count: usize,
    pub rightmost_exponent: i32,
    pub sign: i8,
    pub rods: Vec<RodState>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum AbacusAction {
    /// Reset all rods to 0.
    Clear,
    /// Set a specific rod's state directly.
    SetRod { rod_index: usize, upper: bool, lower: u8 },
    /// Toggle the upper bead on a rod (subtract or add 5).
    ToggleUpper { rod_index: usize },
    /// Set lower beads count (0..=4) on a rod.
    SetLower { rod_index: usize, count: u8 },
    /// Move lower beads toward or away from reckoning bar.
    PushLower { rod_index: usize, count: u8 },
    PullLower { rod_index: usize, count: u8 },
}

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error, Serialize, Deserialize)]
pub enum TransitionFailure {
    #[error("rod index {rod_index} out of bounds (rod count is {rod_count})")]
    RodOutOfBounds { rod_index: usize, rod_count: usize },
    #[error("invalid rod state: {message}")]
    InvalidRodState { message: String },
    #[error("arithmetic overflow: value exceeds instrument rod capacity ({rod_count} rods)")]
    Overflow { rod_count: usize },
}

impl AbacusState {
    pub fn new(rod_count: usize, rightmost_exponent: i32) -> Self {
        assert!(rod_count > 0, "abacus must have at least 1 rod");
        AbacusState {
            schema_version: 1,
            rod_count,
            rightmost_exponent,
            sign: 1,
            rods: vec![RodState { upper: false, lower: 0 }; rod_count],
        }
    }

    /// Creates an abacus state initialized with the given exact non-negative integer or finite decimal.
    pub fn from_value(
        value: &Q,
        rod_count: usize,
        rightmost_exponent: i32,
    ) -> Result<Self, TransitionFailure> {
        let mut state = Self::new(rod_count, rightmost_exponent);
        if value.is_negative() {
            state.sign = -1;
        } else {
            state.sign = 1;
        }

        let abs_val = value.abs();
        // Scale by 10^(-rightmost_exponent) to convert to an integer representation of rods
        let scale = if rightmost_exponent >= 0 {
            Q::from_integer(BigInt::from(10).pow(rightmost_exponent as u32))
        } else {
            frac(1, 10).pow(-rightmost_exponent)
        };

        let scaled = abs_val / scale;
        if !scaled.denom().is_one() {
            return Err(TransitionFailure::InvalidRodState {
                message: "value has fractional digits beyond rightmost exponent".into(),
            });
        }

        let mut rem = scaled.to_integer();
        let ten = BigInt::from(10);

        for i in (0..rod_count).rev() {
            let digit_val = (&rem % &ten).to_string().parse::<u8>().unwrap_or(0);
            rem /= &ten;
            state.rods[i] = RodState::from_digit(digit_val)?;
        }

        if !rem.is_zero() {
            return Err(TransitionFailure::Overflow { rod_count });
        }

        Ok(state)
    }

    /// Evaluates the exact mathematical value represented on the rods.
    pub fn compute_value(&self) -> Q {
        let mut sum = Q::zero();
        let ten = BigInt::from(10);
        let n = self.rod_count;

        for (i, rod) in self.rods.iter().enumerate() {
            let digit = rod.digit();
            if digit == 0 {
                continue;
            }
            let exp = self.rightmost_exponent + (n as i32) - 1 - (i as i32);
            let place_weight = if exp >= 0 {
                Q::from_integer(ten.pow(exp as u32))
            } else {
                frac(1, 10).pow(-exp)
            };
            sum += Q::from_integer(BigInt::from(digit)) * place_weight;
        }

        if self.sign < 0 && !sum.is_zero() {
            -sum
        } else {
            sum
        }
    }

    /// Plain text speech announcement for accessibility (Section 10.3).
    /// Example: "Tens: 1. Ones: 5. Value: 15."
    pub fn describe_state(&self) -> String {
        let mut parts = Vec::new();
        let n = self.rod_count;
        for (i, rod) in self.rods.iter().enumerate() {
            let exp = self.rightmost_exponent + (n as i32) - 1 - (i as i32);
            let place_name = match exp {
                0 => "Ones".into(),
                1 => "Tens".into(),
                2 => "Hundreds".into(),
                3 => "Thousands".into(),
                -1 => "Tenths".into(),
                -2 => "Hundredths".into(),
                p if p > 3 => format!("10^{p}"),
                p => format!("10^({p})"),
            };
            parts.push(format!("{place_name}: {}", rod.digit()));
        }
        let val_str = to_canonical(&self.compute_value());
        format!("{}. Value: {}.", parts.join(". "), val_str)
    }
}

/// Applies an abacus action to produce a new immutable state.
pub fn apply_abacus_action(
    state: &AbacusState,
    action: &AbacusAction,
) -> Result<AbacusState, TransitionFailure> {
    let mut next = state.clone();
    match action {
        AbacusAction::Clear => {
            for rod in &mut next.rods {
                *rod = RodState { upper: false, lower: 0 };
            }
        }
        AbacusAction::SetRod { rod_index, upper, lower } => {
            if *rod_index >= next.rod_count {
                return Err(TransitionFailure::RodOutOfBounds {
                    rod_index: *rod_index,
                    rod_count: next.rod_count,
                });
            }
            next.rods[*rod_index] = RodState::new(*upper, *lower)?;
        }
        AbacusAction::ToggleUpper { rod_index } => {
            if *rod_index >= next.rod_count {
                return Err(TransitionFailure::RodOutOfBounds {
                    rod_index: *rod_index,
                    rod_count: next.rod_count,
                });
            }
            let current = next.rods[*rod_index];
            next.rods[*rod_index] = RodState {
                upper: !current.upper,
                lower: current.lower,
            };
        }
        AbacusAction::SetLower { rod_index, count } => {
            if *rod_index >= next.rod_count {
                return Err(TransitionFailure::RodOutOfBounds {
                    rod_index: *rod_index,
                    rod_count: next.rod_count,
                });
            }
            if *count > 4 {
                return Err(TransitionFailure::InvalidRodState {
                    message: format!("lower beads count must be 0..=4, got {count}"),
                });
            }
            next.rods[*rod_index].lower = *count;
        }
        AbacusAction::PushLower { rod_index, count } => {
            if *rod_index >= next.rod_count {
                return Err(TransitionFailure::RodOutOfBounds {
                    rod_index: *rod_index,
                    rod_count: next.rod_count,
                });
            }
            let cur = next.rods[*rod_index].lower;
            let sum = cur + *count;
            if sum > 4 {
                return Err(TransitionFailure::InvalidRodState {
                    message: format!("cannot push {count} lower beads; already at {cur} (max 4)"),
                });
            }
            next.rods[*rod_index].lower = sum;
        }
        AbacusAction::PullLower { rod_index, count } => {
            if *rod_index >= next.rod_count {
                return Err(TransitionFailure::RodOutOfBounds {
                    rod_index: *rod_index,
                    rod_count: next.rod_count,
                });
            }
            let cur = next.rods[*rod_index].lower;
            if *count > cur {
                return Err(TransitionFailure::InvalidRodState {
                    message: format!("cannot pull {count} lower beads; only {cur} engaged"),
                });
            }
            next.rods[*rod_index].lower = cur - *count;
        }
    }
    Ok(next)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::number::int;

    #[test]
    fn canonical_15_representation_from_spec() {
        // From Section 10.1: 5 rods, rightmost_exponent 0, representing 15
        let state = AbacusState::from_value(&int(15), 5, 0).unwrap();
        assert_eq!(state.rod_count, 5);
        assert_eq!(state.rightmost_exponent, 0);
        assert_eq!(state.sign, 1);
        assert_eq!(
            state.rods,
            vec![
                RodState { upper: false, lower: 0 },
                RodState { upper: false, lower: 0 },
                RodState { upper: false, lower: 0 },
                RodState { upper: false, lower: 1 },
                RodState { upper: true, lower: 0 },
            ]
        );
        assert_eq!(state.compute_value(), int(15));
    }

    #[test]
    fn decimal_radix_support() {
        // 2 rods with rightmost_exponent = -1: tenths rod
        let val = frac(25, 10); // 2.5
        let state = AbacusState::from_value(&val, 2, -1).unwrap();
        assert_eq!(state.rods[0].digit(), 2);
        assert_eq!(state.rods[1].digit(), 5);
        assert_eq!(state.compute_value(), frac(5, 2));
    }

    #[test]
    fn transition_actions_and_invariants() {
        let mut st = AbacusState::new(3, 0);
        // Push 3 lower on units (index 2)
        st = apply_abacus_action(&st, &AbacusAction::PushLower { rod_index: 2, count: 3 }).unwrap();
        assert_eq!(st.compute_value(), int(3));

        // Toggle upper on units (+5 -> 8)
        st = apply_abacus_action(&st, &AbacusAction::ToggleUpper { rod_index: 2 }).unwrap();
        assert_eq!(st.compute_value(), int(8));

        // Cannot push 2 more lower (3+2 = 5 > 4)
        assert!(apply_abacus_action(&st, &AbacusAction::PushLower { rod_index: 2, count: 2 }).is_err());

        // Clear resets everything
        st = apply_abacus_action(&st, &AbacusAction::Clear).unwrap();
        assert_eq!(st.compute_value(), int(0));
    }

    #[test]
    fn overflow_detected() {
        // Value 100 on a 2-rod abacus overflows
        let res = AbacusState::from_value(&int(100), 2, 0);
        assert!(matches!(res, Err(TransitionFailure::Overflow { rod_count: 2 })));
    }
}
