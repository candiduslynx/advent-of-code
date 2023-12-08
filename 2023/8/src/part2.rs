use std::fs::read;
use std::io::BufRead;

use crate::node;
use crate::node::Dir;

pub(crate) fn solve(path: &str) -> u64 {
    let mut lines: Vec<String> = read(path)
        .unwrap()
        .lines()
        .map(|s| s.unwrap())
        .filter(|s| !s.is_empty())
        .collect();

    let dirs = lines[0].trim().chars().map(|c| Dir::from_char(c).unwrap()).collect();
    lines.swap_remove(0);
    let nodes = node::to_nodes(lines);

    nodes.keys().filter(|s| s.ends_with("A")).map(|s| {
        node::path(&nodes, &dirs, s, |pos| pos.ends_with("Z"))
    }).fold(1u64, |s, c| lcm(s, c))
}

fn gcd(a: u64, b: u64) -> u64 {
    let (mut a, mut b) = (a, b);
    while b > 0 {
        (a, b) = (b, a % b);
    }

    a
}

fn lcm(a: u64, b: u64) -> u64 {
    a * b / gcd(a, b)
}