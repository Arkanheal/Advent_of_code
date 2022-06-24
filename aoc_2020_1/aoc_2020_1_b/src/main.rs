use std::fs;
use itertools::Itertools;

fn main() {
    fs::read_to_string("../input.txt").unwrap()
        .lines()
        .map(|line| line.parse::<usize>().unwrap())
        .combinations(3)
        .filter(|line| line.iter().sum::<usize>() == 2020)
        .for_each(|line| println!("{:?}", line.iter().product::<usize>()));
}
