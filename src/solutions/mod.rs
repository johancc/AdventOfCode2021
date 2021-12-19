use crate::common::Solution;
pub mod day01;
pub mod day02;
pub mod day03;

pub const ALL: [&dyn Solution; 3] = [&day01::Day01 {}, &day02::Day02 {}, &day03::Day03 {}];
