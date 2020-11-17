use std::fs;

use day2::*;

fn main() {
    let initial_state = load_program(&fs::read_to_string("input.txt").unwrap());
    for noun in 0..=99 {
        for verb in 0..=99 {
            let mut mem = initial_state.to_vec();
            // [B]efore running the program, replace position 1 with the value 12
            // and replace position 2 with the value 2
            mem[1] = noun;
            mem[2] = verb;
            execute(&mut mem);

            if mem[0] == 19690720 {
                println!("Noun: {} Verb: {}", noun, verb);
                println!("Answer: {}", (100 * noun) + verb);
                return;
            }
        }
    }

    println!("Not found");
}
