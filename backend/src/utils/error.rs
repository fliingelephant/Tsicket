#[derive(Fail, Debug)]
pub enum Error {
    // 401
    #[fail(display = "Unauthorized: {}", _0)]
    Unauthorized(JsonValue),

    // 403
    #[fail(display = "Forbidden: {}", _0)]
    Forbidden(JsonValue),

    // 404
    #[fail(display = "Not Found: {}", _0)]
    NotFound(JsonValue),

    // 422
    #[fail(display = "Unprocessable Entity: {}", _0)]
    UnprocessableEntity(JsonValue),

    // 500
    #[fail(display = "Internal Server Error")]
    InternalServerError,
}