use chrono::Utc;
use std::sync::Arc;
use uuid::Uuid;

use axum::extract::{Form, State};
use axum::http::StatusCode;
use axum::response::Result;

use super::AppState;

#[derive(serde::Deserialize, Debug)]
pub struct FormData {
    pub email: String,
    pub name: String,
}

pub async fn subscribe(
    state: State<Arc<AppState>>,
    user_data: Form<FormData>,
) -> Result<StatusCode> {
    sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        user_data.email,
        user_data.name,
        Utc::now()
    )
    .execute(&state.conn)
    .await
    .unwrap();

    println!("Received user_data: {:?}", user_data);
    Ok(StatusCode::OK)
}
