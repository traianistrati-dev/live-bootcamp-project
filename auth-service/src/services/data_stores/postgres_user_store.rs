use sqlx::PgPool;

use crate::domain::{
    data_stores::{UserStore, UserStoreError},
    email::Email,
    password::HashedPassword,
    User,
};

use color_eyre::eyre::{eyre, Result};

use secrecy::{ExposeSecret, SecretString};

pub struct PostgresUserStore {
    pool: PgPool,
}

impl PostgresUserStore {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl UserStore for PostgresUserStore {
    #[tracing::instrument(name = "Adding user to PostgreSQL", skip_all)]
    async fn add_user(&mut self, user: User) -> Result<(), UserStoreError> {
        sqlx::query!(
            "INSERT INTO users (email, password_hash, requires_2fa) VALUES ($1, $2, $3)",
            user.email.as_ref().expose_secret(),
            user.password.as_ref().expose_secret(),
            user.requires_2fa,
        )
        .execute(&self.pool)
        .await
        .map_err(|e| UserStoreError::UnexpectedError(e.into()))
        .ok();

        Ok(())
    }

    #[tracing::instrument(name = "Getting user from PostgreSQL", skip_all)]
    async fn get_user(&self, email: &Email) -> Result<User, UserStoreError> {
        #[derive(Debug, Clone, sqlx::FromRow, Default)]
        struct UserSql {
            email: String,
            password_hash: String,
            requires_2fa: bool,
        }

        let user = sqlx::query_as!(
            UserSql,
            "SELECT email, password_hash, requires_2fa FROM users WHERE email = $1",
            email.as_ref().expose_secret(),
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|_| UserStoreError::UserNotFound)?
        .ok_or(UserStoreError::UserNotFound)?;

        Ok(User::new(
            // Email::parse(user.email).expect("Valid email"),
            Email::parse(SecretString::new(user.email.into_boxed_str()))
                .map_err(|e| UserStoreError::UnexpectedError(eyre!(e)))?,
            HashedPassword::parse_password_hash(SecretString::new(
                user.password_hash.into_boxed_str(),
            ))
            .map_err(|e| UserStoreError::UnexpectedError(eyre!(e)))?,
            user.requires_2fa,
        ))
    }

    #[tracing::instrument(name = "Validating user credentials in PostgreSQL", skip_all)]
    async fn validate_user(
        &self,
        email: &Email,
        raw_password: &SecretString,
    ) -> Result<(), UserStoreError> {
        let user: User = self.get_user(email).await?;

        user.password // updated password verification
            .verify_raw_password(raw_password)
            .await
            .map_err(|_| UserStoreError::InvalidCredentials)
    }
}
