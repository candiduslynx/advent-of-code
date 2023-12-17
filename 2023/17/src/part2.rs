use crate::city;

pub(crate) fn solve(path: &str) -> u64 {
    let c = city::scan(path);
    c.len() as u64
}
