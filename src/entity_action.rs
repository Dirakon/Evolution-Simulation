pub use crate::direction::Direction;

pub struct EntityAction{
    pub move_direction:Option<Direction>,
    pub attack_direction:Option<Direction>,
}