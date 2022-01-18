pub use crate::direction::Direction;
pub use crate::cell::Cell;

pub struct EntityContext{
    pub cells:[Cell;4],
    pub coordinates: [(i32,i32);4]
}
impl EntityContext{
    pub fn new(cells:[Cell;4], coordinates: [(i32,i32);4])->EntityContext{
        EntityContext{cells,coordinates}
    }
    pub fn get_context_coordinates_by_direction(& self, direction:Direction)->(i32,i32){
        self.coordinates[self.get_context_array_position_by_direction(direction)]
    }
    pub fn get_context_cell_by_direction<'a>(&'a self, direction:Direction)->&'a Cell{
        &self.cells[self.get_context_array_position_by_direction(direction)]
    }
    fn get_context_array_position_by_direction(&self, direction:Direction)->usize{
        return match direction{
            Direction::Up=>0,
            Direction::Right=>1,
            Direction::Down=>2,
            Direction::Left=>3,
        }
    }
}