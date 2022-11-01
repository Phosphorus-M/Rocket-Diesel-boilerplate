#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;

extern crate dotenv;

#[macro_use]
extern crate diesel;

pub use rocket_okapi::swagger_ui::{make_swagger_ui, SwaggerUIConfig};
pub use rocket_okapi::{openapi, openapi_get_routes};

pub mod controllers;
pub mod models;
pub mod schemas;
pub mod utils;
pub use crate::controllers::post_controller;

fn get_docs() -> SwaggerUIConfig {
    SwaggerUIConfig {
        url: "../openapi.json".to_owned(),
        ..Default::default()
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount(
            "/",
            openapi_get_routes![
                post_controller::create,
                post_controller::get_all,
                post_controller::update,
                post_controller::get_by_id,
                post_controller::delete
            ],
        )
        .mount("/swagger", make_swagger_ui(&get_docs()))
}
