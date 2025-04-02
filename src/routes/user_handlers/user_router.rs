use std::sync::Arc;

use axum::{routing::get, Router};

use crate::state::AppState;

use super::users::*;


pub fn user_routes()  -> Router<Arc<AppState>> {
    Router::new()
        .route("/register", get(register_page))
}