use crate::Transactor;
use axum::{extract::State, http::StatusCode, response::IntoResponse};

pub async fn root() -> &'static str {
    "Hello, World!"
}

pub async fn chain_height(State(state): State<Transactor>) -> impl IntoResponse {
    let height = state.chain.get_chain_height().to_string();
    (StatusCode::OK, height)
}