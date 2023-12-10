use std::fs::read;
use std::io::BufRead;

use crate::node::{Dir, Node};

pub(crate) fn solve(path: &str) -> u64 {
    let mut lines: Vec<String> = read(path)
        .unwrap()
        .lines()
        .map(|s| s.unwrap())
        .filter(|s| !s.is_empty())
        .collect();

    let dirs = lines[0]
        .trim()
        .chars()
        .map(|c| Dir::from_char(c).unwrap())
        .collect();
    lines.swap_remove(0);

    let nodes = Node::from_lines(&lines);
    let start = nodes.iter().find(|n| n.label == "AAA").unwrap();

    Node::path_len(start, &dirs, &nodes, |n| n.label.eq("ZZZ"))
}
