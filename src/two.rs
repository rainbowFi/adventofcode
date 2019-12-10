pub fn run_a() {
    // Read the input file as a string and create computer.
    let input = std::fs::read_to_string("input_2.txt").expect("Unable to read file");
    let mut intcomp = Intcomp::from(&input);

    // Restore 1202 state
    intcomp.register[1] = 12;
    intcomp.register[2] = 2;

    // Get result
    intcomp.execute();
    println!("Result: {} ", intcomp.register[0]);
}

#[derive(Debug, Clone)]
struct Intcomp {
    pub register: Vec<i32>,
}

impl Intcomp {
    pub fn from(input: &str) -> Self {
        Self {
            // Convert input string to array of i32
            register: input
                .trim()
                .split(',')
                .map(|s| s.parse().unwrap())
                .collect(),
        }
    }

    pub fn execute(&mut self) {
        let mut opcode_position = 0;
        loop {
            let opcode = self.register[opcode_position];
            let pos_a = self.register[opcode_position + 1] as usize;
            let val_a = self.register[pos_a];
            let pos_b = self.register[opcode_position + 2] as usize;
            let val_b = self.register[pos_b];
            let location = self.register[opcode_position + 3];

            match opcode {
                1 => self.store_calc_result(val_a + val_b, location),
                2 => self.store_calc_result(val_a * val_b, location),
                99 => break,
                _ => panic!("Unknown opcode {}", opcode),
            }

            opcode_position += 4;

            // Catch the case where program length is not divisible by 4
            if (opcode_position + 4) > self.register.len() {
                assert_eq!(self.register[opcode_position], 99);
                break;
            }
        }
    }

    fn store_calc_result(&mut self, result: i32, location: i32) {
        let res_location = location as usize;
        self.register[res_location] = result;
    }

    pub fn string_from_register(self) -> String {
        let vec_of_strings: Vec<_> = self.register.iter().map(|s| s.to_string()).collect();
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
            comp.string_from_register()
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
