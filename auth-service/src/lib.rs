use axum::response::IntoResponse;
use axum::routing::post;
use axum::Router;
use std::error::Error;
use tower_http::services::ServeDir;

// This struct encapsulates our application-related logic.
pub struct Application {
    server: axum::serve::Serve<tokio::net::TcpListener, Router, Router>,
    // address is exposed as a public field
    // so we have access to it in tests.
    pub address: String,
}

impl Application {
    pub async fn build(address: &str) -> Result<Self, Box<dyn Error>> {
        // Move the Router definition from `main.rs` to here.
        // Also, remove the `hello` route.
        // We don't need it at this point!
        let assets_dir = ServeDir::new("assets");
        let router = Router::new()
            .fallback_service(assets_dir)
            .route("/signup", post(signup))
            .route("/login", post(login))
            .route("/logout", post(logout))
            .route("/verify-2fa", post(verify_2fa))
            .route("/verify-token", post(verify_token));

        let listener = tokio::net::TcpListener::bind(address).await?;
        let address = listener.local_addr()?.to_string();
        let server = axum::serve(listener, router);

        // Create a new Application instance and return it
        Ok(Self { address, server })
    }
    pub async fn run(self) -> Result<(), std::io::Error> {
        println!("listening on {}", &self.address);
        self.server.await
    }
}

async fn signup() -> impl IntoResponse {
    axum::http::StatusCode::OK.into_response()
}

async fn login() -> impl IntoResponse {
    axum::http::StatusCode::OK.into_response()
}

async fn logout() -> impl IntoResponse {
    axum::http::StatusCode::OK.into_response()
}

async fn verify_2fa() -> impl IntoResponse {
    axum::http::StatusCode::OK.into_response()
}

async fn verify_token() -> impl IntoResponse {
    axum::http::StatusCode::OK.into_response()
}
