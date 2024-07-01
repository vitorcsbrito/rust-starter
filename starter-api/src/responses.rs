use serde::Serialize;
pub mod custom_responder;
pub mod enum_responder;


#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct ResponseStructure {
    pub message: String,
}