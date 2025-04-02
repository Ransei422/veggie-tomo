use std::sync::Arc;

use axum::{response::IntoResponse, routing::get, Router};

use crate::state::AppState;

use super::users::*;


pub fn user_routes(app_state: Arc<AppState>)  -> Router<Arc<AppState>> {
    Router::new()
        .route("/register", get(register_page))
}