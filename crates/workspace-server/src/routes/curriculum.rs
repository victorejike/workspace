use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::Deserialize;
use serde_json::json;
use uuid::Uuid;
use workspace_db::prelude::*;
use workspace_types::{CurriculumId, DifficultyLevel, ModuleId, SourceType, UserId};

use crate::state::AppState;

#[derive(Deserialize)]
pub struct UserQuery {
    pub user_id: String,
}

#[derive(Deserialize)]
pub struct CreateCurriculumRequest {
    pub user_id: String,
    pub title: String,
    pub description: String,
    pub target_career_role: Option<String>,
}

#[derive(Deserialize)]
pub struct AddModuleRequest {
    pub title: String,
    pub description: String,
    pub order_index: u32,
    pub estimated_hours: f32,
}

#[derive(Deserialize)]
pub struct AddTopicRequest {
    pub title: String,
    pub description: String,
    pub order_index: u32,
    pub estimated_minutes: u32,
    pub difficulty: DifficultyLevel,
    pub key_concepts: Vec<String>,
    pub practical_exercises: Vec<String>,
}

pub async fn list_curricula(
    State(state): State<AppState>,
    Query(query): Query<UserQuery>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let uuid = Uuid::parse_str(&query.user_id).map_err(|_| {
        (
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": "Invalid user_id" })),
        )
    })?;

    let curricula = state
        .curriculum_repo
        .list_user_curricula(&UserId(uuid))
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": e.to_string() })),
            )
        })?;

    Ok(Json(curricula))
}

pub async fn create_curriculum(
    State(state): State<AppState>,
    Json(payload): Json<CreateCurriculumRequest>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let uuid = Uuid::parse_str(&payload.user_id).map_err(|_| {
        (
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": "Invalid user_id" })),
        )
    })?;

    let curriculum = state
        .curriculum_repo
        .create_curriculum(
            &UserId(uuid),
            &payload.title,
            &payload.description,
            SourceType::Manual,
            payload.target_career_role.as_deref(),
        )
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": e.to_string() })),
            )
        })?;

    Ok((StatusCode::CREATED, Json(curriculum)))
}

pub async fn get_curriculum_details(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let uuid = Uuid::parse_str(&id).map_err(|_| {
        (
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": "Invalid curriculum id" })),
        )
    })?;

    let details = state
        .curriculum_repo
        .get_curriculum_details(&CurriculumId(uuid))
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": e.to_string() })),
            )
        })?;

    match details {
        Some(d) => Ok(Json(d)),
        None => Err((
            StatusCode::NOT_FOUND,
            Json(json!({ "error": "Curriculum not found" })),
        )),
    }
}

pub async fn add_module(
    State(state): State<AppState>,
    Path(curriculum_id): Path<String>,
    Json(payload): Json<AddModuleRequest>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let curr_uuid = Uuid::parse_str(&curriculum_id).map_err(|_| {
        (
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": "Invalid curriculum id" })),
        )
    })?;

    let module = state
        .curriculum_repo
        .add_module(
            &CurriculumId(curr_uuid),
            &payload.title,
            &payload.description,
            payload.order_index,
            payload.estimated_hours,
        )
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": e.to_string() })),
            )
        })?;

    Ok((StatusCode::CREATED, Json(module)))
}

pub async fn add_topic(
    State(state): State<AppState>,
    Path(module_id): Path<String>,
    Json(payload): Json<AddTopicRequest>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let mod_uuid = Uuid::parse_str(&module_id).map_err(|_| {
        (
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": "Invalid module id" })),
        )
    })?;

    let topic = state
        .curriculum_repo
        .add_topic(
            &ModuleId(mod_uuid),
            &payload.title,
            &payload.description,
            payload.order_index,
            payload.estimated_minutes,
            payload.difficulty,
            &payload.key_concepts,
            &payload.practical_exercises,
        )
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": e.to_string() })),
            )
        })?;

    Ok((StatusCode::CREATED, Json(topic)))
}
