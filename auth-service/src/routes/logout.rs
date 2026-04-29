use axum::response::IntoResponse;

pub async fn logout() -> impl IntoResponse {
    axum::http::StatusCode::OK.into_response()
}
