use crate::helpers::{self, TestApp};

#[tokio::test]
async fn should_return_201_if_valid_input() {
    //  todo!()
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

    for test_case in test_cases.iter() {
        let app = TestApp::new().await;
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
