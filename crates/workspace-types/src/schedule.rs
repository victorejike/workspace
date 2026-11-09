use chrono::{DateTime, NaiveDate, Utc};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::curriculum::CurriculumId;
use crate::user::UserId;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
pub struct GoalId(pub Uuid);

impl GoalId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Default for GoalId {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for GoalId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// A milestone goal set by the user.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct LearningGoal {
    pub id: GoalId,
    pub user_id: UserId,
    pub curriculum_id: Option<CurriculumId>,
    pub title: String,
    pub target_date: Option<NaiveDate>,
    pub target_hours: f32,
    pub current_hours: f32,
    pub is_completed: bool,
    pub created_at: DateTime<Utc>,
}

/// User's weekly availability schedule.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct LearningSchedule {
    pub user_id: UserId,
    pub daily_committed_hours: f32,
    pub available_days_per_week: u8,
    pub preferred_start_hour: u8, // e.g. 8 for 8 AM
    pub preferred_end_hour: u8,   // e.g. 20 for 8 PM
    pub buffer_revision_percentage: f32, // default 0.25 (25%)
}

impl Default for LearningSchedule {
    fn default() -> Self {
        Self {
            user_id: UserId::new(),
            daily_committed_hours: 2.0,
            available_days_per_week: 5,
            preferred_start_hour: 8,
            preferred_end_hour: 20,
            buffer_revision_percentage: 0.25,
        }
    }
}

/// Calculated realistic timeline projections.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ScheduleProjection {
    pub total_curriculum_hours: f32,
    pub daily_committed_hours: f32,
    pub theoretical_minimum_days: u32,
    pub realistic_projected_days: u32,
    pub estimated_completion_date: NaiveDate,
    pub buffer_days_allocated: u32,
    pub pace_warning: Option<String>,
}
