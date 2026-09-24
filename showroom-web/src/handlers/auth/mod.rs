mod login;
mod logout;
mod reset;
mod signup;

pub use login::{login, login_page};
pub use logout::logout;
pub use reset::{forgot_password, forgot_password_page, reset_password, reset_password_page};
pub use signup::signup;

#[cfg(debug_assertions)]
pub use signup::signup_page;

use axum::response::{IntoResponse, Response};
use boutique::htmx;

fn something_went_wrong() -> Response {
    htmx::fragments::error("Something went wrong, please try again").into_response()
}
