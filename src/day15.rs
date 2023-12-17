use std::sync::OnceLock;

use actix_web::{
    http::{header::ContentType, StatusCode},
    web::{self, Json},
    HttpResponse, Scope,
};
use regex::Regex;
use serde::{Deserialize, Serialize};
use tracing::info;
use unic_emoji_char::is_emoji_presentation;
pub(crate) fn scope() -> Scope {
    web::scope("/15")
        .route("/nice", web::post().to(task1_naughty_or_nice_strings))
        .route("/nice", web::post().to(task1_bad_request))
        .route("/game", web::post().to(task2_game_of_the_year))
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
        return false; // No charters so no 2 letters in a row
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

#[derive(Debug, Serialize)]
struct GameResult {
    result: String,
    reason: String,
}

impl GameResult {
    fn ok() -> Self {
        Self {
            result: "nice".to_string(),
            reason: "that's a nice password".to_string(),
        }
    }
    fn error(msg: &str) -> Self {
        Self {
            result: "naughty".to_string(),
            reason: msg.to_string(),
        }
    }
}

fn compose_response(code: StatusCode, game_result: GameResult) -> HttpResponse {
    HttpResponse::build(code)
        .insert_header(ContentType::html())
        .json(game_result)
}

#[tracing::instrument]
async fn task2_game_of_the_year(Json(Input { input: p }): web::Json<Input>) -> HttpResponse {
    let result = if let Some(e) = rule1(&p) {
        e
    } else if let Some(e) = rule2(&p) {
        e
    } else if let Some(e) = rule3(&p) {
        e
    } else if let Some(e) = rule4(&p) {
        e
    } else if let Some(e) = rule5(&p) {
        e
    } else if let Some(e) = rule6(&p) {
        e
    } else if let Some(e) = rule7(&p) {
        e
    } else if let Some(e) = rule8(&p) {
        e
    } else if let Some(e) = rule9(&p) {
        e
    } else {
        compose_response(StatusCode::OK, GameResult::ok())
    };
    info!("result = {result:?} {:?}", result.body());
    result
}

fn rule1(p: &str) -> Option<HttpResponse> {
    if p.chars().count() >= 8 {
        return None;
    }
    Some(compose_response(
        StatusCode::BAD_REQUEST,
        GameResult::error("8 chars"),
    ))
}

fn rule2(p: &str) -> Option<HttpResponse> {
    static RE_UPPER: OnceLock<Regex> = OnceLock::new();
    static RE_LOWER: OnceLock<Regex> = OnceLock::new();
    static RE_DIGIT: OnceLock<Regex> = OnceLock::new();
    let re_upper = RE_UPPER.get_or_init(|| Regex::new("[A-Z]").expect("Failed to compile regex"));
    let re_lower = RE_LOWER.get_or_init(|| Regex::new("[a-z]").expect("Failed to compile regex"));
    let re_digit = RE_DIGIT.get_or_init(|| Regex::new(r"\d").expect("Failed to compile regex"));
    if re_upper.is_match(p) && re_lower.is_match(p) & re_digit.is_match(p) {
        return None;
    }
    Some(compose_response(
        StatusCode::BAD_REQUEST,
        GameResult::error("more types of chars"),
    ))
}

fn rule3(p: &str) -> Option<HttpResponse> {
    static RE_DIGIT: OnceLock<Regex> = OnceLock::new();
    let re_digit = RE_DIGIT.get_or_init(|| Regex::new(r"\d").expect("Failed to compile regex"));
    if re_digit.find_iter(p).count() >= 5 {
        return None;
    }
    Some(compose_response(
        StatusCode::BAD_REQUEST,
        GameResult::error("55555"),
    ))
}

fn rule4(p: &str) -> Option<HttpResponse> {
    static RE_DIGIT: OnceLock<Regex> = OnceLock::new();
    let re_digit = RE_DIGIT.get_or_init(|| Regex::new(r"\d+").expect("Failed to compile regex"));
    if re_digit
        .find_iter(p)
        .map(|x| x.as_str().parse::<i32>().unwrap())
        .sum::<i32>()
        == 2023
    {
        return None;
    }
    Some(compose_response(
        StatusCode::BAD_REQUEST,
        GameResult::error("math is hard"),
    ))
}

fn rule5(p: &str) -> Option<HttpResponse> {
    let first = 'j';
    let second = 'o';
    let third = 'y';
    let all = [first, second, third];
    let mut expected = Some(first);
    fn fail() -> Option<HttpResponse> {
        Some(compose_response(
            StatusCode::NOT_ACCEPTABLE,
            GameResult::error("not joyful enough"),
        ))
    }

    for c in p.chars() {
        if all.contains(&c) {
            if let Some(exp) = expected {
                if exp == c {
                    expected = match exp {
                        x if x == first => Some(second),
                        x if x == second => Some(third),
                        x if x == third => None,
                        _ => unreachable!("Should only be one of these"),
                    };
                } else {
                    return fail();
                }
            } else {
                return fail();
            }
        }
    }
    if expected.is_none() {
        None
    } else {
        fail()
    }
}

fn rule6(p: &str) -> Option<HttpResponse> {
    let chars: Vec<char> = p.chars().collect();
    for window in chars.windows(3) {
        if !window[0].is_alphabetic() || !window[1].is_alphabetic() || !window[2].is_alphabetic() {
            continue;
        }
        if window[0] == window[2] && window[1] != window[2] {
            return None;
        }
    }
    Some(compose_response(
        StatusCode::UNAVAILABLE_FOR_LEGAL_REASONS,
        GameResult::error("illegal: no sandwich"),
    ))
}

fn rule7(p: &str) -> Option<HttpResponse> {
    for c in p.chars() {
        if ('\u{2980}'..='\u{2BFF}').contains(&c) {
            return None;
        }
    }
    Some(compose_response(
        StatusCode::RANGE_NOT_SATISFIABLE,
        GameResult::error("outranged"),
    ))
}

fn rule8(p: &str) -> Option<HttpResponse> {
    if p.chars().any(is_emoji_presentation) {
        None
    } else {
        Some(compose_response(
            StatusCode::UPGRADE_REQUIRED,
            GameResult::error("😳"),
        ))
    }
}

fn rule9(p: &str) -> Option<HttpResponse> {
    // Taken from https://rust-lang-nursery.github.io/rust-cookbook/cryptography/hashing.html
    use data_encoding::HEXUPPER;
    use ring::digest::{Context, Digest, SHA256};

    fn sha256_digest(x: &str) -> Digest {
        let mut context = Context::new(&SHA256);
        context.update(x.as_bytes());
        context.finish()
    }

    let digest = sha256_digest(p);
    let hex_digest = HEXUPPER.encode(digest.as_ref());
    info!("SHA-256 digest is {}", hex_digest);

    match hex_digest.chars().last().unwrap() {
        'a' | 'A' => None,
        _ => Some(compose_response(
            StatusCode::IM_A_TEAPOT,
            GameResult::error("not a coffee brewer"),
        )),
    }
}
