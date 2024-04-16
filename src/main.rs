use actix_learning::configuration::get_configuration;
use actix_learning::startup::run;
use actix_learning::telemetry::{get_subscriber, init_subscriber};
use sqlx::postgres::PgPoolOptions;
use std::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let subscriber = get_subscriber("actix_learning".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);
    let configuration = get_configuration().expect("Failed to read the Config");
    let connection_pool = PgPoolOptions::new().connect_lazy_with(configuration.database.with_db());
    let address = format!(
        "{}:{}",
        configuration.application.host, configuration.application.port
    );
    let addr = TcpListener::bind(address)?;
    // let addr = TcpListener::bind("127.0.0.1:8080").expect("Failed To Bind localhost:8080");
    run(addr, connection_pool)?.await
}
