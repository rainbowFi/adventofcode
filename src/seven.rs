use crate::intcomp::*;
use itertools::Itertools;

pub fn run_a() {
    let max_thrust = find_max_thruster(
        &std::fs::read_to_string("inputs/input_7.txt").expect("Unable to read file"),
    );
    println!("7a: max thruster signal {}", max_thrust);
}

fn find_max_thruster(program: &str) -> i32 {
    let mut max_thruster = 0;
    for phases in (0..=4).permutations(5) {
        println!("max_thruster {}, phases {:?}", max_thruster, phases);
        let thrust = run_thruster_programs(program, phases);
        if thrust > max_thruster {
            max_thruster = thrust;
        }
    }
    max_thruster
}

fn run_thruster_programs(program: &str, phases: Vec<i32>) -> i32 {
    let base_amp = Intcomp::from(program);
    let mut output: i32 = 0;

    for amp_id in 0..5 {
        println!("Output is now {}", output);
        let mut amp_output = base_amp
            .clone()
            .run_with_input(vec![output, phases[amp_id]]);
        output = amp_output.pop().expect("Error - no output provided");
    }

    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tests_for_a() {
        assert_eq!(
            find_max_thruster("3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0"),
            43210
        );
        assert_eq!(
            find_max_thruster(
                "3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0"
            ),
            54321
        );
        assert_eq!(find_max_thruster("3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0"), 65210);
    }
}
