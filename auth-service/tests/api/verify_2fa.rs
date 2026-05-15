use auth_service::domain::data_stores::TwoFACodeStore;

use auth_service::{
    domain::email::Email, routes::TwoFactorAuthResponse, utils::constants::JWT_COOKIE_NAME,
};

use crate::helpers::TestApp;

#[tokio::test]
async fn should_return_200_if_correct_code() {
    let app = TestApp::new().await;

    let email = "example@email.test";
    let password = "12345678";
    {
        //create new User
        let signup_body = serde_json::json!({
            "email": email,
            "password": password,
            "requires2FA": true
        });

        assert_eq!(app.post_signup(&signup_body).await.status().as_u16(), 201);
    }

    let response_login = app
        .post_login(&serde_json::json!({
            "email": email,
            "password": password
        }))
        .await;

    assert_eq!(response_login.status().as_u16(), 206);

    {
        let auth_cookie = response_login
            .cookies()
            .find(|cookie| cookie.name() == JWT_COOKIE_NAME)
            .expect("No auth cookie found");

        assert!(!auth_cookie.value().is_empty());
    }

    let response_body = response_login
        .json::<TwoFactorAuthResponse>()
        .await
        .expect("Could not deserialize response body to TwoFactorAuthResponse");

    assert_eq!(response_body.message, "2FA required".to_owned());
    assert!(!response_body.login_attempt_id.is_empty());

    let code_tuple = app
        .hashmap_two_fa_code_store
        .read()
        .await
        .get_code(&Email::parse(email.to_owned()).unwrap())
        .await
        .unwrap();

    let response = app
        .post_verify2fa(&serde_json::json!({
            "email": email,
            "loginAttemptId": response_body.login_attempt_id,
            "2FACode": code_tuple.1.as_ref()
        }))
        .await;

    assert_eq!(response.status().as_u16(), 200);
}
