pub use crate::operation::Operation;


#[derive(Copy, Clone)]
pub struct Command {
    pub operation: Operation,
    pub operation_descriptor: i32,
}

impl Command {
    pub fn new_random_command() -> Command {
        Command {
            operation: Operation::get_random_operation(),
            operation_descriptor: Operation::get_random_operation_descriptor(),
        }
    }
    pub fn compress_into_01_pair(&self)->(f32,f32){
        (self.operation.compress_into_01_value(), Operation::compress_descriptor_into_01_value(self.operation_descriptor))
    }
}