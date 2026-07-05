use axum::{
    body::Body,
    extract::FromRequestParts,
    http::{StatusCode, Request, request::Parts},
    response::Response,
};
use std::convert::Infallible;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use tower::Service;

#[derive(Clone, Debug)]
pub struct UsernameSubdomain(pub String);

impl<S: Send + Sync> FromRequestParts<S> for UsernameSubdomain {
    type Rejection = StatusCode;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        parts
            .extensions
            .get::<UsernameSubdomain>()
            .cloned()
            .ok_or(StatusCode::NOT_FOUND)
    }
}

#[derive(Clone)]
pub struct SubdomainRouter {
    base: axum::Router, //      no subdomain  (domain)
    app: axum::Router,  //      app.domain
    user: axum::Router, //      {handle}.domain
    domain: String,
}

enum SubdomainKind {
    Base,
    App,
    User(String),
}

impl SubdomainRouter {
    pub fn new(base: axum::Router, app: axum::Router, user: axum::Router, domain: impl Into<String>) -> Self {
        Self { base, app, user, domain: domain.into() }
    }

    fn classify(&self, host: &str) -> SubdomainKind {
        let host = host.split(':').next().unwrap_or(host);
        let suffix = format!(".{}", self.domain);

        if host == self.domain {
            SubdomainKind::Base
        } else if host == format!("app.{}", self.domain) {
            SubdomainKind::App
        } else if let Some(sub) = host.strip_suffix(&suffix) {
            if sub.is_empty() || sub.contains('.') {
                SubdomainKind::Base
            } else {
                SubdomainKind::User(sub.to_string())
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
            SubdomainKind::User(username) => {
                req.extensions_mut().insert(UsernameSubdomain(username));
                let mut router = self.user.clone();
                Box::pin(async move { router.call(req).await })
            }
        }
    }
}
