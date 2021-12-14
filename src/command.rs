pub use crate::operation::Operation;

use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use rand::prelude::*;

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
}