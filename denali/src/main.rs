use denali::{
    // process_transactions,
    Message,
    Transactor,
    // chain::Chain,
    // create_new_block,
    types::Thing,
};
use tokio::{
    spawn,
    time::{Duration, sleep},
};

#[tokio::main]
async fn main() {
    println!("Hello, denali");

    let mut transactor = Transactor::new();
    let mut transactions = Vec::new();

    // this comes from user input
    let program = "fake_program";
    let payload = r#"
    {
        "fake": 0
    }"#
    .into();
    // end user input
    match program {
        "fake_program" => {
            // Found a match with the vote program address
            let fake = FakeProgram;
            let message = Message {
                payload,
                program: fake,
            };
            transactions.push(message);
        }
        _ => {}
    }

    let _res = transactor.create_new_block(transactions);

    let _xxx = spawn(async move {
        println!("other thread");

        loop {
            sleep(Duration::from_millis(100)).await;
            println!("awoken");
        }
    });
}

#[derive(Clone)]
struct FakeProgram;

impl Thing for FakeProgram {
    fn run(&self, _payload: &str) -> serde_json::Result<()> {
        println!("run");
        Ok(())
    }

    fn verify(&self) -> serde_json::Result<bool> {
        println!("verify");
        Ok(true)
    }
}
