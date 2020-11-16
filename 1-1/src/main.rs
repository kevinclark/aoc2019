use std::fs::File;
use std::io;
use std::io::prelude::*;

use snafu::{ResultExt, Snafu};

#[derive(Debug, Snafu)]
pub enum Error {
    OpenFile { source: io::Error },
    ReadLine { source: io::Error },
    ParseMass { source: std::num::ParseIntError },
}

fn main() -> Result<(), Error> {
    let file = File::open("input.txt").context(OpenFile)?;
    let reader = io::BufReader::new(file);

    let mut acc = 0;

    for line in reader.lines() {
        let i: u32 = line.context(ReadLine)?.parse().context(ParseMass)?;

        acc += (i / 3) - 2;
    }

    println!("Result: {}", acc);

    Ok(())
}
