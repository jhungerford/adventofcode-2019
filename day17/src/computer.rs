use core::fmt;
use std::collections::{HashMap, VecDeque};
use std::fmt::{Debug, Formatter};
use std::fs::File;
use std::io::{BufRead, BufReader};

/// Opcode modes is a number that contains an opcode and parameters.  The opcode is in the
/// last two digits, other digits encode the parameter mode from right to left.
/// For example, 1002 means opcode '02', with the first parameter in mode 0, the second in mode 1,
/// and the third parameter in mode 0.  The third parameter mode is omitted in the instruction.
#[derive(Debug)]
struct OpcodeModes {
    opcode: u32,
    modes: Vec<u32>,
}

impl OpcodeModes {
    /// Parses the given number into an OpcodeModes
    /// 102 means multiply, where a is in immediate mode and b and out are in position mode.
    pub fn parse(number: i64) -> OpcodeModes {
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
    pub fn parameter(&self, computer: &Computer, parameter: usize) -> Parameter {
        // Parameters start at index=instruction + 1, but modes is 0-indexed.
        let parameter_mode = self.parameter_mode(parameter);
        let parameter_value = computer.memory.get(computer.pc + parameter + 1);

        match parameter_mode {
            0 => Parameter::Position(parameter_value as usize),
            1 => Parameter::Immediate(parameter_value),
            2 => Parameter::Relative((parameter_value + computer.relative_base) as usize),
            n => panic!("Invalid parameter mode {}", n),
        }
    }

    /// Returns the index where an instruction should store a value.
    pub fn index_parameter(&self, computer: &Computer, parameter: usize) -> usize {
        let parameter_mode = self.parameter_mode(parameter);
        let parameter_value = computer.memory.get(computer.pc + parameter + 1);

        match parameter_mode {
            0 | 1 => parameter_value as usize,
            2 => (parameter_value + computer.relative_base) as usize,
            n => panic!("Invalid parameter index for mode {}", n),
        }
    }
}

/// A parameter is an instruction input or output, and has a mode that determines how the value is treated.
#[derive(Eq, PartialEq, Debug, Clone)]
enum Parameter {
    /// 0 - parameters are interpreted as positions
    Position(usize),
    /// 1 - parameters are values
    Immediate(i64),
    /// 2 - parameters are pc + relative base
    Relative(usize),
}

impl Parameter {
    /// Returns the value of this parameter in the given program.
    fn value(&self, memory: &Memory) -> i64 {
        match self {
            &Parameter::Position(index) => memory.get(index),
            &Parameter::Immediate(value) => value,
            &Parameter::Relative(index) => memory.get(index),
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
    /// Adjusts the relative base by the value of its only parameter. The relative base increases (or decreases, if the value is negative) by the value of the parameter.
    RelativeBaseOffset {
        by: Parameter
    },
    /// Done with execution.  The program should stop after executing this instruction.
    Halt,
}

impl Instruction {
    /// Parses the instruction at the given program counter.
    fn parse(computer: &Computer) -> Instruction {
        let pc = computer.pc;

        // Opcode: last two digits are the instruction, proceeding are the modes for the parameters.
        let opcode_modes = OpcodeModes::parse(computer.memory.get(pc));

        match opcode_modes.opcode {
            // Add two numbers and stores them in a third.
            1 => Instruction::Add {
                a: opcode_modes.parameter(computer, 0),
                b: opcode_modes.parameter(computer, 1),
                out: opcode_modes.index_parameter(computer, 2),
            },
            // Multiply two numbers and stores them in a third.
            2 => Instruction::Multiply {
                a: opcode_modes.parameter(computer, 0),
                b: opcode_modes.parameter(computer, 1),
                out: opcode_modes.index_parameter(computer, 2),
            },
            // Take an input value and saves it at a position.
            3 => Instruction::Input {
                to: opcode_modes.index_parameter(computer, 0),
            },
            // Output a value to a position.
            4 => Instruction::Output {
                from: opcode_modes.parameter(computer, 0),
            },
            // If the first parameter is non-zero, sets the program counter to the value from the second parameter
            5 => Instruction::JumpIfTrue {
                what: opcode_modes.parameter(computer, 0),
                to: opcode_modes.parameter(computer, 1),
            },
            // If the first parameter is zero, sets the program counter to the value from the second parameter
            6 => Instruction::JumpIfFalse {
                what: opcode_modes.parameter(computer, 0),
                to: opcode_modes.parameter(computer, 1),
            },
            // If the first parameter is less than the second parameter, stores 1 in the position given by the third parameter.  Otherwise stores 0.
            7 => Instruction::LessThan {
                a: opcode_modes.parameter(computer, 0),
                b: opcode_modes.parameter(computer, 1),
                out: opcode_modes.index_parameter(computer, 2),
            },
            // If the first parameter equals the second parameter, stores 1 in the position given by the third parameter.  Otherwise stores 0.
            8 => Instruction::Equals {
                a: opcode_modes.parameter(computer, 0),
                b: opcode_modes.parameter(computer, 1),
                out: opcode_modes.index_parameter(computer, 2),
            },
            // Opcode 9 adjusts the relative base by the value of its only parameter.  The relative base increases (or decreases, if the value is negative) by the value of the parameter.
            9 => Instruction::RelativeBaseOffset {
                by: opcode_modes.parameter(computer, 0),
            },
            // Done with execution.  The program should stop after executing this instruction.
            99 => Instruction::Halt,
            n => panic!("Unknown opcode {} at pc {}", n, pc),
        }
    }

    /// Runs this instruction, modifying the computer if applicable.  Returns the program state.
    fn run(&self, computer: &mut Computer) -> ProgramState {
        let pc = computer.pc;

        computer.pc = match self {
            // Add two numbers and stores them in a third.
            Instruction::Add { a, b, out } => {
                computer.memory.set(*out, a.value(&computer.memory) + b.value(&computer.memory));
                pc + 4
            }
            // Multiply two numbers and stores them in a third.
            Instruction::Multiply { a, b, out } => {
                computer.memory.set(*out, a.value(&computer.memory) * b.value(&computer.memory));
                pc + 4
            }
            // Take an input value and saves it at a position.
            Instruction::Input { to } => {
                if let Some(input) = computer.input.pop_front() {
                    computer.memory.set(*to, input);
                    pc + 2
                } else {
                    return ProgramState::WaitingForInput;
                }
            }
            // Output a value to a position.
            Instruction::Output { from } => {
                computer.output.push_back(from.value(&computer.memory));
                pc + 2
            }
            // If the first parameter is non-zero, sets the program counter to the value from the second parameter
            Instruction::JumpIfTrue { what, to } => {
                if what.value(&computer.memory) != 0 {
                    to.value(&computer.memory) as usize
                } else {
                    pc + 3
                }
            }
            // If the first parameter is zero, sets the program counter to the value from the second parameter
            Instruction::JumpIfFalse { what, to } => {
                if what.value(&computer.memory) == 0 {
                    to.value(&computer.memory) as usize
                } else {
                    pc + 3
                }
            }
            // If the first parameter is less than the second parameter, stores 1 in the position given by the third parameter.  Otherwise stores 0.
            Instruction::LessThan { a, b, out } => {
                computer.memory.set(*out, if a.value(&computer.memory) < b.value(&computer.memory) {
                    1
                } else {
                    0
                });
                pc + 4
            }
            // If the first parameter equals the second parameter, stores 1 in the position given by the third parameter.  Otherwise stores 0.
            Instruction::Equals { a, b, out } => {
                computer.memory.set(*out, if a.value(&computer.memory) == b.value(&computer.memory) {
                    1
                } else {
                    0
                });
                pc + 4
            }
            Instruction::RelativeBaseOffset { by } => {
                computer.relative_base += by.value(&computer.memory);
                computer.pc + 2
            }
            // Done with execution.  The program should stop after executing this instruction.
            Instruction::Halt => {
                return ProgramState::Done;
            }
        };

        ProgramState::Runnable
    }
}

/// Memory contains a sparse representation of memory values.
#[derive(Clone)]
pub struct Memory {
    values: HashMap<usize, i64>
}

impl Debug for Memory {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut keys: Vec<_> = self.values.keys().collect();
        keys.sort();
        keys.into_iter().fold(Ok(true), |result, key| {
            // Success value is whether the key is the first in the list, which shouldn't have a comma before it.
            match result {
                Ok(true) => write!(f, "({}: {})", key, self.values.get(key).unwrap()),
                Ok(false) => write!(f, ", ({}: {})", key, self.values.get(key).unwrap()),
                Err(e) => Err(e),
            }.and_then(|_| Ok(false))
        }).and_then(|_| Ok(()))
    }
}

impl Memory {
    /// Constructs a new memory initialized with the instructions in the program.
    fn for_program(program: &Vec<i64>) -> Memory {
        let values = program.iter().cloned().enumerate().collect::<HashMap<usize, i64>>();

        Memory { values }
    }

