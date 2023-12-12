use std::fs::read;
use std::io::BufRead;

use crate::solve;

pub(crate) fn solve(path: &str) -> u64 {
    read(path)
        .unwrap()
        .lines()
        .map(|s| s.unwrap())
        .filter(|s| !s.is_empty())
        .map(|s| solve::solve(&s, 5))
        .sum()
}
