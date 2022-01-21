use wasm_bindgen::prelude::*;

pub use crate::cell_type::CellType;
pub use crate::color::Color;
#[wasm_bindgen]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Cell {
    pub cell_type: CellType,
    pub color_for_entities: Color,
    pub unique_id_for_entities:u32
}

const EMPTY_COLOR: Color = Color(0, 0, 0);

impl Cell {
    pub fn new_ground() -> Cell {
        Cell {
            cell_type: CellType::Ground,
            color_for_entities: EMPTY_COLOR,
            unique_id_for_entities: 0
        }
    }
    pub fn new_food() -> Cell {
        Cell {
            cell_type: CellType::Food,
            color_for_entities: EMPTY_COLOR,
            unique_id_for_entities: 0
        }
    }
    pub fn new_entity(color: Color, unique_id:u32) -> Cell {
        Cell {
            cell_type: CellType::Entity,
            color_for_entities: color,
            unique_id_for_entities: unique_id
        }
    }
}
