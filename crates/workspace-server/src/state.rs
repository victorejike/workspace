use std::sync::Arc;
use workspace_ai::AiClient;
use workspace_db::prelude::*;

use crate::auth::SecurityService;

#[derive(Clone)]
pub struct AppState {
    pub user_repo: Arc<SqliteUserRepository>,
    pub curriculum_repo: Arc<SqliteCurriculumRepository>,
    pub task_repo: Arc<SqliteTaskRepository>,
    pub focus_repo: Arc<SqliteFocusRepository>,
    pub security: Arc<SecurityService>,
    pub ai_client: Arc<AiClient>,
}

impl AppState {
    pub fn new(db: WorkspaceDb, jwt_secret: String) -> Self {
        let pool = db.pool().clone();
        Self {
            user_repo: Arc::new(SqliteUserRepository::new(pool.clone())),
            curriculum_repo: Arc::new(SqliteCurriculumRepository::new(pool.clone())),
            task_repo: Arc::new(SqliteTaskRepository::new(pool.clone())),
            focus_repo: Arc::new(SqliteFocusRepository::new(pool)),
            security: Arc::new(SecurityService::new(jwt_secret)),
            ai_client: Arc::new(AiClient::default()),
        }
    }
}
