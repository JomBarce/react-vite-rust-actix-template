mod config;
mod db;
mod routes;

use actix_web::{web, App, HttpServer};
// use actix_cors::Cors;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    config::init();
    let db = db::mongo::connect().await;

    println!("Server running on 127.0.0.1:9001");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db.clone()))
            .service(
                web::scope("/api")
                    .route("/hello", web::get().to(routes::hello::hello))
                    .route("/status", web::get().to(routes::status::status))
            )
    })
    .bind(("127.0.0.1", 9001))?
    .run()
    .await
}
