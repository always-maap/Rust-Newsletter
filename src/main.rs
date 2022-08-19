use std::net::TcpListener;

use newsletter::{
    configuration::get_configuration,
    startup::run,
    telementry::{get_subscriber, init_subscriber},
};
use sqlx::PgPool;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let subscriber = get_subscriber("newsletter".into(), "info".into());
    init_subscriber(subscriber);

    let configuration = get_configuration().expect("Failed to load configuration");
    let connection_pool = PgPool::connect(&configuration.database.get_connection_string())
        .await
        .expect("Failed to connect to database");
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address)?;

    run(listener, connection_pool)?.await?;
    Ok(())
}
