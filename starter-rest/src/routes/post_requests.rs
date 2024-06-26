use rocket::serde::json::Json;
use starter_api::post::{CreatePost, PostObj};

#[post("/", format = "json", data = "<post>")]
pub fn create_post(post: Json<CreatePost>) -> Json<PostObj> {
    println!("Creating post...");

    let post = starter_service::create_post(post.into_inner());

    Json(post)
}
