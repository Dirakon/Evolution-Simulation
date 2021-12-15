pub use crate::direction::Direction;
pub use crate::cell::Cell;

pub struct EntityContext{
    pub cells:[Cell;8]
}
impl EntityContext{
    pub fn new(cells:[Cell;8])->EntityContext{
        EntityContext{cells}
    }
    pub fn get_context_cell_by_direction<'a>(&'a self, direction:Direction)->&'a Cell{
        //TODO: implement this
        &self.cells[0]
    }
}