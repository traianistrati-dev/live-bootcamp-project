use crate::helpers::{self, TestApp};

use auth_service::routes::signup::SignupResponse;

#[tokio::test]
async fn should_return_201_if_valid_input() {
    //...

    let app = TestApp::new().await;

    let test_case = serde_json::json!({
        "email": helpers::get_random_email(), // Call helper method to generate email
        "password": "password123",
        "requires2FA": true
    });

    let response = app.post_signup(&test_case).await;

    assert_eq!(response.status().as_u16(), 201);

    let expected_response = SignupResponse {
        message: "User created successfully!".to_owned(),
    };

    // Assert that we are getting the correct response body!
    assert_eq!(
        response
            .json::<SignupResponse>()
            .await
            .expect("Could not deserialize response body to UserBody"),
        expected_response
    );
}

#[tokio::test]
async fn should_return_422_if_malformed_input() {
    let random_email = helpers::get_random_email(); // Call helper method to generate email

    // TODO: add more malformed input test cases
    let test_cases = [
        serde_json::json!({
            "password": "password123",
            "requires2FA": true
        }),
        serde_json::json!({
            "email":random_email,
            "requires2FA": true
        }),
        serde_json::json!({
            "password": "password123",
             "email":random_email
        }),
        serde_json::json!({
            "password": "password123",
             "email":true,
             "requires2FA": 1000
        }),
    ];

    let app = TestApp::new().await;

    for test_case in test_cases.iter() {
        let response = app.post_signup(&test_case).await;

        println!("\x1b[55m response.status:'{:?}' \x1b[0m", response.status());

        assert_eq!(
            response.status().as_u16(),
            422,
            "Failed for input: {:?}",
            test_case
        );
    }
}
