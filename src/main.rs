use actix_learning::configuration::get_configuration;
use actix_learning::startup::run;
use sqlx::PgPool;
use std::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let configuration = get_configuration().expect("Failed to read the Config");
    let connection_pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect To Db!!");
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let addr = TcpListener::bind(address)?;
    // let addr = TcpListener::bind("127.0.0.1:8080").expect("Failed To Bind localhost:8080");
    run(addr, connection_pool)?.await
}
