pub mod auth;
pub mod curriculum;
pub mod dashboard;
pub mod focus;
pub mod health;
pub mod task;

use axum::{
    routing::{get, patch, post},
    Router,
};

use crate::state::AppState;

pub fn build_api_router(state: AppState) -> Router {
    Router::new()
        // Health
        .route("/health", get(health::health_check))
        // Authentication
        .route("/api/v1/auth/register", post(auth::register))
        .route("/api/v1/auth/login", post(auth::login))
        // Dashboard (matches mockup)
        .route("/api/v1/dashboard", get(dashboard::get_dashboard))
        // Curricula & Topics
        .route("/api/v1/curricula", get(curriculum::list_curricula))
        .route("/api/v1/curricula", post(curriculum::create_curriculum))
        .route(
            "/api/v1/curricula/:id",
            get(curriculum::get_curriculum_details),
        )
        .route(
            "/api/v1/curricula/:id/modules",
            post(curriculum::add_module),
        )
        .route(
            "/api/v1/curricula/modules/:module_id/topics",
            post(curriculum::add_topic),
        )
        // Daily Tasks & Timeline
        .route("/api/v1/tasks/daily", get(task::list_daily_tasks))
        .route("/api/v1/tasks", post(task::create_task))
        .route("/api/v1/tasks/:id/status", patch(task::update_task_status))
        // Focus Sessions
        .route("/api/v1/focus/session/start", post(focus::start_session))
        .route("/api/v1/focus/session/tick", post(focus::tick_session))
        .route(
            "/api/v1/focus/session/complete",
            post(focus::complete_session),
        )
        .with_state(state)
}
