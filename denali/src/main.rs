use std::sync::Arc;

use denali::{
    Transactor,
    messaging::{listener, receiver},
    plugin::Plugin,
    processor,
    web::task::web,
};
use tokio::{
    join, spawn,
    sync::{RwLock, mpsc::channel},
};

#[tokio::main]
async fn main() {
    println!("Hello, denali");

    let contract_map = Plugin::stuff();
    let transactor = Arc::new(RwLock::new(Transactor::new()));
    let (tx, rx) = channel(100);
    let (tx_msg_queue, rx_msg_queue) = channel(100);

    // let scanner_thread = spawn({
    //     let contract_map = contract_map.clone();
    //     async move {
    //         // loop {
    //             Plugin::monitor(contract_map);
    //         // }
    // }});

    let listener_thread = spawn(listener(tx));
    let receiver_thread = spawn(receiver(tx_msg_queue, rx));
    let processor_thread = spawn(processor(contract_map, transactor.clone(), rx_msg_queue));
    let web_thread = spawn(web(transactor));

    let _res = join!(
        listener_thread,
        processor_thread,
        receiver_thread,
        // scanner_thread,
        web_thread
    );

    // let handles = spawn_all_tasks(
    //     // contract_map,
    //     transactor
    // );
    // // join_all(_handles).await;
    // for handle in handles {
    // if let Err(e) = handle.await {
    //     eprintln!("Task failed: {:?}", e);
    // }
}

// pub fn spawn_all_tasks(
//     // contract_map: Arc<ContractMap>,
//     transactor: Arc<RwLock<Transactor>>,
// )
// -> Vec<JoinHandle<()>>
// {
// //     vec![
// //         tokio::spawn(processor_task(contract_map, transactor)),
// //         // more tasks here
// //     ]
//     vec![
//         spawn(web(transactor.clone()))
//     ]
// }
