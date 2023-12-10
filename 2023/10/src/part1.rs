use lib::point::Point;
use std::fs::read;
use std::io::BufRead;

use crate::r#loop::get_loop;

pub(crate) fn solve(path: &str) -> usize {
    let lines: Vec<Vec<char>> = read(path)
        .unwrap()
        .lines()
        .map(|s| s.unwrap())
        .filter(|s| !s.is_empty())
        .map(|s| s.chars().collect())
        .collect();

    let start = lines
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
                Some(Point {
                    x: x as isize,
                    y: y.unwrap() as isize,
                })
            }
        })
        .unwrap();

    ['|', '-', 'F', '7', 'J', 'L']
        .iter()
        .find_map(|c| get_loop(&lines, start, c))
        .unwrap()
        .len()
        / 2
}
