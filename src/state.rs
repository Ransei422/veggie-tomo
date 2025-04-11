// == File for app state ==

use std::sync::Arc;
use sqlx::postgres::PgPool;

#[derive(Clone)]
pub struct AppState {
    pub jwt_secret: String,
    pub db_pool: PgPool,
}

impl AppState {
    pub fn new(db_pool: PgPool, jwt_secret: String) -> Arc<Self> {
        Arc::new(
            AppState {
                jwt_secret,
                db_pool,
        })
    }
}
