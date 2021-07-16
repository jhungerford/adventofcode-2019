use std::collections::VecDeque;
use std::fs::File;
use std::io::{BufReader, BufRead};

/// A parameter is an instruction input or output, and has a mode that determines how the value is treated.
#[derive(Eq, PartialEq, Debug, Clone)]
enum Parameter {
    /// 0 - parameters are interpreted as positions
    Position(usize),
    /// 1 - parameters are values
    Immediate(i32),
}

impl Parameter {
    /// Returns the value of this parameter in the given program.
    fn value(&self, program: &Vec<i32>) -> i32 {
        match self {
            Parameter::Position(index) => program[*index],
            Parameter::Immediate(value) => *value,
        }
    }
}

/// Instruction type.  Instructions can have a variable number of program values, determined by their type.
#[derive(Eq, PartialEq, Debug, Clone)]
enum Instruction {
    /// Adds two numbers and stores them in a third. 1, 2, 3, 4 adds the numbers at 2 and 3 and stores them in 4.
    Add {
        a: Parameter,
        b: Parameter,
        out: usize,
    },
    /// Multiplies two numbers and stores them in a third. 1, 2, 3, 4 multiplies the numbers 2 and 3 and stores them in 4.
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
        out: usize,
    },
    /// If the first parameter equals the second parameter, stores 1 in the position given by the third parameter.  Otherwise stores 0.
    Equals {
        a: Parameter,
        b: Parameter,
        out: usize,
    },
    /// Done with execution.  The program should stop after executing this instruction.
    Halt,
}

impl Instruction {
    /// Parses the instruction at the given program counter.
    fn parse(computer: &Computer) -> Instruction {
        let program = &computer.program;
        let pc = computer.pc;

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
            3 => Instruction::Input {
                to: program[pc + 1] as usize,
            },
            // Output a value to a position.
            4 => Instruction::Output {
                from: opcode_modes.parameter(program, pc, 0),
            },
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
            n => panic!("Unknown opcode {} at pc {}", n, pc),
        }
    }

    /// Runs this instruction, modifying the computer if applicable.  Returns the program state.
    fn run(&self, computer: &mut Computer) -> ProgramState {
        let program = &mut computer.program;
        let pc = computer.pc;

        computer.pc = match self {
            // Add two numbers and stores them in a third.
            Instruction::Add { a, b, out } => {
                computer.program[*out] = a.value(program) + b.value(program);
                pc + 4
            }
            // Multiply two numbers and stores them in a third.
            Instruction::Multiply { a, b, out } => {
                program[*out] = a.value(program) * b.value(program);
                pc + 4
            }
            // Take an input value and saves it at a position.
            Instruction::Input { to } => {
                if let Some(input) = computer.input.pop_front() {
                    program[*to] = input;
                    pc + 2
                } else {
                    return ProgramState::WaitingForInput;
                }
            }
            // Output a value to a position.
            Instruction::Output { from } => {
                computer.output.push_back(from.value(program));
                pc + 2
            }
            // If the first parameter is non-zero, sets the program counter to the value from the second parameter
            Instruction::JumpIfTrue { what, to } => {
                if what.value(program) != 0 {
                    to.value(program) as usize
                } else {
                    pc + 3
                }
            }
            // If the first parameter is zero, sets the program counter to the value from the second parameter
            Instruction::JumpIfFalse { what, to } => {
                if what.value(program) == 0 {
                    to.value(program) as usize
                } else {
                    pc + 3
                }
            }
            // If the first parameter is less than the second parameter, stores 1 in the position given by the third parameter.  Otherwise stores 0.
            Instruction::LessThan { a, b, out } => {
                program[*out] = if a.value(program) < b.value(program) {
                    1
                } else {
                    0
                };
                pc + 4
            }
            // If the first parameter equals the second parameter, stores 1 in the position given by the third parameter.  Otherwise stores 0.
            Instruction::Equals { a, b, out } => {
                program[*out] = if a.value(program) == b.value(program) {
                    1
                } else {
                    0
                };
                pc + 4
            }
            // Done with execution.  The program should stop after executing this instruction.
            Instruction::Halt => {
                return ProgramState::Done;
            }
        };

        ProgramState::Runnable
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
        match self.parameter_mode(parameter) {
            // Parameters start at index=instruction + 1, but modes is 0-indexed.
            0 => Parameter::Position(program[pc + parameter + 1] as usize),
            1 => Parameter::Immediate(program[pc + parameter + 1]),
            n => panic!("Invalid parameter mode {}", n),
        }
    }
}

/// State of the program at a specific program counter.
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ProgramState {
    Done,
    Runnable,
    WaitingForInput,
}

/// Intcode computer.
#[derive(Debug, Clone)]
pub struct Computer {
    program: Vec<i32>,
    pc: usize,
    input: VecDeque<i32>,
    output: VecDeque<i32>,
    state: ProgramState,
}

impl Computer {
    /// Loads the intcode program in the given file into a computer.
    pub fn load(filename: &str) -> std::io::Result<Computer> {
        let f = File::open(filename)?;
        let mut reader = BufReader::new(f);
        let mut line = String::new();
        reader.read_line(&mut line)?;
        line = line.trim().to_string();

        let program = line
            .split(",")
            .flat_map(str::parse::<i32>)
            .collect::<Vec<_>>();

        Ok(Computer::new(program))
    }

    /// Constructs a new Computer that will run the given program.
    pub fn new(program: Vec<i32>) -> Computer {
        Computer {
            program,
            pc: 0,
            input: VecDeque::new(),
            output: VecDeque::new(),
            state: ProgramState::Runnable,
        }
    }

    /// Uses the given value as the next input to this computer.  Inputs will be used
    /// in the order they were provided if input is called multiple times.
    pub fn input(&mut self, value: i32) {
        self.input.push_back(value);

        if self.state == ProgramState::WaitingForInput {
            self.state = ProgramState::Runnable;
        }
    }

    /// Returns the last value this computer output, or empty if it hasn't output any values.
    pub fn last_output(&self) -> Option<i32> {
        self.output.back().cloned()
    }

    /// Returns whether this computer can run - if it isn't done or waiting for input.
    pub fn is_runnable(&self) -> bool {
        self.state == ProgramState::Runnable
    }

    /// Runs the program in this computer until it either halts or blocks waiting for input.
    /// Returns the last value the program output.
    pub fn run(&mut self) -> Option<i32> {
        while self.is_runnable() {
            self.step();
        }

        self.last_output()
    }

    /// Runs the next instruction in the program, if possible, returning the program state.
    fn step(&mut self) {
        self.state = Instruction::parse(self).run(self);
    }
}
