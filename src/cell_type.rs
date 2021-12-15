use wasm_bindgen::prelude::*;

#[repr(u8)]
#[wasm_bindgen]
#[derive(Copy, Clone)]
pub enum CellType{
    Entity=0,
    Ground=1,
    Food=2
}