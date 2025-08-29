use axum::{
  http::{header, HeaderValue},
  routing::get_service,
  Router,
};
use server::common::cfg::Configuration;
use server::database::Db;
use std::env;
use tower_http::services::ServeFile;
use tower_http::set_header::SetResponseHeaderLayer;

#[tokio::main]
async fn main() {
  // Load .env
  println!("dotenv loaded");
  dotenvy::dotenv().ok();

  // Parse config and init DB
  println!("Initializing configuration");
  let cfg = Configuration::new();

  println!("Initializing db connection");
  let db = Db::new(&cfg).await.expect("Failed to initialize db");

  if cfg.db_run_migrations {
    println!("Running migrations");
    db.run_migrations().await.expect("Failed to run migrations");
  } else {
    println!("Skipping migrations as DATABASE_RUN_MIGRATIONS is disabled");
  }

  // Cache-Control based APP_ENV state
  let app_env = env::var("APP_ENV").unwrap_or_else(|_| "development".to_string());
  let cache_control_val = match app_env.as_str() {
    "production" => "public, max-age=3600, must-revalidate",
    _ => "no-store",
  };

  // File service for PNG spritesheet
  let file_service = get_service(ServeFile::new("static/assets/spritesheet.png"))
    // Add/override Cache-Control
    .layer(SetResponseHeaderLayer::overriding(
      header::CACHE_CONTROL,
      HeaderValue::from_static(cache_control_val),
    ));

  let app = Router::new()
    // Serve the spritesheet at /spritesheet
    .route_service("/spritesheet", file_service);

  let addr = "0.0.0.0:8000";
  println!("listening on http://{addr}");
  let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
  axum::serve(listener, app).await.unwrap();
}
