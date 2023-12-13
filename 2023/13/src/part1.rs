use std::fs::read;
use std::io::BufRead;

use crate::solve;

pub(crate) fn solve(path: &str) -> u64 {
    let fields = read(path).unwrap().lines().map(|s| s.unwrap()).fold(
        vec![Vec::<Vec<u8>>::new()],
        |mut state, s| {
            match s.len() {
                0 => state.push(vec![]),
                _ => state.last_mut().unwrap().push(s.as_bytes().to_vec()),
            }
            state
        },
    );

    fields
        .iter()
        .map(|f| solve::patterns(f))
        .map(|(x, y)| (x as u64) * 100 + y as u64)
        .sum()
}
