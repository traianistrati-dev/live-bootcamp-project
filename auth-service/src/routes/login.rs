use axum::response::IntoResponse;

pub async fn login() -> impl IntoResponse {
    axum::http::StatusCode::OK.into_response()
}
