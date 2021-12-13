
extern crate serde_json;
extern crate wasm_bindgen;
use std::mem;
use wasm_bindgen::prelude::*;
extern crate console_error_panic_hook;

#[wasm_bindgen]
#[repr(C)]
pub struct Color(pub u8, pub u8, pub u8);

impl Copy for Color {}

impl Clone for Color {
    fn clone(self: &Color) -> Color {
        Color(self.0, self.1, self.2)
    }
}
