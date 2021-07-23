use day17::calibration;
use day17::computer::Computer;

fn main() {
    let computer = Computer::load("input.txt");

    // Running the computer prints the map.
    let mut print_map_comp = computer.clone();
    print_map_comp.run();
    print_map_comp.visit_output(|&value| print!("{}", value as u8 as char));

    println!("Part 1: {}", calibration(&mut computer.clone()));
}
