use std::fs::read;
use std::io::BufRead;

use crate::equation;

pub(crate) fn solve(path: &str) -> u64 {
    let lines: Vec<String> = read(path).unwrap().lines().map(|s| s.unwrap()).collect();
    assert_eq!(lines.len(), 2);

    let time: i64 = lines[0].strip_prefix("Time:").unwrap().replace(" ", "").trim().parse().unwrap();
    let distance: i64 = lines[1].strip_prefix("Distance:").unwrap().replace(" ", "").trim().parse().unwrap();

    equation::solutions(time, distance)
}
