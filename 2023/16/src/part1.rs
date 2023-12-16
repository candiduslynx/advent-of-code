use std::fs::read;
use std::io::BufRead;

pub(crate) fn solve(path: &str) -> usize {
    read(path)
        .unwrap()
        .lines()
        .map(|s| s.unwrap())
        .filter(|s| !s.is_empty())
        .count()
}
