pub mod controllers;
pub mod dal;
pub mod structs;

use crate::controllers::*;

use actix_cors::Cors;
use actix_web::{web, App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header();

        App::new()
            .route(
                "/channels/owner/{id}",
                web::get().to(channel::get_channels_created_by_user),
            )
            .route("/channels/all", web::get().to(channel::get_all_channels))
            .route(
                "/subscriptions/user/{id}",
                web::get().to(subscription::get_subscriptions_by_user),
            )
            .route(
                "/subscription",
                web::post().to(subscription::create_subscription),
            )
            .route(
                "/unsubscribe",
                web::delete().to(subscription::unsubscribe_from_channel),
            )
            .route("/login", web::post().to(user::login_user))
            .route("/user/email/all", web::get().to(user::get_all_emails))
            .route("/register", web::post().to(user::register_new_user))
            .route(
                "/posts/channel/{id}",
                web::get().to(post::get_posts_by_channel),
            )
            .route(
                "/categories/all",
                web::get().to(category::get_all_categories),
            )
            .route("/channel/create", web::post().to(channel::create_channel))
            .wrap(cors)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
