use std::{
    convert::Infallible,
    task::{Context, Poll},
};

use axum::{RequestPartsExt, body::Body, extract::Request, response::Response};
use axum_extra::{
    TypedHeader,
    headers::{Authorization, authorization::Bearer},
};
use futures_util::future::BoxFuture;
use jsonwebtoken::{Algorithm, Validation};
use tower::{Layer, Service};

use crate::{AppState, models::auth::TokenClaims};

use super::AuthError;

#[derive(Clone)]
pub struct JwtAuthLayer {
    state: AppState,
}

impl JwtAuthLayer {
    pub fn new(state: &AppState) -> Self {
        Self {
            state: state.clone(),
        }
    }
}

impl<S> Layer<S> for JwtAuthLayer {
    type Service = JwtAuthService<S>;

    fn layer(&self, inner: S) -> Self::Service {
        Self::Service {
            inner,
            state: self.state.clone(),
        }
    }
}

#[derive(Clone)]
pub struct JwtAuthService<S> {
    inner: S,
    state: AppState,
}

impl<S, B> Service<Request<B>> for JwtAuthService<S>
where
    S: Service<Request<B>, Response = Response<Body>, Error = Infallible> + Clone + Send + 'static,
    S::Future: Send + 'static,
    B: Send + 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: Request<B>) -> Self::Future {
        let state = self.state.clone();
        let clone = self.inner.clone();

        let mut inner = std::mem::replace(&mut self.inner, clone);

        Box::pin(async move {
            let (mut parts, body) = req.into_parts();

            let TypedHeader(Authorization(bearer)) =
                match parts.extract::<TypedHeader<Authorization<Bearer>>>().await {
                    Ok(token) => token,
                    Err(e) => {
                        tracing::error!("Typed Header Auth error: {:?}", e);
                        return Ok(AuthError::InvalidToken.response());
                    }
                };

            let token_data = match jsonwebtoken::decode::<TokenClaims>(
                bearer.token(),
                &state.jwt.decoding_key,
                &Validation::new(Algorithm::RS256),
            ) {
                Ok(claims) => claims,
                Err(e) => {
                    tracing::error!("JWT Auth error: {e}");
                    return Ok(AuthError::InvalidToken.response());
                }
            };

            let mut req = Request::from_parts(parts, body);
            req.extensions_mut().insert(token_data.claims);
            inner.call(req).await
        })
    }
}
