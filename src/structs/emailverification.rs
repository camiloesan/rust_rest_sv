use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct VerificationRequest {
    pub email: String,
    pub code: String,
}
