use std::{collections::HashMap, sync::Arc};

use denali::{
    Transactor,
    constants::PATH,
    messaging::receiver_task,
    plugins::plugin_task::{plugin_builder, plugin_scanner_task},
    processor_task,
    web::web_task::web_task,
};
use tokio::{
    join, spawn,
    sync::{RwLock, mpsc::channel},
};

#[tokio::main(flavor = "multi_thread")]
async fn main() {
    console_subscriber::init();

    let transactor = Arc::new(RwLock::new(Transactor::new()));
    let (tx, rx) = channel(100);
    let (tx_msg_queue, rx_msg_queue) = channel(100);
    let contract_map = Arc::new(RwLock::new(HashMap::new()));
    let (plugin_tx, plugin_rx) = channel(100);

    let plugin_scanner_task = spawn(plugin_scanner_task(PATH.as_ref(), plugin_tx));
    let plugin_builder_task = spawn(plugin_builder(contract_map.clone(), plugin_rx));
    // let message_generator_task = spawn(message_generator_task(tx.clone()));
    let receiver_task = spawn(receiver_task(tx_msg_queue, rx));
    let processor_task = spawn(processor_task(
        contract_map.clone(),
        transactor.clone(),
        rx_msg_queue,
    ));
    let web_task = spawn(web_task(tx, transactor.clone(), contract_map.clone()));

    let _res = join!(
        // message_generator_task,
        processor_task,
        receiver_task,
        plugin_scanner_task,
        web_task,
        plugin_builder_task
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
