use std::sync::Arc;

// use axum::{
//     routing::{
//         get,
//         post
//     },
//     // http::StatusCode,
//     // Json,
//     Router,
// };
use denali::{
    Message,
    Transactor,
    // web::task,
    plugin::Plugin,
    web::{
        task::web,
        // chain_height,
        // endpoints::{
        //     root,
        // submit
        // }
    },
};
use tokio::{
    join, spawn,
    sync::{RwLock, mpsc::channel},
    time::{Duration, sleep},
};

const BATCH_SIZE: usize = 10;

#[tokio::main]
async fn main() {
    println!("Hello, denali");

    let contract_map = Plugin::stuff();

    let transactor = Arc::new(RwLock::new(Transactor::new()));
    let (tx, mut rx) = channel(100);
    let (tx_msg_queue, mut rx_msg_queue) = channel(100);

    let mut flag = 0;

    // let scanner_thread = spawn({
    //     let contract_map = contract_map.clone();
    //     async move {
    //         // loop {
    //             Plugin::monitor(contract_map);
    //         // }
    // }});

    let listener_thread = spawn(async move {
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

            tx.send((program, payload)).await.unwrap();
        }
    });

    let receiver_thread = spawn(async move {
        let mut transactions = Vec::new();

        while let Some(i) = rx.recv().await {
            transactions.push(i);
            if transactions.len() == BATCH_SIZE {
                let batch = std::mem::take(&mut transactions);
                tx_msg_queue.send(batch).await.unwrap();
            }
        }
    });

    let processor_thread = spawn({
        let contract_map = contract_map.clone();
        let transactor = transactor.clone();
        async move {
            println!("notified");

            while let Some(messages) = rx_msg_queue.recv().await {
                for message in messages {
                    // println!("transactions: {message:?}");
                    let name = message.0;
                    // println!("{name:?}");
                    let payload: String = message.1.into();
                    // let program = contract_map;
                    let program = contract_map.read().await;
                    let program = program.get(name).unwrap().thing.as_ref();
                    let message = Message {
                        program,
                        payload: payload.clone(),
                    };
                    let messages = vec![message];
                    let _res = transactor
                        .clone()
                        .write()
                        .await
                        .create_new_block(messages)
                        .await;
                }
            }
        }
    });

    let web_thread = spawn(web(transactor.clone())).await;
    let _tasks = [web_thread];

    let _res = join!(
        listener_thread,
        processor_thread,
        receiver_thread,
        // scanner_thread,
    );
}
