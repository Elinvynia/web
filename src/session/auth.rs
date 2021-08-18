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
            let session = serde_json::from_str(cookie.value()).unwrap();
            return Outcome::Success(Auth(session));
        }

        return Outcome::Forward(());
    }
}
