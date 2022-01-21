#[cfg(test)]
use super::*;
pub use crate::world::World;
#[test]
fn hash_function_works() {
    let number = 149796; // 100100100... 
    let gotten_numbers = (
        GeneticCode::get_number_from_bits(number, 3, 0),
        GeneticCode::get_number_from_bits(number, 3, 1),
        GeneticCode::get_number_from_bits(number, 3, 2),
        GeneticCode::get_number_from_bits(number, 3, 3),
    );

    assert_eq!(gotten_numbers, (63, 0, 0, 31));
}

#[test]
fn world_test_run(){
    let mut world = World::new(64,64);
    for i in 0..1000{
        println!("tick #{}",i);
    world.move_by_x_ticks(1);
    }
}
