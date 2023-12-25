use crate::hail;

pub(crate) fn solve(path: &str) -> u64 {
    let stones = hail::scan(path);
    hail::intersect_xy(&stones, 200000000000000.0..=400000000000000.0)
}
