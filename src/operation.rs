use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use rand::prelude::*;
use enum_variant_counter::EnumVariantCount;

const MIN_OPERATION_DESCRIPTOR: i32 = 0;
const MAX_OPERATION_DESCRIPTOR: i32 = 1024;
#[derive(FromPrimitive, Copy, Clone,EnumVariantCount)]
pub enum Operation {
    Sleep = 0,
    Move = 1,
    RotateLeftBy = 2,
    RotateRightBy = 3,
    MoveTapeBy = 4,
    MoveTapeByIfEnemyAhead = 5,
    MoveTapeByIfSomethingAhead = 6,
    MoveTapeByIfAllyAhead = 7,
    MoveTapeByIfFoodAhead = 8,
    MoveTapeByIfEmptyAhead = 9,
    GiveBirth = 10,
    Bite = 11,
    // TODO: perhaps think of more commands
}

const LAST_OPERATION_INDEX: i32 = LENGTH as i32 - 1;

impl Operation {
    pub fn get_random_operation_descriptor() -> i32 {
        rand::thread_rng().gen_range(MIN_OPERATION_DESCRIPTOR..=MAX_OPERATION_DESCRIPTOR)
    }

    pub fn get_random_operation() -> Operation {
        let operation_code = rand::thread_rng().gen_range(0..=LAST_OPERATION_INDEX);
        Operation::number_to_operation(operation_code)
    }

    fn number_to_operation(number: i32) -> Operation {
        FromPrimitive::from_i32(number).expect("Tried to convert number to operation but failed")
    }
}
