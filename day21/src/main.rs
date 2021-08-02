use crate::computer::Computer;

mod computer;

// Springscript: boolean values
// Registers: T (temporary) and J (jump)
// if J is true at the end of the springstep program, the springdroid will try to jump.  Both start with false.
// Read-only registers: A, B, C, D - one to four tiles away.  true if there's ground, false if there's a hole.
// Instructions:
// AND X Y - sets Y to true if both X and Y are true, otherwise sets Y to false.
// OR X Y - sets Y to true if at least one of X or Y is true, otherwise sets Y to false.
// NOT X Y - sets Y to true if X is false, otherwise sets Y to false.
// Y needs to be a writable register in all three instructions.  X can be any register
// Springdroid can contain at most 15 instructions.



fn main() {
    let mut computer = Computer::load("input.txt");

    /* Working out Part 1 - want to jump if we'll land on ground, and there's a hole in range.
    J = (A == false || B == false || C == false) && D == true

    -- Reset T and J to false.  There has to be ground in range, so at least one of A, B, C, or D will be false.
    AND A T
    AND B T
    AND C T
    AND D T
    AND T J

    -- jump if we'll land on ground (D=true) and there's a hole in range (A, B, or C = false)

    -- hole in range
    NOT A T
    OR T J

    NOT B T
    OR T J

    NOT C T
    OR T J

    -- land on ground
    AND D J

    WALK
     */
    let part1 = computer.run_interactive();
    println!("Part 1: {:?}", part1);
}
