#![feature(decl_macro)]
#[macro_use] extern crate diesel;
#[macro_use] extern crate rocket;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate serde_json;
extern crate dotenv;
extern crate rocket_contrib;
extern crate r2d2;
extern crate r2d2_diesel;
use dotenv::dotenv;
use std::env;
use routes::*;
mod schema;
mod models;
mod db;
mod routes;

fn rocket() -> rocket::Rocket {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = db::init_pool(&database_url);
    rocket::Rocket::ignite()
        .manage(pool)
        .mount("/api/v1", routes![
            get_post, get_all_posts, index, delete_post, create_post, update_post
        ])
        .register(catchers![not_found])
}

fn main() {
    rocket().launch();
}