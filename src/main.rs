use std::net::TcpListener;

use sqlx::PgPool;
use zero2prod::config::get_configuration;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // panic if we can't read config's
    let config = get_configuration().expect("Failed to read config");
    let connection_pool = PgPool::connect(&config.database.connection_string())
        .await
        .expect("Failed to connect to Postgres");

    let address = format!("127.0.0.1:{}", config.application_port);
    let listener = TcpListener::bind(address)?;

    zero2prod::startup::run(listener, connection_pool)?.await
}
