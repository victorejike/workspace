use chrono::{DateTime, Utc};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::user::UserId;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
pub struct CurriculumId(pub Uuid);

impl CurriculumId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Default for CurriculumId {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for CurriculumId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
pub struct ModuleId(pub Uuid);

impl ModuleId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Default for ModuleId {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for ModuleId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
pub struct TopicId(pub Uuid);

impl TopicId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Default for TopicId {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for TopicId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema, Default)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CurriculumStatus {
    #[default]
    Draft,
    Active,
    Completed,
    Archived,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema, Default)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SourceType {
    #[default]
    Manual,
    Pdf,
    Docx,
    Markdown,
    Txt,
    Csv,
    Repository,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema, Default)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DifficultyLevel {
    #[default]
    Beginner,
    Intermediate,
    Advanced,
}

/// A structured Curriculum Roadmap.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct Curriculum {
    pub id: CurriculumId,
    pub user_id: UserId,
    pub title: String,
    pub description: String,
    pub source_type: SourceType,
    pub status: CurriculumStatus,
    pub target_career_role: Option<String>,
    pub estimated_total_hours: f32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// A high-level learning module within a curriculum.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct CurriculumModule {
    pub id: ModuleId,
    pub curriculum_id: CurriculumId,
    pub title: String,
    pub description: String,
    pub order_index: u32,
    pub estimated_hours: f32,
    pub created_at: DateTime<Utc>,
}

/// An actionable topic within a module.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct CurriculumTopic {
    pub id: TopicId,
    pub module_id: ModuleId,
    pub title: String,
    pub description: String,
    pub order_index: u32,
    pub estimated_minutes: u32,
    pub difficulty: DifficultyLevel,
    pub key_concepts: Vec<String>,
    pub practical_exercises: Vec<String>,
    pub created_at: DateTime<Utc>,
}

/// Prerequisite relationship: `topic_id` requires `prerequisite_topic_id`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct TopicPrerequisite {
    pub topic_id: TopicId,
    pub prerequisite_topic_id: TopicId,
}

/// Full hierarchical curriculum data model for UI rendering & AI output.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct CurriculumDetails {
    pub curriculum: Curriculum,
    pub modules: Vec<ModuleWithTopics>,
    pub prerequisites: Vec<TopicPrerequisite>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ModuleWithTopics {
    pub module: CurriculumModule,
    pub topics: Vec<CurriculumTopic>,
}
