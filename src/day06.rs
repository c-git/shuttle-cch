use actix_web::{web, HttpResponse, Scope};
use serde::Serialize;
use tracing::info;
pub(crate) fn scope() -> Scope {
    web::scope("/6").route("", web::post().to(elf_counting))
}

#[derive(Debug, Serialize)]
struct Output {
    elf: usize,
    #[serde(rename = "elf on a shelf")]
    elf_on_a_shelf: usize,
    #[serde(rename = "shelf with no elf on it")]
    shelf_with_no_elf: usize,
}

#[tracing::instrument]
async fn elf_counting(text: String) -> HttpResponse {
    // A more elegant solution is likely able to be found possibly using aho_corasick but for simplicity I'm going to use a simple brute force search
    let elf = count("elf", &text);
    let elf_on_a_shelf = count("elf on a shelf", &text);
    let shelf = count("shelf", &text);
    let result = Output {
        elf,
        elf_on_a_shelf,
        shelf_with_no_elf: shelf - elf_on_a_shelf,
    };
    info!("result={result:?}");
    HttpResponse::Ok().json(result)
}

fn count(key: &str, text: &str) -> usize {
    let key = key.as_bytes();
    let mut text = text.as_bytes();
    let mut result = 0;
    while text.len() >= key.len() {
        if text.starts_with(key) {
            result += 1;
        }
        text = &text[1..];
    }
    result
}
