#[macro_use]
extern crate rocket;

use routes::get_requests;
use crate::routes::post_requests;

mod routes;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![
            get_requests::get_posts,
            post_requests::create_post,
        ])
}
