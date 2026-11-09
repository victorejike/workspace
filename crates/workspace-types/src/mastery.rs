use chrono::{DateTime, NaiveDate, Utc};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::curriculum::TopicId;
use crate::user::UserId;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema, Default)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum MasteryState {
    #[default]
    Introduced,
    InProgress,
    NeedsRevision,
    Mastered,
}

/// Dynamic record of a user's mastery over a specific curriculum topic.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct SkillMastery {
    pub user_id: UserId,
    pub topic_id: TopicId,
    pub mastery_state: MasteryState,
    pub composite_score: u8,
    pub assessments_count: u32,
    pub practical_projects_count: u32,
    pub last_assessed_at: Option<DateTime<Utc>>,
    pub next_spaced_revision_date: Option<NaiveDate>,
    pub updated_at: DateTime<Utc>,
}

/// Graph representation of a skill with dependencies for visual roadmaps.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct SkillGraphNode {
    pub topic_id: TopicId,
    pub title: String,
    pub state: MasteryState,
    pub score: u8,
    pub prerequisites: Vec<TopicId>,
    pub is_unlocked: bool,
}
