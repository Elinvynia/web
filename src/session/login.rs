use crate::{Auth, Db, MultiResponse, Session, WebError};
use rocket::form::{Form, FromForm};
use rocket::http::{Cookie, CookieJar};
use rocket::response::Redirect;
use rocket::{get, post, uri};
use rocket_db_pools::Connection;
use rocket_dyn_templates::{context, Template};

#[get("/login")]
pub fn login_get_authed(_a: Auth) -> Redirect {
    Redirect::to(uri!("/"))
}

#[get("/login", rank = 2)]
pub fn login_get() -> Template {
    Template::render("login", context!())
}

#[post("/login")]
pub fn login_post_authed(_a: Auth) -> Redirect {
    Redirect::to(uri!("/"))
}

#[derive(FromForm)]
pub struct LoginData {
    pub username: String,
    pub password: String,
}

#[post("/login", data = "<form>", rank = 2)]
pub async fn login_post(
    form: Form<LoginData>,
    mut conn: Connection<Db>,
    jar: &CookieJar<'_>,
) -> Result<MultiResponse, WebError> {
    let q = sqlx::query!(
        "SELECT id, username, password FROM user WHERE username = ?",
        &form.username
    )
    .fetch_optional(&mut *conn)
    .await?;
    let row = match q {
        Some(row) => row,
        None => {
            return Ok(MultiResponse::Template(Template::render(
                "login",
                context!(error: "User does not exist."),
            )))
        }
    };

    let valid = bcrypt::verify(&form.password, &row.password)?;
    if !valid {
        return Ok(MultiResponse::Template(Template::render(
            "login",
            context!(error: "Invalid password."),
        )));
    }

    let session = Session {
        user_id: row.id,
        username: row.username,
    };
    let serialized = serde_json::to_string(&session)?;
    let cookie = Cookie::new("session", serialized);
    jar.add_private(cookie);

    Ok(MultiResponse::Redirect(Redirect::to(uri!("/"))))
}
