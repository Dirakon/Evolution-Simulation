use crate::alert;
pub use crate::cell::Cell;
pub use crate::cell_type::CellType;
pub use crate::color::Color;
pub use crate::entity::Entity;
pub use crate::entity_action::EntityAction;
pub use crate::entity_context::EntityContext;
pub use crate::entity_egg::EntityEgg;
pub use crate::operation::Operation;
extern crate wasm_bindgen;

use rand::prelude::*;
use std::collections::HashSet;
use wasm_bindgen::prelude::*;
/* Parameters for the world, later can be transferred to JS-side */
const STARTING_ENTITIES: i32 = 100;

#[wasm_bindgen]
pub struct World {
    cells: Vec<Cell>,
    x_size: usize,
    y_size: usize,
    entities: Vec<Entity>,
    entity_id_provider: u32,
}

#[wasm_bindgen]
impl World {
    #[wasm_bindgen]
    pub fn move_by_x_ticks(&mut self, ticks: u32) {
        for _ in 0..ticks {
            self.entities_do_actions();
            self.entities.retain(|x| !x.marked_as_dead);
            if self.entities.len() < STARTING_ENTITIES as usize{
                for _ in self.entities.len()..STARTING_ENTITIES as usize{
                    self.spawn_new_entity();
                }
            }
        }
    }

    fn entities_do_actions(&mut self) {
        let mut ids_of_entities_to_be_marked_as_dead = HashSet::new();
        let mut entity_eggs = Vec::<EntityEgg>::with_capacity(self.entities.len());
        for entity in &mut self.entities {
            if ids_of_entities_to_be_marked_as_dead.contains(&entity.unique_id) {
                entity.marked_as_dead = true;
                ids_of_entities_to_be_marked_as_dead.remove(&entity.unique_id);
            }
            if entity.marked_as_dead {
                continue;
            }
            let ctx = World::generate_context_for_entity(
                &self.cells,
                self.x_size,
                self.y_size,
                entity.pos_x,
                entity.pos_y,
            );
            let action = entity.make_move(&ctx, 0);

            World::process_potential_entity_attack(
                entity,
                &action,
                &ctx,
                &mut ids_of_entities_to_be_marked_as_dead,
            );
            World::process_potential_entity_birth(entity, &action, &ctx, &mut entity_eggs);
            World::process_potential_entity_move(entity, &action, &ctx, &mut self.cells,self.x_size);
        }
        for entity in &mut self.entities {
            if ids_of_entities_to_be_marked_as_dead.contains(&entity.unique_id) {
                entity.marked_as_dead = true;
            }
            if entity.marked_as_dead {
                let cell_index = World::get_index_static(
                    self.x_size,
                    entity.pos_x as usize,
                    entity.pos_y as usize,
                );
                self.cells[cell_index] = Cell::new_ground();
            }
        }

        self.entities.reserve(entity_eggs.len());
        for egg in entity_eggs {
            let cell_index =
                World::get_index_static(self.x_size, egg.pos_x as usize, egg.pos_y as usize);
            if self.cells[cell_index].cell_type.is(CellType::Ground) {
                println!("{}",self.cells[cell_index].cell_type as i32);
                let new_entity = egg.evolve_into_entity(self.entity_id_provider);
                self.cells[cell_index] =
                    Cell::new_entity(new_entity.genes_hashed_into_color, new_entity.unique_id);
                self.entities.push(new_entity);
                self.entity_id_provider += 1;
            }
        }
    }

    fn process_potential_entity_attack(
        entity: &mut Entity,
        action: &EntityAction,
        ctx: &EntityContext,
        ids_of_entities_to_be_marked_as_dead: &mut HashSet<u32>,
    ) {
        match action.attack_direction {
            Some(dir) => match ctx.get_context_cell_by_direction(dir).cell_type {
                CellType::Entity => {
                    ids_of_entities_to_be_marked_as_dead.insert(
                        ctx.get_context_cell_by_direction(dir)
                            .unique_id_for_entities,
                    );
                    entity.on_successful_bite();
                }
                CellType::Food => {
                    entity.on_successful_bite();
                }
                _ => {}
            },
            _ => {}
        };
    }
    fn process_potential_entity_birth(
        entity: &mut Entity,
        action: &EntityAction,
        ctx: &EntityContext,
        entity_eggs: &mut Vec<EntityEgg>,
    ) {
        match action.spawn_direction {
            Some(dir) => {
                if ctx
                    .get_context_cell_by_direction(dir)
                    .cell_type
                    .is(CellType::Ground)
                {
                    entity_eggs.push(EntityEgg::new(
                        entity.genetic_code.clone(),
                        entity.pos_x,
                        entity.pos_y,
                    ));
                }
            }
            _ => {}
        };
    }

    fn process_potential_entity_move(
        entity: &mut Entity,
        action: &EntityAction,
        ctx: &EntityContext,
        cells: &mut Vec<Cell>,
        x_size: usize,
    ) {
        match action.move_direction {
            Some(dir) => {
                if ctx
                    .get_context_cell_by_direction(dir)
                    .cell_type
                    .is(CellType::Ground)
                {
                    let index_of_old_position = World::get_index_static(
                        x_size,
                        entity.pos_x as usize,
                        entity.pos_y as usize,
                    );
                    cells[index_of_old_position] = Cell::new_ground();
                    let new_position = ctx.get_context_coordinates_by_direction(dir);
                    entity.pos_x = new_position.0;
                    entity.pos_y = new_position.1;
                    let index_of_new_position = World::get_index_static(
                        x_size,
                        entity.pos_x as usize,
                        entity.pos_y as usize,
                    );
                    cells[index_of_new_position] =
                        Cell::new_entity(entity.genes_hashed_into_color, entity.unique_id);
                }
            }
            _ => {}
        };
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
                if pos_y == 0{
                    y_size as i32 - 1
                } else {
                    pos_y - 1
                },
            ),
            (
                // RIGHT
                if pos_x == (x_size as i32 - 1 ){
                    0
                } else {
                    pos_x + 1
                },
                pos_y,
            ),
            (
                // DOWN
                pos_x,
                if pos_y ==( y_size as i32 - 1) {
                    0
                } else {
                    pos_y + 1
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
            entity_id_provider: 0,
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
                let entity = Entity::new_with_random_genes(x_pos, y_pos, self.entity_id_provider);
                self.entity_id_provider += 1;
                self.set_cell(
                    x_pos,
                    y_pos,
                    Cell::new_entity(entity.genes_hashed_into_color, entity.unique_id),
                );
                self.entities.push(entity);
            }

            _ => self.spawn_new_entity(), // TODO: decide with logic in this case
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
