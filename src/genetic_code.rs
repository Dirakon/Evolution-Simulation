pub use crate::command::Command;
pub use crate::color::Color;

/* parameters for manipulation with genetic code, later can be moved on JS side */
const GENETIC_CODE_SIZE:usize = 256;

pub struct GeneticCode {
    genetic_tape:Vec<Command>,
    current_genetic_tape_ptr:usize
}

impl GeneticCode {

    pub fn hash_code_into_color(&self)->Color{
        //TODO: hashing algorithm
        Color(0,0,0)
    }
    pub fn get_random_genetic_code() -> GeneticCode {
        GeneticCode {
            genetic_tape:(0..GENETIC_CODE_SIZE).map(|_| Command::new_random_command()).collect(),
            current_genetic_tape_ptr:0
        }
    }
    pub fn get_next_command(&mut self)->Command{
        self.rotate_tape_by(1);
        self.genetic_tape[self.current_genetic_tape_ptr]
    }
    pub fn rotate_tape_by(&mut self, val:usize){
        self.current_genetic_tape_ptr = (self.current_genetic_tape_ptr+val)%GENETIC_CODE_SIZE;
    }
}


