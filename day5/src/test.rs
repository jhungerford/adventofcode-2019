use super::*;
use std::cell::Cell;

mod parse {
    use super::*;

    #[test]
    fn add() {
        let program = vec![101,2,3,4,99]; // add immediate 2 + position 3, store in position 4
        let instruction = parse_instruction(&program, 0);

        assert_eq!(instruction, Instruction::Add {a: Parameter::Immediate(2), b: Parameter::Position(3), out: 4});
    }

    #[test]
    fn multiply() {
        let program = vec![102,2,3,4,99]; // multiply immediate 2 + position 3, store in position 4
        let instruction = parse_instruction(&program, 0);

        assert_eq!(instruction, Instruction::Multiply {a: Parameter::Immediate(2), b: Parameter::Position(3), out: 4});
    }

    #[test]
    fn input() {
        let program = vec![3,1,99]; // take input, store it in 1
        let instruction = parse_instruction(&program, 0);

        assert_eq!(instruction, Instruction::Input {to: 1});
    }

    #[test]
    fn output() {
        let program = vec![104,0,99]; // output the number 0
        let instruction = parse_instruction(&program, 0);

        assert_eq!(instruction, Instruction::Output {from: Parameter::Immediate(0)});
    }

    #[test]
    fn jump_if_true() {
        let program = vec![105,2,1,99]; // jump to position 1 if immediate 2 is non-zero
        let instruction = parse_instruction(&program, 0);

        assert_eq!(instruction, Instruction::JumpIfTrue {what: Parameter::Immediate(2), to: Parameter::Position(1)});
    }

    #[test]
    fn jump_if_false() {
        let program = vec![1006,2,1,99]; // jump to immediate 1 if position 2 is zerozero
        let instruction = parse_instruction(&program, 0);

        assert_eq!(instruction, Instruction::JumpIfFalse {what: Parameter::Position(2), to: Parameter::Immediate(1)});
    }

    #[test]
    fn less_than() {
        let program = vec![107,2,1,9,99]; // if immediate 2 is less than the value of 1, store 1 in 9.  Otherwise store 0.
        let instruction = parse_instruction(&program, 0);

        assert_eq!(instruction, Instruction::LessThan {a: Parameter::Immediate(2), b: Parameter::Position(1), out: 9});
    }

    #[test]
    fn equals() {
        let program = vec![108,2,1,9,99]; // if immediate 2 is equal to the value of 1, store 1 in 9.  Otherwise store 0.
        let instruction = parse_instruction(&program, 0);

        assert_eq!(instruction, Instruction::Equals {a: Parameter::Immediate(2), b: Parameter::Position(1), out: 9});
    }

    #[test]
    fn halt() {
        let program = vec![102,2,3,4,99]; // pc 4: halt
        let instruction = parse_instruction(&program, 4);

        assert_eq!(instruction, Instruction::Halt);
    }
}

mod opcode_modes {
    use super::*;

    #[test]
    fn parse() {
        let add_modes = OpcodeModes::parse(1001); // 1: add - a is position mode (0), b is immediate mode (1).
        assert_eq!(1, add_modes.opcode);
        assert_eq!(vec![0, 1], add_modes.modes);

        let output_modes = OpcodeModes::parse(104); // output an immediate value.
        assert_eq!(4, output_modes.opcode);
        assert_eq!(vec![1], output_modes.modes);

        let input_modes = OpcodeModes::parse(3); // input a value.
        assert_eq!(3, input_modes.opcode);
        assert_eq!(Vec::<u32>::new(), input_modes.modes);
    }

    #[test]
    fn parameter() {
        let program = vec![1001,2,3,4,99]; // add immediate 2 + position 3, store in position 4
        let modes = OpcodeModes::parse(program[0]);

        assert_eq!(Parameter::Position(2), modes.parameter(&program, 0, 0));
        assert_eq!(Parameter::Immediate(3), modes.parameter(&program, 0, 1));
        assert_eq!(Parameter::Position(4), modes.parameter(&program, 0, 2)); // Not included in the opcode mode prefix - defaults to 0.
    }

