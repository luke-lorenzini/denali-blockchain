use std::{collections::HashMap, sync::Arc};

use axum::{
    // http::StatusCode,
    // Json,
    Router,
    routing::{get, post},
};
use tokio::sync::{RwLock, mpsc::Sender};

use crate::{
    
    messaging::ResponseTx, plugins::Plugin, transactor::Transactor, web::{endpoints::*, WebState}
};

#[tracing::instrument]
pub async fn web_task(
    tx: Sender<ResponseTx>,
    transactor: Arc<RwLock<Transactor>>,
    contract_map: Arc<RwLock<HashMap<String, Plugin>>>,
) {
    let web_state = WebState {
        transactor,
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

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
