use std::collections::HashMap;
use core::fmt;
use std::path::Path;
use std::fs::File;
use std::io::{BufReader, BufRead};

/// A parameter is an instruction input or output, and has a mode that determines how the value is treated.
#[derive(Eq, PartialEq, Debug, Copy, Clone)]
enum Parameter {
    /// 0 - parameters are interpreted as positions
    Position(usize),
    /// 1 - parameters are values
    Immediate(i64),
    /// 2 - parameters are pc + relative_base
    Relative(usize)
}

impl Parameter {
    fn value(self, memory: &Memory) -> i64 {
        match self {
            Parameter::Position(index) => memory.get(index),
            Parameter::Immediate(value) => value,
            Parameter::Relative(index) => memory.get(index) as i64,
        }
    }
}

type IndexParameter = usize;

/// Instruction type.  Instructions can have a variable number of program values, determined by their type.
#[derive(Eq, PartialEq, Debug, Copy, Clone)]
enum Instruction {
    /// Adds two numbers and stores them in a third. 1, 2, 3, 4 adds the numbers at 2 and 3 and stores them in 4.
    Add {
        a: Parameter,
        b: Parameter,
        out: IndexParameter,
    },
    /// Multiplys two numbers and stores them in a third. 1, 2, 3, 4 multiplies the numbers 2 and 3 and stores them in 4.
    Multiply {
        a: Parameter,
        b: Parameter,
        out: IndexParameter,
    },
    /// Takes an input value and saves it at a position.  3, 4 stores an input at 4
    Input { to: IndexParameter },
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
        out: IndexParameter
    },
    /// If the first parameter equals the second parameter, stores 1 in the position given by the third parameter.  Otherwise stores 0.
    Equals {
        a: Parameter,
        b: Parameter,
        out: IndexParameter
    },
    /// Adjusts the computer's relative base by the value of it's only parameter.  immediate 5 adds 5 to the relative base.
    RelativeBaseOffset {
        by: Parameter
    },
    /// Done with execution.  The program should stop after executing this instruction.
    Halt
}

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

        OpcodeModes { opcode, modes }
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
        let index = self.index(computer, parameter);

        match self.parameter_mode(parameter) { // Parameters start at index=instruction + 1, but modes is 0-indexed.
            0 => Parameter::Position(index),
            1 => Parameter::Immediate(index as i64),
            2 => Parameter::Relative(index),
            n => panic!("Invalid parameter mode {}", n),
        }
    }

    /// Returns an IndexParameter for the instruction at pc.
    /// Parameter is 0-indexed, so parameter(vec![3,3,203,50,99], &computer, 0) returns Parameter::Immediate(50).
    pub fn index(&self, computer: &Computer, parameter: usize) -> IndexParameter {
        match self.parameter_mode(parameter) {
            0 => computer.memory.get_usize(computer.pc + parameter + 1),
            1 => computer.memory.get_usize(computer.pc + parameter + 1),
            2 => (computer.memory.get(computer.pc + parameter + 1) + computer.relative_base as i64) as IndexParameter,
            n => panic!("Invalid parameter mode {}", n),
        }
    }
}

/// Memory contains a sparse representation of memory values.
struct Memory {
    values: HashMap<usize, i64>
}

impl fmt::Debug for Memory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
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
        let mut map = &mut HashMap::new();
        map = program.into_iter().enumerate()
            .fold(map, |map, (i, &value)| {
                map.insert(i, value);
                map
            });

        Memory { values: map.to_owned() }
    }

    /// Returns the value of the given memory address.  If the address has never been set,
    /// returns the default value of 0.
    fn get(&self, addr: usize) -> i64 {
        self.values.get(&addr).unwrap_or(&0).clone()
    }

    /// Returns the value of the given memory address as a usize.  Lots of instructions use memory
    /// values as memory indexes, so this method is handy for those cases.
    fn get_usize(&self, addr: usize) -> usize {
        self.get(addr) as usize
    }

    /// Sets the memory at the given address.
    fn set(&mut self, addr: usize, value: i64) {
        self.values.insert(addr, value);
    }
}

/// Computer input and output.
pub trait ProgramIO {
    fn input(&mut self) -> i64;
    fn output(&mut self, value: i64);
}

#[derive(Debug)]
pub struct Computer {
    pc:  usize,
    relative_base: usize,
    memory: Memory,
}

impl Computer {
    /// Constructs a new Computer that will run the given program once `run` is called.
    pub fn new(program: Vec<i64>) -> Computer {
        Computer {
            pc: 0,
            relative_base: 0,
            memory: Memory::for_program(&program),
        }
    }

    /// Constructs a new Computer that will run the program in the given file once `run` is called.
    pub fn from_file<P: AsRef<Path>>(file: P) -> std::io::Result<Computer> {
        let f = File::open(file)?;
        let mut reader = BufReader::new(f);
        let mut line = String::new();
        reader.read_line(&mut line)?;
        line = line.trim().to_string();

        let program = line
            .split(",")
            .flat_map(str::parse::<i64>)
            .collect::<Vec<_>>();

        Ok(Computer::new(program))
    }

