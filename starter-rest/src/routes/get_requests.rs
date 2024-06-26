use std::env::var;
use rocket::serde::json::Json;
use rocket::serde::Serialize;
use rocket::yansi::Paint;

#[derive(Debug, PartialEq, Eq, Serialize)]
pub struct PostObj {
    id: i32,
    title: String,
    body: String,
    published: bool,
}

#[get("/")]
pub fn get_posts() -> Json<Vec<PostObj>> {
    println!("Getting posts...");
    let posts = starter_data::service::get_posts();

    let mut res: Vec<PostObj> = Vec::new();

    for post in posts {
        let tmp = PostObj {
            id: post.id,
            title: post.title,
            body: post.body,
            published: post.published,
        };

        res.push(tmp);
    }

    Json(res)
}
