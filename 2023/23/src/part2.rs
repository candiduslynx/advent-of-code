use crate::hike;

pub(crate) fn solve(path: &str) -> u64 {
    let (field, start, end) = hike::scan(path);

    hike::longest(&field, &start, &end, false) as u64
}
