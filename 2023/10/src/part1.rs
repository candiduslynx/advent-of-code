use std::fs::read;
use std::io::BufRead;

pub(crate) fn solve(path: &str) -> usize {
    let lines: Vec<Vec<char>> = read(path)
        .unwrap()
        .lines()
        .map(|s| s.unwrap())
        .filter(|s| !s.is_empty())
        .map(|s| s.chars().collect())
        .collect();

    let s: (usize, usize) = lines
        .iter()
        .enumerate()
        .find_map(|(x, row)| {
            let y = row
                .iter()
                .enumerate()
                .find_map(|(y, c)| if 'S'.eq(c) { Some(y) } else { None });
            if y.is_none() {
                None
            } else {
                Some((x, y.unwrap()))
            }
        })
        .unwrap();
    println!("start at {s:?}");
    lines.len()
}
