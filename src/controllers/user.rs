use crate::dal;
use crate::dal::users::VERIFICATION_CODES;
use crate::dal::users::{generate_verification_code, send_verification_email};
use crate::structs::emailverification::VerificationRequest;
use crate::structs::registeruser::RegisterRequest;
use crate::structs::user::LoginData;
use actix_web::{web, HttpResponse, Responder};

pub async fn login_user(login_data: web::Json<LoginData>) -> impl Responder {
    let email = login_data.email.clone();
    let password = login_data.password.clone();

    let result = dal::users::login(email, password).await;

    if let Some(user) = result {
        return HttpResponse::Ok().json(user); //200
    }

    HttpResponse::Unauthorized().finish() //401
}

pub async fn request_verification(email: web::Json<String>) -> impl Responder {
    let code = generate_verification_code();

    send_verification_email(email.clone(), code.clone()).await;

    VERIFICATION_CODES
        .lock()
        .unwrap()
        .insert(email.clone(), code);

    HttpResponse::Ok().finish()
}

pub async fn verify_code(data: web::Json<VerificationRequest>) -> impl Responder {
    let VerificationRequest { email, code } = data.into_inner();

    let mut codes = VERIFICATION_CODES.lock().unwrap();

    if let Some(stored_code) = codes.get(&email) {
        if stored_code == &code {
            codes.remove(&email);
            return HttpResponse::Ok().finish();
        }
    }

    HttpResponse::Unauthorized().finish()
}

pub async fn register_new_user(data: web::Json<RegisterRequest>) -> impl Responder {
    let request = dal::users::register_user(data.into_inner()).await;

    if !request {
        return HttpResponse::InternalServerError();
    }

    HttpResponse::Ok()
}
