use std::fs;
use regex::Regex;

fn main() {
    let re = Regex::new(r"^(\d+?)-(\d+?)\s([a-z]):\s([a-z]+?)$").unwrap();
    let lines : Vec<String> = fs::read_to_string("../input.txt").unwrap()
        .lines()
        .map(|line| line.parse::<String>().unwrap()).collect();
    let mut i = 0;
    for line in lines.iter() {
        for capture in re.captures_iter(line) {
            let re_in = Regex::new(format!(r"{}", &capture[3]).as_str()).unwrap();
            if &capture[1].parse::<usize>().unwrap() <= &re_in.captures_iter(&capture[4]).count() &&
                &capture[2].parse::<usize>().unwrap() >= &re_in.captures_iter(&capture[4]).count() {
                i += 1;
            }
        }
    }
    print!("{}", i);
}
