#[macro_use]
extern crate rocket;

use crate::routes::get_requests::{get_all_posts, get_posts};
use crate::routes::post_requests::create_post;
use crate::routes::delete_requests::{delete_post_enum_responder, delete_post_result_response};

mod routes;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![get_posts, get_all_posts])
        .mount("/", routes![create_post,])
        .mount("/", routes![delete_post_enum_responder, delete_post_result_response,])
}
