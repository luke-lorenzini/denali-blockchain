// use std::sync::Arc;

// use axum::{
//     routing::{
//         get,
//         post
//     },
//     // http::StatusCode,
//     // Json,
//     Router,
// };
// use crate::{
//     Message,
//     Transactor,
//     plugin::Plugin,
//     web::{
//         task::web,
//     },
// };
// use futures::future::join_all;
use tokio::{
    // join, spawn,
    sync::{
        mpsc::{
            // channel,
            Receiver,
            Sender,
        },
        // RwLock
    },
    // task::JoinHandle,
    time::{Duration, sleep},
};

pub async fn message_generator_task(tx: Sender<(String, String)>) {
    let mut flag = 0;

    loop {
        sleep(Duration::from_millis(100)).await;

        let payload;
        let program;
        if flag == 0 {
            flag = 1;
            // A fake - working
            payload = r#"
                {
                    "fake": 0
                }"#;
            program = "fake";
        } else if flag == 1 {
            flag = 2;
            // A bank
            payload = r#"
                {
                    "payer": 0,
                    "payee": 1,
                    "amount": 10.0
                }"#;
            program = "bank";
        } else {
            flag = 0;
            // A vote - working
            payload = r#"
                {
                    "candidate": "candidate1"
                }"#;
            program = "vote";
        }

        tx.send((program.into(), payload.into())).await.unwrap();
    }
}

pub async fn receiver_task(
    tx_msg_queue: Sender<Vec<(String, String)>>,
    mut rx: Receiver<(String, String)>,
) {
    const BATCH_SIZE: usize = 10;
    let mut transactions = Vec::new();

    while let Some(i) = rx.recv().await {
        transactions.push(i);
        if transactions.len() == BATCH_SIZE {
            let batch = std::mem::take(&mut transactions);
            tx_msg_queue.send(batch).await.unwrap();
        }
    }
}
