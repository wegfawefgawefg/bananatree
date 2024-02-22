use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Banana {
    pub id: Option<i32>,
    pub user_id: i32,
    pub content: String,
}

#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub username: String,
}
