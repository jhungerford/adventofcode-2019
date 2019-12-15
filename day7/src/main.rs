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
fn run_instruction<IN: ProgramInput, OUT: ProgramOutput>(instruction: &Instruction, program: &mut Vec<i32>, pc: usize, input: &mut IN, output: &mut OUT) -> usize {
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
    fn read(&mut self) -> i32;
}

/// Program input that returns the values in order, then panics if any more input is requestedd
struct ListProgramInput {
    values: Vec<i32>,
    index: usize,
}

impl ListProgramInput {
    pub fn new(values: Vec<i32>) -> ListProgramInput {
        ListProgramInput {values: values, index: 0}
    }
}

impl ProgramInput for ListProgramInput {
    fn read(&mut self) -> i32 {
        let value = self.values[self.index];
        self.index += 1;
        value
    }
}

trait ProgramOutput {
    fn output(&mut self, value: i32);
}

/// Program output that only remembers that last value that was outputed.
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
fn run_program<IN: ProgramInput, OUT: ProgramOutput>(program: &Vec<i32>, input: &mut IN, output: &mut OUT) -> Vec<i32> {
    let ref mut running_program = program.clone();
    
    let mut pc = 0;
    let mut instruction = parse_instruction(&running_program, pc);

    while instruction != Instruction::Halt {
        pc = run_instruction(&instruction, running_program, pc, input, output);
        instruction = parse_instruction(&running_program, pc);
    }

    running_program.to_owned()
}

/// Runs the program on the given phase setting (0-4) and input, and returns the result.
fn amplifier_output(program: &Vec<i32>, phase_setting: i32, input: i32) -> i32 {
    let program_input = &mut ListProgramInput::new(vec![phase_setting, input]);
    let program_output = &mut LastValueProgramOutput::new();

    run_program(program, program_input, program_output);

    program_output.value
}

/// Runs the program on a series of amplifiers, using the given phase settings and passing the output from one amp to the next.
fn chain_output(program: &Vec<i32>, phase_settings: Vec<i32>) -> i32 {
    phase_settings.iter().fold(0, |input, phase_setting| amplifier_output(program, *phase_setting, input))
}

/*
/// Given an array of items, returns an iterator that visits all of the permutations.
/// https://en.wikipedia.org/wiki/Heap%27s_algorithm
fn permutations<T: Clone + Copy + Sized>(items: &[T]) -> Vec<&Vec<T>> {
    let mut acc = Vec::new();

    let vec_items = items.to_vec();
    permutations_inner(items.len(), vec_items, &mut acc);
    
    acc.to_owned()
}

fn permutations_inner<'a, T: Clone + Copy + Sized>(k: usize, items: Vec<T>, acc: &mut Vec<&'a Vec<T>>) {
    if k == 1 {
        acc.push(&items);
    } else {
        // Generate permutations with the k-th unaltered.  Initially, k == length(items)
        permutations_inner(k - 1, items, acc);

        // Generate permutations for k-th swapped with each k-1 initial.
        for i in 1..k as usize - 1 {
            // Swap choice is dependent on k's parity - even means swap with i, odd means swap with 0.
            let to_index = if k % 2 == 0 {i} else {0};

            // let items_copy: &'a mut Vec<T> = items.to_vec();
            // let items_copy = &mut items.to_vec();
            // items_copy.swap(k - 1, to_index);

            let mut vec_items = items.to_vec();
            vec_items.swap(k-1, to_index);

            permutations_inner(k - 1, vec_items.to_owned(), acc);
        }
    }
}
*/

// I can't figure out how to enable #[feature(collections)] to get permutations from std::slice, so permutations() is from:
// https://doc.rust-lang.org/1.1.0/src/collections/slice.rs.html#148

#[derive(Copy, Clone)]
enum Direction { Pos, Neg }

#[derive(Copy, Clone)]
struct SizeDirection {
    size: usize,
    dir: Direction,
}

struct ElementSwaps {
    sdir: Vec<SizeDirection>,
    emit_reset: bool,
    swaps_made: usize,
}

impl ElementSwaps {
    fn new(length: usize) -> ElementSwaps {
        ElementSwaps {
            emit_reset: true,
            sdir: (0..length).map(|i| SizeDirection{ size: i, dir: Direction::Neg }).collect(),
            swaps_made: 0
        }
    }
}

impl Iterator for ElementSwaps {
    type Item = (usize, usize);

    #[inline]
    fn next(&mut self) -> Option<(usize, usize)> {
        fn new_pos_wrapping(i: usize, s: Direction) -> usize {
            i.wrapping_add(match s { Direction::Pos => 1, Direction::Neg => !0 })
        }

        fn new_pos(i: usize, s: Direction) -> usize {
            match s { Direction::Pos => i + 1, Direction::Neg => i - 1 }
        }

        let max = self.sdir.iter().cloned().enumerate()
            .filter(|&(i, sd)|
                new_pos_wrapping(i, sd.dir) < self.sdir.len() &&
                self.sdir[new_pos(i, sd.dir)].size < sd.size)
            .max_by(|&(_, sd1), &(_, sd2)| sd1.size.cmp(&sd2.size));

        match max {
            Some((i, sd)) => {
                let j = new_pos(i, sd.dir);
                self.sdir.swap(i, j);

                // Swap the direction of each larger SizeDirection
                for x in &mut self.sdir {
                    if x.size > sd.size {
                        x.dir = match x.dir { Direction::Pos => Direction::Neg, Direction::Neg => Direction::Pos };
                    }
                }
                self.swaps_made += 1;
                Some((i, j))
            },
            None => if self.emit_reset {
                self.emit_reset = false;
                if self.sdir.len() > 1 {
                    // The last swap
                    self.swaps_made += 1;
                    Some((0, 1))
                } else {
                    // Vector is of the form [] or [x], and the only permutation is itself
                    self.swaps_made += 1;
                    Some((0,0))
                }
            } else { None }
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        // For a vector of size n, there are exactly n! permutations.
        let n: usize = (2..self.sdir.len() + 1).product();
        (n - self.swaps_made, Some(n - self.swaps_made))
    }
}

impl<T: Clone> Iterator for Permutations<T> {
    type Item = Vec<T>;

    #[inline]
    fn next(&mut self) -> Option<Vec<T>> {
        match self.swaps.next() {
            None => None,
            Some((0,0)) => Some(self.v.clone()),
            Some((a,b)) => {
                let elt = self.v.clone();
                self.v.swap(a,b);
                Some(elt)
            }
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.swaps.size_hint()
    }
}

struct Permutations<T> {
    swaps: ElementSwaps,
    v: Vec<T>,
}

fn permutations<T>(s: &[T]) -> Permutations<T> where T: Clone {
    Permutations {
        swaps: ElementSwaps::new(s.len()),
        v: s.to_vec(),
    }
}

/// Returns the maximum output that a five-phase series of amplifier programs can produce with
/// permutations of 0-4 as phase settings.
fn max_output(program: &Vec<i32>) -> i32 {
    let phase_settings = [0,1,2,3,4];

    permutations(&phase_settings)
        .map(|permutation| chain_output(program, permutation))
        .max()
        .unwrap()
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

    // Part 1: passing 0-4, then the output from the previous phase, what's the maximum output for a 5-amp series?
    println!("Part 1: {}", max_output(&program));

    Ok(())
}
