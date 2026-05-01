use axum::response::IntoResponse;
use axum::{http::StatusCode, Json};
use serde::{Deserialize, Serialize};

use crate::app_state::AppState;
use axum::extract::State;

#[derive(Deserialize)]
pub struct SignupRequest {
    pub email: String,
    pub password: String,
    #[serde(rename = "requires2FA")]
    pub requires_2fa: bool,
}

// pub async fn signup(Json(request): Json<SignupRequest>) -> impl IntoResponse {
//     StatusCode::OK.into_response()
// }


pub async fn signup(
    // TODO: Use Axum's state extractor to pass in AppState
    State(state): State<AppState>,
    Json(request): Json<SignupRequest>,
) -> impl IntoResponse {
    // Create a new `User` instance using data in the `request`
    let user = crate::domain::User {
        email: request.email,
        password: request.password,
        requires_2fa: request.requires_2fa,
    };
    let mut user_store = state.user_store.write().await;
    
    // TODO: Add `user` to the `user_store`. Simply unwrap the returned `Result` enum type for now.
    user_store.add_user(user).unwrap();

    let response = Json(SignupResponse {
        message: "User created successfully!".to_string(),
    });

    (StatusCode::CREATED, response)
}

//...

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct SignupResponse {
    pub message: String,
}
