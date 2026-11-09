use chrono::{DateTime, Utc};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::user::UserId;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
pub struct IntegrationId(pub Uuid);

impl IntegrationId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Default for IntegrationId {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum IntegrationProvider {
    GitHub,
    VsCode,
    Chrome,
    WhatsApp,
    Email,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct UserIntegration {
    pub id: IntegrationId,
    pub user_id: UserId,
    pub provider: IntegrationProvider,
    pub is_enabled: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GitHubLearningRepoConfig {
    pub integration_id: IntegrationId,
    pub repo_full_name: String, // e.g. "victorejike/workspace-learning"
    pub branch: String,
    pub documentation_root: String, // e.g. "workspace-learning/"
    pub auto_sync_on_completion: bool,
    pub last_synced_at: Option<DateTime<Utc>>,
}
