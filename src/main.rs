use rocket::fs::{relative, FileServer};
use rocket_db_pools::Database;
use rocket_dyn_templates::Template;

mod error;
pub use error::WebError;
mod index;
mod session;
pub use session::{Auth, Session};
mod util;
pub use util::MultiResponse;

#[derive(Database)]
#[database("web")]
pub struct Db(sqlx::MySqlPool);

#[rocket::launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", index::routes())
        .mount("/", session::routes())
        .mount("/static", FileServer::from(relative!("static")))
        .attach(Template::fairing())
        .attach(Db::init())
}
