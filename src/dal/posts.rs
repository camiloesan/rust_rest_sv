use crate::dal::data_access;
use crate::structs::post::Post;
use actix_web::cookie::time::Date;
use mysql::{params, prelude::Queryable, Row};

pub async fn get_posts_by_channel(channel_id: u32) -> Vec<Post> {
    let mut conn = data_access::get_connection();
    let query = "SELECT * FROM posts WHERE channel_id = :channel_id";

    let mut posts: Vec<Post> = Vec::new();

    conn.exec_map(
        &query,
        params! { "channel_id" => channel_id },
        |mut row: Row| {
            let pdate: Date = row.take("publish_date").unwrap();
            let pdate_str = pdate.to_string();

            let post = Post {
                post_id: row.take("post_id").unwrap(),
                channel_id: row.take("channel_id").unwrap(),
                file_id: row.take("file_id").unwrap(),
                title: row.take("title").unwrap(),
                description: row.take("description").unwrap(),
                publish_date: pdate_str,
            };
            posts.push(post);
        },
    )
    .expect("failed to get developer information");

    posts
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio;

    #[tokio::test]
    async fn test_get_posts_by_channel() {
        let posts = get_posts_by_channel(1).await;
        assert_eq!(posts.len(), 2);
    }
}
