use starter_api::post::{CreatePost, PostObj};
use starter_data::post_service;
use std::error::Error;

pub fn get_posts() -> Vec<PostObj> {
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

    res
}

pub fn get_all_posts() -> Vec<PostObj> {
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

    res
}

pub fn create_post(post: CreatePost) -> PostObj {
    println!("Creating post...");

    let post = post_service::create_post(&*post.title, &*post.body);

    PostObj {
        id: post.id,
        title: post.title,
        body: post.body,
        published: post.published,
    }
}

pub fn delete_post(id: i32) -> Result<i32, impl Error> {
    println!("Deleting post...");

    match post_service::find_post(id) {
        Ok(p) => {
            post_service::delete_post(p.id);
            Ok(p.id)
        }
        Err(err) => Err(err),
    }
}
