// use std::ffi::c_void;

use denali::{storage::State, types::Thing};
// use log::debug;
// use serde::Deserialize;
use serde_json::Result;

#[unsafe(no_mangle)]
pub extern "C" fn create_thing() -> *mut dyn Thing {
    println!("Creating fake");
    let fake = Fake;
    let boxed_fake = Box::new(fake);
    let raw_fake = Box::into_raw(boxed_fake);
    // raw_fake as *mut c_void
    raw_fake
}

#[derive(Clone)]
struct Fake;

impl Thing for Fake {
    fn name(&self) -> &'static str {
        "fake"
    }

    fn run(&self, _payload: &str, state: &mut State) -> Result<()> {
        println!("run fake");
        let s = state.get_value("test");
        println!("{s:?}");
        Ok(())
    }

    fn verify(&self) -> Result<bool> {
        println!("verify fake");
        Ok(true)
    }
}
