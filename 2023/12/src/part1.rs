use std::fs::read;
use std::io::BufRead;

use rayon::prelude::*;

use crate::solve;

pub(crate) fn solve(path: &str) -> u64 {
    read(path)
        .unwrap()
        .lines()
        .map(|s| s.unwrap())
        .filter(|s| !s.is_empty())
        .par_bridge()
        .map(|s| solve::solve(s.as_str(), 1))
        .sum()
}
