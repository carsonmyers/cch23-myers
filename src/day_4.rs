use rocket::http::Status;
use rocket::post;
use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct ReindeerStatsShort {
    #[allow(unused)]
    name: String,
    strength: i64,
}

#[post("/4/strength", data = "<stats>")]
pub fn strength(stats: Json<Vec<ReindeerStatsShort>>) -> (Status, String) {
    let res = stats.iter().map(|stats| stats.strength).sum::<i64>();

    (Status::Ok, format!("{}", res))
}

#[derive(Debug, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct ReindeerStats {
    name: String,
    strength: i64,
    speed: f64,
    height: i64,
    antler_width: i64,
    snow_magic_power: i64,
    favorite_food: String,
    #[serde(rename = "cAnD13s_3ATeN-yesT3rdAy")]
    candies_eaten_yesterday: i64,
}

#[derive(Debug, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct ContestResults {
    fastest: String,
    tallest: String,
    magician: String,
    consumer: String,
}

#[post("/4/contest", data = "<stats>")]
pub fn contest(stats: Json<Vec<ReindeerStats>>) -> Json<ContestResults> {
    let fastest = stats
        .iter()
        .max_by(|a, b| a.speed.partial_cmp(&b.speed).unwrap())
        .expect("fastest reindeer");
    let tallest = stats
        .iter()
        .max_by_key(|stats| stats.height)
        .expect("tallest reindeer");
    let magician = stats
        .iter()
        .max_by_key(|stats| stats.snow_magic_power)
        .expect("magicest reindeer");
    let consumer = stats
        .iter()
        .max_by_key(|stats| stats.candies_eaten_yesterday)
        .expect("hungriest reindeer");

    Json(ContestResults {
        fastest: format!(
            "Speeding past the finish line with a strength of {} is {}",
            fastest.strength, fastest.name
        ),
        tallest: format!(
            "{} is standing tall with his {} cm wide antlers",
            tallest.name, tallest.antler_width
        ),
        magician: format!(
            "{} could blast you away with a snow magic power of {}",
            magician.name, magician.snow_magic_power
        ),
        consumer: format!(
            "{} ate lots of candies, but also some {}",
            consumer.name, consumer.favorite_food
        ),
    })
}
