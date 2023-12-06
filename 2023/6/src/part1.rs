use std::fs::read;
use std::io::BufRead;


pub(crate) fn solve(path: &str) -> u64 {
    read(path).unwrap().lines().count() as u64
}
