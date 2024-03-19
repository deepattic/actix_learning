use std::net::TcpListener;

use actix_learning::run;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let addr = TcpListener::bind("127.0.0.1:8080").expect("Failed To Bind localhost:8080");
    run(addr)?.await
}
