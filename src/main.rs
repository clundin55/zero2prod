use sqlx::PgPool;
use std::net::TcpListener;
use zero2prod::configuration::get_config;
use zero2prod::startup::run;

use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let config = get_config()?;
    let db_url = config.database.connection_string();
    let conn = PgPool::connect(&db_url)
        .await
        .expect("Unable to connect to postgre db");
    let listener = TcpListener::bind(format!("localhost:{}", config.application_port))?;
    run(listener, conn).await
}
