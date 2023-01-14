use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
mod controllers;
use crate::controllers::authenticate_controller;

#[get("/healthcheck")]
async fn healthcheck() -> impl Responder {
    HttpResponse::Ok().body("ok")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(healthcheck)
            .service(authenticate_controller::authenticate)
            .service(authenticate_controller::login)
            .service(authenticate_controller::register)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
