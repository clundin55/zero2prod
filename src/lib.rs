use std::net::TcpListener;

use anyhow::Result;
use axum::{routing::get, Router};

pub async fn health_check() {}

pub async fn run(listener: TcpListener) -> Result<()> {
    let app = Router::new().route("/health_check", get(health_check));
    axum::Server::bind(&listener.local_addr()?)
        .serve(app.into_make_service())
        .await?;
    Ok(())
}
