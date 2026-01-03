use actix_web::{web, HttpResponse, Responder};
use mongodb::Database;
use serde::Serialize;

#[derive(Serialize)]
pub struct StatusResponse {
    message: &'static str,
    status: &'static str,
}

pub async fn status(db: web::Data<Database>) -> impl Responder {
    let status = match db
        .run_command(mongodb::bson::doc! { "ping": 1 }, None)
        .await
        {
            Ok(_) => "ok",
            Err(_) => "error",
        };

    HttpResponse::Ok().json(StatusResponse {
        message: "Server is running!",
        status,
    })
}
