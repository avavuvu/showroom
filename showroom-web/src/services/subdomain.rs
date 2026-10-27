use axum::{
    body::Body,
    extract::FromRequestParts,
    http::{StatusCode, Request, request::Parts},
    response::Response,
};
use boutique::AppError;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use std::convert::Infallible;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use tower::Service;

use crate::{models::publication::{self, Entity as Publication}, state::{AppState, Urls}};

#[derive(Clone, Debug)]
pub struct PublicationSlug(pub String);

impl<S: Send + Sync> FromRequestParts<S> for PublicationSlug {
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        parts
            .extensions
            .get::<PublicationSlug>()
            .cloned()
            .ok_or(AppError::NotFound)
    }
}

pub struct CurrentPublication(pub publication::Model);

impl FromRequestParts<AppState> for CurrentPublication {
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, state: &AppState) -> Result<Self, Self::Rejection> {
        let PublicationSlug(slug) = PublicationSlug::from_request_parts(parts, state).await?;

        Publication::find()
            .filter(publication::Column::Slug.eq(&slug))
            .one(&state.db)
            .await?
            .map(CurrentPublication)
            .ok_or(AppError::NotFound)
    }
}

#[derive(Clone)]
pub struct SubdomainRouter {
    base: axum::Router,
    app: axum::Router,
    publication: axum::Router,
    urls: Urls,
}

enum SubdomainKind {
    Base,
    App,
    Publication(String),
    Redirect(String),
}

impl SubdomainRouter {
    pub fn new(base: axum::Router, app: axum::Router, publication: axum::Router, urls: Urls) -> Self {
        Self { base, app, publication, urls }
    }

    fn classify(&self, host: &str) -> SubdomainKind {
        let host = host.split(':').next().unwrap_or(host);
        let site = self.urls.site_domain.as_str();
        let publications = self.urls.publication_domain.as_str();
        let is_split = site != publications;

        if host == site {
            SubdomainKind::Base
        } else if host == self.urls.app_domain() {
            SubdomainKind::App
        } else if host == format!("www.{site}") || host == format!("www.{publications}") {
            SubdomainKind::Redirect(self.urls.base())
        } else if self.urls.old_site_domains.iter().any(|old| old == host) {
            SubdomainKind::Redirect(self.urls.base())
        } else if is_split && host == format!("app.{publications}") {
            SubdomainKind::Redirect(self.urls.app())
        } else if is_split && host == publications {
            SubdomainKind::Redirect(self.urls.base())
        } else {
            match host.strip_suffix(&format!(".{publications}")) {
                Some(slug) if !slug.is_empty() && !slug.contains('.') => SubdomainKind::Publication(slug.to_string()),
                _ => SubdomainKind::Base,
            }
        }
    }
}

impl Service<Request<Body>> for SubdomainRouter {
    type Response = Response;
    type Error = Infallible;
    type Future = Pin<Box<dyn Future<Output = Result<Response, Infallible>> + Send>>;

    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, mut req: Request<Body>) -> Self::Future {
        let host = req
            .headers()
            .get(axum::http::header::HOST)
            .and_then(|v| v.to_str().ok())
            .unwrap_or("")
            .to_string();

        match self.classify(&host) {
            SubdomainKind::Base => {
                let mut router = self.base.clone();
                Box::pin(async move { router.call(req).await })
            }
            SubdomainKind::App => {
                let mut router = self.app.clone();
                Box::pin(async move { router.call(req).await })
            }
            SubdomainKind::Publication(slug) => {
                req.extensions_mut().insert(PublicationSlug(slug));
                let mut router = self.publication.clone();
                Box::pin(async move { router.call(req).await })
            }
            SubdomainKind::Redirect(target) => {
                let path = req.uri().path_and_query().map(|p| p.as_str()).unwrap_or("/");
                let location = format!("{target}{path}");
                Box::pin(async move {
                    Ok(Response::builder()
                        .status(StatusCode::MOVED_PERMANENTLY)
                        .header(axum::http::header::LOCATION, location)
                        .body(Body::empty())
                        .unwrap())
                })
            }
        }
    }
}
