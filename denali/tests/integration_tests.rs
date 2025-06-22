use std::sync::{Arc, Mutex};

use async_trait::async_trait;
use denali::{
    Message, Transactor,
    storage::State,
    types::{H256, Thing},
};
use serde::Deserialize;

#[tokio::test]
async fn test_modify_single_value() {
    let mut transactor = Transactor::new();
    let mut transactions = Vec::new();
    let fake = Box::new(FakeProgram) as Box<dyn Thing>;
    let payload = r#"
    {
        "fake": "1"
    }"#
    .into();
    let message = Message {
        payload,
        program: fake.as_ref(),
    };
    transactions.push(message);
    let res = transactor.create_new_block(transactions).await;
    assert_eq!(res, true)
}

#[tokio::test]
async fn test_add_one_block() {
    let mut transactor = Transactor::new();
    let mut transactions = Vec::new();
    let fake = Box::new(FakeProgram) as Box<dyn Thing>;
    let payload = r#"
    {
        "fake": "1"
    }"#
    .into();
    let message = Message {
        payload,
        program: fake.as_ref(),
    };
    transactions.push(message);
    let res = transactor.create_new_block(transactions).await;
    assert_eq!(res, true);
    let res = transactor.get_chain_height();
    let expected = 2;
    assert_eq!(res, expected)
}

#[tokio::test]
async fn test_add_multiple_blocks() {
    let mut transactor = Transactor::new();
    let mut transactions = Vec::new();
    let fake = Box::new(FakeProgram) as Box<dyn Thing>;

    let payload = r#"
    {
        "fake": "1"
    }"#
    .into();
    let message = Message {
        payload,
        program: fake.as_ref(),
    };
    transactions.push(message);
    let res = transactor.create_new_block(transactions.clone()).await;
    transactions.clear();
    assert_eq!(res, true);

    let payload = r#"
    {
        "fake": "2"
    }"#
    .into();
    let message = Message {
        payload,
        program: fake.as_ref(),
    };
    transactions.push(message);
    let res = transactor.create_new_block(transactions.clone()).await;
    transactions.clear();
    assert_eq!(res, true);

    let payload = r#"
    {
        "fake": "3"
    }"#
    .into();
    let message = Message {
        payload,
        program: fake.as_ref(),
    };
    transactions.push(message);
    let res = transactor.create_new_block(transactions.clone()).await;
    transactions.clear();
    assert_eq!(res, true);

    let res = transactor.get_chain_height();
    let expected = 4;
    assert_eq!(res, expected)
}

#[derive(Debug, Deserialize)]
struct Payload {
    #[allow(dead_code)]
    fake: String,
}

#[derive(Clone)]
struct FakeProgram;

#[async_trait]
impl Thing for FakeProgram {
    fn name(&self) -> &'static str {
        "fake"
    }

    async fn run(&self, payload: &str, state: Arc<Mutex<State>>) -> serde_json::Result<H256> {
        println!("run");
        println!("{payload:?}");
        println!("{state:?}");

        let xxx: Payload = serde_json::from_slice(payload.as_bytes()).unwrap();
        println!("{xxx:?}");

        state.lock().unwrap().set_value("fake_program", 0);
        println!("{state:?}");

        Ok(H256::default())
    }

    fn verify(&self) -> serde_json::Result<bool> {
        println!("verify");
        Ok(true)
    }
}
