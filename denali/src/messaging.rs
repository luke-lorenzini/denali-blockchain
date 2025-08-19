use anyhow::Result;
use borsh::{BorshDeserialize, BorshSerialize, to_vec};
use chrono::Utc;
use hex::encode;
use log::trace;
use sha2::{Digest, Sha256};
use tokio::sync::{
    mpsc::{Receiver, Sender},
    oneshot,
};

use crate::{constants::BATCH_SIZE, types::H256};

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
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

#[tracing::instrument]
pub async fn receiver_task(
    // tx_msg_queue: Sender<Vec<(H256, String, String, Meta)>>,
    tx_batch_queue: Sender<Vec<u8>>,
    mut rx: Receiver<ResponseTx>,
    // notify: Arc<Notify>,
) -> Result<()> {
    let mut transactions = Vec::new();

    while let Some(i) = rx.recv().await {
        // confirm the rx'd message has been queued for processing. it could fail, but at this point, it'll be in the ledger
        let mut hasher = Sha256::new();
        hasher.update(Utc::now().timestamp_micros().to_le_bytes());
        hasher.update(&i.program);
        hasher.update(&i.payload);
        // hasher.update(i.metadata);
        let res = hasher.finalize();
        let tx_id: H256 = encode(res).try_into()?;
        trace!("tx hash: {tx_id:?}");
        let ack = ResponseRx {
            status: true,
            tx_id,
        };
        let _ = i.one_shot.send(ack);

        transactions.push((tx_id, i.program, i.payload, i.metadata));
        if transactions.len() == BATCH_SIZE {
            let batch = std::mem::take(&mut transactions);
            let batch = to_vec(&batch).unwrap();
            tx_batch_queue.send(batch).await?;
            // notify.notify_one();
        }
    }

    Ok(())
}
