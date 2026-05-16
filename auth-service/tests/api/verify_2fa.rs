use auth_service::domain::data_stores::TwoFACodeStore;

use auth_service::{
    domain::email::Email, routes::TwoFactorAuthResponse, utils::constants::JWT_COOKIE_NAME,
};

use crate::helpers::TestApp;
use auth_service::domain::data_stores::TwoFACode;

#[tokio::test]
async fn should_return_401_if_same_code_twice() {
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

    let response = app
        .post_login(&serde_json::json!({
            "email": email,
            "password": password
        }))
        .await;

    assert_eq!(response.status().as_u16(), 206);

    let auth_cookie = response
        .cookies()
        .find(|cookie| cookie.name() == JWT_COOKIE_NAME)
        .expect("No auth cookie found");

    assert!(!auth_cookie.value().is_empty());

    let response_body = response
        .json::<TwoFactorAuthResponse>()
        .await
        .expect("Could not deserialize response body to TwoFactorAuthResponse");

    assert_eq!(response_body.message, "2FA required".to_owned());
    assert!(!response_body.login_attempt_id.is_empty());

    let login_attempt_id = response_body.login_attempt_id;

    let code_tuple = app
        .hashmap_two_fa_code_store
        .read()
        .await
        .get_code(&Email::parse(email.to_owned()).unwrap())
        .await
        .unwrap();

    let code = code_tuple.1.as_ref();

    let response = app
        .post_verify2fa(&serde_json::json!({
            "email": email,
            "loginAttemptId": login_attempt_id,
            "2FACode": code
        }))
        .await;

    assert_eq!(response.status().as_u16(), 200);

    let response = app
        .post_verify2fa(&serde_json::json!({
            "email": email,
            "loginAttemptId": login_attempt_id,
            "2FACode": code
        }))
        .await;
    assert_eq!(response.status().as_u16(), 401);
}

#[tokio::test]
async fn should_return_401_if_incorrect_credentials() {
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

    let response_body = response_login
        .json::<TwoFactorAuthResponse>()
        .await
        .expect("Could not deserialize response body to TwoFactorAuthResponse");

    assert_eq!(response_body.message, "2FA required".to_owned());
    assert!(!response_body.login_attempt_id.is_empty());

    {
        let two_fa_code = TwoFACode::default().as_ref().to_owned();

        let test_cases = &[
            serde_json::json!({
                "email": email,
                  "loginAttemptId":response_body.login_attempt_id,
                  "2FACode": "123456"
            }),
            serde_json::json!({
                "email": email,
                  "loginAttemptId":response_body.login_attempt_id,
                  "2FACode": two_fa_code
            }),
        ];

        for test_case in test_cases {
            let verify_2fa_rsponse = app.post_verify2fa(&test_case).await;

            assert_eq!(
                verify_2fa_rsponse.status().as_u16(),
                401,
                "\x1b[41m verify_2fa_response: {:?} \x1b[0m",
                verify_2fa_rsponse
            );
        }
    }
}

#[tokio::test]
async fn should_return_401_if_old_code() {
    // Call login twice. Then, attempt to call verify-fa with the 2FA code from the first login requet. This should fail.
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

    let response = app
        .post_login(&serde_json::json!({
            "email": email,
            "password": password
        }))
        .await;

    assert_eq!(response.status().as_u16(), 206);

    let auth_cookie = response
        .cookies()
        .find(|cookie| cookie.name() == JWT_COOKIE_NAME)
        .expect("No auth cookie found");

    assert!(!auth_cookie.value().is_empty());

    {
        let response = app
            .post_login(&serde_json::json!({
                "email": email,
                "password": password
            }))
            .await;

        assert_eq!(response.status().as_u16(), 206);
    }

    let response_body = response
        .json::<TwoFactorAuthResponse>()
        .await
        .expect("Could not deserialize response body to TwoFactorAuthResponse");

    assert_eq!(response_body.message, "2FA required".to_owned());
    assert!(!response_body.login_attempt_id.is_empty());

    let login_attempt_id = response_body.login_attempt_id;

    let code_tuple = app
        .hashmap_two_fa_code_store
        .read()
        .await
        .get_code(&Email::parse(email.to_owned()).unwrap())
        .await
        .unwrap();

    let code = code_tuple.1.as_ref();

    let response = app
        .post_verify2fa(&serde_json::json!({
            "email": email,
            "loginAttemptId": login_attempt_id,
            "2FACode": code
        }))
        .await;

    assert_eq!(response.status().as_u16(), 401);
    // assert_eq!(response.status().as_u16(), 200);
}

