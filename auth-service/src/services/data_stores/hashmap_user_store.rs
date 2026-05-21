use crate::domain::{
    data_stores::UserStore, data_stores::UserStoreError, email::Email, password::HashedPassword,
    user::User,
};
use std::collections::HashMap;

#[derive(Default)]
pub struct HashmapUserStore {
    pub users: HashMap<Email, User>,
}

#[async_trait::async_trait]
impl UserStore for HashmapUserStore {
    async fn add_user(&mut self, user: User) -> Result<(), UserStoreError> {
        if self.users.contains_key(&user.email) {
            return Err(UserStoreError::UserAlreadyExists);
        } else {
            self.users.insert(user.email.clone(), user);
            return Ok(());
        }
    }

    async fn get_user(&self, email: &Email) -> Result<User, UserStoreError> {
        if let Some(user) = self.users.get(email) {
            Ok(user.clone())
        } else {
            Err(UserStoreError::UserNotFound)
        }
    }

    async fn validate_user(&self, email: &Email, raw_password: &str) -> Result<(), UserStoreError> {
        //   let user = self.get_user(email).await?;
        let user: &User = self.users.get(email).ok_or(UserStoreError::UserNotFound)?;

        user.password // updated password verification
            .verify_raw_password(raw_password)
            .await
            .map_err(|_| UserStoreError::InvalidCredentials)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn helper_build_user_email(email: &str) -> Email {
        Email::parse(email.into()).expect("expects a valid@example.email ")
    }

    async fn helper_build_user_password(pass: &str) -> HashedPassword {
        HashedPassword::parse(pass.into())
            .await
            .expect("expects a valid password minimum 8 chars long")
    }

    #[tokio::test]
    async fn test_add_user() {
        let mut store = HashmapUserStore::default();
        let user = store
            .add_user(User {
                email: helper_build_user_email("test@example.com"),
                password: helper_build_user_password("12345678").await,
                requires_2fa: false,
            })
            .await;
        assert!(user.is_ok());
    }

    #[tokio::test]
    async fn test_get_user() {
        let mut store = HashmapUserStore::default();
        let user = User {
            email: helper_build_user_email("test@example.com"),
            password: helper_build_user_password("password").await,
            requires_2fa: false,
        };
        store.add_user(user).await.unwrap();

        let retrieved_user = store
            .get_user(&helper_build_user_email("test@example.com"))
            .await
            .unwrap();
        assert_eq!(
            retrieved_user.email,
            helper_build_user_email("test@example.com")
        );
    }

    #[tokio::test]
    async fn test_validate_user() {
        let mut store = HashmapUserStore::default();
        let email = helper_build_user_email("test@example.com");

        let user = User {
            email: email.clone(),
            password: helper_build_user_password("password").await,
            requires_2fa: false,
        };
        store.add_user(user).await.unwrap();

        let is_user_valid_result = store.validate_user(&email, "password").await;
        assert!(is_user_valid_result.is_ok());
    }
}
