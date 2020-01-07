use std::fs::File;
use std::io::{prelude::BufRead, BufReader};

pub fn run_a() {
    println!("Running 1a");
    let total_fuel = find_total_fuel_for_modules(true);
    println!("Total module-only fuel is {}", total_fuel)
}

pub fn run_b() {
    println!("Running 1b");
    let total_fuel = find_total_fuel_for_modules(false);
    println!("Total fuel is {}", total_fuel)
}

fn find_total_fuel_for_modules(module_only: bool) -> i64 {
    let file = File::open("inputs/input_1.txt").expect("Unable to open file");
    let file = BufReader::new(file);

    let mut total_fuel = 0;
    for line in file.lines() {
        let line = line.expect("Unable to read line");
        let module_mass: i64 = match line.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if module_only {
            total_fuel += calculate_fuel_for_mass(module_mass);
        } else {
            total_fuel += calculate_module_fuel(module_mass);
        }
    }

    total_fuel
}

fn calculate_module_fuel(mut mass: i64) -> i64 {
    let mut module_fuel = 0;

    loop {
        let fuel_for_mass = calculate_fuel_for_mass(mass);
        if fuel_for_mass <= 0 {
            return module_fuel;
        } else {
            module_fuel += fuel_for_mass;
            mass = fuel_for_mass;
        }
    }
}

fn calculate_fuel_for_mass(mass: i64) -> i64 {
    (mass / 3) - 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tests_for_a() {
        assert_eq!(2, calculate_fuel_for_mass(12));
        assert_eq!(2, calculate_fuel_for_mass(14));
        assert_eq!(654, calculate_fuel_for_mass(1969));
        assert_eq!(33583, calculate_fuel_for_mass(100756));
    }

    #[test]
    fn tests_for_b() {
        assert_eq!(2, calculate_module_fuel(14));
        assert_eq!(966, calculate_module_fuel(1969));
        assert_eq!(50346, calculate_module_fuel(100756));
    }
}
