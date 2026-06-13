use auth_service::app_state::AppState;
use auth_service::{utils, Application};
use wiremock::MockServer;
// use auth_service::services::data_stores::hashmap_two_fa_code_store::HashmapTwoFACodeStore;
//use auth_service::services::data_stores::hashmap_user_store::HashmapUserStore;
// use auth_service::services::data_stores::banned_tokens_store::HashsetBannedTokenStore;
use auth_service::services::data_stores::postgres_user_store::PostgresUserStore;
use auth_service::services::data_stores::redis_banned_token_store::RedisBannedTokenStore;
use auth_service::services::data_stores::redis_two_fa_code_store::RedisTwoFACodeStore;

use auth_service::services::mock_email_client::MockEmailClient;
use std::sync::Arc;
use tokio::sync::RwLock;

pub struct TestApp {
    pub address: String,
    pub cookie_jar: Arc<reqwest::cookie::Jar>,
    pub http_client: reqwest::Client,
    pub banned_tokens_store: Arc<RwLock<RedisBannedTokenStore>>,
    pub two_fa_code_store: Arc<RwLock<RedisTwoFACodeStore>>,
    pub database_name: String,
    pub clean_up_called: bool,
    pub email_server: MockServer, // New!
}

impl TestApp {
    pub async fn new() -> Self {
        //let user_store = Arc::new(RwLock::new(HashmapUserStore::default()));
        let (database_name, pg_pool) = configure_postgresql().await;
        let user_store = Arc::new(RwLock::new(PostgresUserStore::new(pg_pool)));

        let redis_conn = std::sync::Arc::new(tokio::sync::RwLock::new(configure_redis()));
        let banned_tokens_store = std::sync::Arc::new(tokio::sync::RwLock::new(
            RedisBannedTokenStore::new(redis_conn.clone()),
        ));
        let two_fa_code_store = std::sync::Arc::new(tokio::sync::RwLock::new(
            RedisTwoFACodeStore::new(redis_conn),
        ));

        let email_client =
            std::sync::Arc::new(tokio::sync::RwLock::new(MockEmailClient::default()));
        let app_state = AppState::new(
            user_store,
            banned_tokens_store.clone(),
            two_fa_code_store.clone(),
            email_client,
        );

        let app = Application::build(app_state, utils::constants::test::APP_ADDRESS)
            .await
            .expect("Failed to build app");

        let address = format!("http://{}", app.address.clone());

        #[allow(clippy::let_underscore_future)]
        let _ = tokio::spawn(app.run());

        let cookie_jar = Arc::new(reqwest::cookie::Jar::default());
        let http_client = reqwest::Client::builder()
            .cookie_provider(cookie_jar.clone())
            .build()
            .unwrap();

        let email_server = MockServer::start().await; // New!
        let base_url = email_server.uri(); // New!
        let email_client = Arc::new(configure_postmark_email_client(base_url)); // Updated!

        Self {
            address,
            cookie_jar,
            http_client,
            banned_tokens_store,
            two_fa_code_store,
            database_name,
            clean_up_called: false,
            email_server,
        }
    }

