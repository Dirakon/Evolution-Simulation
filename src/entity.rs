pub use crate::cell::Cell;
pub use crate::cell_type::CellType;
pub use crate::color::Color;
pub use crate::direction::Direction;
pub use crate::entity_action::EntityAction;
pub use crate::entity_context::EntityContext;
pub use crate::genetic_code::GeneticCode;
pub use crate::operation::Operation;
pub use crate::world::World;

/* Entity parameters, later can be transferred to JS-side */
const STARTING_ENERGY: i32 = 100;
const LIMIT_OF_COMMANDS_IN_A_ROW: i32 = 10;

pub struct Entity {
    pub pos_x: i32,
    pub pos_y: i32,
    pub genes_hashed_into_color: Color,
    current_direction: Direction,
    genetic_code: GeneticCode,
    energy: i32,
}

impl Entity {
    pub fn new_with_random_genes(pos_x: i32, pos_y: i32) -> Entity {
        Entity::new_from_genes(pos_x, pos_y, GeneticCode::get_random_genetic_code())
    }
    pub fn new_from_genes(pos_x: i32, pos_y: i32, original_genetic_code: GeneticCode) -> Entity {
        Entity {
            pos_x,
            pos_y,
            genes_hashed_into_color: original_genetic_code.hash_code_into_color(),
            genetic_code: original_genetic_code,
            energy: STARTING_ENERGY,
            current_direction: Direction::get_random_direction(),
        }
    }
    pub fn new_from_genes_with_mutation_chance(
        pos_x: i32,
        pos_y: i32,
        initial_genetic_code: GeneticCode,
    ) -> Entity {
        // TODO: implement mutation
        Entity::new_from_genes(pos_x, pos_y, initial_genetic_code)
    }
    pub fn is_genetically_alike_to(&self, color: &Color) -> bool {
        // TODO: implement algorithm for enemy-friend-detection based on color similarity
        false
    }
    pub fn make_move(&mut self, ctx: &EntityContext, commands_in_a_row: i32) -> EntityAction {
        let final_action = EntityAction::new_empty();
        if commands_in_a_row >= LIMIT_OF_COMMANDS_IN_A_ROW {
            return final_action;
        }

        let command = self.genetic_code.get_next_command();

        match command.operation {
            //TODO: implement energy consumption and energy generation
            Operation::Move => final_action.move_direction = Some(self.current_direction),
            Operation::MoveTapeBy => {
                self.genetic_code
                    .rotate_tape_by(command.operation_descriptor);
            }
            Operation::MoveTapeByIfEnemyAhead => {
                let cell_ahead = ctx.get_context_cell_by_direction(self.current_direction);
                match cell_ahead.cell_type {
                    CellType::Entity => {
                        if !self.is_genetically_alike_to(&cell_ahead.color_for_entities) {
                            self.genetic_code
                                .rotate_tape_by(command.operation_descriptor);
                        }
                    }
                };
            }
            Operation::MoveTapeByIfAllyAhead => {
                let cell_ahead = ctx.get_context_cell_by_direction(self.current_direction);
                match cell_ahead.cell_type {
                    CellType::Entity => {
                        if self.is_genetically_alike_to(&cell_ahead.color_for_entities) {
                            self.genetic_code
                                .rotate_tape_by(command.operation_descriptor);
                        }
                    }
                };
            }
            Operation::MoveTapeByIfEmptyAhead => {
                let cell_ahead = ctx.get_context_cell_by_direction(self.current_direction);
                match cell_ahead.cell_type {
                    CellType::Ground => {
                        self.genetic_code
                            .rotate_tape_by(command.operation_descriptor);
                    }
                };
            }
            Operation::MoveTapeByIfFoodAhead => {
                let cell_ahead = ctx.get_context_cell_by_direction(self.current_direction);
                match cell_ahead.cell_type {
                    CellType::Food => {
                        self.genetic_code
                            .rotate_tape_by(command.operation_descriptor);
                    }
                };
            }
            Operation::MoveTapeByIfSomethingAhead => {
                let cell_ahead = ctx.get_context_cell_by_direction(self.current_direction);
                match cell_ahead.cell_type {
                    CellType::Ground => {}
                    _ => {
                        self.genetic_code
                            .rotate_tape_by(command.operation_descriptor);
                    }
                };
            }
            Operation::RotateLeftBy => self
                .genetic_code
                .rotate_tape_by(-command.operation_descriptor),
            Operation::RotateRightBy => self
                .genetic_code
                .rotate_tape_by(command.operation_descriptor),
            Operation::GiveBirth => final_action.spawn_direction = Some(self.current_direction),
            Operation::Bite => final_action.attack_direction = Some(self.current_direction),
            Operation::Sleep => return final_action,
        }
        return if !Entity::any_action_has_been_chosen(&final_action) {
            self.make_move(ctx, commands_in_a_row + 1)
        } else {
            final_action
        };
    }
    fn any_action_has_been_chosen(entity_action: &EntityAction) -> bool {
        entity_action.move_direction.is_some()
            || entity_action.attack_direction.is_some()
            || entity_action.spawn_direction.is_some()
    }
}
