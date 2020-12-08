use std::collections::HashMap;

#[derive(Debug, PartialEq)]
struct OptCode<'s> {
    operation: &'s str,
    argument: isize,
    is_executed: bool,
}

impl<'s> OptCode<'s> {
    fn parse(input: &'s str) -> Self {
        let mut tokens = input.trim().split_ascii_whitespace();
        let operation = tokens.next().expect("Operation name required");
        let argument = tokens
            .next()
            .map(|v| v.parse::<isize>().ok())
            .flatten()
            .expect("Operation argument required");

        OptCode {
            operation,
            argument,
            is_executed: false,
        }
    }

    fn execute(&mut self, opt_machine_state: &OptMachineState) -> OptMachineState {
        self.is_executed = true;

        match self.operation {
            "acc" => opt_machine_state
                .shift_instruction_pointer_by(1)
                .update_accumulator_by(self.argument),
            "jmp" => opt_machine_state.shift_instruction_pointer_by(self.argument),
            "nop" => opt_machine_state.shift_instruction_pointer_by(1),
            _ => panic!("unreachable instruction"),
        }
    }
}

#[derive(Clone, Copy)]
struct OptMachineState {
    instruction_pointer: isize,
    accumulator: isize,
}

impl OptMachineState {
    fn shift_instruction_pointer_by(&self, shift: isize) -> Self {
        OptMachineState {
            instruction_pointer: self.instruction_pointer + shift,
            accumulator: self.accumulator,
        }
    }

    fn update_accumulator_by(&self, amount: isize) -> Self {
        OptMachineState {
            instruction_pointer: self.instruction_pointer,
            accumulator: self.accumulator + amount,
        }
    }
}

fn accumulator_before_hang(instructions: &str) -> isize {
    let mut opt_codes = instructions
        .lines()
        .map(|i| OptCode::parse(i))
        .collect::<Vec<OptCode>>();
    let mut opt_machine_state = OptMachineState {
        instruction_pointer: 0,
        accumulator: 0,
    };

    loop {
        let opt_code = &mut opt_codes[opt_machine_state.instruction_pointer as usize];
        if opt_code.is_executed {
            break;
        } else {
            opt_machine_state = opt_code.execute(&opt_machine_state);
        }
    }

    opt_machine_state.accumulator
}

fn accumulator_after_machine_completes(instructions: &str) -> isize {
    let mut opt_codes = instructions
        .lines()
        .map(|i| OptCode::parse(i))
        .collect::<Vec<OptCode>>();

    let mut opt_machine_state = OptMachineState {
        instruction_pointer: 0,
        accumulator: 0,
    };

    loop {
        if opt_machine_state.instruction_pointer as usize >= opt_codes.len() {
            break;
        }

        let opt_code = &mut opt_codes[opt_machine_state.instruction_pointer as usize];
        let new_machine_state = opt_code.execute(&opt_machine_state);

        if new_machine_state.instruction_pointer as usize >= opt_codes.len() {
            opt_machine_state = new_machine_state;
            break;
        }
        let next_opt_code = &mut opt_codes[new_machine_state.instruction_pointer as usize];

        if next_opt_code.is_executed {
            opt_machine_state = opt_machine_state.shift_instruction_pointer_by(1);
        } else {
            opt_machine_state = new_machine_state;
        }
    }

    opt_machine_state.accumulator
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_a_given_opt_code() {
        assert_eq!(
            OptCode::parse("acc +1"),
            OptCode {
                operation: "acc",
                argument: 1,
                is_executed: false
            }
        );
        assert_eq!(
            OptCode::parse("jmp +4"),
            OptCode {
                operation: "jmp",
                argument: 4,
                is_executed: false
            }
        );
        assert_eq!(
            OptCode::parse("acc -99"),
            OptCode {
                operation: "acc",
                argument: -99,
                is_executed: false
            }
        );
    }

    #[test]
    fn get_accumulator_accout_before_machine_hangs() {
        let input = "nop +0
        acc +1
        jmp +4
        acc +3
        jmp -3
        acc -99
        acc +1
        jmp -4
        acc +6";
        assert_eq!(accumulator_before_hang(input), 5);
    }

    #[test]
    fn get_accumulator_count_after_machine_completes_by_skiping_hung() {
        let input = "nop +0
        acc +1
        jmp +4
        acc +3
        jmp -3
        acc -99
        acc +1
        nop -4
        acc +6";
        assert_eq!(accumulator_after_machine_completes(input), 8);
    }
}
