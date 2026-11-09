use axum_extra::extract::cookie::{Cookie, CookieJar, SameSite};
use chrono::Duration;
use sea_orm::{ColumnTrait, DatabaseConnection, DbErr, EntityTrait, QueryFilter};

use crate::{
    models::{
        publication,
        subscriber::{self, Entity as Subscriber},
        user::{self, Entity as User},
    },
    services::{comments::{Commenter, Login, Member, lifecycle, members}, signing::Signer},
    state::{AppState, Urls},
};

const MEMBER: &str = "commenter-member";
const SUBSCRIBER: &str = "commenter-subscriber";
const PENDING: &str = "commenter-pending";
const CONFIRMING: &str = "commenter-confirming";
const SIGN_IN: &str = "comment-sign-in";
const FOOTER: &str = "comment-footer";
const SIGN_IN_TTL: Duration = Duration::minutes(5);
const CREDENTIAL_TTL: Duration = Duration::days(30);
const SUBSCRIBER_TTL: Duration = Duration::days(365);
const CONFIRMING_TTL: Duration = Duration::days(1);
const PENDING_LIMIT: usize = 10;

pub fn comment_url(signer: &Signer, newsletter_url: &str, newsletter_id: &str, subscriber_token: &str) -> String {
    let signature = signer.sign(FOOTER, &[newsletter_id, subscriber_token]);
    format!("{newsletter_url}?commenter={subscriber_token}.{signature}")
}

fn footer_subscriber<'t>(signer: &Signer, newsletter_id: &str, token: &'t str) -> Option<&'t str> {
    let (subscriber_token, signature) = token.rsplit_once('.')?;
    signer.verify(FOOTER, &[newsletter_id, subscriber_token], signature).then_some(subscriber_token)
}

pub fn sign_in_token(signer: &Signer, user: &user::Model) -> String {
    signer.issue(SIGN_IN, &user.id, &user.password, SIGN_IN_TTL)
}

async fn bound_user(db: &DatabaseConnection, signer: &Signer, purpose: &str, token: &str) -> Result<Option<user::Model>, DbErr> {
    let Some(user_id) = Signer::subject(token) else {
        return Ok(None);
    };
    let user = User::find_by_id(user_id).one(db).await?;
    Ok(user.filter(|user| signer.open(purpose, token, &user.password).is_some()))
}

