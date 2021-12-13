pub use crate::genetic_code;
pub use crate::world;

pub struct Entity {
    pos_x: u64,
    pos_y: u64,
    genetic_code: genetic_code::GeneticCode,
}

impl Entity {
    pub fn new(pos_x: u64, pos_y: u64) -> Entity {
        Entity {
            pos_x,
            pos_y,
            genetic_code: genetic_code::GeneticCode::new(),
        }
    }
    pub fn make_move(world: &world::World) {}
}
