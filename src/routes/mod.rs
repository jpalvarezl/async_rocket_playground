extern crate rocket;

use rocket::response::Redirect;
use rocket::Catcher;
use rocket::Route;
use rocket_contrib::json::JsonValue;

mod safes;

pub fn active_routes() -> Vec<Route> {
    routes![safes::safe_info]
}

pub fn error_catchers() -> Vec<Catcher> {
    catchers![not_found, panic]
}

#[catch(404)]
async fn not_found() -> JsonValue {
    json!({
        "status": "error",
        "reason": "Resource was not found."
    })
}

#[catch(500)]
async fn panic() -> JsonValue {
    json!({
        "status": "error",
        "reason": "Server error occurred."
    })
}

#[get("/")]
pub async fn root() -> Redirect {
    Redirect::temporary("https://github.com/gnosis/safe-client-gateway/wiki")
}
