use std::ffi::c_void;

use denali::{
    storage::State,
    types::{RawTraitObject, Thing},
};
// use log::debug;
// use serde::Deserialize;
use serde_json::Result;

// #[unsafe(no_mangle)]
// pub extern "C" fn create_thing() -> *mut dyn Thing {
//     println!("Creating fake");
//     let fake = Fake;
//     let boxed_fake = Box::new(fake);
//     Box::into_raw(boxed_fake)
// }

#[unsafe(no_mangle)]
pub extern "C" fn create_thing() -> *mut c_void {
    println!("Creating fake");
    let boxed_fake: Box<dyn Thing> = Box::new(Fake);
    let raw_fat_ptr = Box::into_raw(boxed_fake);
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
