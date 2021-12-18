use advent_of_code::{common::Solution, solutions};

use criterion::{criterion_group, criterion_main, Criterion};

pub fn day_01(c: &mut Criterion) {
    let solver = solutions::day01::Day01 {};
    c.bench_function("Day 1, Solution A", |b| b.iter(|| solver.part_a()));
    c.bench_function("Day 1, Solution B", |b| b.iter(|| solver.part_b()));
}

criterion_group!(benches, day_01);
criterion_main!(benches);