    #[test]
    fn parameter_multiple_instructions() {
        let program = vec![3,3,104,0,99]; // Store input in 3, output the value.
        let modes = OpcodeModes::parse(program[2]);

        assert_eq!(Parameter::Immediate(0), modes.parameter(&program, 2, 0));
    }
}

mod run {
    use super::*;

    #[test]
    fn add() {
        let input = &AlwaysValueProgramInput::new(1);
        let output = &mut LastValueProgramOutput::new();
        let mut program = vec![101,2,3,4,99]; // add immediate 2 + position 3, store in position 4.  Result: [101,2,3,4,6]
        let instruction = parse_instruction(&program, 0);

        let pc = run_instruction(&instruction, &mut program, 0, input, output);
        assert_eq!(vec![101,2,3,4,6], program);
        assert_eq!(4, pc);
    }

    #[test]
    fn multiply() {
        let input = &AlwaysValueProgramInput::new(1);
        let output = &mut LastValueProgramOutput::new();
        let mut program = vec![102,2,3,4,99]; // multiply immediate 2 + position 3, store in position 4.  Result: [101,2,3,4,8]
        let instruction = parse_instruction(&program, 0);

        let pc = run_instruction(&instruction, &mut program, 0, input, output);
        assert_eq!(vec![102,2,3,4,8], program);
        assert_eq!(4, pc);
    }

    #[test]
    fn input() {
        // Stub program input that remembers whether it was ever queried and always returns 0.
        struct TestProgramInput {
            read_called: Cell<bool>
        }

        impl ProgramInput for TestProgramInput {
            fn read(&self) -> i32 {
                self.read_called.set(true);
                0
            }
        }

        let input = &TestProgramInput {read_called: Cell::new(false)};
        let output = &mut LastValueProgramOutput::new();
        let mut program = vec![3,0,99]; // take input, store in 0.  With an input of 0, result: [0,0,99]
        let instruction = parse_instruction(&program, 0);

        let pc = run_instruction(&instruction, &mut program, 0, input, output);
        assert_eq!(vec![0,0,99], program);
        assert_eq!(2, pc);

        // Make sure the input was queried.
        assert!(input.read_called.get());
    }

    #[test]
    fn output_value() {
        struct TestProgramOutput {
            value: i32,
            output_called: bool,
        }

        impl ProgramOutput for TestProgramOutput {
            fn output(&mut self, value: i32) {
                self.value = value;
                self.output_called = true;
            }
        }

        let input = &AlwaysValueProgramInput::new(1);
        let output = &mut TestProgramOutput {value: -1, output_called: false};
        let mut program = vec![4,0,99]; // Output the value of 0.  Should output 4.
        let instruction = parse_instruction(&program, 0);

        let pc = run_instruction(&instruction, &mut program, 0, input, output);
        assert_eq!(vec![4,0,99], program);
        assert_eq!(2, pc);

        // Assert that something was actually outputed.
        assert!(output.output_called);
        assert_eq!(4, output.value);
    }

    #[test]
    fn output_immediate() {
        let input = &AlwaysValueProgramInput::new(1);
        let output = &mut LastValueProgramOutput::new();
        let mut program = vec![104,50,99]; // Output 50.
        let instruction = parse_instruction(&program, 0);

        let pc = run_instruction(&instruction, &mut program, 0, input, output);
        assert_eq!(vec![104,50,99], program);
        assert_eq!(2, pc);
        assert_eq!(50, output.value);
    }

    #[test]
    fn jump_if_true() {
        let input = &AlwaysValueProgramInput::new(1);
        let output = &mut LastValueProgramOutput::new();
        let mut program = vec![105,2,3,4,1105,0,0,99]; // 2 != 0 so jump to the value of 3 (4), then do nothing since 0 == 0.
        
        let mut instruction = parse_instruction(&program, 0);
        let mut pc = run_instruction(&instruction, &mut program, 0, input, output);
        assert_eq!(4, pc);

        instruction = parse_instruction(&program, pc);
        pc = run_instruction(&instruction, &mut program, pc, input, output);
        assert_eq!(7, pc);
    }

