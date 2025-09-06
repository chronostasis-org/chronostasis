use crate::common::cfg::Config;
use axum::body::Bytes;
use std::fs;
use std::io;
use std::path::Path;

pub async fn get_spritesheet(cfg: &Config) -> Result<Bytes, io::Error> {
  let base_path = match &cfg.static_assets_path {
    Some(path) => path,
    None => "static/assets",
  };

  let path = Path::new(base_path).join("spritesheet.png");
  log::info!("Path for spritesheet is {}/spritesheet.png", base_path);

  let content = fs::read(path)?;

  Ok(Bytes::from(content))
}
