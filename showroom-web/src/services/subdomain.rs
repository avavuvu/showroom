use axum::{
    body::Body,
    extract::FromRequestParts,
    http::{StatusCode, Request, request::Parts},
    response::Response,
};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use std::convert::Infallible;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use tower::Service;

use crate::{models::publication::{self, Entity as Publication}, state::AppState};

#[derive(Clone, Debug)]
pub struct PublicationSlug(pub String);

impl<S: Send + Sync> FromRequestParts<S> for PublicationSlug {
    type Rejection = StatusCode;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        parts
            .extensions
            .get::<PublicationSlug>()
            .cloned()
            .ok_or(StatusCode::NOT_FOUND)
    }
}

pub struct CurrentPublication(pub publication::Model);

impl FromRequestParts<AppState> for CurrentPublication {
    type Rejection = StatusCode;

    async fn from_request_parts(parts: &mut Parts, state: &AppState) -> Result<Self, Self::Rejection> {
        let PublicationSlug(slug) = PublicationSlug::from_request_parts(parts, state).await?;

        Publication::find()
            .filter(publication::Column::Slug.eq(&slug))
            .one(&state.db)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
            .map(CurrentPublication)
            .ok_or(StatusCode::NOT_FOUND)
    }
}

#[derive(Clone)]
pub struct SubdomainRouter {
    base: axum::Router, //      main_domain (show.room.lc)
    app: axum::Router,  //      app.domain
    publication: axum::Router, // {slug}.domain
    domain: String,      // room.lc
    main_domain: String, // show.room.lc
}

enum SubdomainKind {
    Base,
    App,
    Publication(String),
    Redirect, // bare domain → main_domain
}

impl SubdomainRouter {
    pub fn new(
        base: axum::Router,
        app: axum::Router,
        publication: axum::Router,
        domain: impl Into<String>,
        main_domain: impl Into<String>,
    ) -> Self {
        Self { base, app, publication, domain: domain.into(), main_domain: main_domain.into() }
    }

    fn classify(&self, host: &str) -> SubdomainKind {
        let host = host.split(':').next().unwrap_or(host);
        let suffix = format!(".{}", self.domain);

        if host == self.main_domain {
            SubdomainKind::Base
        } else if host == self.domain {
            if self.domain != self.main_domain && cfg!(not(debug_assertions)) {
                SubdomainKind::Redirect
            } else {
                SubdomainKind::Base
            }
        } else if host == format!("app.{}", self.domain) {
            SubdomainKind::App
        } else if let Some(sub) = host.strip_suffix(&suffix) {
            if sub.is_empty() || sub.contains('.') {
                SubdomainKind::Base
            } else {
                SubdomainKind::Publication(sub.to_string())
            }
        } else {
            SubdomainKind::Base
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
            SubdomainKind::Redirect => {
                let path = req.uri().path_and_query().map(|p| p.as_str()).unwrap_or("/");
                let location = format!("https://{}{}", self.main_domain, path);
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