    #[test]
    fn jump_if_false() {
        let input = &AlwaysValueProgramInput::new(1);
        let output = &mut LastValueProgramOutput::new();
        let mut program = vec![106,0,3,4,1106,1,0,99]; // 0 == 0 so jump to the value of 3 (4), then do nothing since 1 != 0.
        
        let mut instruction = parse_instruction(&program, 0);
        let mut pc = run_instruction(&instruction, &mut program, 0, input, output);
        assert_eq!(4, pc);

        instruction = parse_instruction(&program, pc);
        pc = run_instruction(&instruction, &mut program, pc, input, output);
        assert_eq!(7, pc);
    }

    #[test]
    fn less_than() {
        let input = &AlwaysValueProgramInput::new(1);
        let output = &mut LastValueProgramOutput::new();
        let mut program = vec![107,2,1,0,1107,2,3,1,99]; // 2 >= 2, so store 0 in position 0.  2 < 3, so store 1 in position 1.
        
        let mut instruction = parse_instruction(&program, 0);
        let mut pc = run_instruction(&instruction, &mut program, 0, input, output);
        assert_eq!(vec![0,2,1,0,1107,2,3,1,99], program);
        assert_eq!(4, pc);

        instruction = parse_instruction(&program, pc);
        pc = run_instruction(&instruction, &mut program, pc, input, output);
        assert_eq!(vec![0,1,1,0,1107,2,3,1,99], program);
        assert_eq!(8, pc);
    }

    #[test]
    fn equals() {
        let input = &AlwaysValueProgramInput::new(1);
        let output = &mut LastValueProgramOutput::new();
        let mut program = vec![108,2,1,0,1108,2,3,1,99]; // 2 == 2, so store 1 in position 0.  2 != 3, so store 0 in position 1.
        
        let mut instruction = parse_instruction(&program, 0);
        let mut pc = run_instruction(&instruction, &mut program, 0, input, output);
        assert_eq!(vec![1,2,1,0,1108,2,3,1,99], program);
        assert_eq!(4, pc);

        instruction = parse_instruction(&program, pc);
        pc = run_instruction(&instruction, &mut program, pc, input, output);
        assert_eq!(vec![1,0,1,0,1108,2,3,1,99], program);
        assert_eq!(8, pc);
    }

    #[test]
    #[should_panic]
    fn halt() {
        let input = &AlwaysValueProgramInput::new(1);
        let output = &mut LastValueProgramOutput::new();
        let mut program = vec![99]; // Halt!
        let instruction = parse_instruction(&program, 0);

        run_instruction(&instruction, &mut program, 0, input, output);
    }
}

#[test]
fn always_value_program_input() {
    let one_input = &AlwaysValueProgramInput::new(1);
    assert_eq!(1, one_input.read());
    assert_eq!(1, one_input.read());
    assert_eq!(1, one_input.read());

    let zero_input = &AlwaysValueProgramInput::new(0);
    assert_eq!(0, zero_input.read());
    assert_eq!(0, zero_input.read());
}

#[test]
fn last_value_program_output() {
    let output = &mut LastValueProgramOutput::new();
    assert_eq!(0, output.value);

    output.output(1);
    assert_eq!(1, output.value);

    output.output(219);
    assert_eq!(219, output.value);
}

mod run_program {
    use super::*;

    #[test]
    fn add_multiply() {
        // Couple examples from day2, which just supported the add and multiply instructions.
        check_program(&vec![1,9,10,3,2,3,11,0,99,30,40,50], &vec![3500,9,10,70,2,3,11,0,99,30,40,50], 1);
        check_program(&vec![1,0,0,0,99], &vec![2,0,0,0,99], 1);
        check_program(&vec![2,3,0,3,99], &vec![2,3,0,6,99], 1);
        check_program(&vec![2,4,4,5,99,0], &vec![2,4,4,5,99,9801], 1);
        check_program(&vec![1,1,1,4,99,5,6,0,99], &vec![30,1,1,4,2,5,6,0,99], 1);
    }

    #[test]
    fn input_output() {
        check_program(&vec![1101,100,-1,4,0], &vec![1101,100,-1,4,99], 1); // find 100 - 1, store in position 4.
        let output = check_program(&vec![3,3,104,0,99], &vec![3,3,104,50,99], 50); // Take input (50), store it in position 3, output the input.
        assert_eq!(output, 50);
    }

