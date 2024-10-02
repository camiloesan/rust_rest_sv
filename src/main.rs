pub mod dal;

use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Employee {
    id: u32,
    name: String,
}

// GET / endpoint
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello, World!")
}

async fn get_all_channels() -> impl Responder {
    let channels = dal::channel::get_all_channels().await;
    HttpResponse::Ok().json(channels)
}

async fn get_channels_created_by_user(user_id: web::Path<u32>) -> impl Responder {
    let channels = dal::channel::get_channels_created_by_user(*user_id).await;
    HttpResponse::Ok().json(channels)
}

async fn get_subscriptions_by_user(user_id: web::Path<u32>) -> impl Responder {
    let channels = dal::channel::get_subscriptions_by_user(*user_id).await;
    HttpResponse::Ok().json(channels)
}

// POST /object endpoint
async fn create_employee(employee: web::Json<Employee>) -> impl Responder {
    HttpResponse::Created().json(employee.into_inner())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/employee", web::post().to(create_employee))
            .route(
                "/channels/owner/{id}",
                web::get().to(get_channels_created_by_user),
            )
            .route("/channels/all", web::get().to(get_all_channels))
            .route(
                "/subscriptions/user/{id}",
                web::get().to(get_subscriptions_by_user),
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
