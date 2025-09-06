use server::api::router::app_router;
use server::common::cfg::Configuration;
use server::database::Db;
use tokio::net::TcpListener;

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
  let app = app_router(cfg, db);

  // Start Axum server using recommended axum::serve API
  println!("Starting Axum server on 0.0.0.0:8000");
  let listener = TcpListener::bind("0.0.0.0:8000").await.unwrap();
  axum::serve(listener, app.into_make_service())
    .await
    .unwrap();
}
