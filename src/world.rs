pub use crate::color::Color;
pub use crate::entity::Entity;
pub use crate::operation::Operation;
extern crate wasm_bindgen;

use std::collections::LinkedList;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct World {
    pixels: Vec<Color>,
    x_size: usize,
    y_size: usize,
    entities: LinkedList<Entity>,
}

#[wasm_bindgen]
impl World {
    #[wasm_bindgen]
    pub fn move_by_x_ticks(&self, ticks: u32) {
        for tick in (0..ticks) {
            //TODO: actual simulation tick
        }
    }
    #[wasm_bindgen]
    pub fn get_pixels_pointer(&self) -> *const Color {
        self.pixels.as_ptr()
    }
    #[wasm_bindgen(skip)]
    pub fn new(x_size: usize, y_size: usize) -> World {

        let pixels = vec![Color(0, 2, 255); x_size * y_size];
        let mut entities = LinkedList::new();
        let mut world: World = World {
            pixels,
            x_size,
            y_size,
            entities,
        };
        world.init();
        return world;
    }
    fn init(&mut self) {
        //TODO: actual entity initialization
        self.entities.push_back(Entity::new(0, 0));
    }
}
