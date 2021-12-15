pub use crate::color::Color;
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
    pub fn make_move(&mut self, commands_in_a_row: i32) {
        if commands_in_a_row >= LIMIT_OF_COMMANDS_IN_A_ROW {
            return;
        }

        let command = self.genetic_code.get_next_command();

        match command.operation {
            Operation::Move => return,
            Operation::MoveTapeBy => {
                self.genetic_code
                    .rotate_tape_by(command.operation_descriptor as usize);
                self.make_move( commands_in_a_row + 1);
            }
            Operation::MoveTapeByIfEnemyAhead => return,
            Operation::MoveTapeByIfAllyAhead => return,
            Operation::MoveTapeByIfEmptyAhead => return,
            Operation::MoveTapeByIfFoodAhead => return,
            Operation::MoveTapeByIfSomethingAhead => return,
            Operation::RotateLeftBy => return,
            Operation::RotateRightBy => return,
            Operation::GiveBirth => return,
            Operation::Bite => return,
            Operation::Sleep => return,
        }
    }
}
