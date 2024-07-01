use rocket::http::Status;
use rocket::serde::json::Json;
use starter_api::responses::custom_responder::{DynamicStatusResponder, NotFoundResponder};
use starter_api::responses::enum_responder::EnumResponder;
use starter_api::responses::enum_responder::EnumResponder::{NotFoundResponse, RemovedResponse};
use starter_api::responses::ResponseStructure;

#[delete("/<id>")]
pub fn delete_post_enum_responder(id: i32) -> EnumResponder {
    println!("Deleting post...");

    let res = starter_service::delete_post(id);

    match res {
        Ok(_) => RemovedResponse(Json(ResponseStructure {
            message: "Post deleted with success".to_string(),
        })),
        Err(_) => NotFoundResponse(Json(ResponseStructure {
            message: "Post not found".to_string(),
        })),
    }
}

#[delete("/id/<id>")]
pub fn delete_post_result_response(
    id: i32,
) -> Result<
    DynamicStatusResponder<Json<ResponseStructure>>,
    NotFoundResponder<Json<ResponseStructure>>,
> {
    println!("Deleting post...");

    let res = starter_service::delete_post(id);

    match res {
        Ok(_) => Ok(DynamicStatusResponder {
            inner: (
                Status::Ok,
                Json(ResponseStructure {
                    message: "Post deleted with success".to_string(),
                }),
            ),
        }),
        Err(_) => Err(NotFoundResponder {
            inner: Json(ResponseStructure {
                message: "Post not found".to_string(),
            }),
        }),
    }
}
