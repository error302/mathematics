//! AXIOM Mathematics Academy — Pure Mathematical Core (Section 13)
//!
//! Provides exact arithmetic over Q, bounded scalar grammar, deterministic RNG v1,
//! RFC 8785 JSON canonical hashing, Soroban abacus reducer, exercise generation,
//! and answer checking with actionable mathematical feedback.
//!
//! Has no dependencies on HTTP, database, DOM, system clock, or ambient randomness.

pub mod abacus;
pub mod checker;
pub mod exercise;
pub mod jcs;
pub mod number;
pub mod parse;
pub mod rng;

pub use abacus::{apply_abacus_action, AbacusAction, AbacusState, RodState, TransitionFailure};
pub use checker::{check, Disposition, GradeOutcome};
pub use exercise::{generate, GenerationFailure, ProblemArtifact, ReplayIdentity};
pub use jcs::{canonical_hash, canonicalize, parse_strict, CanonicalError};
pub use number::{from_canonical, to_canonical, to_finite_decimal, to_latex, to_speech, Q};
pub use parse::{evaluate, normalize, parse_scalar, Ast, EvalFailure, MathBudget, ParseFailure};
pub use rng::{decode_seed, shuffle, uniform_inclusive, AxiomRng, DrawSource, GENERATOR_VERSION};
