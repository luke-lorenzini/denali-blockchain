use std::{collections::HashMap, sync::Arc};

use clap::Parser;
use denali::{
    constants::PATH,
    messaging::receiver_task,
    plugins::plugin_task::{plugin_builder, plugin_scanner_task},
    processor::{Processor, processor_task::processor_task},
    quinn::{client::start_quinn_client, server::start_quinn_server},
    web::web_task::web_task,
};
use tokio::{
    join, spawn,
    sync::{RwLock, mpsc::channel},
};

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct Args {
    #[clap(short, long, default_value_t = false, action = clap::ArgAction::Set)]
    replica: bool,
}

#[tokio::main(flavor = "multi_thread")]
async fn main() {
    // #[cfg(feature = "console")]
    // console_subscriber::init();

    let args = Args::parse();
    let replica = args.replica;

    let channel_size = 100;
    let processor = Arc::new(RwLock::new(Processor::new(replica)));
    let (tx, rx) = channel(channel_size);
    let (tx_msg_queue, rx_msg_queue) = channel(channel_size);
    let contract_map = Arc::new(RwLock::new(HashMap::new()));
    let (plugin_tx, plugin_rx) = channel(channel_size);

    let handle = if !replica {
        spawn(async {
            let _res = start_quinn_server().await;
        })
    } else {
        spawn(async {
            let _res = start_quinn_client().await;
        })
    };

    let plugin_scanner_task = spawn(plugin_scanner_task(PATH.as_ref(), plugin_tx));
    let plugin_builder_task = spawn(plugin_builder(contract_map.clone(), plugin_rx));
    let receiver_task = spawn(receiver_task(tx_msg_queue, rx));
    let processor_task = spawn(processor_task(
        contract_map.clone(),
        processor.clone(),
        rx_msg_queue,
    ));
    let web_task = spawn(web_task(
        tx,
        processor.clone(),
        contract_map.clone(),
        replica,
    ));

    let _res = join!(
        processor_task,
        receiver_task,
        plugin_scanner_task,
        web_task,
        plugin_builder_task,
        handle
    );

    // let handles = spawn_all_tasks(
    //     // contract_map,
    //     processor
    // );
    // // join_all(_handles).await;
    // for handle in handles {
    // if let Err(e) = handle.await {
    //     eprintln!("Task failed: {:?}", e);
    // }
}

// pub fn spawn_all_tasks(
//     // contract_map: Arc<ContractMap>,
//     processor: Arc<RwLock<Processor>>,
// )
// -> Vec<JoinHandle<()>>
// {
// //     vec![
// //         tokio::spawn(processor_task(contract_map, processor)),
// //         // more tasks here
// //     ]
//     vec![
//         spawn(web(processor.clone()))
//     ]
// }
