use std::ops::BitXor;
use std::path::PathBuf;

use rocket::get;
use rocket::http::Status;

#[get("/1/<packets..>")]
pub fn calibrate(packets: PathBuf) -> (Status, String) {
    match packets
        .into_iter()
        .map(|segment| segment.to_string_lossy().parse::<i64>())
        .reduce(|acc, n| acc.and_then(|a| n.map(|b| a.bitxor(b))))
        .map(|n| n.map(|n| n.pow(3)))
    {
        Some(Ok(id)) => (Status::Ok, id.to_string()),
        Some(Err(err)) => (Status::BadRequest, err.to_string()),
        None => (Status::BadRequest, "0 packets were specified".to_string()),
    }
}
