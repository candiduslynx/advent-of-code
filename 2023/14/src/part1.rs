use std::fs::read;
use std::io::BufRead;

use crate::ground;
use crate::ground::Ground;

pub(crate) fn solve(path: &str) -> u64 {
    let mut g: Vec<Vec<Ground>> = read(path)
        .unwrap()
        .lines()
        .map(|s| s.unwrap())
        .filter(|s| !s.is_empty())
        .map(|s| Ground::from_str(&s))
        .collect();

    ground::tilt_north(&mut g);
    ground::ground_load(&g)
}
