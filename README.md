# adventofcode-2019
Solutions to the [2019 Advent of Code](https://adventofcode.com/2019)

## Running
Solutions are in [Rust](https://www.rust-lang.org/), with a separate application per day.  To run the solution for a day, [install rust](https://www.rust-lang.org/tools/install), clone the repo, cd into a day, and run `cargo run`.  Run tests with `cargo test`.

```bash
$ git clone https://github.com/jhungerford/adventofcode-2019
Cloning into 'adventofcode-2019-2'...
...
Unpacking objects: 100% (54/54), done.

$ cd adventofcode-2019/day1

$ cargo run
   Compiling day1 v0.1.0 (/mnt/c/Users/cthae/dev/adventofcode-2019/day1)
    Finished dev [unoptimized + debuginfo] target(s) in 0.74s
     Running `target/debug/day1`
Part 1: 3416712
Part 2: 5122170

$ cargo test
   Compiling day3 v0.1.0 (/mnt/c/Users/cthae/dev/adventofcode-2019/day3)
    Finished dev [unoptimized + debuginfo] target(s) in 1.12s
     Running target/debug/deps/day3-e7b588d7bac9a8c0

running 6 tests
test tests::closest_intersection_ex1 ... ok
test tests::first_intersection_ex1 ... ok
test tests::closest_intersection_ex3 ... ok
test tests::closest_intersection_ex2 ... ok
test tests::first_intersection_ex3 ... ok
test tests::first_intersection_ex2 ... ok

test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```
