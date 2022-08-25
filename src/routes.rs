use rocket::http::Status;
use rocket_contrib::json::Json;
use super::models::{Post, NewPost};
use serde_json::Value;
use crate::db::DbConn;


#[get("/posts")]
pub fn get_all_posts(conn: DbConn) -> Json<Value> {
    Json(json!({"status": 200, "posts": Post::all(&conn)}))
}


#[get("/posts/<id>")]
pub fn get_post(id: i32, conn: DbConn) -> Json<Value> {
    Json(json!({"status": 200, "posts": Post::get(id, &conn)}))
}


#[post("/posts", format = "application/json", data = "<post>")]
pub fn create_post(post: Json<NewPost>, conn: DbConn) -> Json<Value> {
    Json(json!({"status": 200, "created": Post::insert(&conn, &post.into_inner())}))
}


#[patch("/posts/<id>", format = "application/json", data = "<post>")]
pub fn update_post(id: i32, post: Json<NewPost>, conn: DbConn) -> Result<Json<Value>, Status> {
    let updated = Post::update(&conn, id, &post.into_inner());
    match updated {
        true => Ok(Json(json!({"status": 200, "updated": updated}))),
        false => Err(Status::NotFound),
    }
}


#[delete("/posts/<id>")]
pub fn delete_post(id: i32, conn: DbConn) -> Json<Value> {
    Json(json!({"status": 200, "deleted": Post::delete(&conn, id)}))
}


#[get("/")]
pub fn index() -> &'static str {
    "Hello, world!"
}

#[catch(404)]
pub fn not_found() -> Json<Value> {
    Json(json!({"status": 404, "error": "Resource not found"}))
}