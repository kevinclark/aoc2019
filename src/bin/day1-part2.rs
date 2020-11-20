use std::fs::File;
use std::io::{self, BufRead};

use anyhow::Result;
use clap::{App, Arg};

fn fuel_mass_for(mass: u32) -> u32 {
    if let Some(fuel) = (mass / 3).checked_sub(2) {
        fuel + fuel_mass_for(fuel)
    } else {
        0
    }
}

fn main() -> Result<()> {
    let matches = App::new("day1-part2")
        .arg(Arg::with_name("INPUT").required(true))
        .get_matches();

    let file = File::open(matches.value_of("INPUT").unwrap())?;
    let reader = io::BufReader::new(file);

    let fuel_mass: u32 = reader
        .lines()
        .map(|l| l.unwrap().parse::<u32>().unwrap())
        .map(fuel_mass_for)
        .sum();

    println!("Fuel needed: {}", fuel_mass);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fuel_with_no_additional_weight() {
        assert_eq!(2, fuel_mass_for(14));
    }

    #[test]
    fn fuel_with_additional_weight() {
        assert_eq!(966, fuel_mass_for(1969));
    }

    #[test]
    fn plus_mass() {
        assert_eq!(50346, fuel_mass_for(100756));
    }
}
