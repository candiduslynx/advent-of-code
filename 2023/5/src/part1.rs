use std::fs::read;
use std::io::BufRead;

use crate::almanac::Almanac;

pub(crate) fn solve(path: &str) -> u64 {
    let almanac = Almanac::from_lines(read(path).unwrap().lines());
    almanac.seeds.iter().map(|s| almanac.location_for_seed(s)).min().unwrap()
}
