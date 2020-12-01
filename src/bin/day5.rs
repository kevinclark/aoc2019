use aoc2019::intcode;
use clap::{App, Arg};
use std::fs;

fn main() {
    let matches = App::new("day5")
        .arg(Arg::with_name("INPUT").required(true))
        .get_matches();

    let input =
        &fs::read_to_string(matches.value_of("INPUT").unwrap()).unwrap();

    println!("Part 1");
    let mut mem = intcode::load_program(input);
    let inputs = [1i64];
    intcode::execute(&mut mem, &mut inputs.iter());

    println!("Part 2");
    let mut mem = intcode::load_program(input);
    let inputs = [5i64];
    intcode::execute(&mut mem, &mut inputs.iter());
}
