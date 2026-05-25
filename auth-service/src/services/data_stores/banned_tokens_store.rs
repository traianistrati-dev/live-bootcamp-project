use std::collections::HashSet;

use crate::domain::data_stores::{BannedTokenStore, BannedTokenStoreError};

#[derive(Default)]
pub struct HashsetBannedTokenStore {
    pub banned_tokens: HashSet<String>,
}

#[async_trait::async_trait]
impl BannedTokenStore for HashsetBannedTokenStore {
    async fn add_token(&mut self, token: String) -> Result<(), BannedTokenStoreError> {
        self.banned_tokens.insert(token);
        Ok(())
    }

    async fn contains_token(&self, token: &str) -> Result<bool, BannedTokenStoreError> {
        Ok(self.banned_tokens.contains(token))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_add_banned_token_should_succeed() {
        let mut store = HashsetBannedTokenStore::default();

        assert!(store.add_token(String::from("test_token")).await.is_ok());
    }

    #[tokio::test]
    async fn test_exists_banned_token_should_succeed() {
        let mut store = HashsetBannedTokenStore::default();

        assert!(store.add_token(String::from("test_token")).await.is_ok());

        assert!(store.contains_token("test_token").await.unwrap() == true);

        assert!(store.contains_token("invalid").await.unwrap() == false);
    }
}
