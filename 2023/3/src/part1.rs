use std::fs::read;
use std::io::BufRead;
use crate::schematics;

pub(crate) fn solve() -> u32 {
    schematics::Schema::from_strings(
        read("./input.txt").unwrap().lines().map(|s|s.unwrap()).collect()
    ).numbers.iter().sum()
}