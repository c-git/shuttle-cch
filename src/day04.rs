use actix_web::{web, HttpResponse, Scope};
use float_ord::FloatOrd;
use serde::{Deserialize, Serialize};
use tracing::info;
pub(crate) fn scope() -> Scope {
    web::scope("/4")
        .route("/strength", web::post().to(task1_reindeer_cheer))
        .route(
            "/contest",
            web::post().to(task2_cursed_candy_eating_contest),
        )
}
#[derive(Debug, Deserialize)]
struct Input1 {
    strength: i32,
}

#[tracing::instrument]
async fn task1_reindeer_cheer(reindeer: web::Json<Vec<Input1>>) -> String {
    let result: i32 = reindeer.iter().map(|x| x.strength).sum();
    info!(result);
    result.to_string()
}

#[derive(Debug, Deserialize)]
struct Input2 {
    name: String,
    strength: i32,
    speed: f64,
    height: u32,
    antler_width: u32,
    snow_magic_power: i32,
    favorite_food: String,
    #[serde(rename = "cAnD13s_3ATeN-yesT3rdAy")]
    candy_eaten_yesterday: u32,
}

impl Input2 {
    fn speed(&self) -> FloatOrd<f64> {
        FloatOrd(self.speed)
    }
}
#[derive(Debug, Serialize, Deserialize)]
struct ContestResults {
    fastest: String,
    tallest: String,
    magician: String,
    consumer: String,
}

#[tracing::instrument]
async fn task2_cursed_candy_eating_contest(reindeer: web::Json<Vec<Input2>>) -> HttpResponse {
    // Unwrap is safe by precondition that: "There is at least one reindeer participating in the contest"
    let fastest = reindeer.iter().max_by_key(|x| x.speed()).unwrap();
    let tallest = reindeer.iter().max_by_key(|x| x.height).unwrap();
    let magician = reindeer.iter().max_by_key(|x| x.snow_magic_power).unwrap();
    let consumer = reindeer
        .iter()
        .max_by_key(|x| x.candy_eaten_yesterday)
        .unwrap();
    let result: ContestResults = ContestResults {
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
    };
    info!("result = {result:?}");
    HttpResponse::Ok().json(result)
}
