pub mod auth;
pub mod routes;
pub mod state;

use std::net::SocketAddr;
use tower_http::cors::{Any, CorsLayer};
use tracing::info;
use workspace_db::WorkspaceDb;

use crate::routes::build_api_router;
use crate::state::AppState;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "workspace=debug,tower_http=debug,info".into()),
        )
        .init();

    info!("Initializing WORKSPACE Core Server...");

    let db_path = std::env::var("SQLITE_DB_PATH").unwrap_or_else(|_| "workspace.sqlite".to_string());
    let db_url = format!("sqlite://{}?mode=rwc", db_path);
    let db = WorkspaceDb::connect_sqlite(&db_url).await?;

    let jwt_secret = std::env::var("JWT_SECRET")
        .unwrap_or_else(|_| "workspace_local_dev_secret_key_minimum_32_chars_long".to_string());

    let state = AppState::new(db, jwt_secret);

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = build_api_router(state).layer(cors);

    let host = std::env::var("SERVER_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = std::env::var("SERVER_PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse::<u16>()
        .unwrap_or(3000);

    let addr: SocketAddr = format!("{}:{}", host, port).parse()?;
    info!("WORKSPACE Server listening on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
