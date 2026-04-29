use axum::response::IntoResponse;

pub async fn signup() -> impl IntoResponse {
    axum::http::StatusCode::OK.into_response()
}