    /// Runs the computer's program.  Calling this twice does nothing, since the program halts
    /// after the first run.
    pub fn run<IO: ProgramIO>(&mut self, io: &mut IO) {
        let mut instruction = self.parse_instruction();
        while instruction != Instruction::Halt {
            self.run_instruction(&instruction, io);
            instruction = self.parse_instruction();
        }
    }

    /// Parses the instruction at the given program counter, returning the instruction and new program counter.
    fn parse_instruction(&self) -> Instruction {
        // Opcode: last two digits are the instruction, proceeding are the modes for the parameters.
        let opcode_modes = OpcodeModes::parse(self.memory.get(self.pc));

        match opcode_modes.opcode {
            // Add two numbers and stores them in a third.
            1 => Instruction::Add {
                a: opcode_modes.parameter(&self, 0),
                b: opcode_modes.parameter(&self, 1),
                out: opcode_modes.index(&self, 2),
            },
            // Multiply two numbers and stores them in a third.
            2 => Instruction::Multiply {
                a: opcode_modes.parameter(&self, 0),
                b: opcode_modes.parameter(&self, 1),
                out: opcode_modes.index(&self, 2),
            },
            // Take an input value and saves it at a position.
            3 => Instruction::Input {
                to: opcode_modes.index(&self, 0)
            },
            // Output a value to a position.
            4 => Instruction::Output {
                from: opcode_modes.parameter(&self, 0)
            },
            // If the first parameter is non-zero, sets the program counter to the value from the second parameter
            5 => Instruction::JumpIfTrue {
                what: opcode_modes.parameter(&self, 0),
                to: opcode_modes.parameter(&self, 1),
            },
            // If the first parameter is zero, sets the program counter to the value from the second parameter
            6 => Instruction::JumpIfFalse {
                what: opcode_modes.parameter(&self, 0),
                to: opcode_modes.parameter(&self, 1),
            },
            // If the first parameter is less than the second parameter, stores 1 in the position given by the third parameter.  Otherwise stores 0.
            7 => Instruction::LessThan {
                a: opcode_modes.parameter(&self, 0),
                b: opcode_modes.parameter(&self, 1),
                out: opcode_modes.index(&self, 2),
            },
            // If the first parameter equals the second parameter, stores 1 in the position given by the third parameter.  Otherwise stores 0.
            8 => Instruction::Equals {
                a: opcode_modes.parameter(&self, 0),
                b: opcode_modes.parameter(&self, 1),
                out: opcode_modes.index(&self, 2),
            },
            9 => Instruction::RelativeBaseOffset {
                by: opcode_modes.parameter(&self, 0),
            },
            // Done with execution.  The program should stop after executing this instruction.
            99 => Instruction::Halt,
            n => panic!("Unknown opcode {} at pc {}", n, self.pc)
        }
    }

    /// Executes the given instruction, modifying the computer if applicable.
    fn run_instruction<IO: ProgramIO>(&mut self, instruction: &Instruction, io: &mut IO) {
        match instruction {
            // Add two numbers and stores them in a third.
            &Instruction::Add {a, b, out} => {
                self.memory.set(out, a.value(&self.memory) + b.value(&self.memory));
                self.pc += 4;
            }
            // Multiply two numbers and stores them in a third.
            &Instruction::Multiply {a, b, out} => {
                self.memory.set(out, a.value(&self.memory) * b.value(&self.memory));
                self.pc += 4;
            }
            // Take an input value and saves it at a position.
            &Instruction::Input {to} => {
                self.memory.set(to, io.input());
                self.pc += 2;
            }
            // Output a value to a position.
            &Instruction::Output {from} => {
                io.output(from.value(&self.memory));
                self.pc += 2;
            }
            // If the first parameter is non-zero, sets the program counter to the value from the second parameter
            &Instruction::JumpIfTrue {what, to} => {
                if what.value(&self.memory) != 0 {
                    self.pc = to.value(&self.memory) as usize;
                } else {
                    self.pc += 3;
                }
            }
            // If the first parameter is zero, sets the program counter to the value from the second parameter
            &Instruction::JumpIfFalse {what, to} => {
                if what.value(&self.memory) == 0 {
                    self.pc = to.value(&self.memory) as usize;
                } else {
                    self.pc += 3;
                }
            }
            // If the first parameter is less than the second parameter, stores 1 in the position given by the third parameter.  Otherwise stores 0.
            &Instruction::LessThan {a, b, out} => {
                self.memory.set(out, if a.value(&self.memory) < b.value(&self.memory) {1} else {0});
                self.pc += 4;
            }
            // If the first parameter equals the second parameter, stores 1 in the position given by the third parameter.  Otherwise stores 0.
            &Instruction::Equals {a, b, out} => {
                self.memory.set(out, if a.value(&self.memory) == b.value(&self.memory) {1} else {0});
                self.pc += 4;
            }
            &Instruction::RelativeBaseOffset {by} => {
                self.relative_base = (self.relative_base as i64 + by.value(&self.memory)) as usize;
                self.pc += 2;
            }
            // Done with execution.  The program should stop after executing this instruction.
            &Instruction::Halt => {
                panic!("Halt should not be run.");
            }
        }
    }
}