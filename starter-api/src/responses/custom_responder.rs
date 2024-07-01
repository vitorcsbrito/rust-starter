use rocket::http::Status;
use rocket::Responder;
use rocket::serde::Serialize;

#[derive(Responder)]
#[response(status = 404, content_type = "json")]
pub struct NotFoundResponder<T> {
    pub inner: T,
}

#[derive(Responder, Serialize)]
#[serde(crate = "rocket::serde")]
#[response(content_type = "json")]
pub struct DynamicStatusResponder<T> {
    pub inner: (Status, T),
}

#[derive(Responder,Serialize)]
#[serde(crate = "rocket::serde")]
#[response(status = 201, content_type = "json")]
pub struct CreatedResponder<T> {
    pub inner: T,
}