use std::{
    // collections::HashMap, ffi::c_void, 
    path::Path, 
    // sync::{ Arc}
};

// use libloading::Library;
// use tokio::{sync::{ RwLock}};

use futures::{
    channel::mpsc::{channel, Receiver},
    SinkExt, StreamExt,
};
use notify::{Config, Event, RecommendedWatcher, RecursiveMode, Watcher};

// use crate::Thing;

pub async fn plugin_scanner_task() {
    // let scanner_thread = spawn(async {
        // let path = std::env::args()
        //     .nth(1)
        //     .expect("Arg 1 needs to be a path");
        let path = "./plugins";
        println!("watching {path:?}");
        
        // futures::executor::block_on(async {
            if let Err(e) = async_watch(path).await {
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

async fn async_watch<P: AsRef<Path>>(path: P) -> notify::Result<()> {
    let (mut watcher, mut rx) = async_watcher()?;

    watcher.watch(path.as_ref(), RecursiveMode::Recursive)?;

    while let Some(res) = rx.next().await {
        match res {
            Ok(event) => println!("changed: {event:?}"),
            Err(e) => println!("watch error: {e:?}"),
        }
    }

    Ok(())
}
