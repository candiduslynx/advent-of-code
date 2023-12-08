use std::fs::read;
use std::io::BufRead;
use crate::node;

pub(crate) fn solve(path: &str) -> u64 {
    const START: &str = "AAA";
    const FINISH: &str = "ZZZ";

    let mut lines: Vec<String> = read(path)
        .unwrap()
        .lines()
        .map(|s| s.unwrap())
        .filter(|s| !s.is_empty())
        .collect();

    let lr = lines[0].trim().to_owned();
    lines.swap_remove(0);

    let nodes = &node::to_nodes(lines);
    let mut pos: &String = &START.to_string();
    lr.chars()
        .cycle()
        .enumerate()
        .find(|(_, next)| {
            pos = node::next(nodes, pos, next);

            pos == FINISH
        })
        .unwrap()
        .0 as u64 + 1
}
