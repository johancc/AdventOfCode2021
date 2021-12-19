const WIDTH: usize = 12;
const COUNT: usize = 1000;

use crate::common::{self, Solution};

pub struct Day03 {}

impl Day03 {
    fn get_data(&self) -> Vec<usize> {
        common::load("03")
            .lines()
            .map(|line| usize::from_str_radix(line, 2).unwrap())
            .collect()
    }
}

impl Solution for Day03 {
    fn name(&self) -> String {
        "Binary Diagnostic".to_owned()
    }
    fn part_a(&self) -> String {
        let gamma = self
            .get_data()
            .iter()
            .fold(vec![0; WIDTH], |count, bits| {
                count
                    .into_iter()
                    .enumerate()
                    .map(|(i, n)| n + ((bits & 1 << i) >> i))
                    .collect()
            })
            .into_iter()
            .enumerate()
            .map(|(i, b)| ((b >= COUNT / 2) as u32) << i)
            .sum::<u32>();
        let epsilon = !gamma & ((1 << WIDTH) - 1);
        format!("{}", gamma * epsilon)
    }

    fn part_b(&self) -> String {
        let diagnotics = self.get_data();
        let oxygen = (0..WIDTH)
            .rev()
            .scan(diagnotics.clone(), |oxy, i| {
                let one = oxy.iter().filter(|n| *n & 1 << i > 0).count() >= (oxy.len() + 1) / 2;
                oxy.drain_filter(|n| (*n & 1 << i > 0) != one);
                oxy.first().copied()
            })
            .last()
            .unwrap();
        let co2 = (0..WIDTH)
            .rev()
            .scan(diagnotics, |co2, i| {
                let one = co2.iter().filter(|n| *n & 1 << i > 0).count() >= (co2.len() + 1) / 2;
                co2.drain_filter(|n| (*n & 1 << i > 0) == one);
                co2.first().copied()
            })
            .last()
            .unwrap();
        format!("{}", oxygen * co2)
    }
}
