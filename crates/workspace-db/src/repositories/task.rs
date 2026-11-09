use async_trait::async_trait;
use chrono::{DateTime, NaiveDate, Utc};
use sqlx::{Row, SqlitePool};
use uuid::Uuid;
use workspace_types::{
    CreateTaskRequest, DailyTask, TaskId, TaskPriority, TaskStatus, TaskType, TopicId,
    UpdateTaskStatusRequest, UserId,
};

use crate::error::DbError;

#[async_trait]
pub trait TaskRepository: Send + Sync {
    async fn create_task(
        &self,
        user_id: &UserId,
        req: &CreateTaskRequest,
    ) -> Result<DailyTask, DbError>;

    async fn list_tasks_for_date(
        &self,
        user_id: &UserId,
        date: NaiveDate,
    ) -> Result<Vec<DailyTask>, DbError>;

    async fn update_task_status(
        &self,
        user_id: &UserId,
        task_id: &TaskId,
        req: &UpdateTaskStatusRequest,
    ) -> Result<(), DbError>;
}

pub struct SqliteTaskRepository {
    pool: SqlitePool,
}

impl SqliteTaskRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl TaskRepository for SqliteTaskRepository {
    async fn create_task(
        &self,
        user_id: &UserId,
        req: &CreateTaskRequest,
    ) -> Result<DailyTask, DbError> {
        let id = TaskId::new();
        let now = Utc::now().to_rfc3339();
        let date_str = req.scheduled_date.to_string();

        let type_str = match req.task_type {
            TaskType::Study => "STUDY",
            TaskType::Coding => "CODING",
            TaskType::Project => "PROJECT",
            TaskType::Assessment => "ASSESSMENT",
            TaskType::Revision => "REVISION",
        };

        let prio_str = match req.priority {
            TaskPriority::Low => "LOW",
            TaskPriority::Important => "IMPORTANT",
            TaskPriority::High => "HIGH",
        };

        let topic_id_str = req.topic_id.map(|t| t.to_string());

        sqlx::query(
            "INSERT INTO daily_tasks (id, user_id, topic_id, title, description, task_type, priority, status, scheduled_date, scheduled_time_start, scheduled_time_end, estimated_minutes, actual_minutes, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, 'PENDING', ?, ?, ?, ?, 0, ?, ?)"
        )
        .bind(id.to_string())
        .bind(user_id.to_string())
        .bind(&topic_id_str)
        .bind(&req.title)
        .bind(&req.description)
        .bind(type_str)
        .bind(prio_str)
        .bind(&date_str)
        .bind(&req.scheduled_time_start)
        .bind(&req.scheduled_time_end)
        .bind(req.estimated_minutes as i32)
        .bind(&now)
        .bind(&now)
        .execute(&self.pool)
        .await?;

        let created_at = chrono::DateTime::parse_from_rfc3339(&now)
            .map(|d| d.with_timezone(&Utc))
            .unwrap_or_else(|_| Utc::now());

        Ok(DailyTask {
            id,
            user_id: *user_id,
            topic_id: req.topic_id,
            title: req.title.clone(),
            description: req.description.clone(),
            task_type: req.task_type,
            priority: req.priority,
            status: TaskStatus::Pending,
            scheduled_date: req.scheduled_date,
            scheduled_time_start: req.scheduled_time_start.clone(),
            scheduled_time_end: req.scheduled_time_end.clone(),
            estimated_minutes: req.estimated_minutes,
            actual_minutes: 0,
            skip_reason: None,
            completed_at: None,
            created_at,
            updated_at: created_at,
        })
    }

