use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use rand::prelude::*;
use std::mem;

const LAST_COMMAND_INDEX: i32 = 5;

#[derive(FromPrimitive)]
pub enum Command {
    // all non-defined numbers are mapped to MOVE_GENETIC_TAPE_BY by default,
    // their absolute value is by how much we move the genetic tape
    MOVE_GENETIC_TAPE_BY = -1,

    SLEEP = 0,
    MOVE_LEFT = 1,
    MOVE_RIGHT = 2,
    MOVE_UP = 3,
    MOVE_DOWN = 4,
    NEXT_COMMAND_IF_ENEMY = 5,
    // TODO: finish commands
}

pub fn get_last_command_index() -> i32 {
    //const size_of_one = mem::size_of::<Command>();
    return LAST_COMMAND_INDEX; // couldn't figure a better way to do it...
}

pub fn get_random_command() -> Command {
    let command_index = rand::thread_rng().gen_range(0..=get_last_command_index());
    FromPrimitive::from_i32(command_index)
        .expect("Tried to convert number {command_index} to command but failed")
}

pub fn translate_raw_data_to_command(raw_data:i32)->Command{
    if raw_data<0{
        return Command::MOVE_GENETIC_TAPE_BY;
    }else{
        return FromPrimitive::from_i32(raw_data)
            .expect("Tried to convert number {raw_data} to command but failed");
    }
}

pub fn get_random_command_raw() -> i32 {
    rand::thread_rng().gen_range(-get_last_command_index()..=get_last_command_index())
}
