use actix_learning::configuration::get_configuration;
use actix_learning::email_client::EmailClient;
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

    let url =
        reqwest::Url::parse(&configuration.email_client.base_url).expect("Cannot get the base url");
    let sender_email = configuration
        .email_client
        .sender()
        .expect("Invalid sender email address");
    let email_client = EmailClient::new(url, sender_email, configuration.email_client.api_key);

    let address = format!(
        "{}:{}",
        configuration.application.host, configuration.application.port
    );
    let addr = TcpListener::bind(address)?;
    let _ = run(addr, connection_pool, email_client)?.await;
    Ok(())
}
