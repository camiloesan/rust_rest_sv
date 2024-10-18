use crate::dal::data_access;
use mysql::{prelude::Queryable, Row};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Category {
    category_id: u32,
    name: String,
}

pub async fn get_all_categories() -> Vec<Category> {
    let mut conn = data_access::get_connection();
    let query = "SELECT category_id, name FROM categories";
    let mut categories: Vec<Category> = Vec::new();

    conn.query_map(&query, |mut row: Row| {
        let category = Category {
            category_id: row.take("category_id").unwrap(),
            name: row.take("name").unwrap(),
        };
        categories.push(category);
    })
    .expect("Failed to fetch categories information");

    categories
}