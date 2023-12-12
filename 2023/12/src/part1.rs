use std::fs::read;
use std::io::BufRead;

use crate::solve;

pub(crate) fn solve(path: &str) -> u64 {
    read(path)
        .unwrap()
        .lines()
        .map(|s| s.unwrap())
        .filter(|s| !s.is_empty())
        .enumerate()
        .map(|(i, s)| {
            let res = solve::solve(&s, 1);
            println!("{i:4} >>> {s} -> {res}");
            res
        })
        .sum()
}
