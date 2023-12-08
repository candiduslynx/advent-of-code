use std::fs::read;
use std::io::BufRead;
use crate::node;
use crate::node::Dir;

pub(crate) fn solve(path: &str) -> u64 {
    const START: &str = "AAA";
    const FINISH: &str = "ZZZ";

    let mut lines: Vec<String> = read(path)
        .unwrap()
        .lines()
        .map(|s| s.unwrap())
        .filter(|s| !s.is_empty())
        .collect();

    let dirs = lines[0].trim().chars().map(|c| Dir::from_char(c).unwrap()).collect();
    lines.swap_remove(0);
    let nodes = node::to_nodes(lines);
    node::path(&nodes, &dirs, &START.to_string(), |pos|pos.eq(FINISH))
}
