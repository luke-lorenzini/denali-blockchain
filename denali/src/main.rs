use std::{collections::HashMap, sync::Arc};

use denali::{
     messaging::{receiver_task, 
        // message_generator_task
    }, plugins::{
        plugin_task::plugin_scanner_task, 
        // Plugin,
        plugin_task::plugin_builder,
    }, processor_task, web::web_task::web_task, Transactor
};
use tokio::{
    join, 
    spawn,
    sync::{mpsc::{channel, 
        // Receiver
        }, 
        RwLock},
};

// use macros::HelloMacro;
// use denali::types::HelloMacro;

#[tokio::main]
async fn main() {
    let transactor = Arc::new(RwLock::new(Transactor::new()));
    let (tx, rx) = channel(100);
    let (tx_msg_queue, rx_msg_queue) = channel(100);
    let contract_map = Arc::new(RwLock::new(HashMap::new()));
    let (plugin_tx, plugin_rx) = channel(100);

    let path = "./plugins";

    // let vote_name = "vote";
    // let fake_name = "fake";
    // let bank_name = "bank";
    // let vote_path = "/home/luke/repos/denali/target/debug/libvote.so";
    // let fake_path = "/home/luke/repos/denali/target/debug/libfake.so";
    // let bank_path = "/home/luke/repos/denali/target/debug/libbank.so";
    // let _ = Plugin::stuff(vote_name, vote_path.as_ref(), contract_map.clone()).await;
    // let _ = Plugin::stuff(fake_name, fake_path.as_ref(), contract_map.clone()).await;
    // let _ = Plugin::stuff(bank_name, bank_path.as_ref(), contract_map.clone()).await;

    let plugin_scanner_task = spawn(plugin_scanner_task(path.as_ref(), plugin_tx));
    let plugger_builder_task = spawn(plugin_builder(contract_map. clone() ,plugin_rx));
    // let message_generator_task = spawn(message_generator_task(tx.clone()));
    let receiver_task = spawn(receiver_task(tx_msg_queue, rx));
    let processor_task = spawn(processor_task(contract_map.clone(), transactor.clone(), rx_msg_queue));
    let web_task = spawn(web_task(tx, transactor.clone(), contract_map.clone()));
    

    let _res = join!(
        // message_generator_task,
        processor_task,
        receiver_task,
        plugin_scanner_task,
        web_task,
        plugger_builder_task
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
