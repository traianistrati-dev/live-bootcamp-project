// mod services;
// mod app_state;

use auth_service::app_state::AppState;
use auth_service::services::hashmap_user_store::HashmapUserStore;
use auth_service::utils;

#[tokio::main]
async fn main() {
    let user_store = std::sync::Arc::new(tokio::sync::RwLock::new(HashmapUserStore::default()));
    let app_state = AppState::new(user_store);

    let app = auth_service::Application::build(app_state, utils::constants::prod::APP_ADDRESS)
        .await
        .expect("Failed to build app");

    app.run().await.expect("Failed to run app");
}
