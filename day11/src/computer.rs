use std::collections::HashMap;
use core::fmt;
use std::path::Path;
use std::fs::File;
use std::io::{BufReader, BufRead};

/// A parameter is an instruction input or output, and has a mode that determines how the value is treated.
#[derive(Eq, PartialEq, Debug, Copy, Clone)]
struct Parameter {
    index: IndexParameter,
    mode: ParameterMode,
}

/// ParameterMode determines how a parameter is interpreted.
#[derive(Eq, PartialEq, Debug, Copy, Clone)]
enum ParameterMode {
    /// 0 - parameters are interpreted as positions
    Position,
    /// 1 - parameters are values
    Immediate,
    /// 2 - parameters are pc + relative_base
    Relative,
}

impl ParameterMode {
    fn from_value(value: usize) -> ParameterMode {
        match value {
            0 => ParameterMode::Position,
            1 => ParameterMode::Immediate,
            2 => ParameterMode::Relative,
            n => panic!("Invalid parameter mode {}", n),
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

impl Instruction {
    /// Returns the instruction at the computer's program counter.
    fn parse(computer: &Computer) -> Instruction {
        use Instruction::*;

        // Opcode: last two digits are the instruction, proceeding are the modes for the parameters.
        let opcode_modes = OpcodeModes::for_computer(computer);

        match opcode_modes.opcode {
            // Add two numbers and stores them in a third.
            1 => Add {
                a: opcode_modes.parameter(0),
                b: opcode_modes.parameter(1),
                out: opcode_modes.parameter(2).index,
            },
            // Multiply two numbers and stores them in a third.
            2 => Multiply {
                a: opcode_modes.parameter(0),
                b: opcode_modes.parameter(1),
                out: opcode_modes.parameter(2).index,
            },
            // Take an input value and saves it at a position.
            3 => Input {
                to: opcode_modes.parameter(0).index
            },
            // Output a value to a position.
            4 => Output {
                from: opcode_modes.parameter(0)
            },
            // If the first parameter is non-zero, sets the program counter to the value from the second parameter
            5 => JumpIfTrue {
                what: opcode_modes.parameter(0),
                to: opcode_modes.parameter(1),
            },
            // If the first parameter is zero, sets the program counter to the value from the second parameter
            6 => JumpIfFalse {
                what: opcode_modes.parameter(0),
                to: opcode_modes.parameter(1),
            },
            // If the first parameter is less than the second parameter, stores 1 in the position given by the third parameter.  Otherwise stores 0.
            7 => LessThan {
                a: opcode_modes.parameter(0),
                b: opcode_modes.parameter(1),
                out: opcode_modes.parameter(2).index,
            },
            // If the first parameter equals the second parameter, stores 1 in the position given by the third parameter.  Otherwise stores 0.
            8 => Equals {
                a: opcode_modes.parameter(0),
                b: opcode_modes.parameter(1),
                out: opcode_modes.parameter(2).index,
            },
            9 => RelativeBaseOffset {
                by: opcode_modes.parameter(0),
            },
            // Done with execution.  The program should stop after executing this instruction.
            99 => Halt,
            n => panic!("Unknown opcode {} at pc {}", n, computer.pc)
        }
    }

    /// Executes the given instruction, modifying the computer if applicable.
    fn run<IO: ProgramIO>(self, computer: &mut Computer, io: &mut IO) {
        use Instruction::*;

        match self {
            // Add two numbers and stores them in a third.
            Add {a, b, out} => {
                computer.memory.set(out, computer.memory.get(a) + computer.memory.get(b));
                computer.pc += 4;
            }
            // Multiply two numbers and stores them in a third.
            Multiply {a, b, out} => {
                computer.memory.set(out, computer.memory.get(a) * computer.memory.get(b));
                computer.pc += 4;
            }
            // Take an input value and saves it at a position.
            Input {to} => {
                computer.memory.set(to, io.input());
                computer.pc += 2;
            }
            // Output a value to a position.
            Output {from} => {
                io.output(computer.memory.get(from));
                computer.pc += 2;
            }
            // If the first parameter is non-zero, sets the program counter to the value from the second parameter
            JumpIfTrue {what, to} => {
                if computer.memory.get(what) != 0 {
                    computer.pc = computer.memory.get(to) as usize;
                } else {
                    computer.pc += 3;
                }
            }
            // If the first parameter is zero, sets the program counter to the value from the second parameter
            JumpIfFalse {what, to} => {
                if computer.memory.get(what) == 0 {
                    computer.pc = computer.memory.get(to) as usize;
                } else {
                    computer.pc += 3;
                }
            }
            // If the first parameter is less than the second parameter, stores 1 in the position given by the third parameter.  Otherwise stores 0.
            LessThan {a, b, out} => {
                computer.memory.set(out, if computer.memory.get(a) < computer.memory.get(b) {1} else {0});
                computer.pc += 4;
            }
            // If the first parameter equals the second parameter, stores 1 in the position given by the third parameter.  Otherwise stores 0.
            Equals {a, b, out} => {
                computer.memory.set(out, if computer.memory.get(a) == computer.memory.get(b) {1} else {0});
                computer.pc += 4;
            }
            RelativeBaseOffset {by} => {
                computer.relative_base = (computer.relative_base as i64 + computer.memory.get(by)) as usize;
                computer.pc += 2;
            }
            // Done with execution.  The program should stop after executing this instruction.
            Halt => panic!("Halt should not be run.")
        }
    }
}

#[derive(Debug)]
struct OpcodeModes<'a> {
    opcode: usize,
    modes: Vec<usize>,
    computer: &'a Computer,
}

/// OpcodeModes parses the modes for parameters for instructions.  Instructions are formatted
/// as modes and an opcode.  The last two digits of the instruction are the opcode, and the
/// preceding digits are the modes.  For example, 2102 is the add instruction (02), where
/// parameter 'a' is in immediate mode (1) and parameter 'b' is in relative mode (2).
/// If a parameter doesn't have an explicit mode, it's in position mode (0).
impl OpcodeModes<'_> {
    /// Parses the computer's current instruction into an OpcodeModes.
    /// 102 means multiply, where a is in immediate mode and b and out are in position mode.
    pub fn for_computer(computer: &Computer) -> OpcodeModes<'_> {
        let mut parsing_number = computer.memory.get_value(computer.pc) as usize;

        let opcode = parsing_number % 100;
        parsing_number /= 100;

        let mut modes = Vec::new();
        while parsing_number > 0 {
            modes.push(parsing_number % 10);
            parsing_number /= 10;
        }

        OpcodeModes { opcode, modes, computer }
    }