    /// Returns the value of the given memory address.  If the address has never been set,
    /// returns the default value of 0.
    pub fn get(&self, addr: usize) -> i64 {
        self.values.get(&addr).unwrap_or(&0).clone()
    }

    /// Sets the memory at the given address.
    pub fn set(&mut self, addr: usize, value: i64) {
        self.values.insert(addr, value);
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
    pc: usize,
    relative_base: i64,
    state: ProgramState,
    input: VecDeque<i64>,
    pub output: VecDeque<i64>,
    pub memory: Memory,
}

impl Computer {
    /// Loads the intcode program in the given file into a computer.
    pub fn load(filename: &str) -> Computer {
        let f = File::open(filename).unwrap();
        let mut reader = BufReader::new(f);
        let mut line = String::new();
        reader.read_line(&mut line).unwrap();
        line = line.trim().to_string();

        let program = line
            .split(",")
            .flat_map(str::parse::<i64>)
            .collect::<Vec<_>>();

        Computer::new(program)
    }

    /// Constructs a new Computer that will run the given program.
    pub fn new(program: Vec<i64>) -> Computer {
        Computer {
            pc: 0,
            relative_base: 0,
            state: ProgramState::Runnable,
            input: VecDeque::new(),
            output: VecDeque::new(),
            memory: Memory::for_program(&program),
        }
    }

