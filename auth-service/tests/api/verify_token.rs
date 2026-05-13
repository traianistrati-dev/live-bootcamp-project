use crate::helpers::TestApp;
use auth_service::{domain::data_stores::BannedTokenStore, utils::constants::JWT_COOKIE_NAME};

#[tokio::test]
async fn should_return_200_valid_token() {
    let app = TestApp::new().await;

    let email = "example@email.test";
    let password = "12345678";

    {
        //create new User
        let test_case = serde_json::json!({
            "email": email,
            "password": password,
            "requires2FA": false
        });

        assert_eq!(app.post_signup(&test_case).await.status().as_u16(), 201);
    }

    let response = app
        .post_login(&serde_json::json!({
            "email": email,
            "password": password,
        }))
        .await;

    assert_eq!(response.status().as_u16(), 200);

    let auth_cookie = response
        .cookies()
        .find(|cookie| cookie.name() == auth_service::utils::constants::JWT_COOKIE_NAME)
        .expect("No auth cookie found");

    let token = auth_cookie.value();

    assert!(!token.is_empty());

    {
        //verify token
        let response = app
            .post_verify_token(&serde_json::json!({
                "token": token,
            }))
            .await;

        assert_eq!(response.status().as_u16(), 200);
    }
}

#[tokio::test]
async fn should_return_401_if_banned_token() {
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

    let contains_banned_token = app
        .banned_tokens_store
        .read()
        .await
        .contains_banned_token(token)
        .await
        .expect("Failed to check banned token");

    assert_eq!(contains_banned_token, true);

    {
        assert_eq!(
            app.post_verify_token(&serde_json::json!({
                "token": token,
            }))
            .await
            .status()
            .as_u16(),
            401
        );
    }
}

#[tokio::test]
async fn should_return_401_if_invalid_token() {
    let response = TestApp::new()
        .await
        .post_verify_token(&serde_json::json!({
            "token": "invalid",
        }))
        .await;

    assert_eq!(response.status().as_u16(), 401);
}

#[tokio::test]
async fn should_return_422_if_malformed_input() {
    let test_cases = &[
        // serde_json::json!({
        //      "token":"invalid",
        // }),
        serde_json::json!({
             "token":true,
        }),
        serde_json::json!({
             "token":111,
        }),
        serde_json::json!({
             "token":{},
        }),
        serde_json::json!({}),
    ];

    let app = TestApp::new().await;
    for test_case in test_cases.iter() {
        let response = app.post_verify_token(test_case).await;

        assert_eq!(response.status().as_u16(), 422);
    }
}
