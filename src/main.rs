#![feature(async_closure, proc_macro_hygiene, decl_macro, option_result_contains)]

extern crate log;

#[macro_use]
extern crate rocket;

#[macro_use]
extern crate rocket_contrib;

mod models;
mod routes;
mod service;
mod utils;

use crate::routes::*;
use std::time::Duration;

#[launch]
fn rocket() -> _ {
    env_logger::init();

    let client = reqwest::Client::builder()
        .connect_timeout(Duration::from_millis(20000))
        .build()
        .unwrap();

    rocket::build()
        .mount("/", active_routes())
        .register("/", error_catchers())
        .manage(client)
}
