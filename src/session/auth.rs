use rocket::http::{Cookie, Status};
use rocket::request::{FromRequest, Outcome, Request};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Session {
    pub user_id: u64,
    pub username: String,
}

pub struct Auth(pub Session);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Auth {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let cookies = request.cookies();

        if let Some(cookie) = cookies.get_private("session") {
            let session = match serde_json::from_str(cookie.value()) {
                Ok(s) => s,
                Err(_) => {
                    cookies.remove_private(Cookie::named("session"));
                    return Outcome::Failure((Status::InternalServerError, ()));
                }
            };
            return Outcome::Success(Auth(session));
        }

        return Outcome::Forward(());
    }
}
