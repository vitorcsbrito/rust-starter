#[delete("/<id>")]
pub fn delete_post(id: i32) {
    println!("Deleting post...");

    starter_service::delete_post(id);
}
