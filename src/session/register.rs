use crate::{Auth, MultiResponse};
use rocket::response::Redirect;
use rocket::{get, post, uri};
use rocket_dyn_templates::{context, Template};

#[get("/register")]
pub fn register_get_authed(_a: Auth) -> Redirect {
    Redirect::to(uri!("/"))
}

#[get("/register", rank = 2)]
pub fn register_get() -> Template {
    Template::render("register", context!())
}

#[post("/register")]
pub fn register_post_authed(_a: Auth) -> Redirect {
    Redirect::to(uri!("/"))
}

#[post("/register")]
pub async fn register_post() -> MultiResponse {
    todo!()
}
