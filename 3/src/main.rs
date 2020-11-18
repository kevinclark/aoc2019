use crossed_wires::*;
use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let mut lines = input.trim().split("\n");

    let first_wire = path_points(&parse_path(lines.next().unwrap()));
    let second_wire = path_points(&parse_path(lines.next().unwrap()));

    let min = first_wire
        .intersection(&second_wire)
        .inspect(|p| println!("Intersection: {:?}", p))
        .min_by_key(|p| p.0.abs() + p.1.abs())
        .unwrap();

    println!("Min: {:?}", min);
    println!("Manhattan distance: {:?}", min.0.abs() + min.1.abs());
}
