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
pub fn init_world(x_size: usize, y_size: usize) -> World {
    // alert(&mem::size_of::<u8>().to_string());// -> 1 byte
    //  alert(&mem::size_of::<Color>().to_string());// -> 3 bytes
    World::new(x_size, y_size)
}

#[wasm_bindgen]
#[repr(C)]
pub struct Color(u8, u8, u8);

impl Copy for Color {}

impl Clone for Color {
    fn clone(self: &Color) -> Color {
        Color(self.0, self.1, self.2)
    }
}

#[wasm_bindgen]
pub struct World {
    pixels: Vec<Color>,
    x_size: usize,
    y_size: usize,
}

#[wasm_bindgen]
impl World {
    #[wasm_bindgen]
    pub fn move_by_x_ticks(&self, ticks: u32) {
        for tick in (0..ticks) {}
    }
    #[wasm_bindgen]
    pub fn get_pixels_pointer(&self) -> *const Color {
        self.pixels.as_ptr()
    }
    #[wasm_bindgen(skip)]
    pub fn new(x_size: usize, y_size: usize) -> World {
        let mut pixels = vec![Color(0, 2, 255); x_size*y_size];
        World {
            pixels,
            x_size,
            y_size,
        }
    }

    pub fn get(&self, x: usize, y: usize) -> Color {
        self.pixels[x + y * self.x_size]
    }
}
