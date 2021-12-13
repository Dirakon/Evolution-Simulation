pub mod world;
pub mod color;
pub mod entity;
pub mod genetic_code;
pub mod command;

extern crate serde_json;
extern crate wasm_bindgen;
use std::mem;
use wasm_bindgen::prelude::*;
extern crate console_error_panic_hook;

#[macro_use]
extern crate serde_derive;

#[wasm_bindgen]
extern "C" {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn init_world(x_size: usize, y_size: usize) -> world::World {
    // alert(&mem::size_of::<u8>().to_string());// -> 1 byte
    //  alert(&mem::size_of::<Color>().to_string());// -> 3 bytes
    world::World::new(x_size, y_size)
}
