use rocket::serde::json::Json;
use crate::model::post::PostObj;

#[get("/")]
pub fn get_posts() -> Json<Vec<PostObj>> {
    println!("Getting posts...");
    let posts = starter_data::post_service::get_posts();

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

#[get("/all")]
pub fn get_all_posts() -> Json<Vec<PostObj>> {
    println!("Getting all posts...");
    let posts = starter_data::post_service::get_all_posts();

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
