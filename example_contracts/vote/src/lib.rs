use std::ffi::c_void;

use denali::{
    storage::State,
    types::{RawTraitObject, Thing},
};
use log::debug;
use serde::Deserialize;
use serde_json::Result;

// #[unsafe(no_mangle)]
// pub extern "C" fn create_thing() -> *mut dyn Thing {
//     println!("Creating vote");
//     let vote = Vote::new(5);
//     let boxed_vote = Box::new(vote);
//     Box::into_raw(boxed_vote)
// }

#[unsafe(no_mangle)]
pub extern "C" fn create_thing() -> *mut c_void {
    println!("Creating vote");
    let boxed_vote: Box<dyn Thing> = Box::new(Vote::new(5));
    let raw_fat_ptr = Box::into_raw(boxed_vote);
    unsafe {
        let (data_ptr, vtable_ptr): (*mut c_void, *mut c_void) = std::mem::transmute(raw_fat_ptr);

        let boxed_raw_trait_object = Box::new(RawTraitObject {
            data_ptr,
            vtable_ptr,
        });
        Box::into_raw(boxed_raw_trait_object) as *mut c_void
    }
}

#[derive(Clone)]
pub struct Vote {
    votes: Vec<u64>,
}

impl Thing for Vote {
    fn name(&self) -> &'static str {
        "vote"
    }

    fn run(&self, payload: &str, _state: &mut State) -> Result<()> {
        println!("vote run");
        vote_program(payload).unwrap();
        Ok(())
    }

    fn verify(&self) -> Result<bool> {
        println!("vote verify");
        Ok(true)
    }
}

impl Vote {
    pub fn new(number_of_candidates: u32) -> Self {
        let votes = vec![0; number_of_candidates as usize];
        println!("VOTE!");
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

    let mut vote = Vote::new(3);
    vote.votes[payload.candidate as usize] += 1;

    Ok(())
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
        let (_, payload) = setup();
        let _res = vote_program(&payload).unwrap();
    }

    #[test]
    fn test_run() {
        // let (vote, payload) = setup();
        // let _res = vote.run(&payload).unwrap();
    }
}
