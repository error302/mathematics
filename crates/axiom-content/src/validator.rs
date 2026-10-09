//! Content and manifest validation, prerequisite cycle detection (Section 12.2).

use crate::manifest::LessonManifest;
use axiom_math::jcs::canonical_hash;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum ValidationError {
    #[error("manifest schema version {0} is not supported")]
    UnsupportedSchema(u32),
    #[error("field '{0}' cannot be empty")]
    EmptyField(String),
    #[error("prerequisite dependency cycle detected involving skill: {0}")]
    DependencyCycle(String),
    #[error("unsupported exercise template: {0}")]
    UnsupportedTemplate(String),
    #[error("manifest hashing failed: {0}")]
    Hashing(String),
}

const SUPPORTED_TEMPLATES: &[&str] = &[
    "fractions.compare.positive",
    "fractions.equivalent.find",
    "arithmetic.whole.addition",
    "abacus.read.state",
];

/// Validates a single lesson manifest.
pub fn validate_manifest(manifest: &LessonManifest) -> Result<String, ValidationError> {
    if manifest.schema_version != 1 {
        return Err(ValidationError::UnsupportedSchema(manifest.schema_version));
    }
    if manifest.id.trim().is_empty() {
        return Err(ValidationError::EmptyField("id".into()));
    }
    if manifest.course_id.trim().is_empty() {
        return Err(ValidationError::EmptyField("course_id".into()));
    }
    if manifest.skill_ids.is_empty() {
        return Err(ValidationError::EmptyField("skill_ids".into()));
    }

    for ex in &manifest.exercise_refs {
        if !SUPPORTED_TEMPLATES.contains(&ex.template_id.as_str()) {
            return Err(ValidationError::UnsupportedTemplate(ex.template_id.clone()));
        }
    }

    let val = serde_json::to_value(manifest)
        .map_err(|e| ValidationError::Hashing(e.to_string()))?;
    canonical_hash(&val).map_err(|e| ValidationError::Hashing(e.to_string()))
}

/// Checks that a set of skill/lesson prerequisite edges forms a directed acyclic graph (DAG).
pub fn check_acyclic_dependencies(
    edges: &HashMap<String, Vec<String>>,
) -> Result<(), ValidationError> {
    // DFS with 3-color cycle detection (white=unvisited, gray=visiting, black=visited)
    #[derive(Clone, Copy, PartialEq)]
    enum Color {
        White,
        Gray,
        Black,
    }

    let mut colors: HashMap<&str, Color> = HashMap::new();
    for node in edges.keys() {
        colors.insert(node.as_str(), Color::White);
    }

    fn visit<'a>(
        node: &'a str,
        edges: &'a HashMap<String, Vec<String>>,
        colors: &mut HashMap<&'a str, Color>,
    ) -> Result<(), ValidationError> {
        colors.insert(node, Color::Gray);
        if let Some(deps) = edges.get(node) {
            for dep in deps {
                let dep_str = dep.as_str();
                match colors.get(dep_str) {
                    Some(Color::Gray) => {
                        return Err(ValidationError::DependencyCycle(dep.clone()));
                    }
                    Some(Color::White) | None => {
                        visit(dep_str, edges, colors)?;
                    }
                    Some(Color::Black) => {}
                }
            }
        }
        colors.insert(node, Color::Black);
        Ok(())
    }

    for node in edges.keys() {
        if colors.get(node.as_str()) == Some(&Color::White) {
            visit(node.as_str(), edges, &mut colors)?;
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::manifest::{ExerciseRef, Prerequisites};

    fn sample_manifest() -> LessonManifest {
        LessonManifest {
            schema_version: 1,
            id: "foundation.fractions.compare.lesson".into(),
            version: "1.0.0".into(),
            locale: "en".into(),
            status: "published".into(),
            course_id: "F04".into(),
            unit_id: "foundation.fractions.compare".into(),
            outcomes: vec!["Compare positive rational numbers using equivalent fractions.".into()],
            skill_ids: vec!["foundation.fractions.compare".into()],
            prerequisites: Prerequisites {
                required: vec!["foundation.fractions.equivalence".into()],
                recommended: vec!["foundation.numberline.order".into()],
            },
            exercise_refs: vec![ExerciseRef {
                template_id: "fractions.compare.positive".into(),
                template_version: "1.0.0".into(),
                difficulty_band: 1,
            }],
            assessment_dimensions: vec!["conceptual".into(), "procedural".into(), "transfer".into()],
            content_file: "lesson.en.md".into(),
            license_id: "axiom-original-content".into(),
            source_refs: Vec::new(),
            review_record_ids: vec!["review-math-001".into()],
        }
    }

    #[test]
    fn valid_manifest_hashes_consistently() {
        let m = sample_manifest();
        let h1 = validate_manifest(&m).unwrap();
        let h2 = validate_manifest(&m).unwrap();
        assert_eq!(h1, h2);
        assert_eq!(h1.len(), 64);
    }

    #[test]
    fn cycle_detection_catches_cycles() {
        let mut edges = HashMap::new();
        edges.insert("skill_a".into(), vec!["skill_b".into()]);
        edges.insert("skill_b".into(), vec!["skill_c".into()]);
        edges.insert("skill_c".into(), vec!["skill_a".into()]); // cycle!

        assert!(matches!(
            check_acyclic_dependencies(&edges),
            Err(ValidationError::DependencyCycle(_))
        ));

        // DAG without cycle
        let mut dag = HashMap::new();
        dag.insert("skill_a".into(), vec!["skill_b".into()]);
        dag.insert("skill_b".into(), vec!["skill_c".into()]);
        dag.insert("skill_c".into(), vec![]);
        assert!(check_acyclic_dependencies(&dag).is_ok());
    }
}
