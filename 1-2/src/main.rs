use std::fs::File;
use std::io::{self, BufRead};

use anyhow::Result;

fn fuel_mass_for(mass: u32) -> u32 {
    if let Some(fuel) = (mass / 3).checked_sub(2) {
        fuel + fuel_mass_for(fuel)
    } else {
        0
    }
}

fn main() -> Result<()> {
    let file = File::open("input.txt")?;
    let reader = io::BufReader::new(file);

    let fuel_mass: u32 = reader
        .lines()
        .map(|l| l.unwrap().parse::<u32>().unwrap())
        .map(|m| fuel_mass_for(m))
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
