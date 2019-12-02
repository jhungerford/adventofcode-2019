use std::io::BufReader;
use std::io::prelude::*;
use std::fs::File;

fn run_program(program: &mut Vec<i32>) {
    let mut pc = 0;

    while program[pc] != 99 {
        let source_index_1 = program[pc + 1] as usize;
        let source_index_2 = program[pc + 2] as usize;
        let result_index = program[pc + 3] as usize;
        
        match program[pc] {
            1 => { 
                // 1 means add the numbers at positions pc + 1 and pc + 2, and store them in pc + 3
                program[result_index] = program[source_index_1] + program[source_index_2]
            }
            2 => { 
                // 2 means multiply the numbers at position pc + 1 and pc + 2, and store them in pc + 3
                program[result_index] = program[source_index_1] * program[source_index_2]
            }
            _ => {
                panic!("Unknown opcode {} at position {} - {:?}", program[pc], pc, program);
            }
        }

        pc += 4
    };
}

fn main() -> std::io::Result<()> {
    let f = File::open("input.txt")?;
    let mut reader = BufReader::new(f);
    let mut line = String::new();
    reader.read_line(&mut line)?;
    line = line.trim().to_string();

    let nums = line.split(",").flat_map(str::parse::<i32>).collect::<Vec<_>>();

    // Part 1: replace position 1 with 12 and position 2 with 2 - what value is left at position 0 after the program halts?
    let mut part1 = nums.clone();
    part1[1] = 12;
    part1[2] = 2;
    run_program(&mut part1);
    
    println!("Part 1: {}", part1[0]);

    // Part 2: what pair of inputs (replacing values 1 and 2) produces the output (value 0) 19690720?
    // Calculate 100 * noun (value 1) * verb (value 2) that produce 19690720

    let mut i = 0;
    let mut part2 = nums.clone();
    while part2[0] != 19690720 {
        let mut j = 0;

        while j <= i && part2[0] != 19690720 {
            j += 1;
            part2 = nums.clone();
            part2[1] = i;
            part2[2] = j;
            run_program(&mut part2);

            if part2[0] != 19690720 {
                part2 = nums.clone();
                part2[1] = j;
                part2[2] = i;
                run_program(&mut part2);
            }
        }

        i += 1;
    }

    println!("Part 2: {}", 100 * part2[1] + part2[2]);

    Ok(())
}
