use denali::{Message, Transactor, storage::State, types::Thing};
use tokio::{
    join, spawn,
    sync::mpsc::channel,
    time::{Duration, sleep},
};

const BATCH_SIZE: usize = 10;

#[tokio::main]
async fn main() {
    println!("Hello, denali");

    let (tx, mut rx) = channel(100);
    let (tx_msg_queue, mut rx_msg_queue) = channel(100);

    let listener_thread = spawn(async move {
        loop {
            sleep(Duration::from_millis(100)).await;

            let payload = r#"
            {
                "fake": 0
            }"#
            .into();
            let fake = FakeProgram;
            let message = Message {
                payload,
                program: fake,
            };

            tx.send(message).await.unwrap();
        }
    });

    let receiver_thread = spawn({
        async move {
            let mut transactions = Vec::new();

            while let Some(i) = rx.recv().await {
                transactions.push(i);
                if transactions.len() == BATCH_SIZE {
                    tx_msg_queue.send(transactions.clone()).await.unwrap();
                    transactions.clear();
                }
            }
        }
    });

    let processor_thread = spawn(async move {
        let mut transactor = Transactor::new();
        println!("notified");

        while let Some(messages) = rx_msg_queue.recv().await {
            let _res = transactor.create_new_block(messages);
        }
    });

    let _res = join!(receiver_thread, listener_thread, processor_thread);
}

#[derive(Clone)]
struct FakeProgram;

impl Thing for FakeProgram {
    fn run(&self, _payload: &str, state: &mut State) -> serde_json::Result<()> {
        println!("run");
        state.get_value("test");
        Ok(())
    }

    fn verify(&self) -> serde_json::Result<bool> {
        println!("verify");
        Ok(true)
    }
}
