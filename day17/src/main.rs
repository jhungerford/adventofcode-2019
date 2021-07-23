use day17::calibration;
use day17::computer::Computer;

fn main() {
    let computer = Computer::load("input.txt");

    // Part 1: sum the alignment parameters.
    println!("Part 1: {}", calibration(&mut computer.clone()));

    // Part 2: run the robot along the scaffold, collecting robots.
    let mut notify_robot = computer.clone();
    notify_robot.memory.set(0, 2); // Interactive mode.

    // Working out the sequence by hand:
    /*
R,8,L,4,R,4,R,10,R,8,R,8,L,4,R,4,R,10,R,8,L,12,L,12,R,8,R,8,R,10,R,4,R,4,L,12,L,12,R,8,R,8,R,10,R,4,R,4,L,12,L,12,R,8,R,8,R,10,R,4,R,4,R,10,R,4,R,4,R,8,L,4,R,4,R,10,R,8
A: R,8,L,4,R,4,R,10,R,8
A: R,8,L,4,R,4,R,10,R,8
B: L,12,L,12,R,8,R,8
C: R,10,R,4,R,4
B: L,12,L,12,R,8,R,8
C: R,10,R,4,R,4
B: L,12,L,12,R,8,R,8
C: R,10,R,4,R,4
C: R,10,R,4,R,4
A: R,8,L,4,R,4,R,10,R,8

A,A,B,C,B,C,B,C,C,A
     */

    // Main movement routine.
    notify_robot.run();
    notify_robot.print_output();
    notify_robot.text_input("A,A,B,C,B,C,B,C,C,A\n");

    // Subroutine A.
    notify_robot.run();
    notify_robot.print_output();
    notify_robot.text_input("R,8,L,4,R,4,R,10,R,8\n");

    // Subroutine B.
    notify_robot.run();
    notify_robot.print_output();
    notify_robot.text_input("L,12,L,12,R,8,R,8\n");

    // Subroutine C.
    notify_robot.run();
    notify_robot.print_output();
    notify_robot.text_input("R,10,R,4,R,4\n");

    // Continuous video feed?.
    notify_robot.run();
    notify_robot.print_output();
    notify_robot.text_input("n\n");

    // Run the program.
    println!("Part 2: {:?}", notify_robot.run());
}
