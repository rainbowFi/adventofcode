use crate::intcomp::*;

pub fn run_a() {
    // Read the input file as a string and create computer.
    let mut intcomp = Intcomp::from_file("input_5.txt");

    intcomp.execute_with_input(Some(1));
    println!("5a Output is: {:?}", intcomp.output);
}

pub fn run_b() {
    // Read the input file as a string and create computer.
    let mut intcomp = Intcomp::from_file("input_5.txt");

    intcomp.execute_with_input(Some(5));
    println!("5b Output is: {:?}", intcomp.output);
}
