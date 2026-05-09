use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

use crate::{
    app_state::AppState,
    domain::{self, data_stores::UserStoreError, email::Email, password::Password},
};

#[derive(Deserialize, Serialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

impl LoginRequest {
    pub fn new(email: &str, password: &str) -> Self {
        LoginRequest {
            email: email.to_owned(),
            password: password.to_owned(),
        }
    }
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

    let email = Email::parse(request.email).map_err(|_| StatusCode::BAD_REQUEST)?;

    let password = Password::parse(request.password).map_err(|_| StatusCode::BAD_REQUEST)?;

    match user_store.validate_user(&email, &password).await {
        Err(UserStoreError::InvalidCredentials) | Err(UserStoreError::UserNotFound) => {
            Err(StatusCode::UNAUTHORIZED)
        }
        Err(_) => Err(StatusCode::UNPROCESSABLE_ENTITY),
        Ok(_) => Ok(StatusCode::OK),
    }
}
