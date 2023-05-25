mod health_check;
mod subscriptions;

pub use health_check::*;
use sqlx::PgPool;
pub use subscriptions::*;

pub struct AppState {
    conn: PgPool,
}

impl AppState {
    pub fn with_conn(conn: PgPool) -> Self {
        AppState { conn }
    }
}
