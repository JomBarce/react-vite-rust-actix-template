use actix_web::{HttpResponse, Responder};
use serde::Serialize;

#[derive(Serialize)]
struct HelloResponse {
    message: String,
}

pub async fn hello() -> impl Responder {
    HttpResponse::Ok().json(HelloResponse {
        message: "Hello World!".to_string(),
    })
}
