use crate::hike;

pub(crate) fn solve(path: &str) -> u64 {
    let (field, start, end) = hike::scan(path);
    hike::dfs(
        &field,
        &mut vec![vec![false; field[0].len()]; field.len()],
        0,
        &end,
        start,
        0,
    )
    .unwrap() as u64
}
