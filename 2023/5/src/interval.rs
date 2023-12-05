#[derive(Debug, Copy, Clone)]
pub(crate) struct Interval {
    pub start: u64,
    pub end: u64,
    pub value: u64,
}

impl Interval {
    fn contains(&self, elem: &u64) -> bool {
        self.start <= *elem && *elem <= self.end
    }

    pub(crate) fn value_for(&self, elem: &u64) -> Option<u64> {
        if self.contains(elem) {
            Some(self.value + (*elem - self.start))
        } else {
            None
        }
    }
    pub(crate) fn from_str(s: &str) -> Option<Self> {
        let values: Vec<u64> = s.split_whitespace().
            filter_map(|s| s.trim().parse().ok()).collect();
        if values.len() < 3 {
            None
        } else {
            Some(Interval {
                start: values[1],
                end: values[1] + values[2] - 1,
                value: values[0],
            })
        }
    }
}

pub(crate) type Mapping = Vec<Interval>;

pub(crate) fn value_for(intervals: &Mapping, elem: &u64) -> u64 {
    intervals.iter().
        find(|i| i.contains(elem)).
        map_or(*elem, |i| i.value_for(elem).unwrap())
}
