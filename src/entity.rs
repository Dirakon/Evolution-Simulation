pub use crate::genetic_code::GeneticCode;
pub use crate::operation::Operation;
pub use crate::world::World;

/* Entity parameters, later can be transferred to JS-side */
const STARTING_ENERGY: i32 = 100;
const LIMIT_OF_COMMANDS_IN_A_ROW: i32 = 10;

pub struct Entity {
    pos_x: u64,
    pos_y: u64,
    genetic_code: GeneticCode,
    energy: i32,
}

impl Entity {
    pub fn new(pos_x: u64, pos_y: u64) -> Entity {
        Entity {
            pos_x,
            pos_y,
            genetic_code: GeneticCode::get_random_genetic_code(),
            energy: STARTING_ENERGY,
        }
    }
    pub fn make_move(&mut self, world: &World, commands_in_a_row: i32) {
        if commands_in_a_row >= LIMIT_OF_COMMANDS_IN_A_ROW {
            return;
        }

        let command = self.genetic_code.get_next_command();

        match command.operation {
            Operation::Move => return,
            Operation::MoveTapeBy => {
                self.genetic_code
                    .rotate_tape_by(command.operation_descriptor as usize);
                self.make_move(world, commands_in_a_row + 1);
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
