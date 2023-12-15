use std::fs::read;
use std::hash::Hasher;
use std::io::BufRead;

use crate::lens;
use crate::lens::ShiftSumHasher;

pub(crate) fn solve(path: &str) -> u64 {
    read(path)
        .unwrap()
        .lines()
        .map(|s| s.unwrap())
        .filter(|s| !s.is_empty())
        .map(|s| {
            s.split(",")
                .map(|s| ShiftSumHasher::calc(s.as_bytes()))
                .sum::<u64>()
        })
        .sum()
}
