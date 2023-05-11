use actix_web::{HttpResponse, error::ResponseError};
use derive_more::Display;

#[derive(Debug, Display)]
pub enum Error {
    #[display(fmt = "Internal Server Error {}", _0)]
    InternalServerError(String),

    #[display(fmt = "BadRequest: {}", _0)]
    BadRequest(String),

    #[display(fmt = "Unauthorized, {}", _0)]
    Unauthorized(String),
}

pub trait Success {
    fn message(message: String) -> HttpResponse;
    fn list(object: surrealdb::sql::Object) -> HttpResponse;
    fn lists(vec_object: Vec<surrealdb::sql::Object>) -> HttpResponse;
}
pub struct Response;

impl Success for Response {
    fn message(message: String) -> HttpResponse {
        return HttpResponse::Ok().json(serde_json::json!({"status": 200,"message": "success", "ok": message }));
    }
    fn list(object: surrealdb::sql::Object) -> HttpResponse {
        return HttpResponse::Ok().json(serde_json::json!({"status": 200,"message": "success", "data": object }));
    }
    fn lists(vec_object: Vec<surrealdb::sql::Object>) -> HttpResponse {
        return HttpResponse::Ok().json(serde_json::json!({"status": 200, "message": "success", "data": vec_object }));
    }
}
impl ResponseError for Error {
    fn error_response(&self) -> HttpResponse {
        match self {
            Error::InternalServerError(ref message) => HttpResponse::InternalServerError()
                .json(serde_json::json!({"status": 500, "message": "fail", "error": message})),
            Error::BadRequest(ref message) => HttpResponse::BadRequest()
                .json(serde_json::json!({"status": 400, "message": "fail", "error": message})),
            Error::Unauthorized(ref message) => HttpResponse::Unauthorized()
                .json(serde_json::json!({"status": 401, "message": "fail", "error": message})),
        }
    }
}