use super::controllers;
use axum::{
    routing::{get, post},
    Router,
};
use mongodb::Client;

pub async fn transaction_routes() -> Router {
    let mongodb_client = Client::with_uri_str("mongodb://localhost:27017/")
        .await
        .unwrap();

    let routes = Router::new()
        .route("/", get(|| async { "Hello base path!" }))
        .route("/getter", get(|| async { "Hello getter!" }))
        .route("/poster", post(controllers::add_transaction))
        .with_state(mongodb_client);

    Router::new().nest("/transactions", routes)
}
