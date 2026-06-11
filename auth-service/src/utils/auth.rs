use axum_extra::extract::cookie::{Cookie, SameSite};
use chrono::Utc;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Validation};
use secrecy::ExposeSecret;
use serde::{Deserialize, Serialize};

use super::constants::{JWT_COOKIE_NAME, JWT_SECRET};
use crate::domain::email::Email;
use jsonwebtoken::errors::Error;

use color_eyre::eyre::{eyre, Context, ContextCompat, Result};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum GenerateTokenError {
    #[error("Token error")]
    TokenError(#[source] Error),
    #[error("Unexpected error")]
    UnexpectedError,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

// Create cookie with a new JWT auth token
#[tracing::instrument(name = "auth generate cookie", skip_all)]
pub fn generate_auth_cookie(email: &Email) -> Result<Cookie<'static>> {
    let token = generate_auth_token(email)?;
    Ok(create_auth_cookie(token))
}

// Create cookie and set the value to the passed-in token string
#[tracing::instrument(name = "auth create cookie", skip_all)]
fn create_auth_cookie(token: String) -> Cookie<'static> {
    let cookie = Cookie::build((JWT_COOKIE_NAME, token))
        .path("/") // apply cookie to all URLs on the server
        .http_only(true) // prevent JavaScript from accessing the cookie
        .same_site(SameSite::Lax) // send cookie with "same-site" requests, and with "cross-site" top-level navigations.
        .build();

    cookie
}

// This value determines how long the JWT auth token is valid for
pub const TOKEN_TTL_SECONDS: i64 = 600; // 10 minutes

// Create JWT auth token
#[tracing::instrument(name = "auth generate token", skip_all)]
fn generate_auth_token(email: &Email) -> color_eyre::eyre::Result<String> {
    let delta = chrono::Duration::try_seconds(TOKEN_TTL_SECONDS)
        .wrap_err("failed to create 10 minute time delta")?;
    // .ok_or(GenerateTokenError::UnexpectedError)?;
    // Create JWT expiration time
    let exp = Utc::now()
        .checked_add_signed(delta)
        .ok_or(eyre!("failed to create JWT expiration time"))?
        .timestamp();

    // Cast exp to a usize, which is what Claims expects
    let exp: usize = exp.try_into().wrap_err("failed to cast exp to a usize")?;

    let sub = email.as_ref().to_owned().expose_secret().to_string();

    let claims = Claims { sub, exp };

    create_token(&claims)

    // .map_err(GenerateTokenError::TokenError)
}

// Check if JWT auth token is valid by decoding it using the JWT secret
#[tracing::instrument(name = "auth validate token", skip_all)]
pub async fn validate_token(
    token: &str,
    banned_tokens_store: crate::app_state::BannedTokenStoreType,
) -> Result<Claims> {
    let banned_token = banned_tokens_store.read().await.contains_token(token).await;

    //    println!("\x1b[32m contains banned_token {:?} \x1b[0m", banned_token);

    match banned_token {
        Ok(value) => {
            if value {
                return Err(eyre!("token is banned"));
            }
        }
        Err(e) => {
            return Err(e.into());
        }
    }

    decode::<Claims>(
        token,
        &DecodingKey::from_secret(JWT_SECRET.as_bytes()),
        &Validation::default(),
    )
    .map(|data| data.claims)
    .wrap_err("failed to decode token")
}

// Create JWT auth token by encoding claims using the JWT secret
#[tracing::instrument(name = "auth create token", skip_all)]
fn create_token(claims: &Claims) -> Result<String> {
    encode(
        &jsonwebtoken::Header::default(),
        &claims,
        &EncodingKey::from_secret(JWT_SECRET.as_bytes()),
    )
    .wrap_err("failed to create token")
}

#[cfg(test)]
mod tests {

    use super::*;
    use secrecy::SecretString;

    #[tokio::test]
    async fn test_generate_auth_cookie() {
        let email = Email::parse(SecretString::new(
            "test@example.com".to_owned().into_boxed_str(),
        ))
        .unwrap();
        let cookie = generate_auth_cookie(&email).unwrap();
        assert_eq!(cookie.name(), JWT_COOKIE_NAME);
        assert_eq!(cookie.value().split('.').count(), 3);
        assert_eq!(cookie.path(), Some("/"));
        assert_eq!(cookie.http_only(), Some(true));
        assert_eq!(cookie.same_site(), Some(SameSite::Lax));
    }

    #[tokio::test]
    async fn test_create_auth_cookie() {
        let token = "test_token".to_owned();
        let cookie = create_auth_cookie(token.clone());
        assert_eq!(cookie.name(), JWT_COOKIE_NAME);
        assert_eq!(cookie.value(), token);
        assert_eq!(cookie.path(), Some("/"));
        assert_eq!(cookie.http_only(), Some(true));
        assert_eq!(cookie.same_site(), Some(SameSite::Lax));
    }

    #[tokio::test]
    async fn test_generate_auth_token() {
        let email = Email::parse(SecretString::new(
            "test@example.com".to_owned().into_boxed_str(),
        ))
        .unwrap();
        let result = generate_auth_token(&email).unwrap();
        assert_eq!(result.split('.').count(), 3);
    }

    #[tokio::test]
    async fn test_validate_token_with_valid_token() {
        let email = Email::parse(SecretString::new(
            "test@example.com".to_owned().into_boxed_str(),
        ))
        .unwrap();
        let token = generate_auth_token(&email).unwrap();
        let banned_token_store = std::sync::Arc::new(tokio::sync::RwLock::new(
            crate::services::data_stores::banned_tokens_store::HashsetBannedTokenStore::default(),
        ));
        let result = validate_token(&token, banned_token_store)
            .await
            .expect("Claims and not banned_token is Store");

        assert_eq!(result.sub, "test@example.com");

        let exp = Utc::now()
            .checked_add_signed(chrono::Duration::try_minutes(9).expect("valid duration"))
            .expect("valid timestamp")
            .timestamp();

        assert!(result.exp > exp as usize);
    }

    #[tokio::test]
    async fn test_validate_token_with_invalid_token() {
        let token = "invalid_token".to_owned();
        let banned_token_store = std::sync::Arc::new(tokio::sync::RwLock::new(
            crate::services::data_stores::banned_tokens_store::HashsetBannedTokenStore::default(),
        ));
        let result = validate_token(&token, banned_token_store).await;
        assert!(result.is_err());
    }
}
