use regex::Regex;

fn main() {
    let re = Regex::new(r"^(\d+?)-(\d+?)\s([a-z]):\s([a-z]+?)$").unwrap();
    println!("{}",
    include_str!("../../input.txt")
        .lines()
        .map(|line| re.captures(line).unwrap())
        .filter(|line| {
            let len = line[4].matches(&line[3]).count();
            len >= line[1].parse().unwrap() && len <= line[2].parse().unwrap()
        }).count());
}
