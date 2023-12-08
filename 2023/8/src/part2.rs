use std::fs::read;
use std::io::BufRead;

use crate::node;

pub(crate) fn solve(path: &str) -> u64 {
    let mut lines: Vec<String> = read(path)
        .unwrap()
        .lines()
        .map(|s| s.unwrap())
        .filter(|s| !s.is_empty())
        .collect();

    let lr = lines[0].trim().to_owned();
    lines.swap_remove(0);

    let nodes = &node::to_nodes(lines);

    let mut pos: Vec<&String> = nodes.keys().filter(|s| s.ends_with("A")).collect();
    lr.chars()
        .cycle()
        .enumerate()
        .find(|(_, next)| {
            node::vec_next(nodes, &mut pos, next);
            pos.iter().filter(|&&s| !s.ends_with("Z")).count() == 0
        })
        .unwrap()
        .0 as u64 + 1
}
