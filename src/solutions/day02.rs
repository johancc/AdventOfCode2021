use crate::common::{self, Solution};
use itertools::Itertools;

pub struct Day02 {}

enum Direction {
    FORWARD,
    DOWN,
    UP,
}

impl Day02 {
    fn get_data(&self) -> Vec<(Direction, i32)> {
        common::load("02")
            .lines()
            .map(|l| l.splitn(2, " ").collect_tuple().unwrap())
            .map(|(dir, amt)| (dir, amt.parse::<i32>().unwrap()))
            .map(|(dir, amt)| match dir {
                "forward" => (Direction::FORWARD, amt),
                "down" => (Direction::DOWN, amt),
                "up" => (Direction::UP, amt),
                _ => unreachable!("Invalid direction: {}", dir),
            })
            .collect()
    }
}

impl Solution for Day02 {
    fn name(&self) -> String {
        "Dive!".to_owned()
    }

    fn part_a(&self) -> String {
        let (x, y) = self
            .get_data()
            .iter_mut()
            .fold((0, 0), |(x, y), (dir, amt)| match dir {
                Direction::FORWARD => (x + *amt, y),
                Direction::DOWN => (x, y + *amt),
                Direction::UP => (x, y - *amt),
            });
        format!("{}", x * y)
    }
    fn part_b(&self) -> String {
        let (_, x, y) = self
            .get_data()
            .iter_mut()
            .fold((0, 0, 0), |(aim, x, y), (dir, amt)| match dir {
                Direction::DOWN => (aim + *amt, x, y),
                Direction::FORWARD => (aim, x + *amt, y + (aim * *amt)),
                Direction::UP => (aim - *amt, x, y),
            });
        format!("{}", x * y)
    }
}
