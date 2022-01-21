pub mod color;
pub mod command;
pub mod entity;
pub mod genetic_code;
pub mod operation;
pub mod world;
pub mod direction;
pub mod tests;
pub mod cell_type;
pub mod cell;
pub mod entity_action;
pub mod entity_context;
pub mod entity_egg;
use wasm_bindgen::prelude::*;

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
