use std::sync::Arc;

use async_trait::async_trait;
use denali::{storage::State, types::Thing};
use macros::generate_create_thing;
use semver::Version;
use serde_json::Result;

#[generate_create_thing(args())]
#[derive(Clone)]
struct HelloWorld;

#[async_trait]
impl Thing for HelloWorld {
    fn name(&self) -> &'static str {
        "hello-world"
    }

    fn version(&self) -> Version {
        Version::parse("0.1.0").unwrap()
    }

    async fn run(&self, payload: &str, state: Arc<State>) -> Result<(bool, Vec<String>)> {
        println!("running hello-world");
        let result = program(payload, state).await?;
        Ok(result)
    }

    fn verify(&self) -> Result<bool> {
        println!("verifying hello-world");
        Ok(true)
    }
}

impl HelloWorld {
    fn new() -> Self {
        println!("Hello, world!");
        Self
    }
}

async fn program(_payload: &str, _state: Arc<State>) -> Result<(bool, Vec<String>)> {
    // need something like this in the "logs" (transaction metadata)
    // Program <YourProgramID> invoke [1]
    // Program log: Hello, Solana!
    // Program log: This is a value: 42
    // Program <YourProgramID> success

    let mut res = vec!["Hello, Denali".into()];

    // #[derive(Debug, Deserialize)]
    // struct Ballot {
    //     candidate: String,
    // }

    // let payload: Ballot = serde_json::from_str(payload)?;
    // println!("payload: {payload:?}");

    // let current_count = state
    //     .lock()
    //     .await
    //     .get_value(&payload.candidate);
    // println!("{current_count:?}");
    // state
    //     .lock()
    //     .await
    //     .set_value(&payload.candidate, current_count + 1);

    res.push("Program complete".into());

    Ok((true, res))
}
