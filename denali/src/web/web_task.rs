use std::{collections::HashMap, sync::Arc};

use axum::{
    // http::StatusCode,
    // Json,
    Router,
    routing::{get, post},
};
use tokio::sync::{RwLock, mpsc::Sender};

use crate::{
    Transactor,
    plugins::Plugin,
    web::WebState,
    web::endpoints::{chain_height, get_something, root, submit},
};

pub async fn web_task(
    tx: Sender<(String, String)>,
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
        .route("/chain-height", get(chain_height))
        .route("/submit", post(submit))
        .route("/get-something", get(get_something))
        .with_state(web_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
