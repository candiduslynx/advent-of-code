use std::fs::read;
use std::io::BufRead;

use crate::almanac::Almanac;
use crate::range::Range;

pub(crate) fn solve(path: &str) -> u64 {
    let almanac = Almanac::from_lines(read(path).unwrap().lines());
    almanac.locations_for_ranges(Range::from_pairs(&almanac.seeds))[0].start
}
