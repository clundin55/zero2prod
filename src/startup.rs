use anyhow::Result;
use axum::{
    routing::{get, post},
    Router,
};
use std::net::TcpListener;

use crate::routes;

pub async fn run(listener: TcpListener) -> Result<()> {
    let app = Router::new()
        .route("/health_check", get(routes::health_check))
        .route("/subscriptions", post(routes::subscribe));
    axum::Server::from_tcp(listener)?
        .serve(app.into_make_service())
        .await?;
    Ok(())
}
