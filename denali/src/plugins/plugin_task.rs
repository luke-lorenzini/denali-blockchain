use std::{collections::HashMap, path::Path, sync::Arc};

use futures::{
    SinkExt, StreamExt,
    channel::mpsc::{Receiver, channel},
};
use glob::glob;
use notify::{
    Config, Event, EventKind, RecommendedWatcher, RecursiveMode, Watcher,
    event::{ModifyKind, RenameMode},
};
use tokio::sync::{
    RwLock,
    mpsc::{Receiver as TokioReceive, Sender},
};

use crate::plugins::Plugin;

pub async fn plugin_builder(
    contract_map: Arc<RwLock<HashMap<String, Plugin>>>,
    mut plugin_rx: TokioReceive<(&str, Option<Plugin>)>,
) {
    while let Some(v) = plugin_rx.recv().await {
        if v.1.is_some() {
            contract_map
                .write()
                .await
                .insert(v.0.into(), v.1.expect("Already checked"));
        } else {
            contract_map.write().await.remove(v.0).unwrap();
        }
    }
}

pub async fn plugin_scanner_task(path: &Path, plugin_tx: Sender<(&str, Option<Plugin>)>) {
    search_for_existing_plugins(path, plugin_tx.clone()).await;

    // futures::executor::block_on(async {
    if let Err(e) = async_watch(path, plugin_tx).await {
        println!("error: {e:?}");
    }
    // });
    // });
}

fn async_watcher() -> notify::Result<(RecommendedWatcher, Receiver<notify::Result<Event>>)> {
    let (mut tx, rx) = channel(1);

    let watcher = RecommendedWatcher::new(
        move |res| {
            futures::executor::block_on(async {
                tx.send(res).await.unwrap();
            })
        },
        Config::default(),
    )?;

    Ok((watcher, rx))
}

async fn async_watch<P: AsRef<Path>>(
    path: P,
    plugin_tx: Sender<(&str, Option<Plugin>)>,
) -> notify::Result<()> {
    let (mut watcher, mut rx) = async_watcher()?;

    watcher.watch(path.as_ref(), RecursiveMode::NonRecursive)?;
    while let Some(res) = rx.next().await {
        // todo fix this nesting
        match res {
            Ok(event) => {
                for path in event.paths {
                    if let EventKind::Modify(ModifyKind::Name(v)) = event.kind {
                        match v {
                            RenameMode::To => {
                                let p = Plugin::build(path).await;
                                let _x = plugin_tx.send((p.0, Some(p.1))).await;
                            }
                            RenameMode::From => {
                                // todo this needs fixing
                                // let _x = plugin_tx.send(("vote", None)).await;
                            }
                            _ => (),
                        }
                    }
                }
            }
            Err(e) => println!("watch error: {e:?}"),
        }
    }

    Ok(())
}

async fn search_for_existing_plugins(path: &Path, plugin_tx: Sender<(&str, Option<Plugin>)>) {
    let path = path.join("*.so");
    let path = path.to_str().unwrap();

    for entry in glob(path).expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => {
                println!("found {:?}", path.display());
                let xxx = path;
                let p = Plugin::build(xxx).await;
                let _x = plugin_tx.send((p.0, Some(p.1))).await;
            }
            Err(e) => println!("{:?}", e),
        }
    }
}
