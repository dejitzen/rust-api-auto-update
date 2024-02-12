use std::env;

use actix_web::{web, App, HttpResponse, HttpServer, Responder};
mod check_for_update;

async fn hello() -> impl Responder {
    check_for_update::check_for_update().await;
    HttpResponse::Ok().body("Hello world")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().route("/hello", web::get().to(hello)))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
