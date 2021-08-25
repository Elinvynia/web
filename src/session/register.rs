use crate::{Auth, Db, WebError};
use rocket::form::{Form, FromForm};
use rocket::response::Redirect;
use rocket::{get, post, uri};
use rocket_db_pools::Connection;
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

#[derive(FromForm)]
pub struct RegisterData {
    pub username: String,
    pub password: String,
}

#[post("/register", rank = 2, data = "<form>")]
pub async fn register_post(
    form: Form<RegisterData>,
    mut conn: Connection<Db>,
) -> Result<Template, WebError> {
    let row = sqlx::query!("SELECT * FROM user WHERE username = ?", &form.username)
        .fetch_optional(&mut *conn)
        .await?;

    if row.is_some() {
        return Ok(Template::render(
            "register",
            context!(error: "User already exists."),
        ));
    }

    let password = bcrypt::hash(&form.password, bcrypt::DEFAULT_COST)?;
    sqlx::query!(
        "INSERT INTO user (username, password) VALUES (?, ?)",
        form.username,
        password
    )
    .execute(&mut *conn)
    .await?;

    Ok(Template::render(
        "register",
        context!(message: "Registered successfully!"),
    ))
}
