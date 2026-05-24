use crate::helpers::TestApp;
use test_macros::auto_db_cleanup;

#[auto_db_cleanup]
#[tokio::test]
async fn should_return_206_if_valid_credentials_and_2fa_enabled() {
    let mut app = TestApp::new().await;
    let email = "example@email.test";
    let password = "12345678";

    {
        // create new test User
        let response = app
            .post_signup(&serde_json::json!({
                "email": email,
                "password": password,
                "requires2FA": true
            }))
            .await;
        assert_eq!(response.status().as_u16(), 201);
    }

    let response = app
        .post_login(&serde_json::json!({
            "email": email,
            "password": password
        }))
        .await;

    assert_eq!(response.status().as_u16(), 206);

    assert_eq!(
        response
            .json::<auth_service::routes::TwoFactorAuthResponse>()
            .await
            .expect("Could not deserialize response body to TwoFactorAuthResponse")
            .message,
        "2FA required".to_owned()
    );
    //todo!();
}
#[auto_db_cleanup]
#[tokio::test]
async fn should_return_422_if_malformed_credentials() {
    let email = "example@email.test";

    let password = "12345678";

    let test_cases = &[
        serde_json::json!({
            "password": password,
        }),
        serde_json::json!({
            "email":email,
        }),
        serde_json::json!({
            "password": 1111,
             "email":email
        }),
        serde_json::json!({
            "password": password,
             "email":true,
        }),
        serde_json::json!({
            "password": true,
             "email":true,
        }),
    ];

    let mut app = TestApp::new().await;
    helper_post_login_test_cases(test_cases, 422, &mut app).await;
}
#[auto_db_cleanup]
#[tokio::test]
async fn should_return_400_if_invalid_input() {
    let email = "example@email.test";
    let email_invalid = "example_mail.test";

    let password = "12345678";
    let password_invalid = "1234567";

    let test_cases = &[
        serde_json::json!({
             "email":"",
            "password": "",
        }),
        serde_json::json!({
             "email":"",
            "password": password,
        }),
        serde_json::json!({
             "email":email,
            "password": "",
        }),
        serde_json::json!({
             "email":email,
            "password": password_invalid,
        }),
        serde_json::json!({
             "email":email_invalid,
            "password": password,
        }),
        serde_json::json!({
             "email":email_invalid,
            "password": password_invalid,
        }),
    ];

    let mut app = TestApp::new().await;

    helper_post_login_test_cases(test_cases, 400, &mut app).await;
}
#[auto_db_cleanup]
#[tokio::test]
async fn should_return_401_if_incorrect_credentials() {
    let email = "example@email.test";
    let email_invalid = "_example@email.test";

    let password = "12345678";
    let password_invalid = "_12345678";

    //{
    // create new test User
    let mut app = TestApp::new().await;
    let response = app
        .post_signup(&serde_json::json!({
            "email": email,
            "password": password,
            "requires2FA": false
        }))
        .await;
    assert_eq!(response.status().as_u16(), 201);
    //}

    let test_cases = &[
        serde_json::json!({
             "email":email,
            "password": password_invalid,
        }),
        serde_json::json!({
             "email":email_invalid,
            "password": password,
        }),
    ];

    helper_post_login_test_cases(test_cases, 401, &mut app).await;
}
#[auto_db_cleanup]
#[tokio::test]
async fn should_return_200_if_valid_credentials_and_2fa_disabled() {
    let email = "example@email.test";
    let password = "12345678";

    let mut app = TestApp::new().await;
    {
        // create new test User
        let response = app
            .post_signup(&serde_json::json!({
                "email": email,
                "password": password,
                "requires2FA": false
            }))
            .await;
        assert_eq!(response.status().as_u16(), 201);
    }

    let login_body = serde_json::json!({
        "email": email,
        "password": password,
    });

    let response = app.post_login(&login_body).await;

    assert_eq!(response.status().as_u16(), 200);

    let auth_cookie = response
        .cookies()
        .find(|cookie| cookie.name() == auth_service::utils::constants::JWT_COOKIE_NAME)
        .expect("No auth cookie found");

    assert!(!auth_cookie.value().is_empty());
}

/// Test Helper method
async fn helper_post_login_test_cases(
    test_cases: &[serde_json::Value],
    expected_status_code: u16,
    app: &mut TestApp,
) {
    // let app = TestApp::new().await;
    for test_case in test_cases.iter() {
        let response = app.post_login(test_case).await;

        assert_eq!(
            response.status().as_u16(),
            expected_status_code,
            "Failed for input: {:?}",
            test_case
        );
    }
}
