#![feature(async_closure, proc_macro_hygiene, decl_macro, option_result_contains)]

#[macro_use]
extern crate rocket;

#[macro_use]
extern crate rocket_contrib;

mod routes;
mod utils;

use crate::routes::*;
use std::time::Duration;

#[launch]
fn rocket() -> _ {
    let client = reqwest::Client::builder()
        .connect_timeout(Duration::from_millis(20000))
        .build()
        .unwrap();

    rocket::build()
        .mount("/", active_routes())
        .register("/", error_catchers())
        .manage(client)
}
