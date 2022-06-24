use std::fs;
use itertools::Itertools;

fn main() {
    // AoC 2020: day 1
    fs::read_to_string("../input.txt").unwrap()
        .lines()
        .map(|line| line.parse::<i32>().unwrap())
        .combinations(2)
        .filter(|line| line.into_iter().sum::<i32>() == 2020)
        .for_each(|line| println!("{}", line.into_iter().product::<i32>()))
        ;
}
