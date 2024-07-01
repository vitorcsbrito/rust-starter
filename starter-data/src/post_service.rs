use diesel::prelude::*;

use crate::db::establish_connection;
use crate::models::post::{NewPost, Post};
use crate::schema::posts::id;

pub fn get_posts() -> Vec<Post> {
    use super::schema::posts::dsl::*;

    let connection = &mut establish_connection();
    let results = posts
        .filter(published.eq(true))
        .limit(5)
        .select(Post::as_select())
        .load(connection)
        .expect("Error loading posts");

    println!("Displaying {} posts", results.len());
    results
}

pub fn get_all_posts() -> Vec<Post> {
    use super::schema::posts::dsl::*;

    let connection = &mut establish_connection();
    let results = posts
        .select(Post::as_select())
        .load(connection)
        .expect("Error loading posts");

    println!("Displaying {} posts", results.len());
    results
}

pub fn create_post(title: &str, body: &str) -> Post {
    use super::schema::posts;

    let new_post = NewPost {
        title,
        body,
        published: &true,
    };

    let conn = &mut establish_connection();
    diesel::insert_into(posts::table)
        .values(&new_post)
        .returning(Post::as_returning())
        .get_result(conn)
        .expect("Error saving new post")
}

pub fn delete_post(post_id: i32) {
    use super::schema::posts;

    let conn = &mut establish_connection();
    diesel::delete(posts::table)
        .filter(id.eq(post_id))
        .execute(conn)
        .expect("Error saving new post");
}

pub fn find_post<'a>(post_id: i32) -> Result<Post, diesel::result::Error> {
    use super::schema::posts::dsl::*;

    let conn = &mut establish_connection();
    posts.find(post_id).first::<Post>(conn)
}
