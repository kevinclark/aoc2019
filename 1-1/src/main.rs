use std::fs::File;
use std::io;
use std::io::prelude::*;

use snafu::{ResultExt, Snafu};

#[derive(Debug, Snafu)]
pub enum Error {
    UnableToOpenFile { source: io::Error },
    UnableToReadLine { source: io::Error },
    UnableToParseMass { source: std::num::ParseIntError },
}

fn main() -> Result<(), Error> {
    let file = File::open("input.txt").context(UnableToOpenFile)?;
    let reader = io::BufReader::new(file);

    let mut acc = 0;

    for line in reader.lines() {
        let i: u32 = line
            .context(UnableToReadLine)?
            .parse()
            .context(UnableToParseMass)?;

        acc += (i / 3) - 2;
    }

    println!("Result: {}", acc);

    Ok(())
}
