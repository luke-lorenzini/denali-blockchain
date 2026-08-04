use std::sync::Arc;

use anyhow::{Result, anyhow};
use async_trait::async_trait;
use borsh::{BorshDeserialize, BorshSerialize, from_slice, to_vec};
use denali::{storage::State, types::Thing};
use log::trace;
use macros::generate_create_thing;
use semver::Version;
use serde::Deserialize;

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

    fn version(&self) -> Result<Version> {
        Version::parse("0.1.0").map_err(|e| anyhow!("Semver failure: {e}"))
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

#[derive(BorshDeserialize, BorshSerialize, Debug)]
struct ContractData {
    candidates: Vec<u32>,
}

struct _ContractUsers {
    users: Vec<String>,
}

async fn vote_program(payload: &str, state: Arc<State>) -> Result<Vec<String>> {
    #[derive(Debug, Deserialize)]
    struct Ballot {
        candidate: String,
    }

    let payload: Ballot = serde_json::from_str(payload)?;
    trace!("payload: {payload:?}");

    let locked_state = state;
    let current_state = match locked_state.get_value("vote") {
        Some(value) => {
            let mut current_state = from_slice::<ContractData>(value.as_ref())?;
            let idx = if payload.candidate == "0" {
                0
            } else if payload.candidate == "1" {
                1
            } else {
                2
            };
            current_state.candidates[idx] += 1;
            current_state
        }
        None => ContractData {
            candidates: vec![0; CANDIDATES as usize],
        },
    };
    trace!("current_state: {current_state:?}");
    let encoded_state = to_vec(&current_state)?;
    locked_state.set_value("vote", &encoded_state);

    Ok(vec![])
}

#[cfg(test)]
mod test {
    use super::*;

    fn _setup() -> (Vote, String) {
        let payload = r#"
        {
            "candidate": "0"
        }"#
        .into();
        let number_of_candidates = 3;
        let vote = Vote::new(number_of_candidates);
        (vote, payload)
    }

    #[tokio::test]
    async fn test_vote_program() {
        let payload = r#"
        {
            "candidate": "0"
        }"#;
        let state = Arc::new(State::new());
        let _res = vote_program(payload, state.clone()).await;
        let _res = vote_program(payload, state.clone()).await;
        let payload = r#"
        {
            "candidate": "1"
        }"#;
        let _res = vote_program(payload, state.clone()).await;
        let _res = vote_program(payload, state.clone()).await;
        let _res = vote_program(payload, state.clone()).await;
        let _res = vote_program(payload, state).await;
    }

    #[test]
    fn test_run() {}
}
