use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

use crate::domain::errors::AuthAPIError;
use crate::utils::auth::validate_token;

#[derive(Deserialize, Serialize)]
pub struct VerifyTokenRequest {
    pub token: String,
}

pub async fn verify_token(
    Json(request): Json<VerifyTokenRequest>,
) -> Result<impl IntoResponse, AuthAPIError> {
    match validate_token(&request.token).await {
        Ok(_) => Ok(StatusCode::OK),
        Err(_) => Err(AuthAPIError::InvalidToken),
    }
}
