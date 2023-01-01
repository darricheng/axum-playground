use std::collections::HashMap;

use super::model::Transaction;
use axum::{
    extract::{self, Query, State},
    http::StatusCode,
    response::IntoResponse,
};
use mongodb::{bson::doc, Client};

const DB_NAME: &str = "masterFinanceTracker";
const COLLECTION_NAME: &str = "myTransactions";

pub async fn add_transaction(
    extract::State(state): State<Client>,
    extract::Json(json_payload): extract::Json<Transaction>,
) -> impl IntoResponse {
    let collection = state.database(DB_NAME).collection(COLLECTION_NAME);
    let result = collection.insert_one(json_payload, None).await;
    match result {
        Ok(_) => StatusCode::CREATED,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

pub async fn get_transaction(
    extract::State(state): State<Client>,
    params: Option<Query<HashMap<String, String>>>,
) -> impl IntoResponse {
    let collection = state.database(DB_NAME).collection(COLLECTION_NAME);

    let result = match params {
        Some(params) => {
            let filter = doc!(params);
            collection.find(filter, None).await
        }
        None => collection.find(None, None).await,
    };
}
