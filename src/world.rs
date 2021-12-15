pub use crate::cell::Cell;
pub use crate::cell_type::CellType;
pub use crate::color::Color;
pub use crate::entity::Entity;
pub use crate::operation::Operation;
extern crate wasm_bindgen;

use rand::prelude::*;
use std::collections::LinkedList;
use wasm_bindgen::prelude::*;

/* Parameters for the world, later can be transferred to JS-side */
const STARTING_ENTITIES: i32 = 20;

#[wasm_bindgen]
pub struct World {
    cells: Vec<Cell>,
    x_size: usize,
    y_size: usize,
    entities: LinkedList<Entity>,
}

#[wasm_bindgen]
impl World {
    #[wasm_bindgen]
    pub fn move_by_x_ticks(&mut self, ticks: u32) {
        for tick in 0..ticks {
            //TODO: finish implementing actual simulation tick
            for entity in &mut self.entities{
                entity.make_move( 0);
            }
        }
    }
    #[wasm_bindgen]
    pub fn get_cells_pointer(&self) -> *const Cell {
        self.cells.as_ptr()
    }
    #[wasm_bindgen(skip)]
    pub fn new(x_size: usize, y_size: usize) -> World {
        let mut world = World {
            cells: vec![Cell::new_ground(); x_size * y_size],
            x_size,
            y_size,
            entities: LinkedList::new(),
        };
        world.init_entities();
        world
    }
    fn init_entities(&mut self) {
        for _ in 0..STARTING_ENTITIES {
            self.spawn_new_entity();
        }
    }
    fn spawn_new_entity(&mut self) {
        let (x_pos, y_pos) = self.get_random_position();
        let cell_to_occupy = self.get_cell(x_pos, y_pos);
        match cell_to_occupy.cell_type {
            CellType::Entity => {return}, // TODO: decide with logic in this case
            _ => {
                let entity = Entity::new_with_random_genes(x_pos, y_pos);
                self.set_cell(
                    x_pos,
                    y_pos,
                    Cell::new_entity(entity.genes_hashed_into_color),
                );
                self.entities.push_back(entity);
            }
        }
    }
    fn get_cell<'a>(&'a self, x_pos: i32, y_pos: i32) -> &'a Cell {
        &self.cells[self.get_index(x_pos as usize, y_pos as usize)]
    }
    fn set_cell(&mut self, x_pos: i32, y_pos: i32, cell: Cell) {
        let index = self.get_index(x_pos as usize, y_pos as usize);
        self.cells[index] = cell;
    }
    fn get_index(&self, x_pos: usize, y_pos: usize) -> usize {
        y_pos * self.x_size + x_pos
    }
    fn get_random_index(&self) -> usize {
        rand::thread_rng().gen_range(0..self.cells.len())
    }
    fn get_random_position(&self) -> (i32, i32) {
        (
            rand::thread_rng().gen_range(0..self.x_size as i32),
            rand::thread_rng().gen_range(0..self.y_size as i32),
        )
    }
}
