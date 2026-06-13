// mod services;
// mod app_state;

use auth_service::app_state::AppState;
// use auth_service::services::data_stores::banned_tokens_store::HashsetBannedTokenStore;
// use auth_service::services::data_stores::hashmap_user_store::HashmapUserStore;
// use auth_service::services::data_stores::hashmap_two_fa_code_store::HashmapTwoFACodeStore;
use auth_service::services::data_stores::postgres_user_store::PostgresUserStore;
use auth_service::services::data_stores::redis_two_fa_code_store::RedisTwoFACodeStore;

use auth_service::services::data_stores::redis_banned_token_store::RedisBannedTokenStore;
//use auth_service::services::mock_email_client::MockEmailClient;
use auth_service::utils;
use auth_service::utils::tracing::init_tracing;

#[tokio::main]
async fn main() {
    color_eyre::install().expect("Failed to install color_eyre");
    init_tracing().expect("Failed to initialize tracing");
    //let user_store = std::sync::Arc::new(tokio::sync::RwLock::new(HashmapUserStore::default()));
    //let banned_tokens_store =std::sync::Arc::new(tokio::sync::RwLock::new(HashsetBannedTokenStore::default()));
    //let two_fa_code_store = std::sync::Arc::new(tokio::sync::RwLock::new(HashmapTwoFACodeStore::default()));
    //
    let redis_conn = std::sync::Arc::new(tokio::sync::RwLock::new(configure_redis()));
    let banned_tokens_store = std::sync::Arc::new(tokio::sync::RwLock::new(
        RedisBannedTokenStore::new(redis_conn.clone()),
    ));
    let two_fa_code_store = std::sync::Arc::new(tokio::sync::RwLock::new(
        RedisTwoFACodeStore::new(redis_conn),
    ));

    // let email_client = std::sync::Arc::new(tokio::sync::RwLock::new(MockEmailClient));
    let email_client =
        std::sync::Arc::new(tokio::sync::RwLock::new(configure_postmark_email_client())); // Updated!

    let pg_pool = configure_postgresql().await;
    let user_store = std::sync::Arc::new(tokio::sync::RwLock::new(PostgresUserStore::new(pg_pool)));

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
    let pg_pool = auth_service::get_postgres_pool(&utils::constants::DATABASE_URL)
        .await
        .expect("Failed to create Postgres connection pool!");

    // Run database migrations against our test database!
    sqlx::migrate!()
        .run(&pg_pool)
        .await
        .expect("Failed to run migrations");

    pg_pool
}

fn configure_redis() -> redis::Connection {
    auth_service::get_redis_client(utils::constants::REDIS_HOST_NAME.to_owned())
        .expect("Failed to get Redis client")
        .get_connection()
        .expect("Failed to get Redis connection")
}
use auth_service::services::postmark_email_client::PostmarkEmailClient;
// New!
fn configure_postmark_email_client() -> PostmarkEmailClient {
    use utils::constants::prod::email_client;

    let http_client = reqwest::Client::builder()
        .timeout(email_client::TIMEOUT)
        .build()
        .expect("Failed to build HTTP client");

    PostmarkEmailClient::new(
        email_client::BASE_URL.to_owned(),
        auth_service::domain::email::Email::parse(secrecy::SecretString::new(
            email_client::SENDER.to_owned().into_boxed_str(),
        ))
        .unwrap(),
        utils::constants::POSTMARK_AUTH_TOKEN.to_owned(),
        http_client,
    )
}
