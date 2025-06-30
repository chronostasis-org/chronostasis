use axum::{
    routing::get,
    Router,
};

#[tokio::main]
async fn main() {
    // Connect to DB
    let db: DatabaseConnection = Database::connect("postgres://username:password@host/database?currentSchema=my_schema").await?;

    // build our application with a single route
    let app = Router::new().route("/", get(|| async { "Hello, World!" }));

    // run our app with hyper, listening globally on port 8000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}