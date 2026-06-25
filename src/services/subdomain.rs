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
    base: axum::Router, //      no subdomain
    app: axum::Router, //       app.domain
    user: axum::Router, //      {user}.domain
}

impl SubdomainRouter {
    pub fn new(base: axum::Router, app: axum::Router, user: axum::Router) -> Self {
        Self { base, app, user }
    }

    fn parse_subdomain(host: &str) -> Option<&str> {
        let host = host.split(':').next()?;
        if host.matches('.').count() < 2 {
            return None;
        }
        let dot = host.find('.')?;
        let sub = &host[..dot];
        if sub.is_empty() { None } else { Some(sub) }
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

        match Self::parse_subdomain(&host) {
            None => {
                let mut router = self.base.clone();
                Box::pin(async move { router.call(req).await })
            }
            Some("app") => {
                let mut router = self.app.clone();
                Box::pin(async move { router.call(req).await })
            }
            Some(username) => {
                req.extensions_mut()
                    .insert(UsernameSubdomain(username.to_string()));
                let mut router = self.user.clone();
                Box::pin(async move { router.call(req).await })
            }
        }
    }
}
