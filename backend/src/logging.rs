use env_logger::Env;

pub fn init_logger() {
  env_logger::Builder::from_env(Env::default().default_filter_or("warn"))
    .format_timestamp_secs()
    .init();
}

// use tracing_subscriber::{fmt, EnvFilter};

// pub fn init_logger() {
//   // Route `log` macros to `tracing`
//   let _ = tracing_log::LogTracer::init();

//   // RUST_LOG controls levels, e.g. "info,tower_http=info,axum=info,server=debug"
//   let env_filter = EnvFilter::try_from_default_env()
//     .unwrap_or_else(|_| EnvFilter::new("info,tower_http=info,axum=info"));

//   fmt()
//     .with_env_filter(env_filter)
//     .with_target(true)
//     .with_file(true)
//     .with_line_number(true)
//     .compact()
//     .init();
// }
