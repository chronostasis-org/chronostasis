use server::common::cfg::Configuration;
use server::database::Db;
use server::routes::app_router;
use tokio::net::TcpListener;
use std::sync::Arc;

#[tokio::main]
async fn main() {
  // Loads the .env file located in the environment's current directory or its parents in sequence.
  // .env used only for development, so we discard error in all other cases.
  println!("dotenv loaded");
  dotenvy::dotenv().ok();

  // Parse configuration from the environment.
  // This will exit with a help message if something is wrong.
  println!("Initializing configuration");
  let cfg = Configuration::new();

  // Initialize db connection.
  println!("Initializing db connection");
  let db = Db::new(&cfg).await.expect("Failed to initialize db");
  let db = Arc::new(db);

  // Run migrations if enabled
  if cfg.db_run_migrations {
    println!("Running migrations");
    db.run_migrations().await.expect("Failed to run migrations");
  } else {
    println!("Skipping migrations as DATABASE_RUN_MIGRATIONS is disabled");
  }

  // Set up Axum router using routes::app_router
  let app = app_router(db.clone());

  // Start Axum server using recommended axum::serve API
  println!("Starting Axum server on 0.0.0.0:8000");
  let listener = TcpListener::bind("0.0.0.0:8000").await.unwrap();
  axum::serve(listener, app.into_make_service())
      .await
      .unwrap();
}
