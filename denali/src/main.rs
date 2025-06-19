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
    types::Thing,
    // storage::State,
    // web::{chain_height, root}
};
use tokio::{
    join, spawn,
    sync::{Mutex, RwLock, mpsc::channel},
    time::{Duration, sleep},
};

const BATCH_SIZE: usize = 10;

type Contract = unsafe extern "C" fn() -> Box<dyn Thing>;
// type Contract = unsafe fn() -> *mut c_void;
// type Contract = unsafe fn() -> *mut dyn Thing;

async fn stuff(contract_map: Arc<Mutex<HashMap<&'static str, Box<dyn Thing + 'static>>>>) {
    // load vote
    let vote = "vote";
    unsafe {
        let lib =
            libloading::Library::new("/home/luke/repos/denali/target/debug/libvote.so").unwrap();
        let func: libloading::Symbol<Contract> = lib.get(b"create_thing").unwrap();
        let xxx = func();
        // let xxx = Box::from_raw(xxx as *mut dyn Thing);

        contract_map.lock().await.insert(vote, xxx);
        let v = contract_map.lock().await.get(vote).unwrap().verify();
        println!("{v:?}");
    }

    // load fake
    let fake = "fake";
    unsafe {
        let lib =
            libloading::Library::new("/home/luke/repos/denali/target/debug/libfake.so").unwrap();
        let func: libloading::Symbol<Contract> = lib.get(b"create_thing").unwrap();
        let xxx = func();
        // let xxx = Box::from_raw(xxx as *mut dyn Thing);

        contract_map.lock().await.insert(fake, xxx);
        let v = contract_map.lock().await.get(fake).unwrap().verify();
        println!("{v:?}");
    }

    // load bank
    let bank = "bank";
    unsafe {
        let lib =
            libloading::Library::new("/home/luke/repos/denali/target/debug/libbank.so").unwrap();
        let func: libloading::Symbol<Contract> = lib.get(b"create_thing").unwrap();
        let xxx = func();
        // let xxx = Box::from_raw(xxx as *mut dyn Thing);

        contract_map.lock().await.insert(bank, xxx);
        let v = contract_map.lock().await.get(bank).unwrap().verify();
        println!("{v:?}");
    }
}

#[tokio::main]
async fn main() {
    println!("Hello, denali");

    let contract_map = Arc::new(Mutex::new(HashMap::new()));

    stuff(contract_map.clone()).await;

    let transactor = Arc::new(RwLock::new(Transactor::new()));
    let (tx, mut rx) = channel(100);
    let (tx_msg_queue, mut rx_msg_queue) = channel(100);

    let listener_thread = spawn(async move {
        loop {
            sleep(Duration::from_millis(100)).await;

            // // A fake
            // let payload = r#"
            // {
            //     "fake": 0
            // }"#;
            // let program = "fake";

            // A vote
            let payload = r#"
            {
                "candidate": 1
            }"#;
            let program = "vote";

            // A bank
            // let payload = r#"
            // {
            //     "payer": 0,
            //     "payee": 1,
            //     "amount": 10,
            // }"#;
            // let program = "bank";

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
            let name = messages.clone().last().unwrap().0;
            println!("{name:?}");
            let payload: String = messages.clone().last().clone().unwrap().1.into();
            let program = contract_map.lock().await;
            let program = program.get(name).unwrap();
            let message = Message {
                program,
                payload: payload.clone(),
            };
            let messages = vec![message];
            // let _res = transactor.clone().write().await.create_new_block(messages);

            // weird thing that needs to live in unsafe to prevent seg fault
            unsafe {
                let _lib =
                    libloading::Library::new("/home/luke/repos/denali/target/debug/libvote.so")
                        .unwrap();
                // let func: libloading::Symbol<Contract> = lib.get(b"create_thing").unwrap();
                // let xxx = func();
                // let message = Message {
                //     program: &xxx,
                //     payload: payload.clone()
                // };
                // let messages = vec![message];
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
