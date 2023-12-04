use std::fs::read;
use std::io::BufRead;

pub(crate) fn solve() -> u32 {
    read("./input.txt").unwrap().lines().
        map(|s| s.unwrap().chars().filter_map(|c|c.to_digit(10)).collect::<Vec<u32>>()). // digits from the line
        map(|digits|digits.first().unwrap() * 10 + digits.last().unwrap()). // take first & last
        sum()
}