    /// Uses the given value as the next input to this computer.  Inputs will be used
    /// in the order they were provided if input is called multiple times.
    pub fn input(&mut self, value: i64) {
        self.input.push_back(value);

        if self.state == ProgramState::WaitingForInput {
            self.state = ProgramState::Runnable;
        }
    }

    /// Provides the given ascii string as input for the computer.
    pub fn text_input(&mut self, value: &str) {
        println!("> {}", value);
        value.chars().for_each(|c| self.input(c as i64));
    }

    /// Passes each value in the output to the given visitor.
    pub fn visit_output(&self, visit: fn(&i64)) {
        self.output.iter().for_each(visit)
    }

    /// Returns the last value this computer output, or empty if it hasn't output any values.
    pub fn last_output(&self) -> Option<i64> {
        self.output.back().cloned()
    }

    /// Prints any new ascii output from the computer that hasn't been printed yet.
    /// Consumes the output, so the output methods won't see the output again.
    pub fn print_output(&mut self) {
        while let Some(value) = self.output.pop_front() {
            print!("{}", value as u8 as char);
        }
    }

    /// Returns whether this computer can run - if it isn't done or waiting for input.
    pub fn is_runnable(&self) -> bool {
        self.state == ProgramState::Runnable
    }

    /// Runs the program in this computer until it either halts or blocks waiting for input.
    /// Returns the last value the program output.
    pub fn run(&mut self) -> Option<i64> {
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
