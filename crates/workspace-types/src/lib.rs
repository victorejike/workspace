pub mod assessment;
pub mod curriculum;
pub mod focus;
pub mod integration;
pub mod journal;
pub mod mastery;
pub mod resource;
pub mod schedule;
pub mod task;
pub mod user;

pub use assessment::*;
pub use curriculum::*;
pub use focus::*;
pub use integration::*;
pub use journal::*;
pub use mastery::*;
pub use resource::*;
pub use schedule::*;
pub use task::*;
pub use user::*;

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{NaiveDate, Utc};

    #[test]
    fn test_user_preferences_default() {
        let prefs = UserPreferences::default();
        assert_eq!(prefs.theme, ThemePreference::Dark);
        assert_eq!(prefs.preferred_daily_hours, 2.0);
        assert_eq!(prefs.available_days_per_week, 5);
        assert!(prefs.notifications_enabled);
    }

    #[test]
    fn test_task_serialization_roundtrip() {
        let user_id = UserId::new();
        let task = DailyTask {
            id: TaskId::new(),
            user_id,
            topic_id: None,
            title: "Study Rust Concurrency".to_string(),
            description: "Understand channels and Arc<Mutex<T>>".to_string(),
            task_type: TaskType::Study,
            priority: TaskPriority::High,
            status: TaskStatus::Pending,
            scheduled_date: NaiveDate::from_ymd_opt(2026, 11, 9).unwrap(),
            scheduled_time_start: Some("09:00".to_string()),
            scheduled_time_end: Some("11:00".to_string()),
            estimated_minutes: 120,
            actual_minutes: 0,
            skip_reason: None,
            completed_at: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        let json = serde_json::to_string(&task).expect("Serialization failed");
        let deserialized: DailyTask = serde_json::from_str(&json).expect("Deserialization failed");
        assert_eq!(task.title, deserialized.title);
        assert_eq!(task.priority, deserialized.priority);
    }
}
