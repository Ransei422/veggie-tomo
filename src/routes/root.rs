use std::sync::Arc;
use axum::{
    extract::State,
    response::Html,
    http::StatusCode,
};
use crate::state::AppState;


pub async fn root(State(_app_state): State<Arc<AppState>>) -> impl axum::response::IntoResponse {
    return (StatusCode::OK, Html("Hello World"))
}