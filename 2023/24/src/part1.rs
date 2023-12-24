use crate::hail;

pub(crate) fn solve(path: &str) -> u64 {
    let stones = hail::scan(path);
    stones.len() as u64
}
