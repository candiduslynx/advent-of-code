use crate::dig;

pub(crate) fn solve(path: &str) -> u64 {
    dig::solve(path, true).unwrap()
}
