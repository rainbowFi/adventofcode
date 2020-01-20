use crate::intcomp::*;

pub fn run_a() {
    // Test mode
    let result = Intcomp::from_file("inputs/input_9.txt").run_with_input(vec![1]);
    println!("9a: output {:?}", result.get_output());
}

pub fn run_b() {
    // sensor boost mode
    let result = Intcomp::from_file("inputs/input_9.txt").run_with_input(vec![2]);
    println!("9b: output {:?}", result.get_output());
}
