use advent_of_code::{common::Solution, solutions};

use criterion::{criterion_group, criterion_main, Criterion};

pub fn day_01(c: &mut Criterion) {
    let solver = solutions::day01::Day01 {};
    c.bench_function("Day 1, Solution A", |b| b.iter(|| solver.part_a()));
    c.bench_function("Day 1, Solution B", |b| b.iter(|| solver.part_b()));
}
pub fn day_02(c: &mut Criterion) {
    let solver = solutions::day02::Day02 {};
    c.bench_function("Day 2, Solution A", |b| b.iter(|| solver.part_a()));
    c.bench_function("Day 2, Solution B", |b| b.iter(|| solver.part_b()));
}

pub fn day_03(c: &mut Criterion) {
    let solver = solutions::day03::Day03 {};
    c.bench_function("Day 3, Solution A", |b| b.iter(|| solver.part_a()));
    c.bench_function("Day 3, Solution B", |b| b.iter(|| solver.part_b()));
}

criterion_group!(benches, day_01, day_02, day_03);
criterion_main!(benches);