    pub async fn get_root(&self) -> reqwest::Response {
        self.http_client
            .get(&format!("{}/", &self.address))
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn post_signup<Body>(&self, body: &Body) -> reqwest::Response
    where
        Body: serde::Serialize,
    {
        self.http_client
            .post(&format!("{}/signup", &self.address))
            .json(body)
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn post_login<Body>(&self, body: &Body) -> reqwest::Response
    where
        Body: serde::Serialize,
    {
        self.http_client
            .post(&format!("{}/login", &self.address))
            .json(body)
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn post_logout(&self) -> reqwest::Response {
        let json = "{}";
        self.http_client
            .post(&format!("{}/logout", &self.address))
            .json(json)
            .send()
            .await
            .expect("Failed to execute request.")
    }
    pub async fn post_verify2fa<Body>(&self, body: &Body) -> reqwest::Response
    where
        Body: serde::Serialize,
    {
        self.http_client
            .post(&format!("{}/verify-2fa", &self.address))
            .json(body)
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn post_verify_token<Body>(&self, body: &Body) -> reqwest::Response
    where
        Body: serde::Serialize,
    {
        self.http_client
            .post(format!("{}/verify-token", &self.address))
            .json(body)
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn clean_up(&mut self) {
        // if !self.clean_up_called.load(Ordering::Relaxed) {
        if !self.clean_up_called {
            let db_name = self.database_name.clone();
            // let clean_up_called = self.clean_up_called.clone();

            delete_database(&db_name).await;
            // tokio::spawn(async move {
            //     clean_up_called.store(true, Ordering::Relaxed);
            // });
            self.clean_up_called = true;
        }
    }
}

impl Drop for TestApp {
    fn drop(&mut self) {
        if !self.clean_up_called {
            panic!(
                "TestApp clean_up() was not called before drop() for database {}",
                self.database_name
            );
        }
    }
}

pub fn get_random_email() -> String {
    format!("{}@example.com", uuid::Uuid::new_v4())
}

fn configure_redis() -> redis::Connection {
    auth_service::get_redis_client(utils::constants::REDIS_HOST_NAME.to_owned())
        .expect("Failed to get Redis client")
        .get_connection()
        .expect("Failed to get Redis connection")
}

async fn configure_postgresql() -> (String, sqlx::PgPool) {
    let postgresql_conn_url = auth_service::utils::constants::DATABASE_URL.to_owned();

    // We are creating a new database for each test case, and we need to ensure each database has a unique name!
    let db_name = uuid::Uuid::new_v4().to_string();

    configure_database(&postgresql_conn_url, &db_name).await;

    let postgresql_conn_url_with_db = format!("{}/{}", postgresql_conn_url, db_name);

    // Create a new connection pool and return it
    let pg_pool = auth_service::get_postgres_pool(&postgresql_conn_url_with_db)
        .await
        .expect("Failed to create Postgres connection pool!");

    (db_name, pg_pool)
}

async fn configure_database(db_conn_string: &str, db_name: &str) {
    use sqlx::postgres::PgPoolOptions;
    use sqlx::Executor;

    // Create database connection
    let connection = PgPoolOptions::new()
        .connect(db_conn_string)
        .await
        .expect("Failed to create Postgres connection pool.");

    // Create a new database
    connection
        .execute(format!(r#"CREATE DATABASE "{}";"#, db_name).as_str())
        .await
        .expect("Failed to create database.");

    // Connect to new database
    let db_conn_string = format!("{}/{}", db_conn_string, db_name);

    let connection = PgPoolOptions::new()
        .connect(&db_conn_string)
        .await
        .expect("Failed to create Postgres connection pool.");

    // Run migrations against new database
    sqlx::migrate!()
        .run(&connection)
        .await
        .expect("Failed to migrate the database");
}

async fn delete_database(db_name: &str) {
    use sqlx::postgres::PgConnectOptions;
    use sqlx::Executor;
    use sqlx::{Connection, PgConnection};
    use std::str::FromStr;

    let postgresql_conn_url: String = auth_service::utils::constants::DATABASE_URL.to_owned();

    let connection_options = PgConnectOptions::from_str(&postgresql_conn_url)
        .expect("Failed to parse PostgreSQL connection string");

    let mut connection = PgConnection::connect_with(&connection_options)
        .await
        .expect("Failed to connect to Postgres");

    // Kill any active connections to the database
    connection
        .execute(
            format!(
                r#"
                SELECT pg_terminate_backend(pg_stat_activity.pid)
                FROM pg_stat_activity
                WHERE pg_stat_activity.datname = '{}'
                  AND pid <> pg_backend_pid();
        "#,
                db_name
            )
            .as_str(),
        )
        .await
        .expect("Failed to drop the database.");

    // Drop the database
    connection
        .execute(format!(r#"DROP DATABASE "{}";"#, db_name).as_str())
        .await
        .expect("Failed to drop the database.");
}

// New!
use auth_service::services::postmark_email_client::PostmarkEmailClient;
use secrecy::SecretString;
fn configure_postmark_email_client(base_url: String) -> PostmarkEmailClient {
    use utils::constants::test::email_client;

    let postmark_auth_token = SecretString::new("auth_token".to_owned().into_boxed_str());

    let sender = auth_service::domain::email::Email::parse(SecretString::new(
        email_client::SENDER.to_owned().into_boxed_str(),
    ))
    .unwrap();

    let http_client = reqwest::Client::builder()
        .timeout(email_client::TIMEOUT)
        .build()
        .expect("Failed to build HTTP client");

    PostmarkEmailClient::new(base_url, sender, postmark_auth_token, http_client)
}
