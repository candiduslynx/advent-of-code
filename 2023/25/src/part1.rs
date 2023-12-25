use crate::wires;

pub(crate) fn solve(path: &str) -> u64 {
    let w = wires::scan(path);
    w.len() as u64
}
