pub mod error;
pub mod pool;
pub mod repositories;

pub use error::*;
pub use pool::*;
pub use repositories::*;

pub mod prelude {
    pub use crate::error::DbError;
    pub use crate::pool::WorkspaceDb;
    pub use crate::repositories::curriculum::{CurriculumRepository, SqliteCurriculumRepository};
    pub use crate::repositories::focus::{FocusRepository, SqliteFocusRepository};
    pub use crate::repositories::task::{TaskRepository, SqliteTaskRepository};
    pub use crate::repositories::user::{SqliteUserRepository, UserRepository};
}

#[cfg(test)]
mod tests {
    use super::prelude::*;
    use chrono::Utc;
    use workspace_types::{
        CreateTaskRequest, DifficultyLevel, SourceType, TaskPriority, TaskStatus, TaskType,
        UpdateTaskStatusRequest,
    };

    #[tokio::test]
    async fn test_full_repository_lifecycle_in_memory() {
        let db = WorkspaceDb::connect_in_memory()
            .await
            .expect("Failed to create in-memory database");

        let user_repo = SqliteUserRepository::new(db.pool().clone());
        let curr_repo = SqliteCurriculumRepository::new(db.pool().clone());
        let task_repo = SqliteTaskRepository::new(db.pool().clone());
        let focus_repo = SqliteFocusRepository::new(db.pool().clone());

        // 1. Create User
        let user = user_repo
            .create_user("learner@workspace.dev", "argon2hash", "Jane Developer")
            .await
            .expect("Failed to create user");

        assert_eq!(user.email, "learner@workspace.dev");

        // 2. Fetch Profile
        let profile = user_repo
            .get_profile(&user.id)
            .await
            .expect("Failed to fetch profile")
            .expect("Profile not found");
        assert_eq!(profile.display_name, "Jane Developer");

        // 3. Create Curriculum, Module, and Topic
        let curriculum = curr_repo
            .create_curriculum(
                &user.id,
                "Rust Systems Engineering",
                "Master systems programming in Rust",
                SourceType::Manual,
                Some("Systems Engineer"),
            )
            .await
            .expect("Failed to create curriculum");

        let module = curr_repo
            .add_module(&curriculum.id, "Ownership & Lifetimes", "Core memory model", 0, 20.0)
            .await
            .expect("Failed to add module");

        let topic = curr_repo
            .add_topic(
                &module.id,
                "Borrow Checker & References",
                "Deep dive into shared and mutable borrows",
                0,
                90,
                DifficultyLevel::Intermediate,
                &["borrowing".to_string(), "aliasing".to_string()],
                &["Implement a circular buffer".to_string()],
            )
            .await
            .expect("Failed to add topic");

        let details = curr_repo
            .get_curriculum_details(&curriculum.id)
            .await
            .expect("Failed to fetch curriculum details")
            .expect("Curriculum not found");

        assert_eq!(details.modules.len(), 1);
        assert_eq!(details.modules[0].topics.len(), 1);
        assert_eq!(details.modules[0].topics[0].title, "Borrow Checker & References");

        // 4. Create and update Daily Task
        let task_req = CreateTaskRequest {
            topic_id: Some(topic.id),
            title: "Study Borrow Checker".to_string(),
            description: "Work through chapter 4 of Rust Book".to_string(),
            task_type: TaskType::Study,
            priority: TaskPriority::Important,
            scheduled_date: Utc::now().date_naive(),
            scheduled_time_start: Some("09:00".to_string()),
            scheduled_time_end: Some("10:30".to_string()),
            estimated_minutes: 90,
        };

        let task = task_repo
            .create_task(&user.id, &task_req)
            .await
            .expect("Failed to create daily task");

        assert_eq!(task.priority, TaskPriority::Important);
        assert_eq!(task.status, TaskStatus::Pending);

        // Update task to completed
        task_repo
            .update_task_status(
                &user.id,
                &task.id,
                &UpdateTaskStatusRequest {
                    status: TaskStatus::Completed,
                    actual_minutes_delta: Some(85),
                    skip_reason: None,
                },
            )
            .await
            .expect("Failed to update task status");

        let tasks_today = task_repo
            .list_tasks_for_date(&user.id, Utc::now().date_naive())
            .await
            .expect("Failed to list tasks");
        assert_eq!(tasks_today.len(), 1);
        assert_eq!(tasks_today[0].status, TaskStatus::Completed);
        assert_eq!(tasks_today[0].actual_minutes, 85);

        // 5. Start Focus Session
        let session = focus_repo
            .start_session(&user.id, Some(task.id))
            .await
            .expect("Failed to start session");
        assert!(session.is_active);

        focus_repo
            .update_session_ticks(&session.id, 1800, 120) // 30m active, 2m idle
            .await
            .expect("Failed to update ticks");

        let metrics = focus_repo
            .get_today_dashboard_metrics(&user.id)
            .await
            .expect("Failed to fetch dashboard metrics");

        assert!(metrics.deep_work_seconds_today >= 1800);
        assert_eq!(metrics.completed_goals_today, 1);
    }
}
