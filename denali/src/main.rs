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
    types::{RawTraitObject, Thing},
    // storage::State,
    // web::{chain_height, root}
};
use tokio::{
    join, spawn,
    sync::{Mutex, RwLock, mpsc::channel},
    time::{Duration, sleep},
};

const BATCH_SIZE: usize = 10;

fn stuff() -> HashMap<String, Box<(dyn Thing)>> {
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

        contract_map.insert(vote.into(), owned_plugin_box);
        let v = contract_map.get(vote).unwrap().verify();
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

        contract_map.insert(fake.into(), owned_plugin_box);
        let v = contract_map.get(fake).unwrap().verify();
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

        contract_map.insert(bank.into(), owned_plugin_box);
        let v = contract_map.get(bank).unwrap().verify();
        println!("{v:?}");
    }

    contract_map
}

#[tokio::main]
async fn main() {
    println!("Hello, denali");

    let contract_map = stuff();
    let contract_map = Arc::new(Mutex::new(contract_map));

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
                flag += 1;
                // A fake - working
                payload = r#"
                {
                    "fake": 0
                }"#;
                program = "fake";
            } else if flag == 1 {
                flag += 1;
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
                    "candidate": 1
                }"#;
                program = "vote";
            }

            tx.send((program, payload)).await.unwrap();
        }
    });

    let receiver_thread = spawn({
        async move {
            let mut transactions = Vec::new();

            while let Some(i) = rx.recv().await {
                transactions.push(i);
                if transactions.len() == BATCH_SIZE {
                    let batch = std::mem::take(&mut transactions);
                    tx_msg_queue.send(batch).await.unwrap();
                }
            }
        }
    });

    let processor_thread = spawn(async move {
        println!("notified");

        while let Some(messages) = rx_msg_queue.recv().await {
            unsafe {
                let name = messages.clone().last().unwrap().0;
                let _lib = match name {
                    "fake" => {
                        libloading::Library::new("/home/luke/repos/denali/target/debug/libfake.so")
                            .unwrap()
                    }
                    "vote" => {
                        libloading::Library::new("/home/luke/repos/denali/target/debug/libvote.so")
                            .unwrap()
                    }
                    "bank" => {
                        libloading::Library::new("/home/luke/repos/denali/target/debug/libbank.so")
                            .unwrap()
                    }
                    _ => todo!("Invalid name"),
                };
                println!("{name:?}");
                let payload: String = messages.clone().last().unwrap().1.into();
                let program = contract_map.lock().await;
                let program = program.get(name).unwrap();
                let message = Message {
                    program,
                    payload: payload.clone(),
                };
                let messages = vec![message];
                let _res = transactor.clone().write().await.create_new_block(messages);
            }
        }
    });

    // let web_thread = spawn(async {
    //     let app = Router::new()
    //     // `GET /` goes to `root`
    //     .route("/", get(root))
    //     .route("/chain_height", get(chain_height));

    //     let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    //     axum::serve(listener, app).await.unwrap();
    // });

    let _res = join!(receiver_thread, listener_thread, processor_thread);
}
