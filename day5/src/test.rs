use super::*;
use std::cell::Cell;

#[test]
fn test_parse_add() {
    let program = vec![101,2,3,4,99]; // add immediate 2 + position 3, store in position 4
    let (instruction, pc) = parse_instruction(&program, 0);

    assert_eq!(instruction, Instruction::Add {a: Parameter::Immediate(2), b: Parameter::Position(3), out: 4});
    assert_eq!(4, pc);
}

#[test]
fn test_parse_multiply() {
    let program = vec![102,2,3,4,99]; // multiply immediate 2 + position 3, store in position 4
    let (instruction, pc) = parse_instruction(&program, 0);

    assert_eq!(instruction, Instruction::Multiply {a: Parameter::Immediate(2), b: Parameter::Position(3), out: 4});
    assert_eq!(4, pc);
}

#[test]
fn test_parse_input() {
    let program = vec![3,1,99]; // take input, store it in 1
    let (instruction, pc) = parse_instruction(&program, 0);

    assert_eq!(instruction, Instruction::Input {to: 1});
    assert_eq!(2, pc);
}

#[test]
fn test_parse_output() {
    let program = vec![104,0,99]; // output the number 0
    let (instruction, pc) = parse_instruction(&program, 0);

    assert_eq!(instruction, Instruction::Output {from: Parameter::Immediate(0)});
    assert_eq!(2, pc);
}

#[test]
fn test_parse_halt() {
    let program = vec![102,2,3,4,99]; // pc 4: halt
    let (instruction, pc) = parse_instruction(&program, 4);

    assert_eq!(instruction, Instruction::Halt);
    assert_eq!(4, pc);
}

#[test]
fn test_parse_opcode_modes() {
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
fn test_opcode_modes_parameter() {
    let program = vec![1001,2,3,4,99]; // add immediate 2 + position 3, store in position 4
    let modes = OpcodeModes::parse(program[0]);

    assert_eq!(Parameter::Position(2), modes.parameter(&program, 0, 0));
    assert_eq!(Parameter::Immediate(3), modes.parameter(&program, 0, 1));
    assert_eq!(Parameter::Position(4), modes.parameter(&program, 0, 2)); // Not included in the opcode mode prefix - defaults to 0.
}

#[test]
fn test_opcode_modes_parameter_multiple_instructions() {
    let program = vec![3,3,104,0,99]; // Store input in 3, output the value.
    let modes = OpcodeModes::parse(program[2]);
    
    assert_eq!(Parameter::Immediate(0), modes.parameter(&program, 2, 0));
}

#[test]
fn test_run_add() {
    let input = &AlwaysValueProgramInput::new(1);
    let output = &mut LastValueProgramOutput::new();
    let mut program = vec![101,2,3,4,99]; // add immediate 2 + position 3, store in position 4.  Result: [101,2,3,4,6]
    let (instruction, _pc) = parse_instruction(&program, 0);

    run_instruction(&instruction, &mut program, input, output);
    assert_eq!(vec![101,2,3,4,6], program);
}

#[test]
fn test_run_multiply() {
    let input = &AlwaysValueProgramInput::new(1);
    let output = &mut LastValueProgramOutput::new();
    let mut program = vec![102,2,3,4,99]; // multiply immediate 2 + position 3, store in position 4.  Result: [101,2,3,4,8]
    let (instruction, _pc) = parse_instruction(&program, 0);

    run_instruction(&instruction, &mut program, input, output);
    assert_eq!(vec![102,2,3,4,8], program);
}

#[test]
fn test_run_input() {
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
    let (instruction, _pc) = parse_instruction(&program, 0);

    run_instruction(&instruction, &mut program, input, output);
    assert_eq!(vec![0,0,99], program);

    // Make sure the input was queried.
    assert!(input.read_called.get());
}

#[test]
fn test_run_output_value() {
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
    let (instruction, _pc) = parse_instruction(&program, 0);

    run_instruction(&instruction, &mut program, input, output);
    assert_eq!(vec![4,0,99], program);

    // Assert that something was actually outputed.
    assert!(output.output_called);
    assert_eq!(4, output.value);
}

#[test]
fn test_run_output_immediate() {
    let input = &AlwaysValueProgramInput::new(1);
    let output = &mut LastValueProgramOutput::new();
    let mut program = vec![104,50,99]; // Output 50.
    let (instruction, _pc) = parse_instruction(&program, 0);

    run_instruction(&instruction, &mut program, input, output);
    assert_eq!(vec![104,50,99], program);
    assert_eq!(50, output.value);
}

#[test]
fn test_always_value_program_input() {
    let one_input = &AlwaysValueProgramInput::new(1);
    assert_eq!(1, one_input.read());
    assert_eq!(1, one_input.read());
    assert_eq!(1, one_input.read());

    let zero_input = &AlwaysValueProgramInput::new(0);
    assert_eq!(0, zero_input.read());
    assert_eq!(0, zero_input.read());
}

#[test]
fn test_last_value_program_output() {
    let output = &mut LastValueProgramOutput::new();
    assert_eq!(0, output.value);

    output.output(1);
    assert_eq!(1, output.value);

    output.output(219);
    assert_eq!(219, output.value);
}

#[test]
fn test_run_program_add_multiply() {
    // Couple examples from day2, which just supported the add and multiply instructions.
    check_program(&vec![1,9,10,3,2,3,11,0,99,30,40,50], &vec![3500,9,10,70,2,3,11,0,99,30,40,50], 1);
    check_program(&vec![1,0,0,0,99], &vec![2,0,0,0,99], 1);
    check_program(&vec![2,3,0,3,99], &vec![2,3,0,6,99], 1);
    check_program(&vec![2,4,4,5,99,0], &vec![2,4,4,5,99,9801], 1);
    check_program(&vec![1,1,1,4,99,5,6,0,99], &vec![30,1,1,4,2,5,6,0,99], 1);
}

#[test]
fn test_run_program_all_instructions() {
    check_program(&vec![1101,100,-1,4,0], &vec![1101,100,-1,4,99], 1); // find 100 - 1, store in position 4.
    let output = check_program(&vec![3,3,104,0,99], &vec![3,3,104,50,99], 50); // Take input (50), store it in position 3, output the input.
    assert_eq!(output, 50);
}

/// Runs the given program, checking that it produced the expected result.  
/// Input is always the supplied input value, and this method returns the last outputed value
/// or 0 the program didn't output anything.
fn check_program(original_program: &Vec<i32>, expected: &Vec<i32>, input_value: i32) -> i32 {
    let input = &AlwaysValueProgramInput::new(input_value);
    let output = &mut LastValueProgramOutput::new();
    let program = &original_program.clone();

    let result = run_program(program, input, output);

    assert_eq!(original_program, program); // Original program shouldn't be modified.
    assert_eq!(expected, &result);

    output.value
}