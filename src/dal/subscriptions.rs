use crate::dal::data_access;
use mysql::{params, prelude::Queryable};

pub async fn subscribe_to_channel(user_id: u32, channel_id: u32) -> bool {
    let mut conn = data_access::get_connection();
    let query = "INSERT INTO subscriptions (user_id, channel_id) VALUES (:user_id, :channel_id)";
    let result = conn
        .exec_iter(
            &query,
            params! {
                "user_id" => user_id,
                "channel_id" => channel_id,
            },
        )
        .expect("failed to subscribe to channel")
        .affected_rows();

    result == 1
}

pub async fn unsubscribe_from_channel(user_id: u32, channel_id: u32) -> bool {
    let mut conn = data_access::get_connection();
    let query = "DELETE FROM subscriptions WHERE user_id = :user_id AND channel_id = :channel_id";
    let result = conn
        .exec_iter(
            &query,
            params! {
                "user_id" => user_id,
                "channel_id" => channel_id,
            },
        )
        .expect("failed to unsubscribe from channel")
        .affected_rows();

    result == 1
}

#[cfg(test)]
mod tests {
    use crate::dal::subscriptions::*;
    use tokio;

    #[tokio::test]
    async fn test_unsubscribe_from_channel() {
        // create user
        // create channel
        subscribe_to_channel(1, 1).await;
        let result = unsubscribe_from_channel(1, 1).await;

        assert!(result);
    }

    #[tokio::test]
    async fn test_subscribe_to_channel() {
        let result = subscribe_to_channel(2, 1).await;
        unsubscribe_from_channel(2, 1).await;

        assert!(result);
    }
}
