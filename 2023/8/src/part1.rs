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

    let nodes = node::to_nodes(lines);
    let mut pos: &str = START;
    lr.chars()
        .cycle()
        .enumerate()
        .find(|(_, next)| {
            let (l,r) = nodes.get(pos).unwrap();
            match next {
                'L' => pos = l,
                'R' => pos = r,
                _ => panic!("unsupported dst {next}"),
            }

            pos == FINISH
        })
        .unwrap()
        .0 as u64 + 1
}