fn cookie(name: &'static str, value: String, ttl: Duration, domain: Option<&str>, urls: &Urls) -> Cookie<'static> {
    let mut cookie = Cookie::build((name, value))
        .path("/")
        .http_only(true)
        .same_site(SameSite::Lax)
        .secure(urls.https)
        .max_age(time::Duration::seconds(ttl.num_seconds()))
        .build();
    if let Some(domain) = domain {
        cookie.set_domain(domain.to_string());
    }
    cookie
}

fn expired(name: &'static str, domain: Option<&str>) -> Cookie<'static> {
    let mut cookie = Cookie::build((name, "")).path("/").build();
    if let Some(domain) = domain {
        cookie.set_domain(domain.to_string());
    }
    cookie
}

pub async fn sign_in_member(state: &AppState, token: &str, jar: CookieJar) -> Result<(CookieJar, bool), DbErr> {
    let Some(user) = bound_user(&state.db, &state.signer, SIGN_IN, token).await? else {
        return Ok((jar, false));
    };
    let value = state.signer.issue(MEMBER, &user.id, &user.password, CREDENTIAL_TTL);
    Ok((jar.add(cookie(MEMBER, value, CREDENTIAL_TTL, Some(&state.urls.publication_domain), &state.urls)), true))
}

pub async fn sign_in_subscriber(state: &AppState, publication: &publication::Model, newsletter_id: &str, token: &str, jar: CookieJar) -> Result<CookieJar, DbErr> {
    let Some(subscriber_token) = footer_subscriber(&state.signer, newsletter_id, token) else {
        return Ok(jar);
    };
    Ok(remember_subscriber(state, publication, subscriber_token, jar).await?.0)
}

pub async fn remember_subscriber(
    state: &AppState,
    publication: &publication::Model,
    subscriber_token: &str,
    jar: CookieJar,
) -> Result<(CookieJar, Option<subscriber::Model>), DbErr> {
    let Some(subscriber) = confirmed_subscriber(&state.db, publication, subscriber_token).await? else {
        return Ok((jar, None));
    };
    let value = state.signer.issue(SUBSCRIBER, &subscriber.token, &subscriber.publication_id, SUBSCRIBER_TTL);
    Ok((jar.add(cookie(SUBSCRIBER, value, SUBSCRIBER_TTL, None, &state.urls)), Some(subscriber)))
}

pub fn hold_confirmation(urls: &Urls, verify_token: &str, jar: CookieJar) -> CookieJar {
    jar.add(cookie(CONFIRMING, verify_token.to_string(), CONFIRMING_TTL, None, urls))
}

pub fn confirming(signer: &Signer, jar: &CookieJar) -> Option<String> {
    let token = jar.get(CONFIRMING)?.value();
    lifecycle::open_verify_token(signer, token).map(str::to_string)
}

pub fn release_confirmation(jar: CookieJar) -> CookieJar {
    jar.remove(expired(CONFIRMING, None))
}

pub fn sign_out(urls: &Urls, jar: CookieJar) -> CookieJar {
    jar.remove(expired(MEMBER, Some(&urls.publication_domain))).remove(expired(SUBSCRIBER, None))
}

async fn confirmed_subscriber(db: &DatabaseConnection, publication: &publication::Model, token: &str) -> Result<Option<subscriber::Model>, DbErr> {
    Subscriber::find_by_id(token)
        .filter(subscriber::Column::PublicationId.eq(&publication.id))
        .filter(subscriber::Column::IsConfirmed.eq(true))
        .one(db)
        .await
}

async fn member(state: &AppState, user: &user::Model) -> Result<Member, DbErr> {
    let mut found = members(&state.db, &state.urls, std::slice::from_ref(&user.id)).await?;
    Ok(found.remove(&user.id).unwrap_or_else(|| Member { user_id: user.id.clone(), name: "Showroom member".to_string(), url: None }))
}

pub async fn resolve(state: &AppState, jar: &CookieJar, viewer: Option<&user::Model>, publication: &publication::Model) -> Result<Commenter, DbErr> {
    if let Some(user) = viewer {
        return Ok(Commenter::Member(member(state, user).await?, Login::Showroom));
    }

    if let Some(token) = jar.get(MEMBER).map(Cookie::value) {
        if let Some(user) = bound_user(&state.db, &state.signer, MEMBER, token).await? {
            return Ok(Commenter::Member(member(state, &user).await?, Login::Link));
        }
    }

    let Some(token) = jar.get(SUBSCRIBER).map(Cookie::value) else {
        return Ok(Commenter::Reader);
    };
    let Some(subscriber_token) = Signer::subject(token) else {
        return Ok(Commenter::Reader);
    };
    let subscriber = confirmed_subscriber(&state.db, publication, &subscriber_token)
        .await?
        .filter(|subscriber| state.signer.open(SUBSCRIBER, token, &subscriber.publication_id).is_some());
    Ok(subscriber.map_or(Commenter::Reader, Commenter::Subscriber))
}

#[derive(Default)]
pub struct Pending(Vec<String>);

impl Pending {
    pub fn read(signer: &Signer, jar: &CookieJar) -> Self {
        let ids = jar
            .get(PENDING)
            .map(|cookie| {
                cookie
                    .value()
                    .split(',')
                    .filter_map(|entry| entry.split_once('.'))
                    .filter(|(id, signature)| signer.verify(PENDING, &[id], signature))
                    .map(|(id, _)| id.to_string())
                    .collect()
            })
            .unwrap_or_default();
        Self(ids)
    }

    pub fn ids(&self) -> &[String] {
        &self.0
    }

    pub fn contains(&self, id: &str) -> bool {
        self.0.iter().any(|pending| pending == id)
    }

    pub fn remember(&mut self, signer: &Signer, urls: &Urls, id: &str, jar: CookieJar) -> CookieJar {
        self.0.retain(|pending| pending != id);
        self.0.push(id.to_string());
        let skip = self.0.len().saturating_sub(PENDING_LIMIT);
        self.0.drain(..skip);
        let value = self.0.iter().map(|id| format!("{id}.{}", signer.sign(PENDING, &[id]))).collect::<Vec<_>>().join(",");
        jar.add(cookie(PENDING, value, lifecycle::UNVERIFIED_TTL, None, urls))
    }
}
