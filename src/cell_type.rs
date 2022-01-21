use wasm_bindgen::prelude::*;

#[repr(u8)]
#[wasm_bindgen]
#[derive(Copy, Clone,Eq,PartialEq)]
pub enum CellType {
    Entity = 0,
    Ground = 1,
    Food = 2,
}

impl CellType {
    pub fn is(&self, cell_type: CellType) -> bool {
        return cell_type == *self;
    }
}
