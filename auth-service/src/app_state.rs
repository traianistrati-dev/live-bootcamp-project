use std::sync::Arc;
use tokio::sync::RwLock;

use crate::domain::data_stores::{BannedTokenStore, TwoFACodeStore, UserStore};

pub type TwoFACodeStoreType = Arc<RwLock<dyn TwoFACodeStore + Send + Sync>>;
pub type UserStoreType = Arc<RwLock<dyn UserStore + Send + Sync>>;
pub type BannedTokenStoreType = Arc<RwLock<dyn BannedTokenStore + Send + Sync>>;
pub type EmailClientType = Arc<RwLock<dyn crate::domain::email_client::EmailClient + Send + Sync>>;

#[derive(Clone)]
pub struct AppState {
    pub user_store: UserStoreType,
    pub banned_tokens_store: BannedTokenStoreType,
    pub two_fa_code_store: TwoFACodeStoreType,
    pub email_client: EmailClientType,
}

impl AppState {
    pub fn new(
        user_store: UserStoreType,
        banned_tokens_store: BannedTokenStoreType,
        two_fa_code_store: TwoFACodeStoreType,
        email_client: EmailClientType,
    ) -> Self {
        Self {
            user_store,
            banned_tokens_store,
            two_fa_code_store,
            email_client,
        }
    }
}
