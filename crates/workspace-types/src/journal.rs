use chrono::{DateTime, NaiveDate, Utc};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::user::UserId;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct DailyReport {
    pub id: Uuid,
    pub user_id: UserId,
    pub report_date: NaiveDate,
    pub title: String,
    pub content_markdown: String,
    pub deep_work_minutes: u32,
    pub tasks_completed_count: u32,
    pub published_to_github: bool,
    pub github_commit_sha: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Structured journal input used by the markdown generator.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct JournalEntryData {
    pub day_number: u32,
    pub topic_title: String,
    pub date: NaiveDate,
    pub learning_objectives: Vec<String>,
    pub completed_activities: Vec<String>,
    pub assessment_status: String,
    pub knowledge_gaps: Vec<String>,
    pub time_spent_minutes: u32,
    pub achievements: Vec<String>,
    pub next_objectives: Vec<String>,
}
