use std::io::Lines;

use crate::interval;
use crate::interval::{Interval, Mapping};
use crate::range::Range;

pub(crate) struct Almanac {
    pub seeds: Vec<u64>,
    mappings: Vec<Mapping>,
}

impl Almanac {
    pub fn location_for_seed(&self, seed: &u64) -> u64 {
        self.mappings.iter().fold(*seed, |val, next| interval::value_for(next, &val))
    }

    pub fn locations_for_ranges(&self, ranges: Vec<Range>) -> Vec<Range> {
        self.mappings.iter().fold(ranges, |val, next| Almanac::apply_mapping(next, val))
    }

    pub(crate) fn from_lines(mut lines: Lines<&[u8]>) -> Self {
        let seeds: Vec<u64> = lines.next().unwrap().unwrap().
            split_whitespace().filter_map(|s| s.trim().parse().ok()).
            collect();

        Almanac { seeds, mappings: Almanac::mapping(lines) }
    }

    fn mapping(lines: Lines<&[u8]>) -> Vec<Mapping> {
        let mut curr: Mapping = Mapping::new();
        let mut mappings: Vec<Mapping> = Vec::new();
        lines.for_each(|val| {
            let s = val.unwrap().trim().to_owned();
            if s.is_empty() {
                if curr.len() > 0 {
                    mappings.push(curr.iter().copied().collect());
                    curr = Mapping::new();
                }
                return;
            }

            if s.ends_with(" map:") {
                // mapping is done
                curr = Mapping::new();
                return;
            }

            curr.push(Interval::from_str(s.as_str()).unwrap());
        });

        // if no newline at the end
        if curr.len() > 0 {
            mappings.push(curr.iter().copied().collect());
        }

        mappings.iter_mut().
            for_each(|m| m.sort_by(|a, b| u64::cmp(&a.start, &b.start)));
        mappings
    }

    fn apply_mapping(m: &Mapping, ranges: Vec<Range>) -> Vec<Range> {
        Range::reduce(ranges.iter().
            flat_map(|r| Almanac::apply_mapping_to_range(m, r)).collect())
    }

    fn apply_mapping_to_range(m: &Mapping, range: &Range) -> Vec<Range> {
        assert!(range.start <= range.end);
        let mut start = range.start;

        let mut res: Vec<Range> = m.iter().
            skip_while(|m| m.start > range.end).
            flat_map(|m| {
                let mut i: Vec<Range> = Vec::new();
                if start > range.end || m.end < start || range.end < m.start {
                    // we're done, just iterate through
                    return i;
                }

                if start < m.start {
                    // have an idempotent part
                    i.push(Range { start, end: m.start - 1 });
                    start = m.start;
                }

                let end = if range.end > m.end { m.end } else { range.end };
                i.push(Range {
                    start: m.value_for(&start).unwrap(),
                    end: m.value_for(&end).unwrap(),
                });
                start = end + 1;

                i
            }).collect();

        if start <= range.end {
            // have an idempotent tail
            res.push(Range { start, end });
        }
        return Range::reduce(res);
    }
}
