use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

/// A parameter is an instruction input or output, and has a mode that determines how the value is treated.
#[derive(Eq, PartialEq, Debug, Copy, Clone)]
enum Parameter {
    /// 0 - parameters are interpreted as positions
    Position(usize),
    /// 1 - parameters are values
    Immediate(i32),
}

/// Instruction type.  Instructions can have a variable number of program values, determined by their type.
#[derive(Eq, PartialEq, Debug, Copy, Clone)]
enum Instruction {
    /// Adds two numbers and stores them in a third. 1, 2, 3, 4 adds the numbers at 2 and 3 and stores them in 4.
    Add {
        a: Parameter,
        b: Parameter,
        out: usize,
    },
    /// Multiplys two numbers and stores them in a third. 1, 2, 3, 4 multiplies the numbers 2 and 3 and stores them in 4.
    Multiply {
        a: Parameter,
        b: Parameter,
        out: usize,
    },
    /// Takes an input value and saves it at a position.  3, 4 stores an input at 4
    Input { to: usize },
    /// Outputs a value to a position.  4, 5 outputs the value at 5.
    Output { from: Parameter },
    /// If the first parameter is non-zero, sets the program counter to the value from the second parameter
    JumpIfTrue { what: Parameter, to: Parameter },
    /// If the first parameter is zero, sets the program counter to the value from the second parameter
    JumpIfFalse { what: Parameter, to: Parameter },
    /// If the first parameter is less than the second parameter, stores 1 in the position given by the third parameter.  Otherwise stores 0.
    LessThan { 
        a: Parameter,
        b: Parameter,
        out: usize
     },
    /// If the first parameter equals the second parameter, stores 1 in the position given by the third parameter.  Otherwise stores 0.
    Equals { 
        a: Parameter,
        b: Parameter,
        out: usize
     },
    /// Done with execution.  The program should stop after executing this instruction.
    Halt
}

/// Parses the instruction at the given program counter, returning the instruction and new program counter.
fn parse_instruction(program: &Vec<i32>, pc: usize) -> Instruction {
    // Opcode: last two digits are the instruction, proceeding are the modes for the parameters.
    let opcode_modes = OpcodeModes::parse(program[pc]);

    match opcode_modes.opcode {
        // Add two numbers and stores them in a third.
        1 => Instruction::Add {
            a: opcode_modes.parameter(program, pc, 0),
            b: opcode_modes.parameter(program, pc, 1),
            out: program[pc + 3] as usize,
        },
        // Multiply two numbers and stores them in a third.
        2 => Instruction::Multiply {
            a: opcode_modes.parameter(program, pc, 0),
            b: opcode_modes.parameter(program, pc, 1),
            out: program[pc + 3] as usize,
        },
        // Take an input value and saves it at a position.
        3 => Instruction::Input { to: program[pc + 1] as usize },
        // Output a value to a position.
        4 => Instruction::Output { from: opcode_modes.parameter(program, pc, 0) },
        // If the first parameter is non-zero, sets the program counter to the value from the second parameter
        5 => Instruction::JumpIfTrue { 
            what: opcode_modes.parameter(program, pc, 0), 
            to: opcode_modes.parameter(program, pc, 1),
        },
        // If the first parameter is zero, sets the program counter to the value from the second parameter
        6 => Instruction::JumpIfFalse { 
            what: opcode_modes.parameter(program, pc, 0), 
            to: opcode_modes.parameter(program, pc, 1),
        },
        // If the first parameter is less than the second parameter, stores 1 in the position given by the third parameter.  Otherwise stores 0.
        7 => Instruction::LessThan { 
            a: opcode_modes.parameter(program, pc, 0),
            b: opcode_modes.parameter(program, pc, 1),
            out: program[pc + 3] as usize,
        },
        // If the first parameter equals the second parameter, stores 1 in the position given by the third parameter.  Otherwise stores 0.
        8 => Instruction::Equals { 
            a: opcode_modes.parameter(program, pc, 0),
            b: opcode_modes.parameter(program, pc, 1),
            out: program[pc + 3] as usize,
        },
        // Done with execution.  The program should stop after executing this instruction.
        99 => Instruction::Halt,
        n => panic!("Unknown opcode {} at pc {}", n, pc)
    }
}

