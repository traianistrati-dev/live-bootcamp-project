use super::email::Email;
use super::password::Password;
use super::user::User;

use crate::domain::AuthAPIError;

#[derive(Debug, PartialEq)]
pub enum UserStoreError {
    UserAlreadyExists,
    UserNotFound,
    InvalidCredentials,
    UnexpectedError,
}

#[async_trait::async_trait]
pub trait BannedTokenStore {
    async fn add_banned_token(&mut self, token: String) -> Result<(), AuthAPIError>;
    async fn contains_banned_token(&self, token: String) -> Result<bool, AuthAPIError>;
}

#[async_trait::async_trait]
pub trait UserStore {
    async fn add_user(&mut self, user: User) -> Result<(), UserStoreError>;
    async fn get_user(&self, email: &Email) -> Result<User, UserStoreError>;
    async fn validate_user(&self, email: &Email, password: &Password)
        -> Result<(), UserStoreError>;
}
