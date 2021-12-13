pub use crate::command;


/* parameters for manipulation with genetic code, later can be moved on JS side */
const GENETIC_CODE_SIZE:usize = 256;

pub struct GeneticCode {
    commands_raw:Vec<i32>,
    current_genetic_tape_ptr:u32
}

impl GeneticCode {
    pub fn new() -> GeneticCode {
        GeneticCode {
            commands_raw:(0..GENETIC_CODE_SIZE).map(|_| command::get_random_command_raw()).collect(),
            current_genetic_tape_ptr:0
        }
    }
    pub fn get_next_command()->command::Command{
        // TODO: actual genetic tape logic
        command::Command::SLEEP
    }
}


