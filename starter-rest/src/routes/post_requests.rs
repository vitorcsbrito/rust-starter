use rocket::http::Status;
use rocket::serde::json::Json;
use starter_api::post::{CreatePost, PostObj};
use starter_api::responses::custom_responder::{DynamicStatusResponder};

#[post("/", format = "json", data = "<post>")]
pub fn create_post(post: Json<CreatePost>) -> DynamicStatusResponder<Json<PostObj>> {
    println!("Creating post...");

    let post = starter_service::create_post(post.into_inner());
    
    DynamicStatusResponder {
        inner: (
            Status::Created,
            Json(post),
        ),
    }
}
