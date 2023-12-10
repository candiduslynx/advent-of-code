#[derive(Debug, Copy, Clone)]
pub(crate) struct Interval {
    pub value: u64,
    pub range: Range,
}

pub(crate) type Mapping = Vec<Interval>;

impl Interval {
    pub(crate) fn value_for(&self, elem: u64) -> u64 {
        if self.range.contains(elem) {
            self.value + (elem - self.range.start)
        } else {
            elem
        }
    }

    pub(crate) fn from_str(s: &str) -> Self {
        let values: Vec<u64> = s.split_whitespace().
            filter_map(|s| s.trim().parse().ok()).collect();
        assert_eq!(values.len(), 3);
        Interval {
            value: values[0],
            range: Range {
                start: values[1],
                end: values[1] + values[2] - 1,
            },
        }
    }
}

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
        if ranges.len() < 2 {
            return ranges;
        }

        let mut m: Vec<Range> = ranges.iter().cloned().collect();
        m.sort_by(|a, b| u64::cmp(&a.start, &b.start));

        m.iter().fold(vec![m[0]], |mut result, curr| {
            // to safely access the last elem we start from non-empty vector
            let last = result.last_mut().unwrap();
            if last.end < curr.start {
                // no intersection, just push
                result.push(*curr);
                return result;
            }

            if last.end < curr.end {
                last.end = curr.end;
            }
            result
        })
    }

    pub(crate) fn contains(&self, elem: u64) -> bool {
        self.start <= elem && elem <= self.end
    }
}