#[tokio::test]
async fn should_return_400_if_invalid_input() {
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

    let response_body = response_login
        .json::<TwoFactorAuthResponse>()
        .await
        .expect("Could not deserialize response body to TwoFactorAuthResponse");

    assert_eq!(response_body.message, "2FA required".to_owned());
    assert!(!response_body.login_attempt_id.is_empty());

    {
        let two_fa_code = TwoFACode::default().as_ref().to_owned();

        let test_cases = &[
            serde_json::json!({
                "email": email,
                  "loginAttemptId":response_body.login_attempt_id,
                  "2FACode": ""
            }),
            serde_json::json!({
                "email": email,
                  "loginAttemptId":"",
                  "2FACode": ""
            }),
            serde_json::json!({
                "email": email,
                  "loginAttemptId":"12345678901234567890",
                  "2FACode": two_fa_code
            }),
            serde_json::json!({
                "email": email,
                  "loginAttemptId":"",
                  "2FACode": "123456"
            }),
            // serde_json::json!({
            //     "email": email,
            //       "loginAttemptId":response_body.login_attempt_id,
            //       "2FACode": "123456"
            // }),
        ];

        for test_case in test_cases {
            let verify_2fa_rsponse = app.post_verify2fa(&test_case).await;

            assert_eq!(
                verify_2fa_rsponse.status().as_u16(),
                400,
                "\x1b[41m verify_2fa_response: {:?} \x1b[0m",
                verify_2fa_rsponse
            );
        }
    }
}

#[tokio::test]
async fn should_return_422_if_malformed_input() {
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

    let response_body = response_login
        .json::<TwoFactorAuthResponse>()
        .await
        .expect("Could not deserialize response body to TwoFactorAuthResponse");

    assert_eq!(response_body.message, "2FA required".to_owned());
    assert!(!response_body.login_attempt_id.is_empty());

    {
        let test_cases = &[
            serde_json::json!({
                "email": email,
                  "loginAttemptId": response_body.login_attempt_id,
                  "2FACode": 1111
            }),
            serde_json::json!({
                "email": email,
                  "loginAttemptId": response_body.login_attempt_id,
                  // "2FACode": "string"
            }),
            serde_json::json!({
                "email": email,
                  // "loginAttemptId": response_body.login_attempt_id,
                  "2FACode": "string"
            }),
            serde_json::json!({
                "email": true,
                  "loginAttemptId":response_body.login_attempt_id,
                  "2FACode": ""
            }),
            serde_json::json!({
                "email": email,
                  "loginAttemptId": {},
                  "2FACode": "string"
            }),
            serde_json::json!({
                "2FACode": "123456",
            }),
            serde_json::json!({
                "email": email,
            }),
            serde_json::json!({
                "loginAttemptId": response_body.login_attempt_id,
            }),
            serde_json::json!({
                "2FACode": "123456",
                "email": email,
            }),
            serde_json::json!({
                "2FACode": "123456",
                "loginAttemptId": response_body.login_attempt_id,
            }),
            serde_json::json!({
                "email": email,
                "loginAttemptId": response_body.login_attempt_id,
            }),
            serde_json::json!({}),
        ];

        for test_case in test_cases {
            let verify_2fa_rsponse = app.post_verify2fa(&test_case).await;

            assert_eq!(
                verify_2fa_rsponse.status().as_u16(),
                422,
                "\x1b[41m verify_2fa_response: {:?} \x1b[0m",
                verify_2fa_rsponse
            );
        }
    }
}

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
