use starter_data::post_service;

#[delete("/<id>")]
pub fn delete_post(id: i32) {
    println!("Deleting post...");

    post_service::delete_post(id);
}
