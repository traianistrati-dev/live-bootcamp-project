// mod services;
// mod app_state;

use auth_service::app_state::AppState;
use auth_service::services::banned_tokens_store::HashsetBannedTokenStore;
use auth_service::services::hashmap_two_fa_code_store::HashmapTwoFACodeStore;
use auth_service::services::hashmap_user_store::HashmapUserStore;
use auth_service::services::mock_email_client::MockEmailClient;
use auth_service::utils;

#[tokio::main]
async fn main() {
    let user_store = std::sync::Arc::new(tokio::sync::RwLock::new(HashmapUserStore::default()));
    let banned_tokens_store =
        std::sync::Arc::new(tokio::sync::RwLock::new(HashsetBannedTokenStore::default()));
    let two_fa_code_store =
        std::sync::Arc::new(tokio::sync::RwLock::new(HashmapTwoFACodeStore::default()));

    let email_client = std::sync::Arc::new(tokio::sync::RwLock::new(MockEmailClient::default()));

    let pg_pool = configure_postgresql().await;

    let app_state = AppState::new(
        user_store,
        banned_tokens_store,
        two_fa_code_store,
        email_client,
    );

    let app = auth_service::Application::build(app_state, utils::constants::prod::APP_ADDRESS)
        .await
        .expect("Failed to build app");

    app.run().await.expect("Failed to run app");
}

pub async fn configure_postgresql() -> sqlx::postgres::PgPool {
    // Create a new database connection pool
    let pg_pool = auth_service::get_postgres_pool(&utils::constants::prod::DATABASE_URL)
        .await
        .expect("Failed to create Postgres connection pool!");

    // Run database migrations against our test database!
    sqlx::migrate!()
        .run(&pg_pool)
        .await
        .expect("Failed to run migrations");

    pg_pool
}
