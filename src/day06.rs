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
}

#[tracing::instrument]
async fn elf_counting(text: String) -> HttpResponse {
    static RE_ELF: OnceLock<Regex> = OnceLock::new();
    let re_elf = RE_ELF.get_or_init(|| Regex::new("elf").expect("Failed to compile regex"));
    let result = Output {
        elf: re_elf.find_iter(&text).count(),
    };
    info!("result={result:?}");
    HttpResponse::Ok().json(result)
}
