use std::env;

use axum::{body::Body, http::StatusCode, middleware::Next, response::Response};
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct Claims {
    pub sub: String,
    pub role: Option<String>,
    pub exp: usize,
}

pub async fn jwt_middleware(
    req: axum::http::Request<Body>,
    next: Next,
) -> Result<Response, StatusCode> {
    let Some(header_val) = req.headers().get(axum::http::header::AUTHORIZATION) else {
        return Err(StatusCode::UNAUTHORIZED);
    };
    let Ok(header_str) = header_val.to_str() else {
        return Err(StatusCode::UNAUTHORIZED);
    };
    let token = header_str.strip_prefix("Bearer ").unwrap_or("");
    if token.is_empty() {
        return Err(StatusCode::UNAUTHORIZED);
    }

    let secret = env::var("JWT_SECRET").unwrap_or_default();
    if secret.is_empty() {
        return Err(StatusCode::UNAUTHORIZED);
    }

    decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::new(jsonwebtoken::Algorithm::HS256),
    )
    .map_err(|_| StatusCode::UNAUTHORIZED)?;

    Ok(next.run(req).await)
}
