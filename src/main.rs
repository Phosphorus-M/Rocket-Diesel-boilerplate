#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate diesel;
extern crate dotenv;

pub mod models;
pub mod schemas;
pub mod utils;
pub mod controllers;
pub use crate::controllers::post_controller;

#[macro_use] extern crate rocket;

fn main() {
    rocket::ignite()
                    .mount("/", routes![post_controller::create, post_controller::get_all, post_controller::update, post_controller::delete, post_controller::get_by_id])
                    .launch();
}
