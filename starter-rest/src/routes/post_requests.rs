use rocket::serde::json::Json;
use starter_data::post_service;
use crate::model::post::{CreatePost, PostObj};

#[post("/", format = "json", data = "<post>")]
pub fn create_post(post: Json<CreatePost>) -> Json<PostObj> {
    println!("Creating post...");

    let post = post_service::create_post(post.title.as_str(), post.body.as_str());

    Json(PostObj {
        id: post.id,
        title: post.title,
        body: post.body,
        published: post.published,
    })
}
