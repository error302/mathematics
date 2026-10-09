//! Lesson manifest and exercise reference schema (Section 12.2).

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Prerequisites {
    pub required: Vec<String>,
    #[serde(default)]
    pub recommended: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExerciseRef {
    pub template_id: String,
    pub template_version: String,
    pub difficulty_band: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LessonManifest {
    pub schema_version: u32,
    pub id: String,
    pub version: String,
    pub locale: String,
    pub status: String,
    pub course_id: String,
    pub unit_id: String,
    pub outcomes: Vec<String>,
    pub skill_ids: Vec<String>,
    pub prerequisites: Prerequisites,
    pub exercise_refs: Vec<ExerciseRef>,
    pub assessment_dimensions: Vec<String>,
    pub content_file: String,
    pub license_id: String,
    #[serde(default)]
    pub source_refs: Vec<String>,
    #[serde(default)]
    pub review_record_ids: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LessonContent {
    pub title: String,
    pub summary: String,
    pub prediction_question: String,
    pub prediction_options: Vec<String>,
    pub prediction_explanation: String,
    pub concept_explanation: String,
    pub worked_example: String,
    pub reflection_prompt: String,
}
