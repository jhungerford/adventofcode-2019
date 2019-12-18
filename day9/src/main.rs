use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

use computer::*;

mod computer;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn run_example_1() {
        let program = vec![109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99];
        let mut computer = Computer::new(program.clone());
        let mut input = AlwaysValueProgramInput::new(0);
        let mut output = BufferedProgramOutput::new();

        computer.run(&mut input, &mut output);

        assert_eq!(&output.values, &program);
    }

    #[test]
    fn run_example_2() {
        let program = vec![1102,34915192,34915192,7,4,7,99,0];
        let mut computer = Computer::new(program);
        let mut input = AlwaysValueProgramInput::new(0);
        let mut output = LastValueProgramOutput::new();

        computer.run(&mut input, &mut output);

        assert_eq!(output.value, 34915192 * 34915192);
    }

    #[test]
    fn run_example_3() {
        let program = vec![104,1125899906842624,99];
        let mut computer = Computer::new(program);
        let mut input = AlwaysValueProgramInput::new(0);
        let mut output = LastValueProgramOutput::new();

        computer.run(&mut input, &mut output);

        assert_eq!(output.value, 1125899906842624);
    }
}

fn main() -> std::io::Result<()> {
    let f = File::open("input.txt")?;
    let mut reader = BufReader::new(f);
    let mut line = String::new();
    reader.read_line(&mut line)?;
    line = line.trim().to_string();

    let program = line
        .split(",")
        .flat_map(str::parse::<i64>)
        .collect::<Vec<_>>();

    // Part 1: what's the output in test mode? (input = 1)
    let mut input = AlwaysValueProgramInput::new(1);
    let mut output = LastValueProgramOutput::new();
    let mut computer = Computer::new(program.clone());

    computer.run(&mut input, &mut output);

    println!("Part 1: {}", output.value);

    // Part 2: what's the output in BOOST mode? (input = 2)
    let mut input = AlwaysValueProgramInput::new(2);
    let mut output = LastValueProgramOutput::new();
    let mut computer = Computer::new(program.clone());

    computer.run(&mut input, &mut output);

    println!("Part 2: {}", output.value);

    Ok(())
}
