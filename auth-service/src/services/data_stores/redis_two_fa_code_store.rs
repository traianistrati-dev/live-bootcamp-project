use std::sync::Arc;

use redis::{Commands, Connection};
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;

use crate::domain::{
    data_stores::{LoginAttemptId, TwoFACode, TwoFACodeStore, TwoFACodeStoreError},
    email::Email,
};

pub struct RedisTwoFACodeStore {
    conn: Arc<RwLock<Connection>>,
}

impl RedisTwoFACodeStore {
    pub fn new(conn: std::sync::Arc<tokio::sync::RwLock<redis::Connection>>) -> Self {
        Self { conn }
    }
}

#[async_trait::async_trait]
impl TwoFACodeStore for RedisTwoFACodeStore {
    async fn add_code(
        &mut self,
        email: Email,
        login_attempt_id: LoginAttemptId,
        code: TwoFACode,
    ) -> Result<(), TwoFACodeStoreError> {
        // TODO:
        // 1. Create a new key using the get_key helper function.
        // 2. Create a TwoFATuple instance.
        // 3. Use serde_json::to_string to serialize the TwoFATuple instance into a JSON string.
        // Return TwoFACodeStoreError::UnexpectedError if serialization fails.
        // 4. Call the set_ex command on the Redis connection to set a new key/value pair with an expiration time (TTL).
        // The value should be the serialized 2FA tuple.
        // The expiration time should be set to TEN_MINUTES_IN_SECONDS.
        // Return TwoFACodeStoreError::UnexpectedError if casting fails or the call to set_ex fails.

        // todo!()
        let key = get_key(&email);

        let two_fa_tuple = TwoFATuple(
            login_attempt_id.as_ref().to_owned(),
            code.as_ref().to_owned(),
        );
        let serialized = serde_json::to_string(&two_fa_tuple)
            .map_err(|e| TwoFACodeStoreError::UnexpectedError(e.into()))?;

        let res: () = self
            .conn
            .write()
            .await
            .set_ex(&key, &serialized, TEN_MINUTES_IN_SECONDS)
            .map_err(|e| TwoFACodeStoreError::UnexpectedError(e.into()))?;

        /*
        hint: in edition 2024, the requirement `!: FromRedisValue` will fail
        hint: use `()` annotations to avoid fallback changes: `::<_, _, ()>`
         */

        Ok(res)
    }

    async fn remove_code(&mut self, email: &Email) -> Result<(), TwoFACodeStoreError> {
        // TODO:
        // 1. Create a new key using the get_key helper function.
        // 2. Call the del command on the Redis connection to delete the 2FA code entry.
        // Return TwoFACodeStoreError::UnexpectedError if the operation fails.

        // todo!()

        let key = get_key(&email);
        let res: () = self
            .conn
            .write()
            .await
            .del(&key)
            .map_err(|e| TwoFACodeStoreError::UnexpectedError(e.into()))?;

        Ok(res)
    }

    async fn get_code(
        &self,
        email: &Email,
    ) -> Result<(LoginAttemptId, TwoFACode), TwoFACodeStoreError> {
        // TODO:
        //+ 1. Create a new key using the get_key helper function.
        // 2. Call the get command on the Redis connection to get the value stored for the key.
        // Return TwoFACodeStoreError::LoginAttemptIdNotFound if the operation fails.
        // If the operation succeeds, call serde_json::from_str to parse the JSON string into a TwoFATuple.
        // Then, parse the login attempt ID string and 2FA code string into a LoginAttemptId and TwoFACode type respectively.
        // Return TwoFACodeStoreError::UnexpectedError if parsing fails.

        //todo!()
        let key = get_key(&email);

        let mut connection = self.conn.write().await;
        let res: Option<String> = connection
            .get(key)
            .map_err(|_| TwoFACodeStoreError::LoginAttemptIdNotFound)?;

        let res: String = res.ok_or(TwoFACodeStoreError::LoginAttemptIdNotFound)?;
        let (login_attempt_id, code) = serde_json::from_str(res.as_str())
            .map_err(|e| TwoFACodeStoreError::UnexpectedError(e.into()))?;

        let login_attempt_id = LoginAttemptId::parse(login_attempt_id)
            .map_err(|e| TwoFACodeStoreError::UnexpectedError(e.into()))?;

        let code =
            TwoFACode::parse(code).map_err(|e| TwoFACodeStoreError::UnexpectedError(e.into()))?;

        Ok((login_attempt_id, code))
    }
}

#[derive(Serialize, Deserialize)]
struct TwoFATuple(pub String, pub String);

const TEN_MINUTES_IN_SECONDS: u64 = 600;
const TWO_FA_CODE_PREFIX: &str = "two_fa_code:";

fn get_key(email: &Email) -> String {
    format!("{}{}", TWO_FA_CODE_PREFIX, email.as_ref())
}
