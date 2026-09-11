use axum::extract::State;
use boutique::{AuthenticatedUser, reset};
use maud::Markup;

use crate::{
    mailer,
    models::publication,
    state::AppState,
    views::{self, PageContext},
};

pub async fn get_settings(
    State(state): State<AppState>,
    AuthenticatedUser(user): AuthenticatedUser,
) -> Markup {
    let publications = publication::for_owner(&user.id, &state.db).await.unwrap_or_default();
    let ctx = PageContext::from_user(&user, state.urls.clone()).with_publications(publications);
    views::dashboard::settings::index(&ctx)
}

pub async fn request_password_change(
    State(state): State<AppState>,
    AuthenticatedUser(user): AuthenticatedUser,
) -> Markup {
    let token = match reset::token_for(&state.auth, &user) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("[password reset] token generation failed: {e}");
            return views::dashboard::settings::change_password_error("Something went wrong, please try again");
        }
    };

    let reset_url = format!("{}/reset-password?token={}", state.urls.base(), token);

    match mailer::send_password_reset(&state.ses, &user.email, &reset_url, &state.urls).await {
        Ok(_) => views::dashboard::settings::change_password_requested(),
        Err(e) => {
            eprintln!("[password reset] email failed for {}: {e}", user.email);
            views::dashboard::settings::change_password_error(&format!("Failed to send email: {e}"))
        }
    }
}
