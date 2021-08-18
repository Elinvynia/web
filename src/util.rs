use rocket::response::{Redirect, Responder};
use rocket_dyn_templates::Template;

#[derive(Debug, Responder)]
pub enum MultiResponse {
    Template(Template),
    Redirect(Redirect),
}
