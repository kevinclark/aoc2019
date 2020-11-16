use std::fs;

use day2::*;

fn main() {
    let mut mem = load_program(&fs::read_to_string("input.txt").unwrap());
    // [B]efore running the program, replace position 1 with the value 12
    // and replace position 2 with the value 2
    mem[1] = 12;
    mem[2] = 2;
    execute(&mut mem);
    println!("{:?}", mem[0]);
}
