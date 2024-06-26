use rocket::serde::json::Json;
use starter_api::post::PostObj;

#[get("/")]
pub fn get_posts() -> Json<Vec<PostObj>> {
    println!("Getting posts...");
    let posts = starter_service::get_posts();

    Json(posts)
}

#[get("/all")]
pub fn get_all_posts() -> Json<Vec<PostObj>> {
    println!("Getting all posts...");
    let posts = starter_service::get_all_posts();

    Json(posts)
}
