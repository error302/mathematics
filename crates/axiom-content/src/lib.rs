//! AXIOM Content Management and Validation (Section 12)

pub mod manifest;
pub mod validator;

pub use manifest::{ExerciseRef, LessonContent, LessonManifest, Prerequisites};
pub use validator::{check_acyclic_dependencies, validate_manifest, ValidationError};