    /// Returns the given parameter.  Parameter is 0-indexed, so a computer with memory
    /// `vec![3,3,104,50,99]` at pc `2` returns `Parameter::Immediate(50)` for `parameter(0)`.
    pub fn parameter(&self, parameter: usize) -> Parameter {
        use ParameterMode::*;

        // Parameters that don't have an explicit opcode mode default to position (0)
        let mode = ParameterMode::from_value(if parameter < self.modes.len() { self.modes[parameter] } else { 0 });
        let index = match mode {
            Position | Immediate => self.computer.memory.get_value(self.computer.pc + parameter + 1),
            Relative => (self.computer.memory.get_value(self.computer.pc + parameter + 1) + self.computer.relative_base as i64),
        } as IndexParameter;

        Parameter {index, mode}
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

    /// Returns the value of the given parameter.
    fn get(&self, parameter: Parameter) -> i64 {
        use ParameterMode::*;

        match parameter.mode {
            Position | Relative => self.get_value(parameter.index),
            Immediate => parameter.index as i64,
        }
    }

    /// Returns the value of the given memory address.  If the address has never been set,
    /// returns the default value of 0.
    fn get_value(&self, addr: usize) -> i64 {
        self.values.get(&addr).unwrap_or(&0).clone()
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
        let mut instruction = Instruction::parse(self);

        while instruction != Instruction::Halt {
            instruction.run(self, io);
            instruction = Instruction::parse(self);
        }
    }
}