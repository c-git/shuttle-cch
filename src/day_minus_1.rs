use actix_web::{error, web};

pub async fn task1_everything_is_ok() -> &'static str {
    "Day -1 Task 1"
}

async fn task2_error() -> actix_web::Result<String> {
    Err(error::ErrorInternalServerError("oops something went wrong"))
}

pub(crate) fn scope() -> actix_web::Scope {
    web::scope("-1").route("/error", web::get().to(task2_error))
}
