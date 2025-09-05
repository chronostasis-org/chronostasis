use std::env;
use tower_http::services::ServeFile;
use crate::database::Db;
use crate::api::handlers::get_user_by_slug::get_user_by_slug;
use axum::{http::{header, HeaderValue}, routing::get_service, Router, routing::get};
use tower_http::set_header::SetResponseHeaderLayer;

pub fn app_router(db: &Db) -> Router {
    let app_env = env::var("APP_ENV").unwrap_or_else(|_| "development".to_string());
    let cache_control_val = match app_env.as_str() {
        "production" => "public, max-age=3600, must-revalidate",
        _ => "no-store",
    };
    let file_service = get_service(ServeFile::new("static/assets/spritesheet.png"))
        // Add/override Cache-Control
        .layer(SetResponseHeaderLayer::overriding(
            header::CACHE_CONTROL,
            HeaderValue::from_static(cache_control_val),
        ));
    Router::new()
        .route("/test", get("Hello, World!"))
        .route("/users/{slug}", get(get_user_by_slug))
        .route_service("/spritesheet", file_service)
        .with_state(db)
}