use crate::intcomp::*;

pub fn run_a() {
    // Read the input file as a string and create computer.
    let mut intcomp = Intcomp::from_file("inputs/input_5.txt");

    intcomp.input.push(1);
    intcomp.execute();
    println!("5a Output is: {:?}", intcomp.output);
}

pub fn run_b() {
    // Read the input file as a string and create computer.
    let mut intcomp = Intcomp::from_file("inputs/input_5.txt");

    intcomp.input.push(1);
    intcomp.execute();
    println!("5b Output is: {:?}", intcomp.output);
}
