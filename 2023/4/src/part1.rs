use std::fs::read;
use std::io::BufRead;
use crate::card;

pub(crate) fn solve(path: &str) -> u64 {
    read(path).unwrap().lines().
        map(|s|card::winning_numbers(s.unwrap())).
        filter(|&p|p>0).map(|p| 2u64.pow(p as u32 - 1)).sum()
}
