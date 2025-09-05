use axum::{http::{header, HeaderValue}, routing::get_service, Router, routing::get};
use server::common::cfg::Configuration;
use server::database::Db;
use tokio::net::TcpListener;
use std::sync::Arc;
use std::env;
use tower_http::services::ServeFile;
use tower_http::set_header::SetResponseHeaderLayer;
use server::api::handlers::get_user_by_slug::get_user_by_slug;
use server::api::router::app_router;

mod logging;

#[tokio::main]
async fn main() {
  // Initialize logging
  logging::init_logger();
  // Load .env
  log::info!("dotenv loaded");
  dotenvy::dotenv().ok();

  // Parse config and init DB
  log::info!("Initializing configuration");
  let cfg = Configuration::new();

  // Initialize db connection.
  log::info!("Initializing db connection");
  let db = Db::new(&cfg).await.expect("Failed to initialize db");

  if cfg.db_run_migrations {
    log::info!("Running migrations");
    db.run_migrations().await.expect("Failed to run migrations");
  } else {
    log::info!("Skipping migrations as DATABASE_RUN_MIGRATIONS is disabled");
  }

  // Cache-Control based APP_ENV state
/*  let app_env = env::var("APP_ENV").unwrap_or_else(|_| "development".to_string());
  let cache_control_val = match app_env.as_str() {
    "production" => "public, max-age=3600, must-revalidate",
    _ => "no-store",
  };*/

  // File service for PNG spritesheet
/*  let file_service = get_service(ServeFile::new("static/assets/spritesheet.png"))
      // Add/override Cache-Control
      .layer(SetResponseHeaderLayer::overriding(
        header::CACHE_CONTROL,
        HeaderValue::from_static(cache_control_val),
      ));*/

  // Set up the main app router from routes::app_router (all app routes)
  let app = app_router(&db);

  // Start Axum server using recommended axum::serve API
  println!("Starting Axum server on 0.0.0.0:8000");
  let listener = TcpListener::bind("0.0.0.0:8000").await.unwrap();
  axum::serve(listener, app.into_make_service())
      .await
      .unwrap();
}

/*use std::sync::Arc;
use axum::{Router, routing::get};
use crate::database::Db;
use crate::modules::users::get_user_by_slug;

pub fn app_router(db: Arc<Db>) -> Router {
  Router::new()
      .route("/test", get("Hello, World!"))
      .route("/users/{slug}", get(get_user_by_slug))
      .with_state(db)
}*/
