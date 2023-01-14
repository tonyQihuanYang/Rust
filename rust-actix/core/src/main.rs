use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

#[get("/healthcheck")]
async fn healthcheck() -> impl Responder {
    HttpResponse::Ok().body("ok")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(healthcheck))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
