use std::fs::File;
use std::io::{self, prelude::*, BufReader};

pub fn run_a() {
    println!("Running 1a");

    let file = File::open("input.txt").expect("Unable to open file");
    let file = BufReader::new(file);

    let mut total_fuel = 0;
    for line in file.lines() {
        let line = line.expect("Unable to read line");
        let module_mass: i32 = match line.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        total_fuel = total_fuel + calculate_module_fuel(module_mass);
    }

    println!("Total fuel is {}", total_fuel)
}

fn calculate_module_fuel(mass: i32) -> i32 {
    (mass / 3) - 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tests() {
        assert_eq!(2, calculate_module_fuel(12));
        assert_eq!(2, calculate_module_fuel(14));
        assert_eq!(654, calculate_module_fuel(1969));
        assert_eq!(33583, calculate_module_fuel(100756));
    }
}
