use std::fs::read;
use std::io::BufRead;
use std::time::Instant;

use crate::solve;

pub(crate) fn solve(path: &str) -> u64 {
    read(path)
        .unwrap()
        .lines()
        .map(|s| s.unwrap())
        .filter(|s| !s.is_empty())
        .enumerate()
        .map(|(i, s)| {
            println!("{i:4} >>> {s}");
            let start = Instant::now();
            let res = solve::solve(&s, 5);
            let took = start.elapsed();
            println!("{i:4} >>> {s} -> {res}, took {took:?}");
            res
        })
        .sum()
}
