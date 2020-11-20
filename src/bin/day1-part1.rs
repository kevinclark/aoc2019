use std::fs::File;
use std::io;
use std::io::prelude::*;

use clap::{App, Arg};
use snafu::{ResultExt, Snafu};

#[derive(Debug, Snafu)]
pub enum Error {
    OpenFile { source: io::Error },
    ReadLine { source: io::Error },
    ParseMass { source: std::num::ParseIntError },
}

fn main() -> Result<(), Error> {
    let matches = App::new("day1-part1")
        .arg(Arg::with_name("INPUT").required(true))
        .get_matches();

    let file = File::open(matches.value_of("INPUT").unwrap()).context(OpenFile)?;
    let reader = io::BufReader::new(file);

    let mut acc = 0;

    for line in reader.lines() {
        let i: u32 = line.context(ReadLine)?.parse().context(ParseMass)?;

        acc += (i / 3) - 2;
    }

    println!("Result: {}", acc);

    Ok(())
}
