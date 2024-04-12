// use actix_web::dev::Server;
// use actix_web::{web, App, HttpResponse, HttpServer, Responder};
// use std::net::TcpListener;
//
// #[derive(serde::Deserialize)]
// struct FormData {
//     name: String,
//     email: String
// }
// async fn health_check() -> impl Responder {
//     HttpResponse::Ok().finish()
// }
// async fn subscribe(_form: web::Form<FormData>) -> impl Responder {
//     HttpResponse::Ok().finish()
// }
// pub fn run(listen: TcpListener) -> Result<Server, std::io::Error> {
//     let server = HttpServer::new( || {
//         App::new()
//             .route("/health_check", web::get().to(health_check))
//             .route("/subscriptions", web::post().to(subscribe))
//     })
//             .listen(listen)?
//             .run();
//             Ok(server)
// }
pub mod configuration;
pub mod routes;
pub mod startup;
