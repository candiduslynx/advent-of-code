use std::fs::read;
use std::io::BufRead;

pub(crate) fn solve() -> u32 {
    read("./input.txt").unwrap().lines().
        map(|s| extract_num(s.unwrap())).
        fold(0, |sum: u32, x| sum + (x as u32))
}

fn extract_num(string: String) -> u8 {
    if string.is_empty() {
        return 0;
    }

    let digits: Vec<u8> = string.split("").filter_map(|s| s.parse().ok()).collect();
    digits.first().unwrap() * 10 + digits.last().unwrap()
}