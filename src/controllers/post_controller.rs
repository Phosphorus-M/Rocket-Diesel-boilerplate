use crate::models::{NewPost, Post, UpdatePost};
use crate::utils::establish_connection;
use rocket::serde::json::Json;
use rocket::{delete, get, post, put};
use rocket_okapi::openapi;

#[openapi(tag = "Post")]
#[get("/post")] // <- route attribute
pub fn get_all() -> Json<Vec<Post>> {
    // <- request handler
    let connection = establish_connection();
    Json(Post::read_all(&connection))
}

#[openapi(tag = "Post")]
#[get("/post/<id>")] // <- route attribute
pub fn get_by_id(id: i32) -> Json<Post> {
    // <- request handler
    let connection = establish_connection();
    Json(Post::read(&connection, id))
}

#[openapi(tag = "Post")]
#[post("/post", data = "<post>")]
pub fn create(post: Json<NewPost>) -> Json<Post> {
    let connection = establish_connection();
    let post = Post::create(&connection, &post);
    Json(post)
}

#[openapi(tag = "Post")]
#[put("/post/<id>", data = "<post>")]
pub fn update(id: i32, post: Json<UpdatePost>) -> Json<Post> {
    let connection = establish_connection();
    let post = Post::update(&connection, id, &post);
    Json(post)
}

#[openapi(tag = "Post")]
#[delete("/post/<id>")]
pub fn delete(id: i32) -> Json<bool> {
    let connection = establish_connection();
    let success = Post::delete(&connection, id);
    Json(success)
}