    #[test]
    fn comparisons() {
        // Using position mode, consider whether the input is equal to 8, output 1 if it is or 0 if not.
        assert_eq!(0, check_program(&vec![3,9,8,9,10,9,4,9,99,-1,8], &vec![3,9,8,9,10,9,4,9,99,0,8], 0));
        assert_eq!(1, check_program(&vec![3,9,8,9,10,9,4,9,99,-1,8], &vec![3,9,8,9,10,9,4,9,99,1,8], 8));
        
        // Using position mode, consider whether the input is less than 8, output 1 if it is or 0 if not.
        assert_eq!(1, check_program(&vec![3,9,7,9,10,9,4,9,99,-1,8], &vec![3,9,7,9,10,9,4,9,99,1,8], 0));
        assert_eq!(0, check_program(&vec![3,9,7,9,10,9,4,9,99,-1,8], &vec![3,9,7,9,10,9,4,9,99,0,8], 8));
        assert_eq!(0, check_program(&vec![3,9,7,9,10,9,4,9,99,-1,8], &vec![3,9,7,9,10,9,4,9,99,0,8], 10));
        
        // Using immediate mode, consider whether the input is equal to 8, output 1 if it is or 0 if not.
        assert_eq!(0, check_program(&vec![3,3,1108,-1,8,3,4,3,99], &vec![3,3,1108,0,8,3,4,3,99], 0));
        assert_eq!(1, check_program(&vec![3,3,1108,-1,8,3,4,3,99], &vec![3,3,1108,1,8,3,4,3,99], 8));
        
        // Using immediate mode, consider whether the input is less than 8, output 1 if it is or 0 if not.
        assert_eq!(1, check_program(&vec![3,3,1107,-1,8,3,4,3,99], &vec![3,3,1107,1,8,3,4,3,99], 0));
        assert_eq!(0, check_program(&vec![3,3,1107,-1,8,3,4,3,99], &vec![3,3,1107,0,8,3,4,3,99], 8));
        
        // Take an input, output 0 if the input was 0 or 1 if it was non-zero
        assert_eq!(0, check_program(&vec![3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9], &vec![3,12,6,12,15,1,13,14,13,4,13,99,0,0,1,9] ,0)); // Position Mode
        assert_eq!(1,check_program(&vec![3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9],&vec![3,12,6,12,15,1,13,14,13,4,13,99,8,1,1,9] ,8)); // Position Mode
        assert_eq!(0,check_program(&vec![3,3,1105,-1,9,1101,0,0,12,4,12,99,1],&vec![3,3,1105,0,9,1101,0,0,12,4,12,99,0] ,0)); // Immediate Mode
        assert_eq!(1,check_program(&vec![3,3,1105,-1,9,1101,0,0,12,4,12,99,1],&vec![3,3,1105,8,9,1101,0,0,12,4,12,99,1] ,8)); // Immediate Mode
        
        // Ask for a single number,output 99 if the input is below 8,1000 if it's 8,or 1001 if it's greater than 8.
        let is_8_program = &vec![
            3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,
            1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,
            999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99
        ];
            
        assert_eq!(999,check_program(&is_8_program.clone(),&vec![3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,7,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99], 7));
        assert_eq!(1000,check_program(&is_8_program.clone(),&vec![3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,1000,8,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99], 8));
        assert_eq!(1001,check_program(&is_8_program.clone(),&vec![3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,1001,9,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99], 9));
    }

    /// Runs the given program, checking that it produced the expected result.  
    /// Input is always the supplied input value, and this method returns the last outputed value
    /// or 0 the program didn't output anything.
    fn check_program(original_program: &Vec<i32>, expected: &Vec<i32>, input_value: i32) -> i32 {
        let input = &AlwaysValueProgramInput::new(input_value);
        let output = &mut LastValueProgramOutput::new();
        let program = &original_program.clone();

        let result = run_program(program, input, output);

        assert_eq!(expected, &result);

        output.value
    }
}
