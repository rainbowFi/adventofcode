use crate::intcomp::*;

pub fn run_a() {
    // Read the input file as a string and create computer.
    let mut intcomp = Intcomp::from_file("inputs/input_2.txt");

    // Restore 1202 state
    intcomp.memory[1] = 12;
    intcomp.memory[2] = 2;

    // Get result
    intcomp.execute();
    println!("2a Result: {} ", intcomp.memory[0]);
}

pub fn run_b() {
    let intcomp = Intcomp::from_file("inputs/input_2.txt");
    const TARGET: i32 = 19_690_720;

    for noun in 0..99 {
        for verb in 0..99 {
            let mut loop_intcomp = intcomp.clone();
            loop_intcomp.memory[1] = noun;
            loop_intcomp.memory[2] = verb;
            loop_intcomp.execute();

            if loop_intcomp.memory[0] == TARGET {
                let result = 100 * noun + verb;
                println!("2b Result: {}", result);
                return;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tests_for_a() {
        let execute_intcomp = |x| {
            let mut comp = Intcomp::from(x);
            comp.execute();
            comp.string_from_memory()
        };
        assert_eq!("2,0,0,0,99", execute_intcomp("1,0,0,0,99"));
        assert_eq!("2,3,0,6,99", execute_intcomp("2,3,0,3,99"));
        assert_eq!("2,4,4,5,99,9801", execute_intcomp("2,4,4,5,99,0"));
        assert_eq!(
            "30,1,1,4,2,5,6,0,99",
            execute_intcomp("1,1,1,4,99,5,6,0,99")
        );
    }
}
