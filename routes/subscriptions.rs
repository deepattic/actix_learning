use actix_web::{web, HttpResponse};

#[derive(serde::Deserialize)]
struct FormData {
    name: String,
    email: String
}


async fn subscribe(_form: web::Form<FormData>) -> HttpResponse {
    HttpResponse::Ok().finish()
}
