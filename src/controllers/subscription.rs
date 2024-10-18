use crate::dal;
use crate::structs::subscription::Subscription;
use actix_web::{web, HttpResponse, Responder};

pub async fn get_subscriptions_by_user(user_id: web::Path<u32>) -> impl Responder {
    let channels = dal::channel::get_subscriptions_by_user(*user_id).await;
    HttpResponse::Ok().json(channels)
}

pub async fn create_subscription(subscription: web::Json<Subscription>) -> impl Responder {
    let user_id = subscription.user_id;
    let channel_id = subscription.channel_id;

    let result = dal::subscriptions::subscribe_to_channel(user_id, channel_id).await;

    if !result {
        return HttpResponse::InternalServerError(); //500 or created
    }

    HttpResponse::Ok() //200
}

pub async fn unsubscribe_from_channel(subscription: web::Json<Subscription>) -> impl Responder {
    let user_id = subscription.user_id;
    let channel_id = subscription.channel_id;

    let result = dal::subscriptions::unsubscribe_from_channel(user_id, channel_id).await;

    if !result {
        return HttpResponse::InternalServerError(); //500 or created
    }

    HttpResponse::Ok() //200
}
