use std::{collections::HashMap, sync::Arc};

use anyhow::{Ok, Result};
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
    web::{
        WebState,
        endpoints::{
            block_header, block_transactions, chain_hash, get_chain, get_height, get_tip, get_tx,
            is_block, root, submit,
        },
    },
};

#[tracing::instrument]
pub async fn web_task(
    tx: Sender<ResponseTx>,
    processor: Arc<RwLock<Processor>>,
    contract_map: Arc<RwLock<HashMap<String, Plugin>>>,
    replica: bool,
    web_port_number: u16,
) -> Result<()> {
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
        .route("/get/chain-hash", get(chain_hash))
        .with_state(web_state);

    let address = format!("0.0.0.0:{web_port_number}");
    let listener = tokio::net::TcpListener::bind(address).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
