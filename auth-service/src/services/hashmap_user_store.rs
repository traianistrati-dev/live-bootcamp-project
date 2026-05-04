use crate::domain::{data_stores::UserStore, data_stores::UserStoreError, user::User};
use std::collections::HashMap;

#[derive(Default)]
pub struct HashmapUserStore {
    pub users: HashMap<String, User>,
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

    async fn get_user(&self, email: &str) -> Result<User, UserStoreError> {
        if let Some(user) = self.users.get(email) {
            Ok(user.clone())
        } else {
            Err(UserStoreError::UserNotFound)
        }
    }

    async fn validate_user(&self, email: &str, password: &str) -> Result<(), UserStoreError> {
        let user = self.get_user(email).await?;
        if user.password == password {
            Ok(())
        } else {
            Err(UserStoreError::InvalidCredentials)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_add_user() {
        let mut store = HashmapUserStore::default();

        let user = store
            .add_user(User {
                email: "test@example.com".into(),
                password: "password".into(),
                requires_2fa: false,
            })
            .await;
        assert!(user.is_ok());
    }

    #[tokio::test]
    async fn test_get_user() {
        let mut store = HashmapUserStore::default();
        let user = User {
            email: "test@example.com".into(),
            password: "password".into(),
            requires_2fa: false,
        };
        store.add_user(user).await.unwrap();

        let retrieved_user = store.get_user("test@example.com").await.unwrap();
        assert_eq!(retrieved_user.email, "test@example.com");
    }

    #[tokio::test]
    async fn test_validate_user() {
        let mut store = HashmapUserStore::default();
        let user = User {
            email: "test@example.com".into(),
            password: "password".into(),
            requires_2fa: false,
        };
        store.add_user(user).await.unwrap();

        let is_user_valid_result = store.validate_user("test@example.com", "password").await;
        assert!(is_user_valid_result.is_ok());
    }
}
