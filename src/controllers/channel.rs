use crate::dal;
use crate::structs::channel::Channel;
use actix_web::{web, HttpResponse, Responder};

pub async fn get_all_channels() -> impl Responder {
    let channels = dal::channel::get_all_channels().await;
    HttpResponse::Ok().json(channels)
}

pub async fn get_channels_created_by_user(user_id: web::Path<u32>) -> impl Responder {
    let channels = dal::channel::get_channels_created_by_user(*user_id).await;
    HttpResponse::Ok().json(channels)
}

pub async fn create_channel(channel: web::Json<Channel>) -> impl Responder {
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
