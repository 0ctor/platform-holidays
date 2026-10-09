use crate::error::ApiError;
use actix_web::{http::header, HttpRequest};
use reqwest::StatusCode;
use serde::Deserialize;
use sha2::{Digest, Sha256};
use std::{
    collections::HashMap,
    sync::Arc,
    time::{Duration, Instant},
};
use tokio::sync::RwLock;

#[derive(Clone)]
pub struct Authenticator {
    client: reqwest::Client,
    introspect_url: String,
    cache: Arc<RwLock<HashMap<String, CachedAuth>>>,
    cache_ttl: Duration,
}

#[derive(Clone)]
struct CachedAuth {
    inserted: Instant,
    user: AuthUser,
}

#[derive(Debug, Deserialize)]
struct IntrospectionResponse {
    status: bool,
    #[serde(default)]
    data: Option<IntrospectionData>,
}

#[derive(Debug, Deserialize, Default, Clone)]
struct IntrospectionData {
    #[serde(default)]
    user_uuid: Option<String>,
    #[serde(default)]
    company_uuid: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct AuthUser {
    pub user_uuid: Option<String>,
    pub company_uuid: Option<String>,
}

impl Authenticator {
    pub fn new(introspect_url: String, timeout: Duration) -> Result<Self, reqwest::Error> {
        Ok(Self {
            client: reqwest::Client::builder().timeout(timeout).build()?,
            introspect_url,
            cache: Arc::new(RwLock::new(HashMap::new())),
            cache_ttl: Duration::from_secs(30),
        })
    }

    pub async fn authenticate(&self, request: &HttpRequest) -> Result<AuthUser, ApiError> {
        let token = bearer_token(request).ok_or(ApiError::Unauthorized)?;
        let cache_key = token_hash(token);

        if let Some(user) = self.cached_user(&cache_key).await {
            return Ok(user);
        }

        let response = self
            .client
            .get(&self.introspect_url)
            .bearer_auth(token)
            .send()
            .await
            .map_err(|error| {
                tracing::warn!(error = %error, "falha ao consultar autenticação");
                ApiError::AuthUnavailable
            })?;

        if response.status() == StatusCode::UNAUTHORIZED
            || response.status() == StatusCode::FORBIDDEN
        {
            return Err(ApiError::Unauthorized);
        }
        if !response.status().is_success() {
            return Err(ApiError::AuthUnavailable);
        }

        let body: IntrospectionResponse = response
            .json()
            .await
            .map_err(|_| ApiError::AuthUnavailable)?;
        if !body.status {
            return Err(ApiError::Unauthorized);
        }

        let data = body.data.unwrap_or_default();
        let user = AuthUser {
            user_uuid: data.user_uuid,
            company_uuid: data.company_uuid,
        };

        let mut cache = self.cache.write().await;
        cache.retain(|_, entry| entry.inserted.elapsed() < self.cache_ttl);
        cache.insert(
            cache_key,
            CachedAuth {
                inserted: Instant::now(),
                user: user.clone(),
            },
        );
        Ok(user)
    }

    async fn cached_user(&self, key: &str) -> Option<AuthUser> {
        let cache = self.cache.read().await;
        cache.get(key).and_then(|entry| {
            if entry.inserted.elapsed() < self.cache_ttl {
                Some(entry.user.clone())
            } else {
                None
            }
        })
    }
}

fn bearer_token(request: &HttpRequest) -> Option<&str> {
    request
        .headers()
        .get(header::AUTHORIZATION)?
        .to_str()
        .ok()?
        .strip_prefix("Bearer ")
        .map(str::trim)
        .filter(|token| !token.is_empty())
}

fn token_hash(token: &str) -> String {
    hex::encode(Sha256::digest(token.as_bytes()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::test::TestRequest;

    #[test]
    fn extrai_bearer() {
        let request = TestRequest::default()
            .insert_header((header::AUTHORIZATION, "Bearer segredo"))
            .to_http_request();
        assert_eq!(bearer_token(&request), Some("segredo"));
        assert_ne!(token_hash("segredo"), "segredo");
    }

    #[test]
    fn rejeita_esquema_incorreto() {
        let request = TestRequest::default()
            .insert_header((header::AUTHORIZATION, "Basic abc"))
            .to_http_request();
        assert_eq!(bearer_token(&request), None);
    }
}
