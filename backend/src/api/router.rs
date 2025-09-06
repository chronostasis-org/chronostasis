use crate::api::handlers::get_asset::get_spritesheet;
use crate::api::handlers::get_user_by_slug::get_user_by_slug;
use crate::common::cfg::Config;
use crate::database::Db;

use axum::{routing::get, Router};

#[derive(Clone)]
pub struct AppState {
  pub db: Db,
  pub cfg: Config,
}

pub fn app_router(cfg: Config, db: Db) -> Router {
  let app_state = AppState { db, cfg };

  Router::new()
    .route("/test", get(|| async { "Hello, World!" }))
    .route("/users/{slug}", get(get_user_by_slug))
    .route("/spritesheet", get(get_spritesheet))
    // .route("/assets/{filename}", get(get_asset))
    .with_state(app_state)
}
