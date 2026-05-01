use auth_service::Application;
//use reqwest::{Client, Response};


use auth_service::services::hashmap_user_store::HashmapUserStore;
use auth_service::app_state::AppState;

pub struct TestApp {
    pub address: String,
    pub http_client: reqwest::Client,
}

impl TestApp {
    pub async fn new() -> Self {
        let user_store = std::sync::Arc::new(tokio::sync::RwLock::new(
            HashmapUserStore::default(),
        ));
        let app_state = AppState::new(user_store);

        let app = Application::build(app_state, "127.0.0.1:0")
            .await
            .expect("Failed to build app");

        let address = format!("http://{}", app.address.clone());

        // Run the auth service in a separate async task
        // to avoid blocking the main test thread.
        #[allow(clippy::let_underscore_future)]
        let _ = tokio::spawn(app.run());

        let http_client = reqwest::Client::new(); //todo!(); // Create a Reqwest http client instance

        //todo!()// Create new `TestApp` instance and return it
        Self {
            address,
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

    // TODO: Implement helper functions for all other routes (signup, login, logout, verify-2fa, and verify-token)

    // pub async fn post_signup(&self) -> reqwest::Response {
    //     let json = "{}";
    //     self.http_client
    //         .post(&format!("{}/signup", &self.address))
    //         .json(json)
    //         .send()
    //         .await
    //         .expect("Failed to execute request.")
    // }

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

    pub async fn post_login(&self) -> reqwest::Response {
        let json = "{}";
        self.http_client
            .post(&format!("{}/login", &self.address))
            .json(json)
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

    pub async fn post_verify_token(&self) -> reqwest::Response {
        let json = "{}";
        self.http_client
            .post(&format!("{}/verify-token", &self.address))
            .json(json)
            .send()
            .await
            .expect("Failed to execute request.")
    }
}

pub fn get_random_email() -> String {
    format!("{}@example.com", uuid::Uuid::new_v4())
}
