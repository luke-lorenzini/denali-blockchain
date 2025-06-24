use std::sync::Arc;

use axum::{extract::{Query, State}, http::StatusCode, response::IntoResponse};
use serde::Deserialize;
use tokio::sync::RwLock;

use crate::Transactor;

#[derive(Debug, Deserialize)]
pub struct Params {
    _program: String,
    _payload: Payload,
}

#[derive(Debug, Deserialize)]
struct Payload {
    _candidate: String,
}

pub async fn root() -> &'static str {
    "Hello, World!"
}

pub async fn chain_height(State(state): State<Arc<RwLock<Transactor>>>) -> impl IntoResponse {
    let height = state.read().await.chain.get_chain_height().to_string();
    (StatusCode::OK, height)
}

// #[axum::debug_handler]
pub async fn submit(
    State(state): State<Arc<RwLock<Transactor>>>, 
    Query(params): Query<Params>,
) 
-> impl IntoResponse 
{
    println!("{state:?}");
    println!("{params:?}");
    (StatusCode::OK, "asdf")
}
