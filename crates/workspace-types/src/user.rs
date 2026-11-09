use chrono::{DateTime, Utc};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Unique identifier for a User.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
pub struct UserId(pub Uuid);

impl UserId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Default for UserId {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for UserId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Core User authentication record.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: UserId,
    pub email: String,
    #[serde(skip_serializing)]
    pub password_hash: String,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Technical experience level of a user.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema, Default)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ExperienceLevel {
    #[default]
    Beginner,
    Intermediate,
    Advanced,
}

/// Personal profile details.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct UserProfile {
    pub user_id: UserId,
    pub display_name: String,
    pub avatar_url: Option<String>,
    pub timezone: String,
    pub experience_level: ExperienceLevel,
    pub career_goal: String,
    pub target_roles: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Theme preferences.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema, Default)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ThemePreference {
    #[default]
    Dark,
    Light,
    System,
}

/// User learning and productivity preferences.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct UserPreferences {
    pub user_id: UserId,
    pub theme: ThemePreference,
    pub preferred_daily_hours: f32,
    pub available_days_per_week: u8,
    pub quiet_hours_start: Option<String>, // e.g. "22:00"
    pub quiet_hours_end: Option<String>,   // e.g. "07:00"
    pub idle_timeout_minutes: u32,
    pub notifications_enabled: bool,
    pub updated_at: DateTime<Utc>,
}

impl Default for UserPreferences {
    fn default() -> Self {
        Self {
            user_id: UserId::new(),
            theme: ThemePreference::Dark,
            preferred_daily_hours: 2.0,
            available_days_per_week: 5,
            quiet_hours_start: Some("22:00".to_string()),
            quiet_hours_end: Some("07:00".to_string()),
            idle_timeout_minutes: 5,
            notifications_enabled: true,
            updated_at: Utc::now(),
        }
    }
}

/// Permission scopes for transparent tracking.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ConsentScope {
    IdleDetection,
    EditorActivity,
    BrowserDocumentationActivity,
    AiTutoringTelemetry,
}

/// User consent audit record.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct UserConsent {
    pub user_id: UserId,
    pub scope: ConsentScope,
    pub granted: bool,
    pub updated_at: DateTime<Utc>,
}
