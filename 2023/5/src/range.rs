#[derive(Debug, Copy, Clone)]
pub(crate) struct Range {
    pub(crate) start: u64,
    pub(crate) end: u64,
}

impl Range {
    pub(crate) fn from_pairs(kv: &Vec<u64>) -> Vec<Range> {
        let starts = kv.iter().step_by(2);
        let ranges = kv.iter().skip(1).step_by(2);
        Range::reduce(starts.zip(ranges).
            map(|(&start, &len)| Range { start, end: start + len - 1 }).collect())
    }

    pub(crate) fn reduce(ranges: Vec<Range>) -> Vec<Range> {
        let mut m: Vec<Range> = ranges.iter().cloned().collect();
        m.sort_by(|a, b| u64::cmp(&a.start, &b.start));

        if m.len() < 2 {
            return m;
        }

        let mut result: Vec<Range> = Vec::new();
        let mut curr: Range = m[0];

        m.iter().for_each(|range| {
            if curr.end < range.start {
                // no intersection, just move curr
                result.push(curr);
                curr = *range;
                return;
            }

            if curr.end < range.end {
                curr.end = range.end;
            }
        });
        result.push(curr);
        result
    }
}