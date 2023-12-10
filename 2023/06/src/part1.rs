use crate::equation;
use std::fs::read;
use std::io::BufRead;

pub(crate) fn solve(path: &str) -> u64 {
    let lines: Vec<String> = read(path).unwrap().lines().map(|s| s.unwrap()).collect();
    assert_eq!(lines.len(), 2);

    let times: Vec<i64> = lines[0]
        .strip_prefix("Time:")
        .unwrap()
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect();
    let distances: Vec<i64> = lines[1]
        .strip_prefix("Distance:")
        .unwrap()
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect();
    assert_eq!(times.len(), distances.len());

    times
        .iter()
        .zip(distances.iter())
        .map(|(time, distance)| equation::solutions(*time, *distance))
        .product()
}
