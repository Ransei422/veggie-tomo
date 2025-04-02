use std::sync::Arc;
use askama::Template;
use axum::{extract::State, response::Html};
use http::StatusCode;

use crate::state::AppState;

/// axum handler for any request that fails to match the router routes.
/// This implementation returns HTTP status code Not Found (404).


#[allow(dead_code)]
#[derive(Template)]
#[template(path="partials/fallback.html")]
struct NothingTemplate<'a> {
    title: &'a str,
}

pub async fn fallback(State(_app_state): State<Arc<AppState>>) -> impl axum::response::IntoResponse {
    let page_template = NothingTemplate {
        title: "Nothing Here"
    };
    return (StatusCode::OK, Html(page_template.render().unwrap()))
}