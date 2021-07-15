use itertools::Itertools;
use rayon::prelude::*;

use crate::computer::Computer;

/// Runs the program on a series of amplifiers, using the given phase settings and passing the output from one amp to the next.
fn chain_output(computer: &Computer, phase_settings: Vec<i32>) -> i32 {
    let amplifiers = (0..5).map(|i| {
        let mut amp = computer.clone();
        amp.input(phase_settings[i]);
        amp
    });

    // Run the amplifiers once - input for the first amplifier is 0
    let mut value = 0;
    for mut amplifier in amplifiers {
        amplifier.input(value);
        value = amplifier.run().unwrap();
    }

    value
}

/// Returns the maximum output that a five-phase series of amplifier programs can produce with
/// permutations of 0-4 as phase settings.
fn max_output(computer: &Computer) -> i32 {
    let phase_settings = vec![0, 1, 2, 3, 4];

    phase_settings.into_iter()
        .permutations(5)
        .par_bridge()
        .map(|permutation| chain_output(computer, permutation))
        .max()
        .unwrap()
}

/// Runs the given program on a loop of amplifiers until they all halt and returns the final output from amplifier E.
/// Input is the phase setting for each amplifier, then the output from the previous amp in the chain.Iterator
/// The first amplifier's initial chained input is 0.
fn looped_output(computer: &Computer, phase_settings: Vec<i32>) -> i32 {
    let mut amplifiers: Vec<Computer> = (0..5).map(|i| {
        let mut amp = computer.clone();
        amp.input(phase_settings[i]);
        amp
    }).collect();

    // Run the amplifiers in a continuous loop until they halt.
    let mut i = 0;
    let mut value = 0;

    loop {
        amplifiers[i].input(value);

        if !amplifiers[i].is_runnable() {
            break;
        }

        value = amplifiers[i].run().unwrap();
        i = (i + 1) % 5;
    }

    // Output comes from the last amplifier.
    amplifiers[4].last_output().unwrap()
}

/// Given a program, returns the maximum output that a looped chain of amplifiers can produce.
fn max_looped_output(computer: &Computer) -> i32 {
    let phase_settings = vec![5, 6, 7, 8, 9];

    phase_settings.into_iter()
        .permutations(5)
        .par_bridge()
        .map(|permutation| looped_output(computer, permutation))
        .max()
        .unwrap()
}

#[cfg(test)]
mod test;
mod computer;

fn main() -> std::io::Result<()> {
    let computer = Computer::load("input.txt")?;

    // Part 1: passing 0-4, then the output from the previous phase, what's the maximum output for a 5-amp series?
    println!("Part 1: {}", max_output(&computer));

    // Part 2: passing 5-9 as the first input instruction then output from the previous phase,
    // loop the output from each amplifier as input to the next until the amplifiers halt.
    // What is the largest output signal output from amplifier E?
    println!("Part 2: {}", max_looped_output(&computer));

    Ok(())
}
