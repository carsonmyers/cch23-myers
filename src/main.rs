mod day_1;
mod day_4;

use rocket::http::Status;
use rocket::{get, routes};

#[get("/-1/error")]
fn error() -> Status {
    Status::InternalServerError
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[shuttle_runtime::main]
async fn main() -> shuttle_rocket::ShuttleRocket {
    let rocket = rocket::build().mount(
        "/",
        routes![
            index,
            error,
            day_1::calibrate,
            day_4::strength,
            day_4::contest,
        ],
    );

    Ok(rocket.into())
}
