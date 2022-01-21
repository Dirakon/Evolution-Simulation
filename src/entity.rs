use crate::alert;
pub use crate::cell::Cell;
pub use crate::cell_type::CellType;
pub use crate::color::Color;
pub use crate::direction::Direction;
pub use crate::entity_action::EntityAction;
pub use crate::entity_context::EntityContext;
pub use crate::genetic_code::GeneticCode;
pub use crate::operation::Operation;
pub use crate::world::World;

use rand::prelude::*;

/* Entity parameters, later can be transferred to JS-side */
const STARTING_ENERGY: i32 = 150;
const LIMIT_OF_COMMANDS_IN_A_ROW: i32 = 10;
const ENERGY_GIVEN_PER_SUCCESSFUL_BITE: i32 = 30;
const ENERGY_COST_OF_MOVE: i32 = 1;
const ENERGY_COST_OF_BITE: i32 = 5;
const ENERGY_COST_OF_BIRTH: i32 = 150;
const ENERGY_COST_OF_INACTION: i32 = 1;
const ENERGY_DIVISOR_FOR_TAPE_ROTATION: i32 = 10;
const COLOR_SIMILARITY_THRESHOLD: u8 = 1;
const MUTATION_CHANCE: i32 = 10; // 1 in *MUTATION_CHANCE* mutates

pub struct Entity {
    pub pos_x: i32,
    pub pos_y: i32,
    pub genes_hashed_into_color: Color,
    pub marked_as_dead: bool,
    pub unique_id: u32,
    current_direction: Direction,
    pub genetic_code: GeneticCode,
    energy: i32,
}

