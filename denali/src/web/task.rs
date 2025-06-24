use std::sync::Arc;

use axum::{
    // http::StatusCode,
    // Json,
    Router,
    routing::{get, post},
};
use tokio::sync::RwLock;

use crate::{
    Transactor,
    web::endpoints::{chain_height, root, submit},
};

pub async fn web(transactor: Arc<RwLock<Transactor>>) {
    let app = Router::new()
        .route("/", get(root))
        .route("/chain_height", get(chain_height))
        .route("/submit", post(submit))
        .with_state(transactor);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
