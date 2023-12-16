use std::sync::OnceLock;

use actix_web::{
    web::{self, Json},
    HttpResponse, Scope,
};
use regex::Regex;
use serde::{Deserialize, Serialize};
use tracing::info;
pub(crate) fn scope() -> Scope {
    web::scope("/15")
        .route("/nice", web::post().to(task1_naughty_or_nice_strings))
        .route("/nice", web::post().to(task1_bad_request))
}

enum PasswordType {
    Nice,
    Naughty,
    BadRequest,
}

#[derive(Debug, Serialize)]
struct ResultJson {
    result: String,
}

impl ResultJson {
    fn new(value: &str) -> Self {
        Self {
            result: value.to_string(),
        }
    }
}

impl PasswordType {
    fn http_response(&self) -> HttpResponse {
        match self {
            PasswordType::Nice => HttpResponse::Ok().json(ResultJson::new("nice")),
            PasswordType::Naughty => HttpResponse::BadRequest().json(ResultJson::new("naughty")),
            PasswordType::BadRequest => HttpResponse::BadRequest().finish(),
        }
    }
}

#[derive(Debug, Deserialize)]
struct Input {
    input: String,
}

#[tracing::instrument]
async fn task1_naughty_or_nice_strings(
    Json(Input { input: password }): web::Json<Input>,
) -> HttpResponse {
    static RE_VOWEL: OnceLock<Regex> = OnceLock::new();
    static RE_NAUGHTY: OnceLock<Regex> = OnceLock::new();
    let re_vowel =
        RE_VOWEL.get_or_init(|| Regex::new("[aeiouy]").expect("Failed to compile regex"));
    let re_naughty =
        RE_NAUGHTY.get_or_init(|| Regex::new("ab|cd|pq|xy").expect("Failed to compile regex"));
    let has_3_vowels = re_vowel.find_iter(&password).count() >= 3;
    let has_no_naughty_substrings = !re_naughty.is_match(&password);
    let has_letter_twice = check_string_for_2_consecutive_letters(&password);
    info!("has_3_vowels: {has_3_vowels}, has_no_naughty_substrings: {has_no_naughty_substrings}, has_letter_twice: {has_letter_twice}");
    if has_3_vowels && has_no_naughty_substrings && has_letter_twice {
        PasswordType::Nice.http_response()
    } else {
        PasswordType::Naughty.http_response()
    }
}

fn check_string_for_2_consecutive_letters(password: &str) -> bool {
    let mut iter = password.chars();
    let mut last_char = if let Some(c) = iter.next() {
        c
    } else {
        return false; // No charters so not 2 letters in a row
    };
    for c in iter {
        if c.is_alphabetic() && c == last_char {
            return true;
        } else {
            last_char = c;
        }
    }
    false
}

#[tracing::instrument]
async fn task1_bad_request() -> HttpResponse {
    PasswordType::BadRequest.http_response()
}
