use axum::{
    extract::{Extension, State},
    middleware::Next,
    response::{IntoResponse, Redirect, Response},
    http::Request,
};
use axum_extra::extract::cookie::CookieJar;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};

use crate::{
    auth::{context::UserContext, cookies, jwt},
    models::{refresh_token::{self, Entity as RefreshToken}, user::Entity as User},
    state::AppState,
};

pub async fn base(
    State(state): State<AppState>,
    mut request: Request<axum::body::Body>,
    next: Next,
) -> impl IntoResponse {
    let mut jar = CookieJar::from_headers(request.headers());
    let mut context = UserContext::default();

    if let Some(jwt_cookie) = jar.get("jwt") {
        match jwt::validate(state.jwt_secret.as_bytes(), jwt_cookie.value()) {
            Ok(claims) => context.user_id = Some(claims.user_id),
            Err(_) => {
                jar = jar
                    .remove(cookies::remove("jwt", &state.urls.cookie))
                    .remove(cookies::remove("refresh", &state.urls.cookie));
            }
        }
    } else if let Some(refresh_cookie) = jar.get("refresh") {
        let token_value = refresh_cookie.value().to_string();
        let record = RefreshToken::find()
            .filter(refresh_token::Column::Token.eq(&token_value))
            .one(&state.db)
            .await
            .ok()
            .flatten();

        if let Some(record) = record {
            if let Ok(Some(user)) = User::find_by_id(&record.user_id).one(&state.db).await {
                let claims = jwt::Claims::new(&user.email, &user.id);
                if let Ok(token) = jwt::generate(state.jwt_secret.as_bytes(), claims) {
                    context.user_id = Some(user.id);
                    jar = jar.add(cookies::make("jwt", token, 1, &state.urls.cookie));
                }
            }
        }
    }

    request.extensions_mut().insert(context);
    let response = next.run(request).await;
    (jar, response).into_response()
}

pub async fn required_auth(
    State(state): State<AppState>,
    Extension(ctx): Extension<UserContext>,
    request: Request<axum::body::Body>,
    next: Next,
) -> Response {
    if !ctx.is_authenticated() {
        let login_url = format!("{}/login", state.urls.base);
        return Redirect::to(&login_url).into_response();
    }
    next.run(request).await
}
