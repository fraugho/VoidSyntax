use serde::{Serialize, Deserialize};

#[derive(Deserialize)]
pub struct LoginForm {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct Submit {
    pub user_id: String,
    pub thumbnail: String,
    pub creator: String,
    pub title: String,
}
