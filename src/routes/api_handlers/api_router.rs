use axum::{
    middleware,
    routing::post,
    Router
};
use std::sync::Arc;


use crate::{state::AppState,
    jwt::jwt};

use super::sockets::*;


pub fn api_routes(app_state: Arc<AppState>) -> Router<Arc<AppState>> {
    Router::new()
        .route("/api/register_vegetable", post(register_vegetables))
        .layer(middleware::from_fn_with_state(app_state.clone() ,jwt::authorize))
}





// curl -X POST http://localhost:3000/register_vegetable \
// -H "Authorization: Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJleHAiOjE3NDM5OTM1MTIsImVtYWlsIjoidGVzdEB0ZXN0LnRlc3QifQ.mJ86qwlrYd5Xdky41Z1odQj4Uv3XtC-tKvLJybVrm7Y" \
// -H "Content-Type: application/json" \
// -d '{"name": "はす"}'
