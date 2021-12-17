pub use crate::direction::Direction;

pub struct EntityAction{
    pub move_direction:Option<Direction>,
    pub attack_direction:Option<Direction>,
    pub spawn_direction:Option<Direction>,
}
impl EntityAction{
    pub fn new_empty()->EntityAction{
        EntityAction{
            move_direction:None,
            attack_direction:None,
            spawn_direction:None
        }
    }
}