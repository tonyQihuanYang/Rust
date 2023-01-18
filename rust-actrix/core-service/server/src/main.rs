use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
mod controllers;
use crate::controllers::authenticate_controller;
use database::{establish_connection, run_migration};

#[get("/healthcheck")]
async fn healthcheck() -> impl Responder {
    HttpResponse::Ok().body("ok")
}

use log::{Level, Metadata, Record};
struct SimpleLogger;
impl log::Log for SimpleLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Info
    }
    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            println!("{} - {}", record.level(), record.args());
        }
    }
    fn flush(&self) {}
}

use log::{LevelFilter, SetLoggerError};
static LOGGER: SimpleLogger = SimpleLogger;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    log::set_logger(&LOGGER).map(|()| log::set_max_level(LevelFilter::Info));

    log::info!("Started");
    println!("wtf");
    let conn = &mut establish_connection();
    run_migration(conn);

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
