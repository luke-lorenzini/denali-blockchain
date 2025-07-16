use std::sync::Arc;

use async_trait::async_trait;
use denali::{storage::State, types::Thing};
use log::trace;
use macros::generate_create_thing;
use semver::Version;
use serde::Deserialize;
use serde_json::Result;

const CANDIDATES: u32 = 3;
#[generate_create_thing(args(CANDIDATES))]
#[derive(Clone, Debug)]
struct Vote {
    _votes: Vec<u64>,
}

#[async_trait]
impl Thing for Vote {
    fn name(&self) -> &'static str {
        "vote"
    }

    fn version(&self) -> Version {
        Version::parse("0.1.0").unwrap()
    }

    async fn run(&self, payload: &str, state: Arc<State>) -> Result<(bool, Vec<String>)> {
        trace!("vote run");
        let logs = vote_program(payload, state).await?;
        Ok((true, logs))
    }

    fn verify(&self) -> Result<bool> {
        trace!("vote verify");
        Ok(true)
    }
}

impl Vote {
    fn new(number_of_candidates: u32) -> Self {
        let votes = vec![0; number_of_candidates as usize];
        println!("Created new vote");
        Self { _votes: votes }
    }
}

async fn vote_program(payload: &str, state: Arc<State>) -> Result<Vec<String>> {
    #[derive(Debug, Deserialize)]
    struct Ballot {
        candidate: String,
    }

    let payload: Ballot = serde_json::from_str(payload)?;
    trace!("payload: {payload:?}");

    let current_count = state
        // .lock()
        // .await
        // .unwrap()
        .get_value(&payload.candidate);
    state
        // .lock()
        // .await
        // .unwrap()
        .set_value(&payload.candidate, current_count + 1);

    Ok(vec![])
}

#[cfg(test)]
mod test {
    use super::*;

    fn setup() -> (Vote, String) {
        let payload = r#"
        {
            "candidate": 0
        }"#
        .into();
        let number_of_candidates = 3;
        let vote = Vote::new(number_of_candidates);
        (vote, payload)
    }

    #[test]
    fn test_vote_program() {
        let (_, _payload) = setup();
        // let _res = vote_program(&payload).unwrap();
    }

    #[test]
    fn test_run() {
        // let (vote, payload) = setup();
        // let _res = vote.run(&payload).unwrap();
    }
}
