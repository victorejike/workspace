use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use workspace_db::prelude::UserRepository;

use crate::state::AppState;

#[derive(Deserialize)]
pub struct RegisterRequest {
    pub email: String,
    pub password: String,
    pub display_name: String,
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct AuthResponse {
    pub token: String,
    pub user_id: String,
    pub email: String,
    pub display_name: String,
}

pub async fn register(
    State(state): State<AppState>,
    Json(payload): Json<RegisterRequest>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    if payload.email.trim().is_empty() || payload.password.len() < 8 {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": "Email required and password must be at least 8 characters" })),
        ));
    }

    if let Ok(Some(_)) = state.user_repo.find_by_email(&payload.email).await {
        return Err((
            StatusCode::CONFLICT,
            Json(json!({ "error": "An account with this email already exists" })),
        ));
    }

    let password_hash = state
        .security
        .hash_password(&payload.password)
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": e })),
            )
        })?;

    let user = state
        .user_repo
        .create_user(&payload.email, &password_hash, &payload.display_name)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": e.to_string() })),
            )
        })?;

    let token = state.security.generate_jwt(&user.id).map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": e })),
        )
    })?;

    Ok((
        StatusCode::CREATED,
        Json(AuthResponse {
            token,
            user_id: user.id.to_string(),
            email: user.email,
            display_name: payload.display_name,
        }),
    ))
}

pub async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let user = match state.user_repo.find_by_email(&payload.email).await {
        Ok(Some(u)) => u,
        _ => {
            return Err((
                StatusCode::UNAUTHORIZED,
                Json(json!({ "error": "Invalid email or password" })),
            ));
        }
    };

    if !state
        .security
        .verify_password(&payload.password, &user.password_hash)
    {
        return Err((
            StatusCode::UNAUTHORIZED,
            Json(json!({ "error": "Invalid email or password" })),
        ));
    }

    let profile = state
        .user_repo
        .get_profile(&user.id)
        .await
        .ok()
        .flatten();
    let display_name = profile
        .map(|p| p.display_name)
        .unwrap_or_else(|| "User".to_string());

    let token = state.security.generate_jwt(&user.id).map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": e })),
        )
    })?;

    Ok(Json(AuthResponse {
        token,
        user_id: user.id.to_string(),
        email: user.email,
        display_name,
    }))
}
