use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Color(pub u8, pub u8, pub u8);

