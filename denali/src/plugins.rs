use std::{ffi::c_void, path::PathBuf};

use libloading::Library;

use crate::Thing;

pub mod plugin_task;

type Contract = unsafe fn() -> *mut c_void;

#[repr(C)]
pub struct RawTraitObject {
    pub data_ptr: *mut c_void,
    pub vtable_ptr: *mut c_void,
}

#[derive(Debug)]
pub struct Plugin {
    _library: Library,
    pub thing: Box<dyn Thing>,
}

impl Plugin {
    pub fn new(library: Library, program: Box<dyn Thing>) -> Self {
        Plugin {
            _library: library,
            // todo: wrap this in an Arc....maybe
            thing: program,
        }
    }

    pub async fn build(path: PathBuf) -> (&'static str, Plugin) {
        unsafe {
            let lib = libloading::Library::new(path).unwrap();
            let func: libloading::Symbol<Contract> = lib.get(b"create_thing").unwrap();
            let xxx = func();

            let boxed_raw_trait_object = Box::from_raw(xxx.cast::<RawTraitObject>());
            let raw_trait_object = *boxed_raw_trait_object;
            let raw_fat_ptr: *mut dyn Thing = std::mem::transmute(raw_trait_object);
            let owned_plugin_box: Box<dyn Thing> = Box::from_raw(raw_fat_ptr);

            let name = owned_plugin_box.name();

            (name, Plugin::new(lib, owned_plugin_box))
        }
    }
}
