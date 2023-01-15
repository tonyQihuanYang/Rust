// use actix_session::Session;
// use crate::apis::auth::services::auth_services::login_user;
// use crate::models::user::LoginCredentials;
// use actix_session::Session;
use actix_web::{get, post, web, HttpResponse};
use server::services::account_service;

#[get("/authenticate")]
pub async fn authenticate() -> HttpResponse {
    HttpResponse::Ok().json("Success")
}

#[post("/login")]
pub async fn login() -> HttpResponse {
    let username = "test";
    let password = "changeme123";
    match account_service::login(username, password) {
        Ok(_) => HttpResponse::Ok().json("Success"),
        Err(_) => return HttpResponse::Unauthorized().json("Unauthorized"),
    }
}

#[post("/register")]
pub async fn register() -> HttpResponse {
    let username = "test";
    let password = "changeme123";
    match account_service::register(username, password) {
        Ok(_) => HttpResponse::Ok().json("Success"),
        Err(_) => HttpResponse::Unauthorized().json("Unauthorized"),
    }
}

// #[post("/register")]
// pub async fn register(credentials: web::Json<LoginCredentials>) -> HttpResponse {
//     match account_service::register(credentials.into_inner()).await {
//         Ok(_) => HttpResponse::Ok().json("Success"),
//         Err(_) => HttpResponse::Unauthorized().json("Unauthorized"),
//     }
// }

// #[post("/authenticate")]
// pub async fn authenticate_post(session: Session) -> HttpResponse {
//     let user_id: Option<String> = session.get("user_id").unwrap_or(None);
//     log::info!("{:?}", user_id);

//     match user_id {
//         Some(_) => HttpResponse::Ok().json("Success"),
//         None => HttpResponse::Unauthorized().json("Unauthorized"),
//     }
// }

// #[post("/login")]
// pub async fn login_post(
//     credentials: web::Json<LoginCredentials>,
//     session: Session,
// ) -> HttpResponse {
//     let credentials = credentials.into_inner();

//     match login_user(credentials).await {
//         Ok(user_found) => {
//             log::info!("{:?}", user_found.clone().get_id_string());
//             session
//                 .insert("user_id", user_found.get_id_string())
//                 .unwrap();
//         }
//         Err(_) => return HttpResponse::Unauthorized().json("Unauthorized"),
//     };
//     HttpResponse::Ok().json("Success")
// }
