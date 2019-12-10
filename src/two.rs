pub fn run_a() {
    // Read the input file as a string and create computer.
    let mut intcomp = Intcomp::from_file("input_2.txt");

    // Restore 1202 state
    intcomp.memory[1] = 12;
    intcomp.memory[2] = 2;

    // Get result
    intcomp.execute();
    println!("2a Result: {} ", intcomp.memory[0]);
}

pub fn run_b() {
    let intcomp = Intcomp::from_file("input_2.txt");
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

#[derive(Debug, Clone)]
struct Intcomp {
    pub memory: Vec<i32>,
}

impl Intcomp {
    pub fn from(program: &str) -> Self {
        Self {
            // Convert input string to array of i32
            memory: program
                .trim()
                .split(',')
                .map(|s| s.parse().unwrap())
                .collect(),
        }
    }

    pub fn from_file(filename: &str) -> Self {
        let program = std::fs::read_to_string(filename).expect("Unable to read file");
        Intcomp::from(&program)
    }

    pub fn execute(&mut self) {
        let mut instruction_ptr = 0;
        loop {
            let opcode = self.memory[instruction_ptr];
            let pos_a = self.memory[instruction_ptr + 1] as usize;
            let val_a = self.memory[pos_a];
            let pos_b = self.memory[instruction_ptr + 2] as usize;
            let val_b = self.memory[pos_b];
            let location = self.memory[instruction_ptr + 3];

            match opcode {
                1 => self.store_calc_result(val_a + val_b, location),
                2 => self.store_calc_result(val_a * val_b, location),
                99 => break,
                _ => panic!("Unknown opcode {}", opcode),
            }

            instruction_ptr += 4;

            // Catch the case where program length is not divisible by 4
            if (instruction_ptr + 4) > self.memory.len() {
                assert_eq!(self.memory[instruction_ptr], 99);
                break;
            }
        }
    }

    fn store_calc_result(&mut self, result: i32, location: i32) {
        let res_location = location as usize;
        self.memory[res_location] = result;
    }

    pub fn string_from_memory(self) -> String {
        let vec_of_strings: Vec<_> = self.memory.iter().map(|s| s.to_string()).collect();
        vec_of_strings.join(",")
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
