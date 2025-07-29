use std::{collections::HashMap, sync::Arc};

use tokio::sync::{Notify, RwLock, mpsc::Receiver};

use crate::{
    messaging::Meta,
    plugins::Plugin,
    processor::{Message, Processor},
    types::H256,
};

#[tracing::instrument]
pub async fn processor_task(
    contract_map: Arc<RwLock<HashMap<String, Plugin>>>,
    processor: Arc<RwLock<Processor>>,
    mut rx_msg_queue: Receiver<Vec<(H256, String, String, Meta)>>,
    notify: Option<Arc<Notify>>,
) {
    // receive a batch of messages
    while let Some(messages) = rx_msg_queue.recv().await {
        let mut transactions = Vec::new();
        // this loop can be parallelized, maybe
        for message in messages {
            let program = contract_map.read().await;
            // println!("{:?}", message.0);

            if let Some(program) = program.get(&message.1) {
                let program = program.thing.clone_box();
                let transaction = Message {
                    program,
                    payload: message.2,
                    tx_id: message.0,
                    metadata: message.3,
                };
                transactions.push(transaction);
            }
        }
        let _res = processor
            .clone()
            .write()
            .await
            .create_new_block(&transactions, notify.clone())
            .await;
    }
}
