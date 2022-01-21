pub use crate::color::Color;
pub use crate::command::Command;
use rand::prelude::*;

/* parameters for manipulation with genetic code, later can be moved on JS side */
const GENETIC_CODE_SIZE: i32 = 512;
const AMOUNT_OF_BITS_PER_OPERATION_SUM: u32 = 9;
const STARTING_BIT: u32 = 0;
const HASHED_NUMBER_MULTIPLIER: u8 = 4;

#[derive(Clone)]
pub struct GeneticCode {
    genetic_tape: Vec<Command>,
    current_genetic_tape_ptr: usize,
}

impl GeneticCode {
    pub fn hash_code_into_color(&self) -> Color {
        let mut operation_sum1 = 0.0;
        let mut operation_sum2 = 0.0;
        for compressed_pair in self.genetic_tape.iter().map(|x| x.compress_into_01_pair()) {
            operation_sum1 += compressed_pair.0;
            operation_sum2 += compressed_pair.1;
        }
        let combined_number =
            ((operation_sum1 as u32) << AMOUNT_OF_BITS_PER_OPERATION_SUM) + operation_sum2 as u32;
        Color(
            GeneticCode::get_number_from_bits(combined_number, 3, 0) * HASHED_NUMBER_MULTIPLIER,
            GeneticCode::get_number_from_bits(combined_number, 3, 1) * HASHED_NUMBER_MULTIPLIER,
            GeneticCode::get_number_from_bits(combined_number, 3, 2) * HASHED_NUMBER_MULTIPLIER,
        )
    }
    pub fn get_number_from_bits(
        original_number: u32,
        bit_index_divisor: u32,
        starting_bit: u32,
    ) -> u8 {
        let mut final_number: u8 = 0;
        let mut id = 0;
        for bit_index in ((starting_bit + STARTING_BIT)
            ..(starting_bit + AMOUNT_OF_BITS_PER_OPERATION_SUM * 2))
            .rev()
        {
            if (id) % bit_index_divisor == 0 {
                let bit = (original_number >> bit_index) & 1;
                final_number = final_number << 1;
                final_number += bit as u8;
            }
            id += 1;
        }
        return final_number;
    }
    pub fn get_random_genetic_code() -> GeneticCode {
        GeneticCode {
            genetic_tape: (0..GENETIC_CODE_SIZE)
                .map(|_| Command::new_random_command())
                .collect(),
            current_genetic_tape_ptr: 0,
        }
    }
    pub fn get_next_command(&mut self) -> Command {
        self.rotate_tape_by(1);
        self.genetic_tape[self.current_genetic_tape_ptr]
    }
    pub fn rotate_tape_by(&mut self, val: i32) {
        self.current_genetic_tape_ptr =
            ((self.current_genetic_tape_ptr as i32 + val) % GENETIC_CODE_SIZE) as usize;
    }
    pub fn make_random_mutation(&mut self) {
        let gene_index_to_mutate = rand::thread_rng().gen_range(0..self.genetic_tape.len());
        self.genetic_tape[gene_index_to_mutate] = Command::new_random_command();
    }
}
