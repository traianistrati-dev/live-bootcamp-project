use std::collections::HashSet;

// use crate::app_state::AppState;

use crate::domain::{data_stores::BannedTokenStore, AuthAPIError};
use crate::utils::auth::validate_token;
//use axum::http::StatusCode;
//use axum::response::IntoResponse;

#[derive(Default)]
struct HashsetBannedTokenStore {
    pub banned_tokens: HashSet<String>,
}

#[async_trait::async_trait]
impl BannedTokenStore for HashsetBannedTokenStore {
    async fn add_banned_token(
        //  State(state): State<AppState>,
        &mut self,
        token: String,
    ) -> Result<(), AuthAPIError> {
        if self.banned_tokens.insert(token) {
            return Ok(());
        }
        Err(AuthAPIError::UnexpectedError)

        // match validate_token(&token).await {
        //     Ok(_) => match self.banned_tokens.insert(token) {
        //         true => Ok(()),
        //         _ => Err(AuthAPIError::UnexpectedError),
        //     },
        //     Err(_) => Err(AuthAPIError::InvalidToken),
        // }
    }

    async fn contains_banned_token(&self, token: String) -> bool {
        self.banned_tokens.contains(&token)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_add_banned_token_should_succeed() {
        let mut store = HashsetBannedTokenStore::default();

        //let res = store.add_banned_token(_get_valid_token()).await;
        assert!(store
            .add_banned_token(String::from("test_token"))
            .await
            .is_ok());
    }

    #[tokio::test]
    async fn test_exists_banned_token_should_succeed() {
        let mut store = HashsetBannedTokenStore::default();

        assert!(store
            .add_banned_token(String::from("test_token"))
            .await
            .is_ok());

        assert_eq!(
            store
                .contains_banned_token(String::from("test_token"))
                .await,
            true
        );

        assert_eq!(
            store.contains_banned_token(String::from("invalid")).await,
            false
        );
    }
}
