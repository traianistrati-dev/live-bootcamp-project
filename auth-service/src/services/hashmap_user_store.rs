use std::collections::HashMap;


use crate::domain::user::User;

#[derive(Debug, PartialEq)]
pub enum UserStoreError {
    UserAlreadyExists,
    UserNotFound,
    InvalidCredentials,
    UnexpectedError,
}

// TODO: Create a new struct called `HashmapUserStore` containing a `users` field
// which stores a `HashMap`` of email `String`s mapped to `User` objects.
// Derive the `Default` trait for `HashmapUserStore`.

#[derive(Default)]
pub struct HashmapUserStore {
    pub users: HashMap<String, User>,
}

impl HashmapUserStore {
    pub fn add_user(&mut self, user: User) -> Result<(), UserStoreError> {
        if self.users.contains_key(&user.email) {
            return Err(UserStoreError::UserAlreadyExists);
        } else {
            self.users.insert(user.email.clone(), user);
            return Ok(());
        }
        // Return `UserStoreError::UserAlreadyExists` if the user already exists,
        // otherwise insert the user into the hashmap and return `Ok(())`.
        // todo!()
    }

    // TODO: Implement a public method called `get_user`, which takes an
    // immutable reference to self and an email string slice as arguments.
    // This function should return a `Result` type containing either a
    // `User` object or a `UserStoreError`.
    // Return `UserStoreError::UserNotFound` if the user can not be found.
    pub fn get_user(&self, email: &str) -> Result<User, UserStoreError> {
        if let Some(user) = self.users.get(email) {
            Ok(user.clone())
        } else {
            Err(UserStoreError::UserNotFound)
        }
    }

    // TODO: Implement a public method called `validate_user`, which takes an
    // immutable reference to self, an email string slice, and a password string slice
    // as arguments. `validate_user` should return a `Result` type containing either a
    // unit type `()` if the email/password passed in match an existing user, or a `UserStoreError`.
    // Return `UserStoreError::UserNotFound` if the user can not be found.
    // Return `UserStoreError::InvalidCredentials` if the password is incorrect.
    pub fn validate_user(&self, email: &str, password: &str) -> Result<(), UserStoreError> {
        let user = self.get_user(email)?;
        if user.password == password {
            Ok(())
        } else {
            Err(UserStoreError::InvalidCredentials)
        }
    }
}

// TODO: Add unit tests for your `HashmapUserStore` implementation
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_add_user() {
        // todo!()
        let mut store = HashmapUserStore::default();

        let user = store
            .add_user(User {
                email: "test@example.com".into(),
                password: "password".into(),
                requires_2fa: false,
            });
        assert!(user.is_ok());
    }

    #[tokio::test]
    async fn test_get_user() {
        //todo!()
        let mut store = HashmapUserStore::default();
        let user = User {
            email: "test@example.com".into(),
            password: "password".into(),
            requires_2fa: false,
        };
        store.add_user(user).unwrap();

        let retrieved_user = store.get_user("test@example.com").unwrap();
        assert_eq!(retrieved_user.email, "test@example.com");
    }

    #[tokio::test]
    async fn test_validate_user() {
        //todo!()
        let mut store = HashmapUserStore::default();
        let user = User {
            email: "test@example.com".into(),
            password: "password".into(),
            requires_2fa: false,
        };
        store.add_user(user).unwrap();

        let is_valid = store.validate_user("test@example.com", "password");
        assert!(is_valid.is_ok());
    }
}
