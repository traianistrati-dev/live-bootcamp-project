use crate::helpers::TestApp;

#[tokio::test]
async fn post_verify_token_should_suceed() {
    let app = TestApp::new().await;
    let response = app.post_verify_token().await;

    assert_eq!(response.status().as_u16(), 200);
    // assert_eq!(
    //     response.headers().get("content-type").unwrap(),
    //     "application/json"
    // );
}
