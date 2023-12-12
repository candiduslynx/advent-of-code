use crate::solve;
use std::fs::read;
use std::io::BufRead;

pub(crate) fn solve(path: &str) -> u64 {
    read(path)
        .unwrap()
        .lines()
        .map(|s| s.unwrap())
        .filter(|s| !s.is_empty())
        .map(|s| possibilities(&s))
        .sum()
}

fn possibilities(s: &str) -> u64 {
    let parts: Vec<&str> = s.split_whitespace().collect();
    assert_eq!(parts.len(), 2);

    let pattern = parts[0];
    let broken: Vec<usize> = parts[1].split(",").map(|s| s.parse().unwrap()).collect();
    solve::remaining(pattern, &broken[..])
}
