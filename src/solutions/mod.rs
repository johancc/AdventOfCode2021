use crate::common::Solution;
pub mod day01;
pub mod day02;

pub const ALL: [&dyn Solution; 2] = [&day01::Day01 {}, &day02::Day02 {}];
