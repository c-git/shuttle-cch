use std::sync::OnceLock;

use actix_web::{web, HttpResponse, Scope};
use regex::Regex;
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
    static RE_ELF: OnceLock<Regex> = OnceLock::new();
    static RE_ELF_ON_A_SHELF: OnceLock<Regex> = OnceLock::new();
    static RE_SHELF: OnceLock<Regex> = OnceLock::new();
    let re_elf = RE_ELF.get_or_init(|| Regex::new("elf").expect("Failed to compile regex"));
    let re_elf_on_a_shelf = RE_ELF_ON_A_SHELF
        .get_or_init(|| Regex::new("elf on a shelf").expect("Failed to compile regex"));
    let re_shelf = RE_SHELF.get_or_init(|| Regex::new("shelf").expect("Failed to compile regex"));
    let elf = re_elf.find_iter(&text).count();
    let elf_on_a_shelf = re_elf_on_a_shelf.find_iter(&text).count();
    let shelf = re_shelf.find_iter(&text).count();
    let result = Output {
        elf,
        elf_on_a_shelf,
        shelf_with_no_elf: shelf - elf_on_a_shelf,
    };
    info!("result={result:?}");
    HttpResponse::Ok().json(result)
}
