use serde::Deserialize;
use std::{str::FromStr, sync::Arc};

pub type Config = Arc<Configuration>;

#[derive(Deserialize, Debug)]
pub struct Configuration {
  /// The environment in which to run the application.
  pub env: Environment,

  /// The port to listen on.
  pub app_port: u16,

  /// The DSN for the database. Currently, only PostgreSQL is supported.
  pub db_dsn: String,

  /// Maximum number of connections in the database pool
  pub db_pool_max_size: u32,

  /// Database connection timeout in seconds
  pub db_timeout: u64,

  /// Whether to run database migrations on startup
  pub db_run_migrations: bool,

  /// Path to static assets directory (optional)
  pub static_assets_path: Option<String>,
}

#[derive(Deserialize, Debug)]
pub enum Environment {
  Development,
  Production,
}

impl Configuration {
  /// Creates a new configuration from environment variables.
  pub fn new() -> Config {
    let env = env_var("APP_ENV")
            .parse::<Environment>()
            .expect("Unable to parse the value of the APP_ENV environment variable. Please make sure it is either \"development\" or \"production\".");

    let app_port = env_var("PORT")
            .parse::<u16>()
            .expect("Unable to parse the value of the PORT environment variable. Please make sure it is a valid unsigned 16-bit integer");

    let db_dsn = env_var("DATABASE_URL");

    // Default pool size is 10 if not specified
    let db_pool_max_size = std::env::var("DATABASE_POOL_MAX_SIZE")
            .unwrap_or_else(|_| "10".to_string())
            .parse::<u32>()
            .expect("Unable to parse the value of the DATABASE_POOL_MAX_SIZE environment variable. Please make sure it is a valid unsigned 32-bit integer");

    // Default timeout is 5 seconds if not specified
    let db_timeout = std::env::var("DATABASE_TIMEOUT")
            .unwrap_or_else(|_| "5".to_string())
            .parse::<u64>()
            .expect("Unable to parse the value of the DATABASE_TIMEOUT environment variable. Please make sure it is a valid unsigned 64-bit integer");

    // Default to true in development, false in production
    let db_run_migrations = std::env::var("DATABASE_RUN_MIGRATIONS")
            .unwrap_or_else(|_| match env {
                Environment::Development => "true".to_string(),
                Environment::Production => "false".to_string(),
            })
            .parse::<bool>()
            .expect("Unable to parse the value of the DATABASE_RUN_MIGRATIONS environment variable. Please make sure it is a valid boolean");

    // Get static assets path if specified
    let static_assets_path = std::env::var("STATIC_ASSETS_PATH").ok();

    let config = Arc::new(Configuration {
      env,
      app_port,
      db_dsn,
      db_pool_max_size,
      db_timeout,
      db_run_migrations,
      static_assets_path,
    });

    // Log the current configuration
    log::info!("Application configuration loaded");

    config
  }

  /// Sets the database DSN.
  /// This method is used in tests to override the database DSN.
  pub fn set_dsn(&mut self, db_dsn: String) {
    self.db_dsn = db_dsn
  }
}

impl FromStr for Environment {
  type Err = String;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      "development" => Ok(Environment::Development),
      "production" => Ok(Environment::Production),
      _ => Err(format!(
        "Invalid environment: {}. Please make sure it is either \"development\" or \"production\".",
        s
      )),
    }
  }
}

pub fn env_var(name: &str) -> String {
  std::env::var(name)
    .map_err(|e| format!("{}: {}", name, e))
    .expect("Missing environment variable")
}
