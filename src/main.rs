use rocket::{get, routes};
use rocket::http::Status;

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
    let rocket = rocket::build().mount("/", routes![index, error]);

    Ok(rocket.into())
}
