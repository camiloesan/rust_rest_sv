use crate::dal::data_access;
use mysql::{params, prelude::Queryable, Row};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Channel {
    channel_id: u32,
    creator_id: u32,
    creator_name: String,
    creator_last_name: String,
    name: String,
    description: String,
}

pub async fn get_all_channels() -> Vec<Channel> {
    let mut conn = data_access::get_connection();
    let query =
        "SELECT channels.*, users.name as creator_name, users.last_name as creator_last_name
        FROM channels INNER JOIN users ON channels.creator_id = users.user_id";
    let mut channels: Vec<Channel> = Vec::new();

    conn.query_map(&query, |mut row: Row| {
        let channel = Channel {
            channel_id: row.take("channel_id").unwrap(),
            creator_id: row.take("creator_id").unwrap(),
            creator_name: row.take("creator_name").unwrap(),
            creator_last_name: row.take("creator_last_name").unwrap(),
            name: row.take("name").unwrap(),
            description: row.take("description").unwrap(),
        };
        channels.push(channel);
    })
    .expect("failed to get developer information");

    channels
}

pub async fn get_channels_created_by_user(user_id: u32) -> Vec<Channel> {
    let mut conn = data_access::get_connection();
    let query = "SELECT channels.*, users.name as creator_name, users.last_name as creator_last_name
    FROM channels INNER JOIN users ON channels.creator_id = users.user_id WHERE creator_id = :creator_id";
    let mut channels: Vec<Channel> = Vec::new();

    conn.exec_map(
        &query,
        params! { "creator_id" => user_id },
        |mut row: Row| {
            let channel = Channel {
                channel_id: row.take("channel_id").unwrap(),
                creator_id: row.take("creator_id").unwrap(),
                creator_name: row.take("creator_name").unwrap(),
                creator_last_name: row.take("creator_last_name").unwrap(),
                name: row.take("name").unwrap(),
                description: row.take("description").unwrap(),
            };
            channels.push(channel);
        },
    )
    .expect("failed to get developer information");

    channels
}

pub async fn get_subscriptions_by_user(user_id: u32) -> Vec<Channel> {
    let mut conn = data_access::get_connection();
    let query =
        "SELECT channels.*, users.name as creator_name, users.last_name as creator_last_name
    FROM channels INNER JOIN users ON channels.creator_id = users.user_id
    WHERE channel_id IN (SELECT channel_id FROM subscriptions WHERE user_id = :user_id)";
    let mut channels: Vec<Channel> = Vec::new();

    conn.exec_map(&query, params! { "user_id" => user_id }, |mut row: Row| {
        let channel = Channel {
            channel_id: row.take("channel_id").unwrap(),
            creator_id: row.take("creator_id").unwrap(),
            creator_name: row.take("creator_name").unwrap(),
            creator_last_name: row.take("creator_last_name").unwrap(),
            name: row.take("name").unwrap(),
            description: row.take("description").unwrap(),
        };
        channels.push(channel);
    })
    .expect("failed to get developer information");

    channels
}

#[cfg(test)]
mod tests {
    use crate::dal::channel::*;
    use tokio;

    #[tokio::test]
    async fn test_get_subscriptions_by_user_exists() {
        // do pre and post
        let channels = get_subscriptions_by_user(1).await;
        assert!(channels.is_empty() == false);
    }

    #[tokio::test]
    async fn test_get_channels() {
        // do pre and post
        let channels = get_all_channels().await;
        println!("Channel name: {}", channels[0].name);
        assert!(channels.is_empty() == false);
    }

    #[tokio::test]
    async fn test_get_channels_by_user_exists() {
        // do pre and post
        let channels = get_channels_created_by_user(2).await;
        println!("Channels count: {}", channels.len());
        assert!(channels.is_empty() == false);
    }

    #[tokio::test]
    async fn test_get_channels_by_user_not_exists() {
        // do pre and post
        let channels = get_channels_created_by_user(100).await;
        assert!(channels.is_empty() == true);
    }
}
