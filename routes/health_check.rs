use actix_web::{HttpResponse, Responder}; 

pub async fn health_check() -> impl HttpResponse {
    HttpResponse::Ok().finish()
}
