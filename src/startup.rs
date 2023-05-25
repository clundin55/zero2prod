use anyhow::Result;
use axum::{
    routing::{get, post},
    Router,
};
use sqlx::PgPool;
use std::{net::TcpListener, sync::Arc};

use crate::routes::{health_check, subscribe, AppState};

pub async fn run(listener: TcpListener, conn: PgPool) -> Result<()> {
    let shared_state = Arc::new(AppState::with_conn(conn));
    let app = Router::new()
        .route("/health_check", get(health_check))
        .route("/subscriptions", post(subscribe))
        .with_state(shared_state);
    axum::Server::from_tcp(listener)?
        .serve(app.into_make_service())
        .await?;
    Ok(())
}
