use rocket::serde::{Deserialize, Serialize};
use rocket::serde::json::Json;
use starter_data::service;

#[derive(Debug, PartialEq, Eq, Deserialize)]
pub struct CreatePost {
    title: String,
    body: String,
}

#[derive(Debug, PartialEq, Eq, Serialize)]
pub struct PostObj {
    id: i32,
    title: String,
    body: String,
    published: bool,
}

#[post("/", format = "json", data = "<post>")]
pub fn create_post(post: Json<CreatePost>) -> Json<PostObj> {
    println!("Creating post...");

    format!("print test {:?}", post);

    let post = service::create_post(post.title.as_str(), post.body.as_str());

    Json(PostObj { id: post.id, title: post.title, body: post.body, published: post.published })
}
