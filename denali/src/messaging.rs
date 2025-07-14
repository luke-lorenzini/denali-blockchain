use borsh::{BorshDeserialize, BorshSerialize};
use chrono::Utc;
use hex::encode;
use sha2::{Digest, Sha256};
use tokio::{
    sync::{
        mpsc::{Receiver, Sender},
        oneshot,
    },
    time::{Duration, sleep},
};

use crate::{constants::BATCH_SIZE, types::H256};

#[derive(
    BorshDeserialize, BorshSerialize, 
    Clone, Debug, PartialEq)]
pub struct Meta {}

pub struct ResponseTx {
    pub one_shot: oneshot::Sender<ResponseRx>,
    pub metadata: Meta,
    pub program: String,
    pub payload: String,
}

pub struct ResponseRx {
    pub status: bool,
    pub tx_id: H256,
}

pub async fn message_generator_task(tx: Sender<(String, String, Meta)>) {
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

        tx.send((program.into(), payload.into(), Meta {}))
            .await
            .unwrap();
    }
}

#[tracing::instrument]
pub async fn receiver_task(
    tx_msg_queue: Sender<Vec<(H256, String, String, Meta)>>,
    mut rx: Receiver<ResponseTx>,
) {
    let mut transactions = Vec::new();

    while let Some(i) = rx.recv().await {
        // confirm the rx'd message has been queued for processing. it could fail, but at this point, it'll be in the ledger
        let mut hasher = Sha256::new();
        hasher.update(Utc::now().timestamp_micros().to_le_bytes());
        hasher.update(i.program.clone());
        hasher.update(i.payload.clone());
        // hasher.update(i.metadata);
        let res = hasher.finalize();
        let tx_id: H256 = encode(res).try_into().unwrap();
        println!("tx hash: {tx_id:?}");
        let ack = ResponseRx {
            status: true,
            tx_id: tx_id.clone(),
        };
        let _ = i.one_shot.send(ack);

        transactions.push((tx_id, i.program, i.payload, i.metadata));
        if transactions.len() == BATCH_SIZE {
            let batch = std::mem::take(&mut transactions);
            tx_msg_queue.send(batch).await.unwrap();
        }
    }
}
