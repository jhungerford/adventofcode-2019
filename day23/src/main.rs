use crate::computer::Computer;
use crate::computer::ProgramState::WaitingForInput;
use std::collections::HashSet;

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

/// Returns the first Y value delivered by the NAT to the computer at address 0 twice in a row.
/// Packets sent to address 255 are handled by a 'NAT', which sends the last packet it received to
/// address 0 if the network is idle (e.g. if all of the computers have an empty packet queue
/// and are continuously trying to receive packets without sending).
fn part2() -> i64 {
    let computer = Computer::load("input.txt");

    let mut networked_computers: Vec<Computer> = (0..50).map(|i| {
        let mut networked_computer = computer.clone();
        networked_computer.input(i);
        networked_computer
    }).collect();

    let (mut nat_x, mut nat_y) = (None, None);
    let mut last_delivered_y = None;

    loop {
        // Network is idle if all computers have empty input buffers and aren't producing output.
        let mut idle = true;

        for i in 0..networked_computers.len() {
            let mut comp = &mut networked_computers[i];
            if comp.state == WaitingForInput && comp.input.is_empty() {
                comp.input(-1);
            } else {
                idle = false;
            }

            comp.run();
            let output = comp.dump_output();

            if !output.is_empty() && output.len() % 3 != 0 {
                panic!("Computer {} produced an incomplete packet {:?}", i, output);
            }

            if !output.is_empty() {
                idle = false;
            }

            for window in output.chunks_exact(3) {
                let (address, x, y) = (window[0] as usize, window[1], window[2]);

                // Packets sent to 255 are sent to the NAT, not a computer.
                if address == 255 {
                    nat_x = Some(x);
                    nat_y = Some(y);
                } else {
                    // Otherwise send the X and Y values in the packet to the destination.
                    networked_computers[address].input(x);
                    networked_computers[address].input(y);
                }

            }
        }

        // If all of the computers are idle, the NAT sends the last packet it received to 0.
        if idle {
            // Answer is the first Y value delivered by the NAT twice in a row.
            if last_delivered_y != None && nat_y == last_delivered_y {
                return nat_y.unwrap();
            } else {
                last_delivered_y = nat_y;
            }

            networked_computers[0].input(nat_x.unwrap());
            networked_computers[0].input(nat_y.unwrap());

        }
    }
}

fn main() {
    println!("Part 1: {}", part1());
    println!("Part 2: {}", part2());
}
