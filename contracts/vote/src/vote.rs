use denali::types::Thing;
use log::debug;
use serde::Deserialize;
use serde_json::Result;

pub struct Votes {
    votes: Vec<u64>,
}

impl Thing for Votes {
    fn run(&self, payload: &str) -> Result<()> {
        vote_program(payload).unwrap();
        Ok(())
    }

    fn verify(&self) -> Result<bool> {
        Ok(true)
    }
}

impl Votes {
    pub fn new(number_of_candidates: u32) -> Self {
        let votes = vec![0; number_of_candidates as usize];
        Self { votes }
    }
}

fn vote_program(payload: &str) -> Result<()> {
    #[derive(Debug, Deserialize)]
    struct Ballot {
        candidate: u32,
    }

    let payload: Ballot = serde_json::from_str(payload)?;
    debug!("payload: {payload:?}");

    let mut votes = Votes::new(3);
    votes.votes[payload.candidate as usize] += 1;

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    fn setup() -> (Votes, String) {
        let payload = r#"
        {
            "candidate": 0
        }"#
        .into();
        let number_of_candidates = 3;
        let votes = Votes::new(number_of_candidates);
        (votes, payload)
    }

    #[test]
    fn test_vote_program() {
        let (_, payload) = setup();
        let _res = vote_program(&payload).unwrap();
    }

    #[test]
    fn test_run() {
        let (vote, payload) = setup();
        let _res = vote.run(&payload).unwrap();
    }
}
