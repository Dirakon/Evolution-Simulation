pub use crate::cell::Cell;
pub use crate::cell_type::CellType;
pub use crate::color::Color;
pub use crate::entity::Entity;
pub use crate::entity_action::EntityAction;
pub use crate::entity_context::EntityContext;
pub use crate::operation::Operation;
extern crate wasm_bindgen;

use rand::prelude::*;
use wasm_bindgen::prelude::*;

/* Parameters for the world, later can be transferred to JS-side */
const STARTING_ENTITIES: i32 = 20;

#[wasm_bindgen]
pub struct World {
    cells: Vec<Cell>,
    x_size: usize,
    y_size: usize,
    entities: Vec<Entity>,
}

#[wasm_bindgen]
impl World {
    #[wasm_bindgen]
    pub fn move_by_x_ticks(&mut self, ticks: u32) {
        for _ in 0..ticks {
            for entity in &mut self.entities {
                let coords = (entity.pos_x, entity.pos_y);
                let ctx = World::generate_context_for_entity(
                    &self.cells,
                    self.x_size,
                    self.y_size,
                    coords.0,
                    coords.1,
                );
                let action = entity.make_move(&ctx, 0);
                match action.attack_direction {
                    Some(dir) => {
                        // TODO: implement effect of entity attack
                        match ctx.get_context_cell_by_direction(dir).cell_type {
                            CellType::Entity => {
                                entity.on_successful_bite();
                            }
                            CellType::Food => {
                                entity.on_successful_bite();
                            }
                            _ => {}
                        }
                    }
                    _ => {}
                };
                match action.spawn_direction {
                    Some(dir) => {
                        // TODO: implement effect of entity giving birth
                        match ctx.get_context_cell_by_direction(dir).cell_type {
                            CellType::Ground => {
                                entity.on_successful_bite();
                            }
                            _ => {}
                        }
                    }
                    _ => {}
                };
                match action.move_direction {
                    Some(dir) => {
                        // TODO: implement effect of entity move
                        match ctx.get_context_cell_by_direction(dir).cell_type {
                            CellType::Ground => {
                                entity.on_successful_bite();
                            }
                            _ => {}
                        }
                    }
                    _ => {}
                };
            }

            self.entities.retain(|x| !x.marked_as_dead);
        }
    }
    #[inline]
    fn generate_context_for_entity(
        cells: &Vec<Cell>,
        x_size: usize,
        y_size: usize,
        pos_x: i32,
        pos_y: i32,
    ) -> EntityContext {
        let poses = [
            (
                // UP
                pos_x,
                if pos_y == y_size as i32 - 1 {
                    y_size as i32 - 1
                } else {
                    pos_y - 1
                },
            ),
            (
                // RIGHT
                if pos_x == x_size as i32 - 1 {
                    x_size as i32 - 1
                } else {
                    pos_x - 1
                },
                pos_y,
            ),
            (
                // DOWN
                pos_x,
                if pos_y == y_size as i32 - 1 {
                    0
                } else {
                    pos_y - 1
                },
            ),
            (
                // LEFT
                if pos_x == 0 {
                    x_size as i32 - 1
                } else {
                    pos_x - 1
                },
                pos_y,
            ),
        ];
        let mut ctx = EntityContext {
            coordinates: poses,
            cells: [Cell::new_ground(); 4],
        };
        for i in 0..4 {
            let cell_index = World::get_index_static(
                x_size,
                ctx.coordinates[i].0 as usize,
                ctx.coordinates[i].1 as usize,
            );
            ctx.cells[i] = cells[cell_index];
        }
        return ctx;
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
            entities: Vec::new(),
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
            CellType::Ground => {
                let entity = Entity::new_with_random_genes(x_pos, y_pos);
                self.set_cell(
                    x_pos,
                    y_pos,
                    Cell::new_entity(entity.genes_hashed_into_color),
                );
                self.entities.push(entity);
            }

            _ => return, // TODO: decide with logic in this case
        }
    }
    fn get_cell<'a>(&'a self, x_pos: i32, y_pos: i32) -> &'a Cell {
        &self.cells[self.get_index(x_pos as usize, y_pos as usize)]
    }
    fn set_cell(&mut self, x_pos: i32, y_pos: i32, cell: Cell) {
        let index = self.get_index(x_pos as usize, y_pos as usize);
        self.cells[index] = cell;
    }
    #[inline]
    fn get_index(&self, x_pos: usize, y_pos: usize) -> usize {
        y_pos * self.x_size + x_pos
    }
    #[inline]
    fn get_index_static(x_size: usize, x_pos: usize, y_pos: usize) -> usize {
        y_pos * x_size + x_pos
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
