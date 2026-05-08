use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

use crate::{
    app_state::AppState,
    domain::{email::Email, password::Password},
};

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct LoginResponse {
    pub message: String,
}

pub async fn login(
    State(state): State<AppState>,
    Json(request): Json<LoginRequest>,
) -> impl IntoResponse {
    let user_store = state.user_store.write().await;

    let email = Email::parse(request.email).unwrap();
    let pass = Password::parse(request.password).unwrap();
    let validate_credentials_result = user_store.validate_user(&email, &pass).await;

    match validate_credentials_result {
        Ok(_) => Ok(StatusCode::OK),
        Err(_) => Err(StatusCode::UNPROCESSABLE_ENTITY),
    }
}
