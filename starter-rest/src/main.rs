#[macro_use]
extern crate rocket;

use rocket::data::ToByteUnit;
use routes::get;

mod routes;

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![get::index])
}
