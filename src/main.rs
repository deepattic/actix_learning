use actix_learning::configuration::get_configuration;
use actix_learning::startup::Application;
use actix_learning::telemetry::{get_subscriber, init_subscriber};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let subscriber = get_subscriber("actix_learning".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let configuration = get_configuration().expect("Failed to read the Configuration File.");

    let application = Application::build(configuration).await?;
    application.run_until_stopped().await?;
    Ok(())
}
