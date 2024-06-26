
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Serialize)]
pub struct PostObj {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}

#[derive(Debug, PartialEq, Eq, Deserialize)]
pub struct CreatePost {
    pub title: String,
    pub body: String,
}