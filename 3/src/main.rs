use crossed_wires::*;
use std::collections::HashMap;
use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let mut lines = input.trim().split("\n");

    let first_wire = path_points(&parse_path(lines.next().unwrap()));
    let second_wire = path_points(&parse_path(lines.next().unwrap()));

    let first_wire_to_steps: HashMap<_, _> = first_wire
        .iter()
        .enumerate()
        .map(|(i, p)| (p, i + 1))
        .collect();

    let second_wire_to_steps: HashMap<_, _> = second_wire
        .iter()
        .enumerate()
        .map(|(i, p)| (p, i + 1))
        .collect();

    let mut min: Option<usize> = None;

    for (point, first_steps) in first_wire_to_steps.iter() {
        if let Some(second_steps) = second_wire_to_steps.get(*point) {
            let combined = first_steps + second_steps;
            println!("Intersection: {:?} at {:?} steps", point, combined);
            if min.map_or(true, |m| m > combined) {
                min = Some(combined);
            }
        }
    }

    println!("Min: {:?}", min);
}
