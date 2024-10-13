pub mod dal;
pub mod structs;

use crate::structs::subscription::Subscription;
use crate::structs::user::LoginData;
use crate::structs::channel::Channel;
use actix_cors::Cors;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};

async fn get_all_categories() -> impl Responder {
    let categories = dal::categories::get_all_categories().await;
    HttpResponse::Ok().json(categories)
}

async fn get_all_channels() -> impl Responder {
    let channels = dal::channel::get_all_channels().await;
    HttpResponse::Ok().json(channels)
}

async fn get_channels_created_by_user(user_id: web::Path<u32>) -> impl Responder {
    let channels = dal::channel::get_channels_created_by_user(*user_id).await;
    HttpResponse::Ok().json(channels)
}

async fn create_channel(channel: web::Json<Channel>) -> impl Responder {
    let creator_id = channel.creator_id;
    let name = channel.name.clone();
    let description = channel.description.clone();
    let category_id = channel.category_id;

    let result = dal::channel::create_channel(creator_id, name, description, category_id).await;

    if !result {
        return HttpResponse::InternalServerError(); //500
    }

    HttpResponse::Ok() //200
}

async fn get_subscriptions_by_user(user_id: web::Path<u32>) -> impl Responder {
    let channels = dal::channel::get_subscriptions_by_user(*user_id).await;
    HttpResponse::Ok().json(channels)
}

// POST /subscription endpoint
async fn create_subscription(subscription: web::Json<Subscription>) -> impl Responder {
    let user_id = subscription.user_id;
    let channel_id = subscription.channel_id;

    let result = dal::subscriptions::subscribe_to_channel(user_id, channel_id).await;

    if !result {
        return HttpResponse::InternalServerError(); //500 or created
    }

    HttpResponse::Ok() //200
}

async fn unsubscribe_from_channel(subscription: web::Json<Subscription>) -> impl Responder {
    let user_id = subscription.user_id;
    let channel_id = subscription.channel_id;

    let result = dal::subscriptions::unsubscribe_from_channel(user_id, channel_id).await;

    if !result {
        return HttpResponse::InternalServerError(); //500 or created
    }

    HttpResponse::Ok() //200
}

async fn get_posts_by_channel(channel_id: web::Path<u32>) -> impl Responder {
    let posts = dal::posts::get_posts_by_channel(*channel_id).await;
    HttpResponse::Ok().json(posts)
}

async fn login_user(login_data: web::Json<LoginData>) -> impl Responder {
    let email = login_data.email.clone();
    let password = login_data.password.clone();

    let result = dal::users::login(email, password).await;

    if let Some(user) = result {
        return HttpResponse::Ok().json(user); //200
    }

    HttpResponse::Unauthorized().finish() //401
}

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
                web::get().to(get_channels_created_by_user),
            )
            .route("/channels/all", web::get().to(get_all_channels))
            .route(
                "/subscriptions/user/{id}",
                web::get().to(get_subscriptions_by_user),
            )
            .route("/subscription", web::post().to(create_subscription))
            .route("/unsubscribe", web::delete().to(unsubscribe_from_channel))
            .route("/login", web::post().to(login_user))
            .route("/posts/channel/{id}", web::get().to(get_posts_by_channel))
            .route("/categories/all", web::get().to(get_all_categories))
            .route("/channel/create", web::post().to(create_channel))
            .wrap(cors)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
