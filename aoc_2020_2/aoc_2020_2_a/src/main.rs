use std::fs;
use regex::Regex;

fn main() {
    let _re = Regex::new(r"^(\d+?)-(\d+?)\s([a-z]):\s([a-z])+?").unwrap();
    fs::read_to_string("../input.txt").unwrap()
        .lines()
        .map(|line| line.parse::<String>().unwrap());
}
