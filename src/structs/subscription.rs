use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Subscription {
    pub user_id: u32,
    pub channel_id: u32,
}
