use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use chrono::NaiveDate;
use serde::Deserialize;
use serde_json::json;
use uuid::Uuid;
use workspace_db::prelude::*;
use workspace_types::{CreateTaskRequest, TaskId, UpdateTaskStatusRequest, UserId};

use crate::state::AppState;

#[derive(Deserialize)]
pub struct DailyTasksQuery {
    pub user_id: String,
    pub date: Option<String>,
}

#[derive(Deserialize)]
pub struct CreateTaskEnvelope {
    pub user_id: String,
    #[serde(flatten)]
    pub task: CreateTaskRequest,
}

#[derive(Deserialize)]
pub struct UpdateTaskStatusEnvelope {
    pub user_id: String,
    #[serde(flatten)]
    pub update: UpdateTaskStatusRequest,
}

pub async fn list_daily_tasks(
    State(state): State<AppState>,
    Query(query): Query<DailyTasksQuery>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let uuid = Uuid::parse_str(&query.user_id).map_err(|_| {
        (
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": "Invalid user_id" })),
        )
    })?;

    let date = match query.date {
        Some(ref d) => NaiveDate::parse_from_str(d, "%Y-%m-%d").unwrap_or_else(|_| {
            chrono::Utc::now().date_naive()
        }),
        None => chrono::Utc::now().date_naive(),
    };

    let tasks = state
        .task_repo
        .list_tasks_for_date(&UserId(uuid), date)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": e.to_string() })),
            )
        })?;

    Ok(Json(tasks))
}

pub async fn create_task(
    State(state): State<AppState>,
    Json(payload): Json<CreateTaskEnvelope>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let uuid = Uuid::parse_str(&payload.user_id).map_err(|_| {
        (
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": "Invalid user_id" })),
        )
    })?;

    let task = state
        .task_repo
        .create_task(&UserId(uuid), &payload.task)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": e.to_string() })),
            )
        })?;

    Ok((StatusCode::CREATED, Json(task)))
}

pub async fn update_task_status(
    State(state): State<AppState>,
    Path(task_id): Path<String>,
    Json(payload): Json<UpdateTaskStatusEnvelope>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let task_uuid = Uuid::parse_str(&task_id).map_err(|_| {
        (
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": "Invalid task id" })),
        )
    })?;

    let user_uuid = Uuid::parse_str(&payload.user_id).map_err(|_| {
        (
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": "Invalid user_id" })),
        )
    })?;

    state
        .task_repo
        .update_task_status(
            &UserId(user_uuid),
            &TaskId(task_uuid),
            &payload.update,
        )
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": e.to_string() })),
            )
        })?;

    Ok(Json(json!({ "status": "updated" })))
}
