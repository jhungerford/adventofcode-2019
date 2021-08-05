use crate::computer::Computer;
use crate::computer::ProgramState::WaitingForInput;

mod computer;

// Boot 50 computers, provide network address as input (0 to 49)
// Packets have two values named X and Y, and are queued by the recipient in the order they're received.
// Send: three output instructions that provide the destination address followed by its X and Y values.
// Receive: input instructions.  Provide -1 if the packet queue is empty, otherwise provide the X value
// then the Y value.  Packet is removed from the queue once it's read.
// Input and output instructions never block.

/// Returns the Y value of the first packet sent to address 255.
fn part1() -> i64 {
    let computer = Computer::load("input.txt");

    let mut networked_computers: Vec<Computer> = (0..50).map(|i| {
        let mut networked_computer = computer.clone();
        networked_computer.input(i);
        networked_computer
    }).collect();

    loop {
        for i in 0..networked_computers.len() {
            let mut comp = &mut networked_computers[i];
            if comp.state == WaitingForInput && comp.input.is_empty() {
                comp.input(-1);
            }

            comp.run();
            let output = comp.dump_output();

            if !output.is_empty() && output.len() % 3 != 0 {
                panic!("Computer {} produced an incomplete packet {:?}", i, output);
            }

            println!("{:>2} Output: {:?}", i, output);
            for window in output.chunks_exact(3) {
                let (address, x, y) = (window[0] as usize, window[1], window[2]);

                // If this is the first packet sent to 255, the Y value is the answer.
                if window[0] == 255 {
                    return y;
                }

                // Otherwise send the X and Y values in the packet to the destination.
                networked_computers[address].input(x);
                networked_computers[address].input(y);
            }
        }
    }
}

fn main() {
    println!("Part 1: {}", part1());
}