    async fn list_tasks_for_date(
        &self,
        user_id: &UserId,
        date: NaiveDate,
    ) -> Result<Vec<DailyTask>, DbError> {
        let date_str = date.to_string();

        let rows = sqlx::query(
            "SELECT id, user_id, topic_id, title, description, task_type, priority, status, scheduled_date, scheduled_time_start, scheduled_time_end, estimated_minutes, actual_minutes, skip_reason, completed_at, created_at, updated_at FROM daily_tasks WHERE user_id = ? AND scheduled_date = ? ORDER BY scheduled_time_start ASC, created_at ASC"
        )
        .bind(user_id.to_string())
        .bind(&date_str)
        .fetch_all(&self.pool)
        .await?;

        let mut tasks = Vec::new();
        for r in rows {
            let id_str: String = r.get("id");
            let id_uuid = Uuid::parse_str(&id_str).map_err(|e| DbError::NotFound(e.to_string()))?;

            let topic_id_str: Option<String> = r.get("topic_id");
            let topic_id = topic_id_str.and_then(|s| Uuid::parse_str(&s).ok().map(TopicId));

            let type_str: String = r.get("task_type");
            let task_type = match type_str.as_str() {
                "CODING" => TaskType::Coding,
                "PROJECT" => TaskType::Project,
                "ASSESSMENT" => TaskType::Assessment,
                "REVISION" => TaskType::Revision,
                _ => TaskType::Study,
            };

            let prio_str: String = r.get("priority");
            let priority = match prio_str.as_str() {
                "LOW" => TaskPriority::Low,
                "HIGH" => TaskPriority::High,
                _ => TaskPriority::Important,
            };

            let st_str: String = r.get("status");
            let status = match st_str.as_str() {
                "IN_PROGRESS" => TaskStatus::InProgress,
                "PAUSED" => TaskStatus::Paused,
                "COMPLETED" => TaskStatus::Completed,
                "SKIPPED" => TaskStatus::Skipped,
                "RESCHEDULED" => TaskStatus::Rescheduled,
                _ => TaskStatus::Pending,
            };

            let sched_date_str: String = r.get("scheduled_date");
            let scheduled_date = NaiveDate::parse_from_str(&sched_date_str, "%Y-%m-%d")
                .unwrap_or(date);

            let completed_str: Option<String> = r.get("completed_at");
            let completed_at = completed_str.and_then(|s| {
                DateTime::parse_from_rfc3339(&s).ok().map(|d| d.with_timezone(&Utc))
            });

            let created_str: String = r.get("created_at");
            let updated_str: String = r.get("updated_at");

            tasks.push(DailyTask {
                id: TaskId(id_uuid),
                user_id: *user_id,
                topic_id,
                title: r.get("title"),
                description: r.get("description"),
                task_type,
                priority,
                status,
                scheduled_date,
                scheduled_time_start: r.get("scheduled_time_start"),
                scheduled_time_end: r.get("scheduled_time_end"),
                estimated_minutes: r.get::<i32, _>("estimated_minutes") as u32,
                actual_minutes: r.get::<i32, _>("actual_minutes") as u32,
                skip_reason: r.get("skip_reason"),
                completed_at,
                created_at: chrono::DateTime::parse_from_rfc3339(&created_str)
                    .map(|d| d.with_timezone(&Utc))
                    .unwrap_or_else(|_| Utc::now()),
                updated_at: chrono::DateTime::parse_from_rfc3339(&updated_str)
                    .map(|d| d.with_timezone(&Utc))
                    .unwrap_or_else(|_| Utc::now()),
            });
        }

        Ok(tasks)
    }

    async fn update_task_status(
        &self,
        user_id: &UserId,
        task_id: &TaskId,
        req: &UpdateTaskStatusRequest,
    ) -> Result<(), DbError> {
        let status_str = match req.status {
            TaskStatus::Pending => "PENDING",
            TaskStatus::InProgress => "IN_PROGRESS",
            TaskStatus::Paused => "PAUSED",
            TaskStatus::Completed => "COMPLETED",
            TaskStatus::Skipped => "SKIPPED",
            TaskStatus::Rescheduled => "RESCHEDULED",
        };

        let now = Utc::now().to_rfc3339();
        let completed_at_str = if req.status == TaskStatus::Completed {
            Some(now.clone())
        } else {
            None
        };

        let delta_minutes = req.actual_minutes_delta.unwrap_or(0) as i32;

        sqlx::query(
            "UPDATE daily_tasks SET status = ?, actual_minutes = actual_minutes + ?, skip_reason = COALESCE(?, skip_reason), completed_at = COALESCE(?, completed_at), updated_at = ? WHERE id = ? AND user_id = ?"
        )
        .bind(status_str)
        .bind(delta_minutes)
        .bind(&req.skip_reason)
        .bind(&completed_at_str)
        .bind(&now)
        .bind(task_id.to_string())
        .bind(user_id.to_string())
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}
