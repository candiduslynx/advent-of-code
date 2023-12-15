use std::fs::read;
use std::io::BufRead;

use crate::hash;

pub(crate) fn solve(path: &str) -> u64 {
    read(path)
        .unwrap()
        .lines()
        .map(|s| s.unwrap())
        .filter(|s| !s.is_empty())
        .map(|s| s.split(",").map(|s| hash::hash(s) as u64).sum::<u64>())
        .sum()
}
