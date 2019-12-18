#[derive(Debug, Clone)]
pub struct Intcomp {
    pub memory: Vec<i32>,
    pub output: Vec<i32>,
    instruction_ptr: usize,
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
            output: vec![],
            instruction_ptr: 0,
        }
    }

    pub fn from_file(filename: &str) -> Self {
        let program = std::fs::read_to_string(filename).expect("Unable to read file");
        Intcomp::from(&program)
    }

    pub fn execute(&mut self) {
        self.execute_with_input(None);
    }

    pub fn execute_with_input(&mut self, input: Option<i32>) {
        loop {
            // Opcode is last two decimal places of the field
            let opcode = self.memory[self.instruction_ptr] % 100;
            let parameter_modes = self.memory[self.instruction_ptr] / 100;
            let mode_a = parameter_modes % 10;
            let mode_b = (parameter_modes / 10) % 10;
            let mode_c = (parameter_modes / 100) % 10;
            self.instruction_ptr += 1;

            //println!("Instruction: {} {} {} {}", opcode, mode_a, mode_b, mode_c);

            match opcode {
                1 => {
                    // Add
                    let val_a = self.get_param(mode_a);
                    let val_b = self.get_param(mode_b);
                    self.store_result(val_a + val_b, mode_c)
                }
                2 => {
                    // Multiply
                    let val_a = self.get_param(mode_a);
                    let val_b = self.get_param(mode_b);
                    self.store_result(val_a * val_b, mode_c)
                }
                3 => {
                    // Store input
                    assert_ne!(input, None);
                    self.store_result(input.unwrap(), mode_a);
                }
                4 => {
                    // Give output
                    let value = self.get_param(mode_a);
                    self.output.push(value);
                }
                5 => {
                    // Jump-if-true
                    let truth_value = self.get_param(mode_a);
                    let jump_value = self.get_param(mode_b);
                    if truth_value != 0 {
                        self.instruction_ptr = jump_value as usize;
                    }
                }
                6 => {
                    // Jump-if-false
                    let truth_value = self.get_param(mode_a);
                    let jump_value = self.get_param(mode_b);
                    if truth_value == 0 {
                        self.instruction_ptr = jump_value as usize;
                    }
                }
                7 => {
                    // Less-than
                    let val_a = self.get_param(mode_a);
                    let val_b = self.get_param(mode_b);
                    self.store_result(if val_a < val_b { 1 } else { 0 }, mode_c);
                }
                8 => {
                    // Equals
                    let val_a = self.get_param(mode_a);
                    let val_b = self.get_param(mode_b);
                    self.store_result(if val_a == val_b { 1 } else { 0 }, mode_c);
                }
                99 => break,
                _ => panic!("Unknown opcode {}", opcode),
            }
        }
    }

    fn get_param(&mut self, mode: i32) -> i32 {
        // Position - 0, Immediate - 1
        let value: i32;
        match mode {
            0 => {
                let position = self.memory[self.instruction_ptr] as usize;
                value = self.memory[position];
            }
            1 => value = self.memory[self.instruction_ptr],
            _ => panic!("Unknown parameter mode {}", mode),
        }
        self.instruction_ptr += 1;
        value
    }

    fn store_result(&mut self, result: i32, mode: i32) {
        let position_value = self.memory[self.instruction_ptr];
        self.instruction_ptr += 1;
        let res_location = match mode {
            0 => position_value,
            _ => panic!("Unknown parameter mode {}", mode),
        } as usize;
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

    fn test_intcomp_with_input(program: &str, input: i32) -> Vec<i32> {
        let mut intcomp = Intcomp::from(program);
        intcomp.execute_with_input(Some(input));
        intcomp.output
    }

    #[test]
    fn tests_for_mode() {
        let execute_intcomp = |x| {
            let mut comp = Intcomp::from(x);
            comp.execute();
            comp.string_from_memory()
        };
        assert_eq!("1002,4,3,4,99", execute_intcomp("1002,4,3,4,33"));
        assert_eq!("1101,100,-1,4,99", execute_intcomp("1101,100,-1,4,0"));
    }

    #[test]
    fn test_input_and_output() {
        assert_eq!(vec![88], test_intcomp_with_input("3,0,4,0,99", 88));
    }

    #[test]
    fn test_less_and_equal() {
        assert_eq!(
            vec![1],
            test_intcomp_with_input("3,9,8,9,10,9,4,9,99,-1,8", 8)
        );
        assert_eq!(
            vec![0],
            test_intcomp_with_input("3,9,8,9,10,9,4,9,99,-1,8", 4)
        );
        assert_eq!(
            vec![1],
            test_intcomp_with_input("3,9,7,9,10,9,4,9,99,-1,8", 4)
        );
        assert_eq!(
            vec![0],
            test_intcomp_with_input("3,9,7,9,10,9,4,9,99,-1,8", 9)
        );
        assert_eq!(
            vec![1],
            test_intcomp_with_input("3,3,1108,-1,8,3,4,3,99", 8)
        );
        assert_eq!(
            vec![0],
            test_intcomp_with_input("3,3,1108,-1,8,3,4,3,99", 9)
        );
        assert_eq!(
            vec![1],
            test_intcomp_with_input("3,3,1107,-1,8,3,4,3,99", 4)
        );
        assert_eq!(
            vec![0],
            test_intcomp_with_input("3,3,1107,-1,8,3,4,3,99", 9)
        );
        assert_eq!(
            vec![0],
            test_intcomp_with_input("3,3,1107,-1,8,3,4,3,99", 8)
        );
    }

    #[test]
    fn test_jump() {
        assert_eq!(
            vec![0],
            test_intcomp_with_input("3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9", 0)
        );
        assert_eq!(
            vec![1],
            test_intcomp_with_input("3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9", 1)
        );
        assert_eq!(
            vec![0],
            test_intcomp_with_input("3,3,1105,-1,9,1101,0,0,12,4,12,99,1", 0)
        );
        assert_eq!(
            vec![1],
            test_intcomp_with_input("3,3,1105,-1,9,1101,0,0,12,4,12,99,1", 1)
        );
    }

    #[test]
    fn test_longer_jumps_and_equals() {
        assert_eq!(vec![999], test_intcomp_with_input("3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99", 7));
        assert_eq!(vec![1000], test_intcomp_with_input("3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99", 8));
        assert_eq!(vec![1001], test_intcomp_with_input("3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99", 9));
    }
}
