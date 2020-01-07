use crate::intcomp::*;
use itertools::Itertools;

const NUM_AMPS: usize = 5;

pub fn run_a() {
    let max_thrust = find_max_single_thruster(
        &std::fs::read_to_string("inputs/input_7.txt").expect("Unable to read file"),
    );
    println!("7a: max thruster signal {}", max_thrust);
}

pub fn run_b() {
    let max_thrust = find_max_looped_thruster(
        &std::fs::read_to_string("inputs/input_7.txt").expect("Unable to read file"),
    );
    println!("7b: max looped thruster signal {}", max_thrust);
}

fn find_max_looped_thruster(program: &str) -> i64 {
    let mut max_thruster = 0;
    for phases in (5..=9).permutations(NUM_AMPS) {
        //println!("max_thruster {}, phases {:?}", max_thruster, phases);
        let thrust = run_thruster_programs(program, phases);
        if thrust > max_thruster {
            max_thruster = thrust;
        }
    }
    max_thruster
}

fn find_max_single_thruster(program: &str) -> i64 {
    let mut max_thruster = 0;
    for phases in (0..=4).permutations(NUM_AMPS) {
        //println!("max_thruster {}, phases {:?}", max_thruster, phases);
        let thrust = run_thruster_programs(program, phases);
        if thrust > max_thruster {
            max_thruster = thrust;
        }
    }
    max_thruster
}

fn run_thruster_programs(program: &str, phases: Vec<i64>) -> i64 {
    let base_amp = Intcomp::from(program);
    let mut output: i64 = 0;
    let mut amp_set = vec![];

    for amp_id in 0..NUM_AMPS {
        let amp_result = base_amp
            .clone()
            .run_with_input(vec![output, phases[amp_id]]);
        output = amp_result
            .get_output()
            .pop()
            .expect("Error - no output provided");
        amp_set.push(amp_result);
    }

    let mut last_from_e = output;

    loop {
        let mut next_amp_run = vec![];

        for amp in amp_set.drain(..) {
            match amp {
                IntResultCode::Completed(_) => return last_from_e,
                IntResultCode::MoreInputRequired(intcomp) => {
                    let amp_result = intcomp.run_with_input(vec![output]);
                    output = amp_result
                        .get_output()
                        .pop()
                        .expect("Error - no additional output");
                    next_amp_run.push(amp_result);
                }
            }
        }

        last_from_e = output;
        amp_set = next_amp_run;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tests_for_a() {
        assert_eq!(
            find_max_single_thruster("3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0"),
            43210
        );
        assert_eq!(
            find_max_single_thruster(
                "3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0"
            ),
            54321
        );
        assert_eq!(find_max_single_thruster("3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0"), 65210);
    }

    #[test]
    fn tests_for_b() {
        assert_eq!(find_max_looped_thruster("3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5"), 139629729);
        assert_eq!(find_max_looped_thruster("3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,-5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10"), 18216);
    }
}
