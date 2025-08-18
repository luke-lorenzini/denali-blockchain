use std::{collections::HashMap, sync::Arc};

use anyhow::{Result, bail};
use clap::Parser;
use denali::{
    constants::PATH,
    messaging::receiver_task,
    plugins::plugin_task::{plugin_builder, plugin_scanner_task},
    processor::{
        Processor,
        processor_task::{processor_receive_batch_txs, processor_task},
    },
    quinn::{Roles, client::start_quinn_client, server::start_quinn_server},
    web::web_task::web_task,
};
use futures::future::join_all;
use tokio::{
    spawn,
    sync::{Notify, RwLock, mpsc::channel},
};

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct Args {
    #[clap(short, long, default_value_t = 2, action = clap::ArgAction::Set)]
    role: u32,

    #[clap(short, long, default_value_t = 3000, action = clap::ArgAction::Set)]
    web_port_number: u16,

    #[clap(short, long, default_value_t = 4434, action = clap::ArgAction::Set)]
    quic_port_number: u16,

    #[clap(short, long, default_value_t = false, action = clap::ArgAction::Set)]
    validator_sync: bool,
}

#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<()> {
    // #[cfg(feature = "console")]
    // console_subscriber::init();

    let args = Args::parse();
    let role = match args.role {
        0 => Roles::Receiver,
        1 => Roles::Validator,
        2 => Roles::Archiver,
        _ => bail!("Invalid type specified"),
    };
    let quic_port_number = args.quic_port_number;
    let web_port_number = args.web_port_number;
    let validator_sync = args.validator_sync;
    let archiver = role == Roles::Archiver;

    let notify = Arc::new(Notify::new());
    let channel_size = 100;
    let processor = Arc::new(RwLock::new(Processor::new(archiver, validator_sync).await?));
    let (tx, rx) = channel(channel_size);
    let (tx_msg_queue, rx_msg_queue) = channel(channel_size);
    let (tx_batch_queue, rx_batch_queue) = channel(channel_size);
    let contract_map = Arc::new(RwLock::new(HashMap::new()));
    let (plugin_tx, plugin_rx) = channel(channel_size);
    let mut handles = vec![];

    match role {
        Roles::Receiver => {
            let server_task = spawn({
                let processor = processor.clone();
                let notify = notify.clone();
                async move { start_quinn_server(processor.clone(), notify.clone(), role).await }
            });
            handles.push(server_task);
            let plugin_scanner_task = spawn(plugin_scanner_task(PATH.as_ref(), plugin_tx));
            handles.push(plugin_scanner_task);
            let plugin_builder_task = spawn(plugin_builder(contract_map.clone(), plugin_rx));
            handles.push(plugin_builder_task);
            let receiver_task = spawn(receiver_task(
                // tx_msg_queue,
                tx_batch_queue,
                rx,
            ));
            handles.push(receiver_task);
            // let processor_task = spawn(processor_task(
            //     contract_map.clone(),
            //     processor.clone(),
            //     rx_msg_queue,
            //     Some(notify.clone()),
            // ));
            // handles.push(processor_task);
            let processor_batch_task = spawn(processor_receive_batch_txs(
                // contract_map.clone(),
                processor.clone(),
                rx_batch_queue,
                notify.clone(),
            ));
            handles.push(processor_batch_task);
            let web_task = spawn(web_task(
                tx,
                processor.clone(),
                contract_map.clone(),
                archiver,
                web_port_number,
            ));
            handles.push(web_task);
        }
        Roles::Validator => {
            let client_task = spawn({
                let processor = processor.clone();
                // let notify = notify.clone();
                async move {
                    start_quinn_client(processor.clone(), quic_port_number, role, tx_msg_queue)
                        .await
                }
            });
            handles.push(client_task);
            let plugin_scanner_task = spawn(plugin_scanner_task(PATH.as_ref(), plugin_tx));
            handles.push(plugin_scanner_task);
            let plugin_builder_task = spawn(plugin_builder(contract_map.clone(), plugin_rx));
            handles.push(plugin_builder_task);
            // let receiver_task = spawn(receiver_task(tx_msg_queue, rx));
            // handles.push(receiver_task);
            let processor_task = spawn(processor_task(
                contract_map.clone(),
                processor.clone(),
                rx_msg_queue,
                Some(notify.clone()),
            ));
            handles.push(processor_task);
            let web_task = spawn(web_task(
                tx,
                processor.clone(),
                contract_map.clone(),
                archiver,
                web_port_number,
            ));
            handles.push(web_task);
        }
        Roles::Archiver => {
            let client_task = spawn({
                let processor = processor.clone();

                async move {
                    start_quinn_client(processor.clone(), quic_port_number, role, tx_msg_queue)
                        .await
                }
            });
            handles.push(client_task);
            let plugin_scanner_task = spawn(plugin_scanner_task(PATH.as_ref(), plugin_tx));
            handles.push(plugin_scanner_task);
            // let receiver_task = spawn(receiver_task(tx_msg_queue, rx));
            // handles.push(receiver_task);
            // let processor_task = spawn(processor_task(
            //     contract_map.clone(),
            //     processor.clone(),
            //     rx_msg_queue,
            //     Some(notify.clone()),
            // ));
            // handles.push(processor_task);
            let web_task = spawn(web_task(
                tx,
                processor.clone(),
                contract_map.clone(),
                archiver,
                web_port_number,
            ));
            handles.push(web_task);
        }
    }

    let results = join_all(handles).await;
    for (i, result) in results.into_iter().enumerate() {
        match result {
            Ok(_) => println!("Task {i} completed successfully."),
            Err(e) => println!("Task {i} failed to join: {e:?}"),
        }
    }

    Ok(())
}
