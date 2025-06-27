use std::{
    collections::HashMap, ffi::c_void, path::{Path, PathBuf}, sync::Arc
};

use libloading::Library;
use tokio::{sync::{ RwLock}};

// use futures::{
//     channel::mpsc::{channel, Receiver},
//     SinkExt, StreamExt,
// };
// use notify::{Config, Event, RecommendedWatcher, RecursiveMode, Watcher};

use crate::Thing;

pub mod plugin_task;

type Contract = unsafe fn() -> *mut c_void;

// pub fn async_watcher() -> notify::Result<(RecommendedWatcher, Receiver<notify::Result<Event>>)> {
//     let (mut tx, rx) = channel(1);

//     let watcher = RecommendedWatcher::new(
//         move |res| {
//             futures::executor::block_on(async {
//                 tx.send(res).await.unwrap();
//             })
//         },
//         Config::default(),
//     )?;

//     Ok((watcher, rx))
// }

// pub async fn async_watch<P: AsRef<Path>>(path: P) -> notify::Result<()> {
//     let (mut watcher, mut rx) = async_watcher()?;

//     let _x = watcher.watch(path.as_ref(), RecursiveMode::Recursive)?;

//     while let Some(res) = rx.next().await {
//         match res {
//             Ok(event) => println!("changed: {event:?}"),
//             Err(e) => println!("watch error: {e:?}"),
//         }
//     }

//     Ok(())
// }

#[repr(C)]
pub struct RawTraitObject {
    pub data_ptr: *mut c_void,
    pub vtable_ptr: *mut c_void,
}

#[derive(Debug)]
pub struct Plugin {
    _library: Library,
    pub thing: Box<dyn Thing>,
}

impl Plugin {
    #[must_use]
    pub fn new(library: Library, program: Box<dyn Thing>) -> Self {
        Plugin {
            _library: library,
            thing: program,
        }
    }

    async fn _stuff(name: &str, path: &Path, contract_map: Arc<RwLock<HashMap<String, Plugin>>>) {
            unsafe {
                let lib = libloading::Library::new(path)
                    .unwrap();
                let func: libloading::Symbol<Contract> = lib.get(b"create_thing").unwrap();
                let xxx = func();

                let boxed_raw_trait_object = Box::from_raw(xxx.cast::<RawTraitObject>());
                let raw_trait_object = *boxed_raw_trait_object;
                let raw_fat_ptr: *mut dyn Thing = std::mem::transmute(raw_trait_object);
                let owned_plugin_box: Box<dyn Thing> = Box::from_raw(raw_fat_ptr);

                let plugin = Plugin::new(lib, owned_plugin_box);
                contract_map.write().await.insert(name.into(), plugin);
            }

            let v = contract_map.read().await.get(name).unwrap().thing.verify();
            println!("Result of verification for {name:?}: {v:?}");
    }

    pub async fn build(
        // path: &Path
        path: PathBuf
    ) -> (&'static str, Plugin) {
        unsafe {
                let lib = libloading::Library::new(path)
                    .unwrap();
                let func: libloading::Symbol<Contract> = lib.get(b"create_thing").unwrap();
                let xxx = func();

                let boxed_raw_trait_object = Box::from_raw(xxx.cast::<RawTraitObject>());
                let raw_trait_object = *boxed_raw_trait_object;
                let raw_fat_ptr: *mut dyn Thing = std::mem::transmute(raw_trait_object);
                let owned_plugin_box: Box<dyn Thing> = Box::from_raw(raw_fat_ptr);

                let name = owned_plugin_box.name();

                (name, Plugin::new(lib, owned_plugin_box)            )
            }
    }
}
