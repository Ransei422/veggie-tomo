use std::sync::Arc;
use axum::{extract::State, response::Html};
use http::StatusCode;

use crate::state::AppState;

/// axum handler for any request that fails to match the router routes.
/// This implementation returns HTTP status code Not Found (404).

pub async fn fallback(State(_app_state): State<Arc<AppState>>) -> impl axum::response::IntoResponse {
    return (StatusCode::OK, Html("{h1} FALLBACK {/h1}"))
}