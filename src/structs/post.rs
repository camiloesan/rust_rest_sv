use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Post {
    pub post_id: u32,
    pub channel_id: u32,
    pub file_id: u32,
    pub title: String,
    pub description: String,
    pub publish_date: String,
}
