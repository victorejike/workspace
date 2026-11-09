use chrono::{DateTime, NaiveDate, Utc};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::curriculum::TopicId;
use crate::user::UserId;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
pub struct TaskId(pub Uuid);

impl TaskId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Default for TaskId {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for TaskId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema, Default)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TaskType {
    #[default]
    Study,
    Coding,
    Project,
    Assessment,
    Revision,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema, Default)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TaskPriority {
    Low,
    #[default]
    Important,
    High,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema, Default)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TaskStatus {
    #[default]
    Pending,
    InProgress,
    Paused,
    Completed,
    Skipped,
    Rescheduled,
}

/// An actionable daily task on the user's timeline.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct DailyTask {
    pub id: TaskId,
    pub user_id: UserId,
    pub topic_id: Option<TopicId>,
    pub title: String,
    pub description: String,
    pub task_type: TaskType,
    pub priority: TaskPriority,
    pub status: TaskStatus,
    pub scheduled_date: NaiveDate,
    pub scheduled_time_start: Option<String>, // e.g. "08:30"
    pub scheduled_time_end: Option<String>,   // e.g. "10:00"
    pub estimated_minutes: u32,
    pub actual_minutes: u32,
    pub skip_reason: Option<String>,
    pub completed_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct CreateTaskRequest {
    pub topic_id: Option<TopicId>,
    pub title: String,
    pub description: String,
    pub task_type: TaskType,
    pub priority: TaskPriority,
    pub scheduled_date: NaiveDate,
    pub scheduled_time_start: Option<String>,
    pub scheduled_time_end: Option<String>,
    pub estimated_minutes: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct UpdateTaskStatusRequest {
    pub status: TaskStatus,
    pub actual_minutes_delta: Option<u32>,
    pub skip_reason: Option<String>,
}
