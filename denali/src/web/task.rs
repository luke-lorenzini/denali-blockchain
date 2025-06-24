use std::sync::Arc;

use axum::{
    routing::{
        get,
        post
    },
    // http::StatusCode,
    // Json,
    Router,
};
use tokio::sync::RwLock;

use crate::{web::endpoints::{
    root, 
    chain_height, 
    submit
},
Transactor};

pub async fn web(transactor: Arc<RwLock<Transactor>>) {
    // let web_thread = spawn(
        // {
            // let transactor = transactor.clone();
            // async move {
                // let transactor = Arc::new(RwLock::new(Transactor::new()));
                // let transactor = Transactor::new();
                let app = Router::new()
                // `GET /` goes to `root`
                .route("/", get(root))
                .route("/chain_height", get(chain_height))
                .route("/submit", post(submit))
                .with_state(transactor);

                let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
                axum::serve(listener, app).await.unwrap();
            // }
        // }
    // )
    // ;
}
