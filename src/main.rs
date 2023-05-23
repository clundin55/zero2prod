use std::net::TcpListener;
use zero2prod::configuration::get_config;
use zero2prod::startup::run;

use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let config = get_config()?;
    let listener = TcpListener::bind(format!("localhost:{}", config.application_port))?;
    run(listener).await
}
