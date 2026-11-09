use chrono::{DateTime, Utc};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::task::TaskId;
use crate::user::UserId;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
pub struct SessionId(pub Uuid);

impl SessionId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Default for SessionId {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for SessionId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema, Default)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ActivitySource {
    #[default]
    Manual,
    TauriDesktop,
    VsCodeExtension,
    ChromeExtension,
}

/// A recorded focus or learning session.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct FocusSession {
    pub id: SessionId,
    pub user_id: UserId,
    pub task_id: Option<TaskId>,
    pub start_time: DateTime<Utc>,
    pub end_time: Option<DateTime<Utc>>,
    pub active_seconds: u32,
    pub idle_seconds: u32,
    pub is_active: bool,
    pub attention_quality_pct: u8, // e.g. 88%
    pub focus_stability_score: u8, // e.g. 64/100
    pub notes: Option<String>,
}

/// Granular activity event during a session.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ActivityEvent {
    pub id: Uuid,
    pub session_id: SessionId,
    pub source: ActivitySource,
    pub event_type: String, // e.g. "CODING_ACTIVE", "DOCS_READING", "IDLE_DETECTED"
    pub metadata_json: Option<String>,
    pub timestamp: DateTime<Utc>,
}

/// Metrics rendered on the dashboard cards (Deep Work Time, Attention Quality, Focus Stability).
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct FocusDashboardMetrics {
    pub deep_work_seconds_today: u32,
    pub attention_quality_pct: u8,
    pub focus_stability_score: u8,
    pub peak_focus_hour: String, // e.g. "11 am"
    pub completed_goals_today: u32,
    pub total_goals_today: u32,
}
