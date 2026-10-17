use axum::extract::State;
use boutique::{AppResult, AuthenticatedUser, reset};
use maud::Markup;

use crate::{
    mailer,
    models::{publication, user},
    state::AppState,
    views::{self, PageContext},
};

pub async fn get_settings(
    State(state): State<AppState>,
    AuthenticatedUser(user): AuthenticatedUser<user::Model>,
) -> AppResult<Markup> {
    let publications = publication::for_owner(&user.id, &state.db).await?;
    let ctx = PageContext::from_user(&user, state.urls.clone()).with_publications(publications);
    Ok(views::dashboard::settings::index(&ctx))
}

pub async fn request_password_change(
    State(state): State<AppState>,
    AuthenticatedUser(user): AuthenticatedUser<user::Model>,
) -> Markup {
    let token = reset::token_for(&state.auth, &user);

    let reset_url = format!("{}/reset-password?token={}", state.urls.base(), token);

    match mailer::send_password_reset(&state.ses, &user.email, &reset_url, &state.urls).await {
        Ok(_) => views::dashboard::settings::change_password_requested(),
        Err(error) => {
            tracing::error!(%error, user = %user.id, "sending a password reset email failed");
            views::dashboard::settings::change_password_error("The email couldn't be sent. Please try again.")
        }
    }
}
