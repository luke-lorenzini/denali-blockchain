use std::{collections::HashMap, sync::Arc};

use axum::{
    // http::StatusCode,
    // Json,
    Router,
    routing::{get, post},
};
use tokio::sync::{RwLock, mpsc::Sender};

use crate::{
    messaging::ResponseTx,
    plugins::Plugin,
    processor::Processor,
    web::{WebState, endpoints::*},
};

#[tracing::instrument]
pub async fn web_task(
    tx: Sender<ResponseTx>,
    processor: Arc<RwLock<Processor>>,
    contract_map: Arc<RwLock<HashMap<String, Plugin>>>,
    replica: bool,
) {
    let web_state = WebState {
        processor,
        tx,
        contract_map,
    };
    let app = Router::new()
        .route("/", get(root))
        .route("/chain-height", get(get_height))
        .route("/tip", get(get_tip))
        .route("/submit", post(submit))
        .route("/is-block", get(is_block))
        .route("/get-block-header", get(block_header))
        .route("/get-block-transactions", get(block_transactions))
        .route("/get-chain", get(get_chain))
        .route("/get-tx", get(get_tx))
        .with_state(web_state);

    let listener = if !replica {
        tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap()
    } else {
        tokio::net::TcpListener::bind("0.0.0.0:3001").await.unwrap()
    };
    axum::serve(listener, app).await.unwrap();
}
