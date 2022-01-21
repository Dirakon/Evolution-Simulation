use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use rand::prelude::*;
use enum_variant_counter::EnumVariantCount;


#[derive(FromPrimitive, Copy, Clone,EnumVariantCount)]
pub enum Direction {
    Up = 0,
    Right = 1,
    Down = 2,
    Left = 3,
}
const TOTAL_DIRECTIONS: i32 = LENGTH as i32;

impl Direction {
    pub fn get_rotated_direction(dir: &Direction, amount: i32) -> Direction {
        Direction::number_to_direction(((*dir) as i32 + amount) % TOTAL_DIRECTIONS)
    }
    pub fn get_random_direction() -> Direction {
        Direction::number_to_direction(rand::thread_rng().gen_range(0..TOTAL_DIRECTIONS))
    }
    fn number_to_direction(number: i32) -> Direction {
        FromPrimitive::from_i32(number).expect("couldn't convert number to direction")
    }
}
