use crate::common::{self, Solution};

pub struct Day01 {}

impl Day01 {
    fn get_data(&self) -> Vec<u32> {
        common::load("01")
            .lines()
            .map(|x| x.parse::<u32>().unwrap())
            .collect::<Vec<u32>>()
    }
}

impl Solution for Day01 {
    fn name(&self) -> String {
        "Sonar Sweep".to_owned()
    }

    fn part_a(&self) -> String {
        let data = self.get_data();
        let count = data.windows(2).filter(|x| x[0] < x[1]).count();
        count.to_string()
    }

    fn part_b(&self) -> String {
        let data = self.get_data();
        data.windows(3)
            // First find the sum of all 3 element windows.
            .map(|w| w.iter().sum())
            .collect::<Vec<u32>>()
            // Then count how many times the net sum of the window increased.
            .windows(2)
            .filter(|w| w[0] < w[1])
            .count()
            .to_string()
    }
}
