#[derive(Debug, Copy, Clone)]
pub(crate) struct Interval {
    pub value: u64,
    pub start: u64,
    pub end: u64,
}

pub(crate) type Mapping = Vec<Interval>;

impl Interval {
    pub(crate) fn value_for(&self, elem: u64) -> u64 {
        if self.start <= elem && elem <= self.end {
            self.value + (elem - self.start)
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
            start: values[1],
            end: values[1] + values[2] - 1,
        }
    }
}
