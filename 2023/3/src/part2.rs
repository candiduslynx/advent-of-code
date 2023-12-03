use std::fs::read;
use std::io::BufRead;
use crate::schematics;

pub(crate) fn solve() -> u32 {
    schematics::Schema::get_numbers(
        read("./input.txt").unwrap().lines().map(|s|s.unwrap()).collect()
    ).iter().sum()
}