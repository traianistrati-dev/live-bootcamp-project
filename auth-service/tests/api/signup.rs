use crate::helpers::{self, TestApp};

use auth_service::{routes::SignupResponse, ErrorResponse};

#[tokio::test]
async fn should_return_400_if_invalid_input() {
    let app = TestApp::new().await;

    let test_cases = [
        serde_json::json!({
            "password": "12345678",
             "email":"user_email.test",
             "requires2FA": false
        }),
        serde_json::json!({
            "password": "123456",
             "email":"user@email.test",
             "requires2FA": false
        }),
    ];

    for test_case in test_cases.iter() {
        let response = app.post_signup(test_case).await;
        assert_eq!(
            response.status().as_u16(),
            400,
            "Failed for input: {:?}",
            test_case
        );

        assert_eq!(
            response
                .json::<ErrorResponse>()
                .await
                .expect("Could not deserialize response body to ErrorResponse")
                .error,
            "Invalid credentials".to_owned()
        );
    }
}

#[tokio::test]
async fn should_return_409_if_email_already_exists() {
    let app = TestApp::new().await;

    let test_user = serde_json::json!({
        "password": "password123",
         "email":"user@email.test",
         "requires2FA": false
    });
    let response = app.post_signup(&test_user).await;

    assert_eq!(response.status().as_u16(), 201);
    // check if  test fails
    // let test_user = serde_json::json!({
    //     "password": "password123",
    //      "email":"user2@email.test",
    //      "requires2FA": false
    // });

    let response = app.post_signup(&test_user).await;

    assert_eq!(response.status().as_u16(), 409);

    assert_eq!(
        response
            .json::<ErrorResponse>()
            .await
            .expect("Could not deserialize response body to ErrorResponse")
            .error,
        "User already exists".to_owned()
    );
}

#[tokio::test]
async fn should_return_201_if_valid_input() {
    let app = TestApp::new().await;

    let test_case = serde_json::json!({
        "email": helpers::get_random_email(),
        "password": "password123",
        "requires2FA": true
    });

    let response = app.post_signup(&test_case).await;

    assert_eq!(response.status().as_u16(), 201);

    let expected_response = SignupResponse {
        message: "User created successfully!".to_owned(),
    };

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
