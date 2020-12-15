use aoc2019::intcode;
use criterion::{criterion_group, criterion_main, Criterion};
use std::fs;

fn criterion_benchmark(c: &mut Criterion) {
    let input = &fs::read_to_string("inputs/5.txt").unwrap();

    c.bench_function("day5/part1/load", |b| {
        b.iter(|| {
            intcode::load_program(input);
        });
    });

    c.bench_function("day5/part1/execute", |b| {
        let mut sink = std::io::sink();
        let program = intcode::load_program(input);
        b.iter(|| {
            let mut mem = program.clone();
            let inputs = [5i64];
            intcode::execute(&mut mem, &mut inputs.iter(), &mut sink);
        });
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
