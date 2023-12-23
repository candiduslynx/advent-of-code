use crate::hike;
use std::collections::VecDeque;

pub(crate) fn solve(path: &str) -> u64 {
    let (field, start, end) = hike::scan(path);
    // 7500 too high, 5000 too low, 6166 incorrect
    hike::dfs(&field, &mut VecDeque::new(), &end, start, 0) as u64
}
