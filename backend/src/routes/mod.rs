use std::sync::Arc;
use axum::{Router, routing::get};
use crate::database::Db;
use crate::modules::users::get_user_by_slug;

pub fn app_router(db: Arc<Db>) -> Router {
    Router::new()
        .route("/test", get("Hello, World!"))
        .route("/users/{slug}", get(get_user_by_slug))
        .with_state(db)
}
