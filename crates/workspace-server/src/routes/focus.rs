use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::Deserialize;
use serde_json::json;
use uuid::Uuid;
use workspace_db::prelude::*;
use workspace_types::{SessionId, TaskId, UserId};

use crate::state::AppState;

#[derive(Deserialize)]
pub struct StartSessionRequest {
    pub user_id: String,
    pub task_id: Option<String>,
}

#[derive(Deserialize)]
pub struct TickSessionRequest {
    pub session_id: String,
    pub active_seconds_delta: u32,
    pub idle_seconds_delta: u32,
}

#[derive(Deserialize)]
pub struct CompleteSessionRequest {
    pub session_id: String,
    pub notes: Option<String>,
}

pub async fn start_session(
    State(state): State<AppState>,
    Json(payload): Json<StartSessionRequest>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let user_uuid = Uuid::parse_str(&payload.user_id).map_err(|_| {
        (
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": "Invalid user_id" })),
        )
    })?;

    let task_id = if let Some(ref t_id) = payload.task_id {
        Uuid::parse_str(t_id).ok().map(TaskId)
    } else {
        None
    };

    let session = state
        .focus_repo
        .start_session(&UserId(user_uuid), task_id)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": e.to_string() })),
            )
        })?;

    Ok((StatusCode::CREATED, Json(session)))
}

pub async fn tick_session(
    State(state): State<AppState>,
    Json(payload): Json<TickSessionRequest>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let session_uuid = Uuid::parse_str(&payload.session_id).map_err(|_| {
        (
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": "Invalid session_id" })),
        )
    })?;

    state
        .focus_repo
        .update_session_ticks(
            &SessionId(session_uuid),
            payload.active_seconds_delta,
            payload.idle_seconds_delta,
        )
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": e.to_string() })),
            )
        })?;

    Ok(Json(json!({ "status": "recorded" })))
}

pub async fn complete_session(
    State(state): State<AppState>,
    Json(payload): Json<CompleteSessionRequest>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let session_uuid = Uuid::parse_str(&payload.session_id).map_err(|_| {
        (
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": "Invalid session_id" })),
        )
    })?;

    state
        .focus_repo
        .complete_session(&SessionId(session_uuid), payload.notes.as_deref())
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": e.to_string() })),
            )
        })?;

    Ok(Json(json!({ "status": "completed" })))
}
