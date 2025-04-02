use std::sync::Arc;
use askama::Template;
use axum::{
    extract::State,
    response::Html,
    http::StatusCode,
};
use crate::state::AppState;

#[allow(dead_code)]
#[derive(Template)]
#[template(path="partials/root.html")]
struct RegisterTemplate<'a> {
    title: &'a str,
}


pub async fn root(State(_app_state): State<Arc<AppState>>) -> impl axum::response::IntoResponse {
    let page_template = RegisterTemplate {
        title: "Root Page"
    };

    (StatusCode::OK, Html(page_template.render().unwrap()))
}