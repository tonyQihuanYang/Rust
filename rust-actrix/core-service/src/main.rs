use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use rust_core_service::{
    controllers::authenticate_controller, establish_connection, run_migration,
};

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

use log::LevelFilter;
static LOGGER: SimpleLogger = SimpleLogger;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    log::set_logger(&LOGGER)
        .map(|()| log::set_max_level(LevelFilter::Info))
        .expect("Error Initialize Logger v1");

    log::info!("Core-Service Starting");
    log::info!("Start DB Migration");
    let conn = &mut establish_connection();
    run_migration(conn);

    log::info!("Start Listening");
    HttpServer::new(|| {
        App::new()
            .service(healthcheck)
            .service(authenticate_controller::authenticate)
            .service(authenticate_controller::login)
            .service(authenticate_controller::register)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
