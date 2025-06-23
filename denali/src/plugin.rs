use std::{collections::HashMap, ffi::c_void, sync::Arc};

use libloading::Library;

use crate::Thing;

#[repr(C)]
pub struct RawTraitObject {
    pub data_ptr: *mut c_void,
    pub vtable_ptr: *mut c_void,
}

pub struct Plugin {
    _library: Library,
    pub thing: Box<dyn Thing>,
}

impl Plugin {
    pub fn new(library: Library, program: Box<dyn Thing>) -> Self {
        Plugin {
            _library: library,
            thing: program,
        }
    }

    pub fn stuff() -> Arc<HashMap<String, Plugin>> {
        let mut contract_map = HashMap::new();
        // load vote
        unsafe {
            // type Contract = unsafe extern "C" fn() -> Box<dyn Thing>;
            type Contract = unsafe fn() -> *mut c_void;
            let vote = "vote";
            let lib = libloading::Library::new("/home/luke/repos/denali/target/debug/libvote.so")
                .unwrap();
            let func: libloading::Symbol<Contract> = lib.get(b"create_thing").unwrap();
            let xxx = func();
            // let xxx = Box::from_raw(xxx as *mut dyn Thing);

            let boxed_raw_trait_object = Box::from_raw(xxx.cast::<RawTraitObject>());
            let raw_trait_object = *boxed_raw_trait_object;
            let raw_fat_ptr: *mut dyn Thing = std::mem::transmute(raw_trait_object);
            let owned_plugin_box: Box<dyn Thing> = Box::from_raw(raw_fat_ptr);

            let plugin = Plugin::new(lib, owned_plugin_box);

            contract_map.insert(vote.into(), plugin);
            let v = contract_map.get(vote).unwrap().thing.verify();
            println!("Result of verification for vote: {v:?}");
        }

        // load fake
        unsafe {
            // type Contract = unsafe extern "C" fn() -> Box<dyn Thing>;
            type Contract = unsafe fn() -> *mut c_void;
            let fake = "fake";
            let lib = libloading::Library::new("/home/luke/repos/denali/target/debug/libfake.so")
                .unwrap();
            let func: libloading::Symbol<Contract> = lib.get(b"create_thing").unwrap();
            let xxx = func();
            // let xxx = Box::from_raw(xxx as *mut dyn Thing);

            let boxed_raw_trait_object = Box::from_raw(xxx.cast::<RawTraitObject>());
            let raw_trait_object = *boxed_raw_trait_object;
            let raw_fat_ptr: *mut dyn Thing = std::mem::transmute(raw_trait_object);
            let owned_plugin_box: Box<dyn Thing> = Box::from_raw(raw_fat_ptr);

            let plugin = Plugin::new(lib, owned_plugin_box);

            contract_map.insert(fake.into(), plugin);
            let v = contract_map.get(fake).unwrap().thing.verify();
            println!("{v:?}");
        }

        // load bank
        unsafe {
            // type Contract = unsafe extern "C" fn() -> Box<dyn Thing>;
            type Contract = unsafe fn() -> *mut c_void;
            let bank = "bank";
            let lib = libloading::Library::new("/home/luke/repos/denali/target/debug/libbank.so")
                .unwrap();
            let func: libloading::Symbol<Contract> = lib.get(b"create_thing").unwrap();
            let xxx = func();
            // let xxx = Box::from_raw(xxx as *mut dyn Thing);

            let boxed_raw_trait_object = Box::from_raw(xxx.cast::<RawTraitObject>());
            let raw_trait_object = *boxed_raw_trait_object;
            let raw_fat_ptr: *mut dyn Thing = std::mem::transmute(raw_trait_object);
            let owned_plugin_box: Box<dyn Thing> = Box::from_raw(raw_fat_ptr);

            let plugin = Plugin::new(lib, owned_plugin_box);

            contract_map.insert(bank.into(), plugin);
            let v = contract_map.get(bank).unwrap().thing.verify();
            println!("{v:?}");
        }

        Arc::new(contract_map)
    }
}
