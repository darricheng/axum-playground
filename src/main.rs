mod transactions;

use axum::Router;

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new().nest("/api", transactions::routes::transaction_routes().await);

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
