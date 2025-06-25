use std::sync::Arc;

use axum::{
    // http::StatusCode,
    // Json,
    Router,
    routing::{get, post},
};
use tokio::sync::{RwLock, mpsc::Sender};

use crate::{
    Transactor,
    web::endpoints::{chain_height, root, submit},
};

#[derive(Clone, Debug)]
pub struct WebState {
    pub transactor: Arc<RwLock<Transactor>>,
    pub tx: Sender<(String, String)>,
}

pub async fn web(tx: Sender<(String, String)>, transactor: Arc<RwLock<Transactor>>) {
    let web_state = WebState { transactor, tx };
    let app = Router::new()
        .route("/", get(root))
        .route("/chain-height", get(chain_height))
        .route("/submit", post(submit))
        .with_state(web_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
