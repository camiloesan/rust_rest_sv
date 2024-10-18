use crate::dal;
use actix_web::{HttpResponse, Responder};

pub async fn get_all_categories() -> impl Responder {
    let categories = dal::categories::get_all_categories().await;
    HttpResponse::Ok().json(categories)
}
