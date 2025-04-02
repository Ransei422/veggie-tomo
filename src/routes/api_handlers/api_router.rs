use axum::{
    middleware,
    routing::get,
    Router
};
use std::sync::Arc;


use crate::{state::AppState,
    jwt::jwt};

use super::sockets::*;


pub fn api_routes(app_state: Arc<AppState>) -> Router<Arc<AppState>> {
    Router::new()
        .route("/api", get(main_page_get))
        .route("/demo.json", get(get_demo_json).put(put_demo_json)
        .layer(middleware::from_fn_with_state(app_state.clone() ,jwt::authorize)))
}