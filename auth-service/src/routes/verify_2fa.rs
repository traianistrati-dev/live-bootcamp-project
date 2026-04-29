use axum::response::IntoResponse;
pub async fn verify_2fa() -> impl IntoResponse {
    axum::http::StatusCode::OK.into_response()
}
