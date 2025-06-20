use denali::{Message, Transactor, storage::State, types::Thing};
use serde::Deserialize;

#[test]
fn test_modify_single_value() {
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
        program: &fake,
    };
    transactions.push(message);
    let res = transactor.create_new_block(transactions);
    assert_eq!(res, true)
}

#[test]
fn test_add_one_block() {
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
        program: &fake,
    };
    transactions.push(message);
    let res = transactor.create_new_block(transactions);
    assert_eq!(res, true);
    let res = transactor.get_chain_height();
    let expected = 2;
    assert_eq!(res, expected)
}

#[test]
fn test_add_multiple_blocks() {
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
        program: &fake,
    };
    transactions.push(message);
    let res = transactor.create_new_block(transactions.clone());
    transactions.clear();
    assert_eq!(res, true);

    let payload = r#"
    {
        "fake": "2"
    }"#
    .into();
    let message = Message {
        payload,
        program: &fake,
    };
    transactions.push(message);
    let res = transactor.create_new_block(transactions.clone());
    transactions.clear();
    assert_eq!(res, true);

    let payload = r#"
    {
        "fake": "3"
    }"#
    .into();
    let message = Message {
        payload,
        program: &fake,
    };
    transactions.push(message);
    let res = transactor.create_new_block(transactions.clone());
    transactions.clear();
    assert_eq!(res, true);

    let res = transactor.get_chain_height();
    let expected = 4;
    assert_eq!(res, expected)
}

#[derive(Debug, Deserialize)]
struct Payload {
    fake: String,
}

#[derive(Clone)]
struct FakeProgram;

impl Thing for FakeProgram {
    fn name(&self) -> &'static str {
        "fake"
    }

    fn run(&self, payload: &str, state: &mut State) -> serde_json::Result<()> {
        println!("run");
        println!("{payload:?}");
        println!("{state:?}");

        let xxx: Payload = serde_json::from_slice(payload.as_bytes()).unwrap();
        println!("{xxx:?}");

        state.set_value("fake_program", 0);
        println!("{state:?}");

        Ok(())
    }

    fn verify(&self) -> serde_json::Result<bool> {
        println!("verify");
        Ok(true)
    }
}
