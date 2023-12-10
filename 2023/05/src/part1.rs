use std::fs::read;
use std::io::BufRead;

use crate::almanac::Almanac;
use crate::interval::Range;

pub(crate) fn solve(path: &str) -> u64 {
    let almanac = Almanac::from_lines(read(path).unwrap().lines());
    let ranges = almanac
        .seeds
        .iter()
        .map(|s| Range { start: *s, end: *s })
        .collect();
    almanac.locations(ranges)[0].start
}
