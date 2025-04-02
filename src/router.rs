use std::sync::Arc;
use routes::root::root;

use crate::jwt::jwt;
use axum::{routing::{get, post},
        Router
};
use crate::{fallback::fallback,
    routes,
    state::AppState
};

use routes::api_handlers::api_router::api_routes;
use routes::user_handlers::user_router::user_routes;





pub fn setup_routes(app_state: Arc<AppState>) -> Router {
    let route = Router::new()
        .fallback(fallback)
        // Main Page
        .route("/", get(root))
        .route("/signin", post(jwt::sign_in_page))
        .route("/hashme", post(jwt::hash_password_page))
        // APIs
        .merge(api_routes(app_state.clone()))
        // Users
        .merge(user_routes(app_state.clone()))
        ;

    route.with_state(app_state)
}