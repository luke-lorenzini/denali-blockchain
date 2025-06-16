// use denali::{
// process_transactions,
// Message,
// Transactor,
// chain::Chain,
// create_new_block,
// types::Thing,
// storage::State,
// };
use tokio::{
    join, spawn,
    time::{Duration, sleep},
};

#[tokio::main]
async fn main() {
    println!("Hello, denali");

    // let mut transactor = Transactor::new();
    // let mut transactions = Vec::new();

    // this comes from user input
    // let program = "fake_program";
    // let payload = r#"
    // {
    //     "fake": 0
    // }"#
    // .into();
    // // end user input
    // match program {
    //     "fake_program" => {
    //         // Found a match with the vote program address
    //         let fake = FakeProgram;
    //         let message = Message {
    //             payload,
    //             program: fake,
    //         };
    //         transactions.push(message);
    //     }
    //     _ => {}
    // }

    // let _res = transactor.create_new_block(transactions);

    let xxx = spawn(async move {
        println!("other thread");

        loop {
            sleep(Duration::from_millis(1000)).await;
            println!("awoken");
        }
    });

    let _res = join!(xxx);
}

// #[derive(Clone)]
// struct FakeProgram;

// impl Thing for FakeProgram {
//     fn run(&self, payload: &str, state: &mut State) -> serde_json::Result<()> {
//         println!("run");

//         state.get_value("test");

//         Ok(())
//     }

//     fn verify(&self) -> serde_json::Result<bool> {
//         println!("verify");
//         Ok(true)
//     }
// }
