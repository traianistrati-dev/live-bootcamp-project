use auth_service::{
    domain::{data_stores::BannedTokenStore, password},
    utils::constants::JWT_COOKIE_NAME,
};
use reqwest::Url;

use crate::helpers::TestApp;

#[tokio::test]
async fn should_return_400_if_jwt_cookie_missing() {
    let app = TestApp::new().await;
    let logout_response = app.post_logout().await;

    assert_eq!(logout_response.status().as_u16(), 400);
}

#[tokio::test]
async fn should_return_401_if_invalid_token() {
    let app = TestApp::new().await;

    // add invalid cookie
    app.cookie_jar.add_cookie_str(
        &format!(
            "{}=invalid; HttpOnly; SameSite=Lax; Secure; Path=/",
            JWT_COOKIE_NAME
        ),
        &Url::parse("http://127.0.0.1").expect("Failed to parse URL"),
    );

    let logout_response = app.post_logout().await;

    assert_eq!(logout_response.status().as_u16(), 401);
}

#[tokio::test]
async fn should_return_200_if_jwt_cookie_is_valid() {
    let app = TestApp::new().await;

    let email = "example@email.test";
    let password = "12345678";

    assert_eq!(
        app.post_signup(&serde_json::json!({
            "email": email,
            "password": password,
            "requires2FA": false
        }))
        .await
        .status()
        .as_u16(),
        201
    );

    let response = app
        .post_login(&serde_json::json!({
            "email": email,
            "password": password,
        }))
        .await;

    assert_eq!(response.status().as_u16(), 200);

    let auth_cookie = response
        .cookies()
        .find(|cookie| cookie.name() == JWT_COOKIE_NAME)
        .expect("No auth cookie found");

    assert!(!auth_cookie.value().is_empty());

    let token = auth_cookie.value();

    assert_eq!(app.post_logout().await.status().as_u16(), 200);

    assert!(app
        .banned_tokens_store
        .read()
        .await
        .contains_banned_token(token.to_string())
        .await
        .expect("Failed to check banned token"));
}
