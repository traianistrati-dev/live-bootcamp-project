use crate::{
    app_state::AppState,
    domain::{email::Email, password::Password},
    utils, AuthAPIError,
};
use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
//use axum_extra::extract::cookie::CookieJar;
use axum_extra::extract::CookieJar;
use serde::{Deserialize, Serialize};

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

#[axum::debug_handler]
pub async fn login(
    State(state): State<AppState>,
    jar: CookieJar, // New!
    Json(request): Json<LoginRequest>,
) -> (CookieJar, Result<axum::response::Response, AuthAPIError>) {
    let email = match Email::parse(request.email) {
        Ok(email) => email,
        Err(_) => return (jar, Err(AuthAPIError::InvalidCredentials)),
    };

    let password = match Password::parse(request.password) {
        Ok(password) => password,
        Err(_) => return (jar, Err(AuthAPIError::InvalidCredentials)),
    };

    let user_store = state.user_store.write().await;

    if user_store.validate_user(&email, &password).await.is_err() {
        return (jar, Err(AuthAPIError::IncorrectCredentials));
    }

    let user = match user_store.get_user(&email).await {
        Ok(user) => user,
        Err(_) => return (jar, Err(AuthAPIError::IncorrectCredentials)),
    };

    let auth_cookie = match utils::auth::generate_auth_cookie(&user.email) {
        Ok(cookie) => cookie,
        Err(_) => return (jar, Err(AuthAPIError::UnexpectedError)),
    };

    let updated_jar = jar.add(auth_cookie);

    (updated_jar, Ok(StatusCode::OK.into_response()))
}
