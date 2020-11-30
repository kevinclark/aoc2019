use aoc2019::intcode;
use clap::{App, Arg};
use std::fs;

fn main() {
    let matches = App::new("day2")
        .arg(Arg::with_name("INPUT").required(true))
        .get_matches();

    let input = &fs::read_to_string(matches.value_of("INPUT").unwrap()).unwrap();

    let initial_state = intcode::load_program(input);

    for noun in 0..=99 {
        for verb in 0..=99 {
            let mut mem = initial_state.to_vec();
            // [B]efore running the program, replace position 1 with the value 12
            // and replace position 2 with the value 2
            mem[1] = noun;
            mem[2] = verb;
            let inputs: [i64; 0] = [];
            intcode::execute(&mut mem, &mut inputs.iter());

            if mem[0] == 19690720 {
                println!("Noun: {} Verb: {}", noun, verb);
                println!("Answer: {}", (100 * noun) + verb);
                return;
            }
        }
    }

    println!("Not found");
}
