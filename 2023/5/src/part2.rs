use std::fs::read;
use std::io::BufRead;

use crate::almanac::Almanac;
use crate::range::Range;

pub(crate) fn solve(path: &str) -> u64 {
    let almanac = Almanac::from_lines(read(path).unwrap().lines());

    let ranges: Vec<Range> = Range::from_pairs(&almanac.seeds);
    let result = almanac.locations_for_ranges(ranges);
    assert!(result.len() > 0);
    result[0].start
}
