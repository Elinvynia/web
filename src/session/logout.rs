use crate::Auth;
use rocket::http::{Cookie, CookieJar};
use rocket::response::Redirect;
use rocket::{get, uri};

#[get("/logout")]
pub fn logout_get_authed(_a: Auth, jar: &CookieJar<'_>) -> Redirect {
    jar.remove_private(Cookie::named("session"));
    Redirect::to(uri!("/"))
}

#[get("/logout", rank = 2)]
pub fn logout_get() -> Redirect {
    Redirect::to(uri!("/"))
}
