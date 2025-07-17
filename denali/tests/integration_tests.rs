use std::sync::Arc;

use async_trait::async_trait;
use denali::{
    messaging::Meta,
    processor::{Message, Processor},
    storage::State,
    types::{H256, Thing},
};
use semver::Version;
use serde::Deserialize;

#[tokio::test]
async fn test_modify_single_value() {
    let mut processor = Processor::new(false);
    let mut transactions = Vec::new();
    let fake = Box::new(FakeProgram) as Box<dyn Thing>;
    let payload = r#"
    {
        "fake": "1"
    }"#
    .into();
    let message = Message {
        payload,
        program: fake.clone_box(),
        tx_id: H256::dummy(),
        metadata: Meta {},
    };
    transactions.push(message);
    let res = processor.create_new_block(transactions).await;
    assert_eq!(res, true)
}

#[tokio::test]
async fn test_add_one_block() {
    let mut processor = Processor::new(false);
    let mut transactions = Vec::new();
    let fake = Box::new(FakeProgram) as Box<dyn Thing>;
    let payload = r#"
    {
        "fake": "1"
    }"#
    .into();
    let message = Message {
        payload,
        program: fake.clone_box(),
        tx_id: H256::dummy(),
        metadata: Meta {},
    };
    transactions.push(message);
    let res = processor.create_new_block(transactions).await;
    assert_eq!(res, true);
    let res = processor.get_height();
    let expected = 2;
    assert_eq!(res, expected)
}

#[tokio::test]
async fn test_add_multiple_blocks() {
    let mut processor = Processor::new(false);
    let mut transactions = vec![];
    let fake = Box::new(FakeProgram) as Box<dyn Thing>;

    let payload = r#"
    {
        "fake": "1"
    }"#
    .into();
    let message = Message {
        payload,
        program: fake.clone_box(),
        tx_id: H256::dummy(),
        metadata: Meta {},
    };
    transactions.push(message);
    let res = processor.create_new_block(transactions).await;
    let mut transactions = vec![];
    assert_eq!(res, true);

    let payload = r#"
    {
        "fake": "2"
    }"#
    .into();
    let message = Message {
        payload,
        program: fake.clone_box(),
        tx_id: H256::dummy(),
        metadata: Meta {},
    };
    transactions.push(message);
    let res = processor.create_new_block(transactions).await;
    let mut transactions = vec![];
    assert_eq!(res, true);

    let payload = r#"
    {
        "fake": "3"
    }"#
    .into();
    let message = Message {
        payload,
        program: fake.clone_box(),
        tx_id: H256::dummy(),
        metadata: Meta {},
    };
    transactions.push(message);
    let res = processor.create_new_block(transactions).await;
    assert_eq!(res, true);

    let res = processor.get_height();
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
    fn version(&self) -> Version {
        Version::parse("0.1.0").unwrap()
    }

    fn name(&self) -> &'static str {
        "fake"
    }

    async fn run(
        &self,
        payload: &str,
        state: Arc<State>,
    ) -> serde_json::Result<(bool, Vec<String>)> {
        println!("run");
        println!("{payload:?}");
        println!("{state:?}");

        let xxx: Payload = serde_json::from_slice(payload.as_bytes()).unwrap();
        println!("{xxx:?}");

        state.set_value("fake_program", &[0]);

        Ok((true, vec![]))
    }

    fn verify(&self) -> serde_json::Result<bool> {
        println!("verify");
        Ok(true)
    }
}
