//! Domain errors

use log::Level;

/// Custom Result type for `AppError`
pub type AppResult<'a, T> = Result<T, AppError<'a>>;

/// Represents the custom error message
#[derive(Debug, Clone)]
pub struct AppError<'a> {
    pub level: Level,
    pub message: &'a str,
    pub details: Option<&'a str>,
}

impl From<uuid::Error> for AppError {
    fn from(err: uuid::Error) -> Self {
        Self {
            level: Level::Error,
            message: "UUID error",
            details: Some(err.to_string().as_str()),
        }
    }
}

#[derive(Debug)]
enum UserError<'a> {
    NotFound(&'a str),
    AlreadyExists(&'a str),
}

impl From<UserError> for AppError {
    fn from(err: UserError) -> Self {
        match err {
            UserError::NotFound(err) => Self {
                level: Level::Error,
                message: "User not found",
                details: Some(err),
            },
            UserError::AlreadyExists(err) => Self {
                level: Level::Error,
                message: "User already exists",
                details: Some(err),
            },
        }
    }
}

#[derive(Debug, Clone)]
enum ApiErrorCode {
    InternalError,
    BadRequest,
    NotFound,
    UnprocessableEntity,
    Timeout,
    Unauthorized,
    TooManyRequests,
    MethodNotAllowed,
    PayloadTooLarge,
}

#[derive(Debug, Clone)]
struct ApiError {
    code: ApiErrorCode,
    message: String,
    details: Option<String>,
}

impl ApiError {
    fn new(code: ApiErrorCode, error: AppError) -> Self {
        if error.level == Level::Error {
            error!("{}", error.message);
        }

        Self {
            code,
            message: error.message.to_string(),
            details: error.details.map(|d| d.to_string()),
        }
    }
}

fn toto(i: bool) -> Result<(), UserError> {
    if i {
        Ok(())
    } else {
        Err(UserError::NotFound("User not found"))
    }
}

fn test() -> AppResult<()> {
    let error = AppError {
        level: Level::Error,
        message: "Test error",
        details: Some("This is a test error"),
    };

    let api_error = ApiError::new(ApiErrorCode::InternalError, error);
    println!("{:?}", api_error);

    Ok(toto(false)?)
}
