use crate::helpers::{self, TestApp};

#[tokio::test]
async fn should_return_422_if_malformed_credentials() {
    let random_email = helpers::get_random_email();
    let password = "password123";

    let test_cases = [
        serde_json::json!({
            "password": password,
        }),
        serde_json::json!({
            "email":random_email,
        }),
        serde_json::json!({
            "password": "password",
             "email":random_email
        }),
        serde_json::json!({
            "password": password,
             "email":true,
        }),
    ];

    let app = TestApp::new().await;

    for test_case in test_cases.iter() {
        let response = app.post_login(&test_case).await;

        assert_eq!(
            response.status().as_u16(),
            422,
            "Failed for input: {:?}",
            test_case
        );
    }
}
