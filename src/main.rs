use std::env;
use axum::{routing::get, Router, Json, Extension};
use std::net::SocketAddr;
use std::sync::Arc;
use dotenvy::dotenv;
use sea_orm::{
  Database, DbErr
};

#[tokio::main]
async fn main() {

  // Load environment variables
  dotenv().ok();

  // Get database connection string from environment variables
  let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

  let db = Database::connect(&database_url)
    .await.expect("Failed to connect to database");
  let db = Arc::new(db); // Use Arc for thread-safe sharing

  // Build the router and add the database connection as an extension
  let app = Router::new()
    .route("/", get(|| async { "Hello, World!" }))
    .route("/hello/:name", get(|| async { "Hello, World!" }))
    .layer(Extension(db));

  let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
  println!("Sever is running at http://{}", addr);

  // run our app with hyper, listening globally on port 3000
  let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
    .await.unwrap();

  axum::serve(listener, app).await.unwrap();

}

async fn root() -> &'static str {
  "Welcome to use Rust sever"
}

struct Greeting {
  message: String,
}

async fn hello(axum::extract::Path(name): axum::extract::Path<String>) -> Json<Greeting> {
  let greeting = Greeting {
    message: format!("Hello, {}!", name),
  };
  Json(greeting)
}
