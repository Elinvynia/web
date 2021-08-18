use rocket::{get, routes, Route};
use rocket_dyn_templates::{context, Template};

#[get("/")]
pub fn index_get() -> Template {
    Template::render("index", context!())
}

pub fn routes() -> Vec<Route> {
    routes!(index_get)
}
