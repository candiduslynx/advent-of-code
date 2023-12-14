use std::fs::read;
use std::io::BufRead;

use crate::ground;

pub(crate) fn solve(path: &str) -> u64 {
    let mut g: Vec<Vec<u8>> = read(path)
        .unwrap()
        .lines()
        .map(|s| s.unwrap())
        .filter(|s| !s.is_empty())
        .map(|s| s.into_bytes())
        .collect();

    ground::tilt_north(&mut g);
    ground::ground_load(&g)
}
