use auth_service::{utils, Application};

use auth_service::app_state::AppState;
use auth_service::services::hashmap_user_store::HashmapUserStore;

pub struct TestApp {
    pub address: String,
    pub cookie_jar: std::sync::Arc<reqwest::cookie::Jar>,
    pub http_client: reqwest::Client,
}

impl TestApp {
    pub async fn new() -> Self {
        let user_store = std::sync::Arc::new(tokio::sync::RwLock::new(HashmapUserStore::default()));
        let app_state = AppState::new(user_store);

        let app = Application::build(app_state, utils::constants::test::APP_ADDRESS)
            .await
            .expect("Failed to build app");

        let address = format!("http://{}", app.address.clone());

        #[allow(clippy::let_underscore_future)]
        let _ = tokio::spawn(app.run());

        // let http_client = reqwest::Client::new();

        let cookie_jar = std::sync::Arc::new(reqwest::cookie::Jar::default());
        let http_client = reqwest::Client::builder()
            .cookie_provider(cookie_jar.clone())
            .build()
            .unwrap();

        Self {
            address,
            cookie_jar,
            http_client,
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
    pub async fn post_verify2fa(&self) -> reqwest::Response {
        let json = "{}";
        self.http_client
            .post(&format!("{}/verify-2fa", &self.address))
            .json(json)
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
