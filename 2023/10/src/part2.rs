use std::fs::read;
use std::io::BufRead;

use lib::point::Point;

use crate::r#loop::get_loop;

pub(crate) fn solve(path: &str) -> u32 {
    let mut lines: Vec<Vec<char>> = read(path)
        .unwrap()
        .lines()
        .map(|s| s.unwrap())
        .filter(|s| !s.is_empty())
        .map(|s| s.chars().collect())
        .collect();

    let start: (usize, usize) = lines
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

    let (proper, cycle) = ['|', '-', 'F', '7', 'J', 'L']
        .into_iter()
        .map(|c| (c, get_loop(&lines, &start, c)))
        .find(|(_, c)| c.is_some())
        .unwrap();
    let cycle = cycle.unwrap();

    // update the char
    lines[start.0][start.1] = proper;

    lines
        .iter()
        .enumerate()
        .map(|(x, row)| {
            let mut inside = false;
            let mut sum = 0u32;
            let mut h_start: Option<char> = None;
            for y in 0..row.len() {
                if cycle.contains(&Point {
                    x: x as isize,
                    y: y as isize,
                }) {
                    let curr = lines[x][y];
                    match curr {
                        '|' => inside = !inside,
                        'F' | 'L' => h_start = Some(curr),
                        '7' => match h_start {
                            Some('F') => {}
                            Some('L') => inside = !inside,
                            _ => panic!("end {curr} found, but start is {h_start:?}"),
                        },
                        'J' => match h_start {
                            Some('F') => inside = !inside,
                            Some('L') => {}
                            _ => panic!("end {curr} found, but start is {h_start:?}"),
                        },
                        _ => {}
                    }
                } else if inside {
                    sum += 1
                }
            }
            sum
        })
        .sum()
}
