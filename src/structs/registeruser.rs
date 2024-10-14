use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct RegisterRequest {
    pub email: String,
    pub name: String,
    pub last_name: String,
    pub password: String,
}