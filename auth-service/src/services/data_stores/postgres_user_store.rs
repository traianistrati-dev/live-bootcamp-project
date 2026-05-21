use sqlx::PgPool;

use crate::domain::{
    data_stores::{UserStore, UserStoreError},
    email::Email,
    password::HashedPassword,
    User,
};

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
    // TODO: Implement all required methods.
    // Note that you will need to make SQL queries against our PostgreSQL instance inside these methods.
    // Ensure to parse the password_hash.
    async fn add_user(&mut self, user: User) -> Result<(), UserStoreError> {
        sqlx::query!(
            "INSERT INTO users (email, password_hash, requires_2fa) VALUES ($1, $2, $3)",
            user.email.as_ref(),
            user.password.as_ref(),
            user.requires_2fa,
        )
        .execute(&self.pool)
        .await
        .map_err(|_| UserStoreError::UserAlreadyExists)
        .ok();

        Ok(())
    }

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
            email.as_ref(),
        )
        .fetch_optional(&self.pool)
        .await;

        match user {
            Ok(user) => match user {
                Some(user) =>
                // Ok(User {
                //     email: Email(user.email),
                //     password: HashedPassword::from(user.password_hash),
                //     requires_2fa: user.requires_2fa,
                // }),
                {
                    Ok(User::new(
                        Email::parse(user.email).expect("Valid email"),
                        HashedPassword::parse(user.password_hash)
                            .await
                            .expect("Valid password"),
                        user.requires_2fa,
                    ))
                }
                None => Err(UserStoreError::UserNotFound),
            },
            _ => Err(UserStoreError::UserNotFound),
        }
    }

    async fn validate_user(&self, email: &Email, raw_password: &str) -> Result<(), UserStoreError> {
        let user: User = self.get_user(email).await?;

        user.password // updated password verification
            .verify_raw_password(raw_password)
            .await
            .map_err(|_| UserStoreError::InvalidCredentials)
    }
}
