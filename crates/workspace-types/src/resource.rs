use chrono::{DateTime, NaiveDate, Utc};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::curriculum::TopicId;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
pub struct ResourceId(pub Uuid);

impl ResourceId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Default for ResourceId {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema, Default)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ResourceType {
    #[default]
    OfficialDocs,
    Tutorial,
    OpenSourceRepository,
    Course,
    StandardSpecification,
    ReleaseNotes,
}

/// A verified educational resource linked to a curriculum topic.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct Resource {
    pub id: ResourceId,
    pub topic_id: Option<TopicId>,
    pub title: String,
    pub description: String,
    pub url: String,
    pub publisher: String,
    pub resource_type: ResourceType,
    pub technology: String,
    pub difficulty_level: String,
    pub published_date: Option<NaiveDate>,
    pub last_updated_date: Option<NaiveDate>,
    pub verified_by_workspace_at: DateTime<Utc>,
    pub reliability_score: u8, // 0-100
    pub is_official: bool,
}
