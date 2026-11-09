use chrono::{DateTime, Utc};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::curriculum::TopicId;
use crate::user::UserId;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
pub struct AssessmentId(pub Uuid);

impl AssessmentId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Default for AssessmentId {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for AssessmentId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
pub struct QuestionId(pub Uuid);

impl QuestionId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Default for QuestionId {
    fn default() -> Self {
        Self::new()
    }
}

/// Three-tier mastery classification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema, Default)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum MasteryLevel {
    #[default]
    NotMastered,
    NeedsRevision,
    Mastered,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct AssessmentQuestion {
    pub id: QuestionId,
    pub question_text: String,
    pub question_type: String, // "CONCEPTUAL_EXPLANATION", "CODE_IMPLEMENTATION", "DEBUGGING"
    pub expected_concepts: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct Assessment {
    pub id: AssessmentId,
    pub topic_id: TopicId,
    pub title: String,
    pub questions: Vec<AssessmentQuestion>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct UserAnswer {
    pub question_id: QuestionId,
    pub answer_text: String,
    pub code_snippet: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct AssessmentAttempt {
    pub id: Uuid,
    pub assessment_id: AssessmentId,
    pub user_id: UserId,
    pub score_percentage: u8,
    pub mastery_level: MasteryLevel,
    pub conceptual_accuracy_score: u8,
    pub practical_code_score: u8,
    pub feedback_markdown: String,
    pub identified_knowledge_gaps: Vec<String>,
    pub attempted_at: DateTime<Utc>,
}
