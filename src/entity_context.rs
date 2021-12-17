pub use crate::direction::Direction;
pub use crate::cell::Cell;

pub struct EntityContext{
    pub cells:[Cell;4]
}
impl EntityContext{
    pub fn new(cells:[Cell;4])->EntityContext{
        EntityContext{cells}
    }
    pub fn get_context_cell_by_direction<'a>(&'a self, direction:Direction)->&'a Cell{
        return match direction{
            Direction::Up=>&self.cells[0],
            Direction::Right=>&self.cells[1],
            Direction::Down=>&self.cells[2],
            Direction::Left=>&self.cells[3],
        }
    }
}