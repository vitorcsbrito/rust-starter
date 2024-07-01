use rocket::Responder;
use rocket::serde::json::Json;
use crate::responses::ResponseStructure;

#[derive(Responder)]
pub enum EnumResponder {
    #[response(status = 200, content_type = "json")]
    RemovedResponse(Json<ResponseStructure>),
    #[response(status = 404, content_type = "json")]
    NotFoundResponse(Json<ResponseStructure>),
}
