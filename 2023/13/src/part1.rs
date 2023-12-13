use std::fs::read;
use std::io::BufRead;

pub(crate) fn solve(path: &str) -> u64 {
    read(path).unwrap().lines().map(|s| s.unwrap()).count() as u64
}
