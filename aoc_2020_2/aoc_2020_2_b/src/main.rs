use itertools::Itertools;
use regex::Regex;

fn main() {
    let re = Regex::new(r"^(\d+?)-(\d+?)\s([a-z]):\s([a-z]+?)$").unwrap();
    println!("{}",
    include_str!("../../input.txt")
        .lines()
        .map(|line| re.captures(line).unwrap())
        .filter(|line| {
            line[4].chars()
                .positions(|x| x.to_string() == line[3])
                .filter(
                    |x| {
                        x+1 == line[1].parse::<usize>().unwrap() 
                            || x+1 == line[2].parse::<usize>().unwrap() 
                    }
                    )
                .count() == 1
        })
        .count())
}