impl Entity {
    pub fn new_with_random_genes(pos_x: i32, pos_y: i32, unique_id: u32) -> Entity {
        Entity::new_from_genes(
            pos_x,
            pos_y,
            GeneticCode::get_random_genetic_code(),
            unique_id,
        )
    }
    pub fn new_from_genes(
        pos_x: i32,
        pos_y: i32,
        original_genetic_code: GeneticCode,
        unique_id: u32,
    ) -> Entity {
        Entity {
            pos_x,
            pos_y,
            genes_hashed_into_color: original_genetic_code.hash_code_into_color(),
            genetic_code: original_genetic_code,
            energy: STARTING_ENERGY,
            current_direction: Direction::get_random_direction(),
            marked_as_dead: false,
            unique_id,
        }
    }
    pub fn new_from_genes_with_mutation_chance(
        pos_x: i32,
        pos_y: i32,
        mut initial_genetic_code: GeneticCode,
        unique_id: u32,
    ) -> Entity {
        if rand::thread_rng().gen_range(0..MUTATION_CHANCE) == 0 {
            initial_genetic_code.make_random_mutation();
        }
        Entity::new_from_genes(pos_x, pos_y, initial_genetic_code, unique_id)
    }
    pub fn is_genetically_alike_to(&self, color: &Color) -> bool {
        self.one_color_tone_is_in_threshold(self.genes_hashed_into_color.0, color.0)
            && self.one_color_tone_is_in_threshold(self.genes_hashed_into_color.1, color.1)
            && self.one_color_tone_is_in_threshold(self.genes_hashed_into_color.2, color.2)
    }
    fn one_color_tone_is_in_threshold(&self, c1: u8, c2: u8) -> bool {
        let diff = if c1 > c2 { c1 - c2 } else { c2 - c1 };
        diff <= COLOR_SIMILARITY_THRESHOLD
    }
    pub fn make_move(&mut self, ctx: &EntityContext, commands_in_a_row: i32) -> EntityAction {
        let mut final_action = EntityAction::new_empty();
        // if (self.energy > 50 || self.energy < 0){
        //     alert(&self.energy.to_string());
        // }
        //  println!("Commands in a row: {}",commands_in_a_row);
        if commands_in_a_row >= LIMIT_OF_COMMANDS_IN_A_ROW {
            self.energy -= ENERGY_COST_OF_INACTION;
            self.mark_as_dead_if_negative_energy();
            return final_action;
        }

        let command = self.genetic_code.get_next_command();

        match command.operation {
            Operation::Move => {
                self.energy -= ENERGY_COST_OF_MOVE;
                final_action.move_direction = Some(self.current_direction);
            }
            Operation::MoveTapeBy => {
                self.genetic_code
                    .rotate_tape_by(command.operation_descriptor);
            }
            Operation::MoveTapeByIfEnemyAhead => {
                let cell_ahead = ctx.get_context_cell_by_direction(self.current_direction);
                if cell_ahead.cell_type.is(CellType::Entity) {
                    if !self.is_genetically_alike_to(&cell_ahead.color_for_entities) {
                        self.genetic_code
                            .rotate_tape_by(command.operation_descriptor);
                    }
                }
            }
            Operation::MoveTapeByIfAllyAhead => {
                let cell_ahead = ctx.get_context_cell_by_direction(self.current_direction);
                if cell_ahead.cell_type.is(CellType::Entity) {
                    if self.is_genetically_alike_to(&cell_ahead.color_for_entities) {
                        self.genetic_code
                            .rotate_tape_by(command.operation_descriptor);
                    }
                }
            }
            Operation::MoveTapeByIfEmptyAhead => {
                let cell_ahead = ctx.get_context_cell_by_direction(self.current_direction);
                if cell_ahead.cell_type.is(CellType::Ground) {
                    self.genetic_code
                        .rotate_tape_by(command.operation_descriptor);
                }
            }
            Operation::MoveTapeByIfFoodAhead => {
                let cell_ahead = ctx.get_context_cell_by_direction(self.current_direction);
                if cell_ahead.cell_type.is(CellType::Food) {
                    self.genetic_code
                        .rotate_tape_by(command.operation_descriptor);
                }
            }
            Operation::MoveTapeByIfSomethingAhead => {
                let cell_ahead = ctx.get_context_cell_by_direction(self.current_direction);
                if !cell_ahead.cell_type.is(CellType::Ground) {
                    self.genetic_code
                        .rotate_tape_by(command.operation_descriptor);
                }
            }
            Operation::MoveTapeByEnergyAmount => self
                .genetic_code
                .rotate_tape_by(self.energy / ENERGY_DIVISOR_FOR_TAPE_ROTATION),
            Operation::RotateLeftBy => {
                self.current_direction = Direction::get_rotated_direction(
                    &self.current_direction,
                    command.operation_descriptor,
                )
            }
            Operation::RotateRightBy => {
                self.current_direction = Direction::get_rotated_direction(
                    &self.current_direction,
                    command.operation_descriptor,
                )
            }
            Operation::GiveBirth => {
                if self.energy >= ENERGY_COST_OF_BIRTH {
                    self.energy -= ENERGY_COST_OF_BIRTH;
                    final_action.spawn_direction = Some(self.current_direction);
                }
            }
            Operation::Bite => {
                self.energy -= ENERGY_COST_OF_BITE;
                final_action.attack_direction = Some(self.current_direction);
            }
        }
        self.mark_as_dead_if_negative_energy();
        return if !Entity::any_action_has_been_chosen(&final_action) {
            self.make_move(ctx, commands_in_a_row + 1)
        } else {
            final_action
        };
    }

    pub fn on_successful_bite(&mut self) {
        // alert("success bite");
        self.energy += ENERGY_GIVEN_PER_SUCCESSFUL_BITE;
        if self.energy > 0 {
            self.marked_as_dead = false;
        }
    }
    fn mark_as_dead_if_negative_energy(&mut self) {
        if self.energy < 0 {
            self.marked_as_dead = true;
        }
    }
    fn any_action_has_been_chosen(entity_action: &EntityAction) -> bool {
        entity_action.move_direction.is_some()
            || entity_action.attack_direction.is_some()
            || entity_action.spawn_direction.is_some()
    }
}
