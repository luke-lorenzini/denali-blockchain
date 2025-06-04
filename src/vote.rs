use serde::Deserialize;
use serde_json::Result;

struct Votes {
    votes: Vec<u64>,
}

impl Votes {
    fn new(number_of_candidates: u32) -> Self {
        let votes = vec![0; number_of_candidates as usize];
        Self { votes }
    }
}

pub fn vote_program(payload: &str) -> Result<()> {
    #[derive(Debug, Deserialize)]
    struct Ballot {
        candidate: u32,
    }

    let payload: Ballot = serde_json::from_str(payload)?;
    println!("payload: {payload:?}");

    let mut votes = Votes::new(3);
    votes.votes[payload.candidate as usize] += 1;

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    fn setup() -> String {
        r#"
        {
            "candidate": 0
        }"#
        .into()
    }

    #[test]
    fn test_vote_program() {
        let payload = setup();
        let _res = vote_program(&payload).unwrap();
    }
}
