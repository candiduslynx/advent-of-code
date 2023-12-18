use crate::city;
use crate::city::{Limits, State};

pub(crate) fn solve(path: &str) -> u64 {
    city::solve(path, next_moves)
}

fn next_moves(c: &Vec<Vec<u8>>, from: &State, limits: &Limits) -> Vec<(State, u64)> {
    city::walk(c, from, from.dir.turn_counterclockwise(), limits, 3, 0)
        .into_iter()
        .chain(city::walk(c, from, from.dir.turn_clockwise(), limits, 3, 0).into_iter())
        .collect()
}
