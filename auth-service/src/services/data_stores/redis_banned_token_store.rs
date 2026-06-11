use std::sync::Arc;

use redis::{Commands, Connection};
use tokio::sync::RwLock;

use crate::{
    domain::data_stores::{BannedTokenStore, BannedTokenStoreError},
    utils::auth::TOKEN_TTL_SECONDS,
};

use color_eyre::eyre::{Result, WrapErr};

pub struct RedisBannedTokenStore {
    conn: Arc<RwLock<Connection>>,
}

impl RedisBannedTokenStore {
    pub fn new(conn: Arc<RwLock<Connection>>) -> Self {
        Self { conn }
    }
}

#[async_trait::async_trait]
impl BannedTokenStore for RedisBannedTokenStore {
    #[tracing::instrument(name = "auth banned token store add token", skip_all)]
    async fn add_token(&mut self, token: String) -> Result<(), BannedTokenStoreError> {
        let key: String = get_key(token.as_str());

        let ttl: u64 = TOKEN_TTL_SECONDS
            .try_into()
            .wrap_err("failed to cast TOKEN_TTL_SECONDS to u64") // New! use color_eyre::eyre::WrapErr
            .map_err(BannedTokenStoreError::UnexpectedError)?; // Updated!

        self.conn
            .write()
            .await
            .set_ex(&key, true, ttl)
            .map_err(|e| BannedTokenStoreError::UnexpectedError(e.into()))
    }

    #[tracing::instrument(name = "auth banned token store contains token", skip_all)]
    async fn contains_token(&self, token: &str) -> Result<bool, BannedTokenStoreError> {
        // Check if the token exists by calling the exists method on the Redis connection
        //todo!()
        let key: String = get_key(token);
        let mut connection = self.conn.write().await;
        let exists = connection
            .exists(&key)
            .map_err(|e| BannedTokenStoreError::UnexpectedError(e.into()))?;
        Ok(exists)
    }
}

// We are using a key prefix to prevent collisions and organize data!
const BANNED_TOKEN_KEY_PREFIX: &str = "banned_token:";

fn get_key(token: &str) -> String {
    format!("{}{}", BANNED_TOKEN_KEY_PREFIX, token)
}
