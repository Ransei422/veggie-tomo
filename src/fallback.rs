// == Axum handler for any request that fails to match the router routes ==

use std::sync::Arc;
use askama::Template;
use http::StatusCode;
use axum::{
    extract::State,
    response::Html
};

use crate::state::AppState;



#[allow(dead_code)]
#[derive(Template)]
#[template(path="partials/fallback.html")]
struct NothingTemplate<'a> {
    title: &'a str,
}

pub async fn fallback(State(_app_state): State<Arc<AppState>>)
    -> impl axum::response::IntoResponse {

    let page_template = NothingTemplate {
        title: "Nothing Here"
    };
    return (StatusCode::OK, Html(page_template.render()
        .unwrap_or(String::from(
            "<!DOCTYPE html>
            <html lang='en'>
                <head></head>
                <body>
                    <h1>Nothing here</h1>
                </body>
            </html>
            "))))
}