/// Executes the given instruction, modifying the program if applicable.
fn run_instruction<IN: ProgramInput, OUT: ProgramOutput>(instruction: &Instruction, program: &mut Vec<i32>, pc: usize, input: &IN, output: &mut OUT) -> usize {
    match instruction {
        // Add two numbers and stores them in a third.
        &Instruction::Add {a, b, out} => {
            program[out] = value(a, program) + value(b, program);
            pc + 4
        }
        // Multiply two numbers and stores them in a third.
        &Instruction::Multiply {a, b, out} => {
            program[out] = value(a, program) * value(b, program);
            pc + 4
        }
        // Take an input value and saves it at a position.
        &Instruction::Input {to} => {
            program[to] = input.read();
            pc + 2
        }
        // Output a value to a position.
        &Instruction::Output {from} => {
            output.output(value(from, program));
            pc + 2
        }
        // If the first parameter is non-zero, sets the program counter to the value from the second parameter
        &Instruction::JumpIfTrue {what, to} => {
            if value(what, program) != 0 {
                value(to, program) as usize
            } else {
                pc + 3
            }
        }
        // If the first parameter is zero, sets the program counter to the value from the second parameter
        &Instruction::JumpIfFalse {what, to} => {
            if value(what, program) == 0 {
                value(to, program) as usize
            } else {
                pc + 3
            }
        }
        // If the first parameter is less than the second parameter, stores 1 in the position given by the third parameter.  Otherwise stores 0.
        &Instruction::LessThan {a, b, out} => {
            program[out] = if value(a, program) < value(b, program) {1} else {0};
            pc + 4
        }
        // If the first parameter equals the second parameter, stores 1 in the position given by the third parameter.  Otherwise stores 0.
        &Instruction::Equals {a, b, out} => {
            program[out] = if value(a, program) == value(b, program) {1} else {0};
            pc + 4
        }
        // Done with execution.  The program should stop after executing this instruction.
        &Instruction::Halt => {
            panic!("Halt should not be run.");
        }
    }
}

fn value(parameter: Parameter, program: &Vec<i32>) -> i32 {
    match parameter {
        Parameter::Position(index) => program[index],
        Parameter::Immediate(value) => value,
    }
}

#[derive(Debug)]
struct OpcodeModes {
    opcode: u32,
    modes: Vec<u32>,
}

impl OpcodeModes {
    /// Parses the given number into an OpcodeModes
    /// 102 means multiply, where a is in immediate mode and b and out are in position mode.
    pub fn parse(number: i32) -> OpcodeModes {
        let mut parsing_number = number;
        
        let opcode = (parsing_number % 100) as u32;
        parsing_number /= 100;

        let mut modes = Vec::<u32>::new();
        while parsing_number > 0 {
            modes.push((parsing_number % 10) as u32);
            parsing_number /= 10;
        }

        OpcodeModes {
            opcode: opcode,
            modes: modes,
        }
    }

    /// Returns the mode for the parameter at the given index.
    fn parameter_mode(&self, index: usize) -> u32 {
        if index < self.modes.len() {
            self.modes[index]
        } else {
            0
        }
    }

    /// Returns a parameter for the instruction at pc.
    /// Parameter is 0-indexed, so parameter(vec![3,3,104,50,99], 2, 0) returns Parameter::Immediate(50).
    pub fn parameter(&self, program: &Vec<i32>, pc: usize, parameter: usize) -> Parameter {
        match self.parameter_mode(parameter) { // Parameters start at index=instruction + 1, but modes is 0-indexed.
            0 => Parameter::Position(program[pc + parameter + 1] as usize),
            1 => Parameter::Immediate(program[pc + parameter + 1]),
            n => panic!("Invalid parameter mode {}", n),
        }
    }
}

trait ProgramInput {
    fn read(&self) -> i32;
}

struct AlwaysValueProgramInput {
    value: i32,
}

impl AlwaysValueProgramInput {
    pub fn new(value: i32) -> AlwaysValueProgramInput {
        AlwaysValueProgramInput { value: value }
    }
}

impl ProgramInput for AlwaysValueProgramInput {
    fn read(&self) -> i32 {
        self.value
    }
}

trait ProgramOutput {
    fn output(&mut self, value: i32);
}

struct LastValueProgramOutput {
    value: i32,
}

impl LastValueProgramOutput {
    fn new() -> LastValueProgramOutput{
        LastValueProgramOutput {value: 0}
    }
}

impl ProgramOutput for LastValueProgramOutput {
    fn output(&mut self, value: i32) {
        self.value = value;
    }
}

/// Runs the given program, using the input and output to serve input and output instructions.
fn run_program<IN: ProgramInput, OUT: ProgramOutput>(program: &Vec<i32>, input: &IN, output: &mut OUT) -> Vec<i32> {
    let ref mut running_program = program.clone();
    
    let mut pc = 0;
    let mut instruction = parse_instruction(&running_program, pc);

    while instruction != Instruction::Halt {
        pc = run_instruction(&instruction, running_program, pc, input, output);
        instruction = parse_instruction(&running_program, pc);
    }

    running_program.to_owned()
}

#[cfg(test)]
mod test;

fn main() -> std::io::Result<()> {
    let f = File::open("input.txt")?;
    let mut reader = BufReader::new(f);
    let mut line = String::new();
    reader.read_line(&mut line)?;
    line = line.trim().to_string();
    
    let program = line
        .split(",")
        .flat_map(str::parse::<i32>)
        .collect::<Vec<_>>();

    // Part 1: passing 1 as input, what does the program output?
    let part1_input = &AlwaysValueProgramInput::new(1);
    let part1_output = &mut LastValueProgramOutput::new();
    run_program(&program, part1_input, part1_output);

    println!("Part 1: {}", part1_output.value);

    // Part 2: passing 5 as input, what does the program output?
    let part2_input = &AlwaysValueProgramInput::new(5);
    let part2_output = &mut LastValueProgramOutput::new();
    run_program(&program, part2_input, part2_output);

    println!("Part 2: {}", part2_output.value);

    Ok(())
}
