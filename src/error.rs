use rocket::http::Status;
use rocket::response::Responder;

#[derive(Debug)]
pub enum WebError {
    Sqlx(sqlx::Error),
    Bcrypt(bcrypt::BcryptError),
    Json(serde_json::Error),
}

impl<'r> Responder<'r, 'static> for WebError {
    fn respond_to(self, _request: &'r rocket::Request<'_>) -> rocket::response::Result<'static> {
        match self {
            WebError::Sqlx(_) => Err(Status::InternalServerError),
            WebError::Bcrypt(_) => Err(Status::InternalServerError),
            WebError::Json(_) => Err(Status::InternalServerError),
        }
    }
}

impl From<sqlx::Error> for WebError {
    fn from(e: sqlx::Error) -> Self {
        WebError::Sqlx(e)
    }
}

impl From<bcrypt::BcryptError> for WebError {
    fn from(e: bcrypt::BcryptError) -> Self {
        WebError::Bcrypt(e)
    }
}

impl From<serde_json::Error> for WebError {
    fn from(e: serde_json::Error) -> Self {
        WebError::Json(e)
    }
}
