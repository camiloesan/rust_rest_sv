use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Channel {
    pub creator_id: u32,
    pub name: String,
    pub description: String,
    pub category_id: u32,
}