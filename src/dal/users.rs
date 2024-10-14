use crate::dal::data_access;
use crate::structs::registeruser::RegisterRequest;
use mysql::{params, prelude::Queryable, Row};
use serde::{Deserialize, Serialize};
use reqwest::Client;
use serde_json::json;
use std::collections::HashMap;
use std::sync::Mutex;
use rand::distributions::Alphanumeric;
use rand::thread_rng;
use rand::Rng;

lazy_static::lazy_static! {
    pub static ref VERIFICATION_CODES: Mutex<HashMap<String, String>> = Mutex::new(HashMap::new());
}

#[derive(Serialize, Deserialize)]
pub struct User {
    pub user_id: u32,
    pub user_type_id: u32,
    pub name: String,
    pub last_name: String,
    pub email: String,
}

pub async fn login(email: String, password: String) -> Option<User> {
    let mut conn = data_access::get_connection();

    let query = "SELECT user_id, user_type_id, name, last_name, email, password 
                 FROM users WHERE email = :email";

    let row: Option<Row> = conn.exec_first(
        query,
        params! { "email" => email },
    ).expect("Failed to execute login query");

    if let Some(mut row) = row {
        let user_id: u32 = row.take("user_id").unwrap();
        let user_type_id: u32 = row.take("user_type_id").unwrap();
        let name: String = row.take("name").unwrap();
        let last_name: String = row.take("last_name").unwrap();
        let email: String = row.take("email").unwrap();
        let password_hash: String = row.take("password").unwrap();

        if password_hash == password {
            return Some(User {
                user_id,
                user_type_id,
                name,
                last_name,
                email,
            });
        }
    }
    None
}

pub fn generate_verification_code() -> String {
    let code: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(6)
        .map(char::from)
        .collect();
    code
}

pub async fn send_verification_email(email: String, code: String) {
    let api_key = "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";
    let from_email = "studyvaultuv@gmail.com";

    let body = json!({
        "personalizations": [{
            "to": [{ "email": email }],
            "subject": "Tu código de verificación"
        }],
        "from": { "email": from_email },
        "content": [{
            "type": "text/plain",
            "value": format!("Tu código de verificación es: {}", code)
        }]
    });

    let client = Client::new();
    let response = client.post("https://api.sendgrid.com/v3/mail/send")
        .bearer_auth(api_key)
        .json(&body)
        .send()
        .await;

    match response {
        Ok(res) => {
            if res.status().is_success() {
                println!("Email sent successfully!");
            } else {
                println!("Failed to send email: {}", res.status());
            }
        }
        Err(e) => println!("Error: {:?}", e),
    }
}

pub async fn register_user(request: RegisterRequest) -> Result<(), String> {
    let user_type_id = if request.email.ends_with("@estudiantes.uv.mx") {
        2
    } else if request.email.ends_with("@uv.mx") {
        1
    } else {
        return Err("Dominio de correo no válido".to_string());
    };

    let mut conn = data_access::get_connection();

    let query = "INSERT INTO users (user_type_id, name, last_name, email, password) VALUES (:user_type_id, :name, :last_name, :email, :password)";

    conn.exec_drop(query, params! {
        "user_type_id" => user_type_id,
        "name" => request.name,
        "last_name" => request.last_name,
        "email" => request.email,
        "password" => request.password,
    }).expect("Failed to execute register query");

    Ok(())
}
