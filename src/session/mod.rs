mod login;
use login::{login_get, login_get_authed, login_post, login_post_authed};
mod logout;
use logout::{logout_get, logout_get_authed};
mod register;
use register::{register_get, register_get_authed, register_post, register_post_authed};
mod auth;
pub use auth::{Auth, Session};

use rocket::{routes, Route};
pub fn routes() -> Vec<Route> {
    routes!(
        login_get_authed,
        login_get,
        login_post_authed,
        login_post,
        logout_get_authed,
        logout_get,
        register_get_authed,
        register_get,
        register_post_authed,
        register_post
    )
}
