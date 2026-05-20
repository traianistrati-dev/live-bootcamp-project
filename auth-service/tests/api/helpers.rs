use auth_service::{utils, Application};

use auth_service::app_state::AppState;
use auth_service::services::data_stores::banned_tokens_store::HashsetBannedTokenStore;
use auth_service::services::data_stores::hashmap_two_fa_code_store::HashmapTwoFACodeStore;
use auth_service::services::data_stores::hashmap_user_store::HashmapUserStore;
use auth_service::services::mock_email_client::MockEmailClient;
use std::sync::Arc;
use tokio::sync::RwLock;

pub struct TestApp {
    pub address: String,
    pub cookie_jar: Arc<reqwest::cookie::Jar>,
    pub http_client: reqwest::Client,
    pub banned_tokens_store: Arc<RwLock<HashsetBannedTokenStore>>,
    pub hashmap_two_fa_code_store: Arc<RwLock<HashmapTwoFACodeStore>>,
}

impl TestApp {
    pub async fn new() -> Self {
        let user_store = Arc::new(RwLock::new(HashmapUserStore::default()));
        let banned_tokens_store = Arc::new(RwLock::new(HashsetBannedTokenStore::default()));
        let hashmap_two_fa_code_store = Arc::new(RwLock::new(HashmapTwoFACodeStore::default()));
        let email_client =
            std::sync::Arc::new(tokio::sync::RwLock::new(MockEmailClient::default()));
        let app_state = AppState::new(
            user_store,
            banned_tokens_store.clone(),
            hashmap_two_fa_code_store.clone(),
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

        Self {
            address,
            cookie_jar,
            http_client,
            banned_tokens_store,
            hashmap_two_fa_code_store,
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
}

pub fn get_random_email() -> String {
    format!("{}@example.com", uuid::Uuid::new_v4())
}
