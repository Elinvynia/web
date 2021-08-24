use crate::Auth;
use rocket::{get, routes, Route};
use rocket_dyn_templates::{context, Template};

#[get("/")]
pub fn index_get_authed(a: Auth) -> Template {
    Template::render("index", context!(session: a.0))
}

#[get("/", rank = 2)]
pub fn index_get() -> Template {
    Template::render("index", context!())
}

pub fn routes() -> Vec<Route> {
    routes!(index_get_authed, index_get)
}
