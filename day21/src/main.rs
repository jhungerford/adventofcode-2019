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
    // Uncomment to run the computer in interactive mode.
    // computer.run_interactive();


    /* Working out Part 1 - want to jump if we'll land on ground, and there's a hole in range.
    J = (A == false || B == false || C == false) && D == true

    -- set T to true and J to false.
    OR J T
    NOT J J
    OR J T
    NOT T J

    -- jump if there's a hole in range (A or B or C = false)
    AND A T
    AND B T
    AND C T
    NOT T J

    -- land on ground (D=true)
    AND D J

    WALK
     */
    computer.reset();
    println!("Part 1: {:?}", computer.run_input(vec![
        "OR J T",
        "NOT J J",
        "OR J T",
        "NOT T J",
        "AND A T",
        "AND B T",
        "AND C T",
        "NOT T J",
        "AND D J",
        "WALK",
    ]));

    /* Part 2: sensors out to nine tiles (A - I).  Jump is still 4.
    Jump if there's ground 4 tiles out (D=true), a hole to jump over (A or B or C = false),
    and we won't get stuck (E=true or H=true)

    Don't jump on x, because we'll end up stuck.  Jump on ^ instead.

    #####.#.##..#####
      x ^ x ^ x

    -- set T to true and J to false.
    OR J T
    NOT J J
    OR J T
    NOT T J

    -- don't get stuck (E=true or H=true)
    OR E J
    OR H J

    -- jump if there's a hole in range (A or B or C = false)
    AND A T
    AND B T
    AND C T
    NOT T T
    AND T J

    -- land on ground (D=true)
    AND D J

    RUN
     */
    computer.reset();
    println!("Part 2: {:?}", computer.run_input(vec![
        "OR J T",
        "NOT J J",
        "OR J T",
        "NOT T J",
        "OR E J",
        "OR H J",
        "AND A T",
        "AND B T",
        "AND C T",
        "NOT T T",
        "AND T J",
        "AND D J",
        "RUN",
    ]));
}
