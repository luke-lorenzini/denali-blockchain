use std::ffi::c_void;
use std::{collections::HashMap, sync::Arc};

// use axum::{
//     routing::{
//         get,
//         // post
//     },
//     // http::StatusCode,
//     // Json,
//     Router,
// };
use denali::{
    Message,
    Transactor,
    types::{Plugin, RawTraitObject, Thing},
    // storage::State,
    // web::{chain_height, root}
};
use tokio::{
    join, spawn,
    sync::{RwLock, mpsc::channel},
    time::{Duration, sleep},
};

const BATCH_SIZE: usize = 10;

fn stuff() -> HashMap<String, Plugin> {
    let mut contract_map = HashMap::new();
    // load vote
    unsafe {
        // type Contract = unsafe extern "C" fn() -> Box<dyn Thing>;
        type Contract = unsafe fn() -> *mut c_void;
        let vote = "vote";
        let lib =
            libloading::Library::new("/home/luke/repos/denali/target/debug/libvote.so").unwrap();
        let func: libloading::Symbol<Contract> = lib.get(b"create_thing").unwrap();
        let xxx = func();
        // let xxx = Box::from_raw(xxx as *mut dyn Thing);

        let boxed_raw_trait_object = Box::from_raw(xxx as *mut RawTraitObject);
        let raw_trait_object = *boxed_raw_trait_object;
        let raw_fat_ptr: *mut dyn Thing = std::mem::transmute(raw_trait_object);
        let owned_plugin_box: Box<dyn Thing> = Box::from_raw(raw_fat_ptr);

        let plugin = Plugin::new(lib, owned_plugin_box);

        contract_map.insert(vote.into(), plugin);
        let v = contract_map.get(vote).unwrap().thing.verify();
        println!("Result of verification for vote: {v:?}");
    }

    // load fake
    unsafe {
        // type Contract = unsafe extern "C" fn() -> Box<dyn Thing>;
        type Contract = unsafe fn() -> *mut c_void;
        let fake = "fake";
        let lib =
            libloading::Library::new("/home/luke/repos/denali/target/debug/libfake.so").unwrap();
        let func: libloading::Symbol<Contract> = lib.get(b"create_thing").unwrap();
        let xxx = func();
        // let xxx = Box::from_raw(xxx as *mut dyn Thing);

        let boxed_raw_trait_object = Box::from_raw(xxx as *mut RawTraitObject);
        let raw_trait_object = *boxed_raw_trait_object;
        let raw_fat_ptr: *mut dyn Thing = std::mem::transmute(raw_trait_object);
        let owned_plugin_box: Box<dyn Thing> = Box::from_raw(raw_fat_ptr);

        let plugin = Plugin::new(lib, owned_plugin_box);

        contract_map.insert(fake.into(), plugin);
        let v = contract_map.get(fake).unwrap().thing.verify();
        println!("{v:?}");
    }

    // load bank
    unsafe {
        // type Contract = unsafe extern "C" fn() -> Box<dyn Thing>;
        type Contract = unsafe fn() -> *mut c_void;
        let bank = "bank";
        let lib =
            libloading::Library::new("/home/luke/repos/denali/target/debug/libbank.so").unwrap();
        let func: libloading::Symbol<Contract> = lib.get(b"create_thing").unwrap();
        let xxx = func();
        // let xxx = Box::from_raw(xxx as *mut dyn Thing);

        let boxed_raw_trait_object = Box::from_raw(xxx as *mut RawTraitObject);
        let raw_trait_object = *boxed_raw_trait_object;
        let raw_fat_ptr: *mut dyn Thing = std::mem::transmute(raw_trait_object);
        let owned_plugin_box: Box<dyn Thing> = Box::from_raw(raw_fat_ptr);

        let plugin = Plugin::new(lib, owned_plugin_box);

        contract_map.insert(bank.into(), plugin);
        let v = contract_map.get(bank).unwrap().thing.verify();
        println!("{v:?}");
    }

    contract_map
}

#[tokio::main]
async fn main() {
    println!("Hello, denali");

    let contract_map = stuff();
    let contract_map = Arc::new(contract_map);

    let transactor = Arc::new(RwLock::new(Transactor::new()));
    let (tx, mut rx) = channel(100);
    let (tx_msg_queue, mut rx_msg_queue) = channel(100);

    let mut flag = 0;
    let listener_thread = spawn(async move {
        loop {
            sleep(Duration::from_millis(100)).await;

            let payload;
            let program;
            if flag == 0 {
                flag = 1;
                // A fake - working
                payload = r#"
                {
                    "fake": 0
                }"#;
                program = "fake";
            } else if flag == 1 {
                flag = 2;
                // A bank
                payload = r#"
                {
                    "payer": 0,
                    "payee": 1,
                    "amount": 10.0
                }"#;
                program = "bank";
            } else {
                flag = 0;
                // A vote - working
                payload = r#"
                {
                    "candidate": "candidate1"
                }"#;
                program = "vote";
            }

            tx.send((program, payload)).await.unwrap();
        }
    });

    let receiver_thread = spawn(async move {
        let mut transactions = Vec::new();

        while let Some(i) = rx.recv().await {
            transactions.push(i);
            if transactions.len() == BATCH_SIZE {
                let batch = std::mem::take(&mut transactions);
                tx_msg_queue.send(batch).await.unwrap();
            }
        }
    });

    let processor_thread = spawn(async move {
        println!("notified");

        while let Some(messages) = rx_msg_queue.recv().await {
            for message in messages {
                // println!("transactions: {message:?}");
                let name = message.0;
                // println!("{name:?}");
                let payload: String = message.1.into();
                let program = contract_map.clone();
                let program = program.get(name).unwrap().thing.as_ref();
                let message = Message {
                    program,
                    payload: payload.clone(),
                };
                let messages = vec![message];
                let _res = transactor
                    .clone()
                    .write()
                    .await
                    .create_new_block(messages)
                    .await;
            }
        }
    });

    // let web_thread = spawn(async move {
    //     // let transactor = Arc::new(RwLock::new(Transactor::new()));
    //     let app = Router::new()
    //     // `GET /` goes to `root`
    //     .route("/", get(root))
    //     // .route("/chain_height", get(chain_height))
    //     .with_state(transactor.clone());

    //     let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    //     axum::serve(listener, app).await.unwrap();
    // });

    let _res = join!(receiver_thread, listener_thread, processor_thread);
}
