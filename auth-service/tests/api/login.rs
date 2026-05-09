use crate::helpers::TestApp;

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

    helper_post_login_test_cases(test_cases, 422).await;
}

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

    helper_post_login_test_cases(test_cases, 400).await;
}

#[tokio::test]
async fn should_return_401_if_incorrect_credentials() {
    let email = "example@email.test";
    let email_invalid = "_example@email.test";

    let password = "12345678";
    let password_invalid = "_12345678";

    {
        // create new test User
        let app = TestApp::new().await;
        let response = app
            .post_signup(&serde_json::json!({
                "email": email,
                "password": password,
                "requires2FA": false
            }))
            .await;
        assert_eq!(response.status().as_u16(), 201);
    }

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

    helper_post_login_test_cases(test_cases, 401).await;
}

/// Test Helper method
async fn helper_post_login_test_cases(test_cases: &[serde_json::Value], expected_status_code: u16) {
    let app = TestApp::new().await;
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
