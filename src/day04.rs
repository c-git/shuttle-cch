use actix_web::{web, Scope};
use serde::Deserialize;
use tracing::info;
pub(crate) fn scope() -> Scope {
    web::scope("/4").route("/strength", web::post().to(task1_reindeer_cheer))
}
#[derive(Debug, Deserialize)]
struct Input {
    strength: i32,
}

#[tracing::instrument]
async fn task1_reindeer_cheer(reindeer: web::Json<Vec<Input>>) -> String {
    let result: i32 = reindeer.iter().map(|x| x.strength).sum();
    info!(result);
    result.to_string()
}
