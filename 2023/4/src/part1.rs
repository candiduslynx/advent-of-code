use std::fs::read;
use std::io::BufRead;
use crate::card;

pub(crate) fn solve() -> u64 {
    read("./input.txt").unwrap().lines().
        map(|s|card::Card::from_string(s.unwrap())).
        map(|c| c.winning_numbers().len()).filter(|&p|p>0).
        map(|p| 2u64.pow(p as u32 - 1)).sum()
}
