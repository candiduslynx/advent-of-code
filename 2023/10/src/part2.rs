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
                    x: x as i64,
                    y: y.unwrap() as i64,
                })
            }
        })
        .unwrap();

    let (proper, cycle) = ['|', '-', 'F', '7', 'J', 'L']
        .iter()
        .map(|c| (c, get_loop(&lines, start, c)))
        .find(|(_, c)| c.is_some())
        .unwrap();
    let cycle = cycle.unwrap();

    // update the char
    lines[start.x as usize][start.y as usize] = *proper;

    lines
        .iter()
        .enumerate()
        .map(|(x, row)| {
            let mut inside = false;
            let mut sum = 0u32;
            let mut h_start = &'?';
            for y in 0..row.len() {
                if cycle.contains(&Point {
                    x: x as i64,
                    y: y as i64,
                }) {
                    match lines[x][y] {
                        '|' => inside = !inside,
                        'F' | 'L' => h_start = &lines[x][y],
                        '7' => match h_start {
                            'L' => inside = !inside,
                            _ => {}
                        },
                        'J' => match h_start {
                            'F' => inside = !inside,
                            _ => {}
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
